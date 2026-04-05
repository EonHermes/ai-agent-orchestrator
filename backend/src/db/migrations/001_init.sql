-- Create agents table
CREATE TABLE IF NOT EXISTS agents (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    endpoint_url TEXT NOT NULL,
    capabilities JSON NOT NULL,           -- Array of capability strings
    status TEXT NOT NULL CHECK(status IN ('active', 'inactive', 'error')),
    metadata JSON,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_agents_status ON agents(status);
CREATE INDEX IF NOT EXISTS idx_agents_updated ON agents(updated_at DESC);

-- Create tasks table
CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    user_query TEXT NOT NULL,
    parsed_plan JSON,
    status TEXT NOT NULL CHECK(status IN ('pending', 'dispatched', 'completed', 'failed', 'cancelled')),
    error_message TEXT,
    created_at TIMESTAMP NOT NULL,
    started_at TIMESTAMP,
    completed_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_created ON tasks(created_at DESC);

-- Create sub_tasks table
CREATE TABLE IF NOT EXISTS sub_tasks (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    capability TEXT NOT NULL,
    input JSON,
    output JSON,
    error TEXT,
    status TEXT NOT NULL CHECK(status IN ('pending', 'running', 'completed', 'failed')),
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    latency_ms INTEGER,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (agent_id) REFERENCES agents(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_sub_tasks_task ON sub_tasks(task_id);
CREATE INDEX IF NOT EXISTS idx_sub_tasks_agent ON sub_tasks(agent_id);
CREATE INDEX IF NOT EXISTS idx_sub_tasks_status ON sub_tasks(status);

-- Create executions table (audit log)
CREATE TABLE IF NOT EXISTS executions (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL,
    agent_id TEXT,
    step INTEGER NOT NULL,
    action TEXT NOT NULL,
    input_snapshot JSON,
    output_snapshot JSON,
    latency_ms INTEGER,
    success BOOLEAN NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (agent_id) REFERENCES agents(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_executions_task ON executions(task_id);
CREATE INDEX IF NOT EXISTS idx_executions_agent ON executions(agent_id);
CREATE INDEX IF NOT EXISTS idx_executions_timestamp ON executions(timestamp DESC);

-- Create agent_performance table (learning data)
CREATE TABLE IF NOT EXISTS agent_performance (
    id TEXT PRIMARY KEY,
    agent_id TEXT NOT NULL,
    capability TEXT NOT NULL,
    success_count INTEGER DEFAULT 0,
    failure_count INTEGER DEFAULT 0,
    avg_latency_ms REAL,
    last_updated TIMESTAMP NOT NULL,
    FOREIGN KEY (agent_id) REFERENCES agents(id) ON DELETE CASCADE,
    UNIQUE(agent_id, capability)
);

CREATE INDEX IF NOT EXISTS idx_performance_agent ON agent_performance(agent_id);

-- Triggers for updating agent_performance
CREATE TRIGGER IF NOT EXISTS update_performance_on_subtask_complete
AFTER UPDATE OF status ON sub_tasks
WHEN NEW.status IN ('completed', 'failed') AND OLD.status IN ('pending', 'running')
BEGIN
    INSERT INTO agent_performance (id, agent_id, capability, success_count, failure_count, avg_latency_ms, last_updated)
    VALUES (
        lower(hex(randomblob(16))),
        NEW.agent_id,
        NEW.capability,
        CASE WHEN NEW.status = 'completed' THEN 1 ELSE 0 END,
        CASE WHEN NEW.status = 'failed' THEN 1 ELSE 0 END,
        NEW.latency_ms,
        CURRENT_TIMESTAMP
    )
    ON CONFLICT(agent_id, capability) DO UPDATE SET
        success_count = success_count + CASE WHEN NEW.status = 'completed' THEN 1 ELSE 0 END,
        failure_count = failure_count + CASE WHEN NEW.status = 'failed' THEN 1 ELSE 0 END,
        avg_latency_ms = (
            (avg_latency_ms * (success_count + failure_count) + NEW.latency_ms) / 
            (success_count + failure_count + 1)
        ),
        last_updated = CURRENT_TIMESTAMP
    WHERE agent_performance.agent_id = NEW.agent_id 
      AND agent_performance.capability = NEW.capability;
END;

-- +goose Up
CREATE TABLE IF NOT EXISTS agents (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    endpoint_url TEXT NOT NULL,
    capabilities TEXT NOT NULL,  -- JSON array
    status TEXT NOT NULL CHECK(status IN ('active', 'inactive', 'error')),
    metadata TEXT,  -- JSON object
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    user_query TEXT NOT NULL,
    parsed_plan TEXT,  -- JSON array of steps
    status TEXT NOT NULL CHECK(status IN ('pending', 'dispatched', 'completed', 'failed', 'cancelled')),
    error_message TEXT,
    created_at TIMESTAMP NOT NULL,
    started_at TIMESTAMP,
    completed_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sub_tasks (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    capability TEXT NOT NULL,
    input TEXT,  -- JSON object
    output TEXT,  -- JSON object
    error TEXT,
    status TEXT NOT NULL CHECK(status IN ('pending', 'running', 'completed', 'failed')),
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    latency_ms INTEGER,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (agent_id) REFERENCES agents(id)
);

CREATE TABLE IF NOT EXISTS executions (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL,
    agent_id TEXT,
    step INTEGER NOT NULL,
    action TEXT NOT NULL,
    input_snapshot TEXT,  -- JSON object
    output_snapshot TEXT,  -- JSON object
    latency_ms INTEGER,
    success BOOLEAN NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (agent_id) REFERENCES agents(id)
);

CREATE TABLE IF NOT EXISTS agent_performance (
    id TEXT PRIMARY KEY,
    agent_id TEXT NOT NULL,
    capability TEXT NOT NULL,
    success_count INTEGER NOT NULL DEFAULT 0,
    failure_count INTEGER NOT NULL DEFAULT 0,
    avg_latency_ms REAL,
    last_updated TIMESTAMP NOT NULL,
    FOREIGN KEY (agent_id) REFERENCES agents(id),
    UNIQUE(agent_id, capability)
);

CREATE INDEX IF NOT EXISTS idx_agents_status ON agents(status);
CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_sub_tasks_task_id ON sub_tasks(task_id);
CREATE INDEX IF NOT EXISTS idx_sub_tasks_agent_id ON sub_tasks(agent_id);
CREATE INDEX IF NOT EXISTS idx_executions_task_id ON executions(task_id);
CREATE INDEX IF NOT EXISTS idx_executions_timestamp ON executions(timestamp);
CREATE INDEX IF NOT EXISTS idx_agent_performance_agent ON agent_performance(agent_id);

-- +goose Down
DROP TABLE IF EXISTS executions;
DROP TABLE IF EXISTS sub_tasks;
DROP TABLE IF EXISTS tasks;
DROP TABLE IF EXISTS agent_performance;
DROP TABLE IF EXISTS agents;
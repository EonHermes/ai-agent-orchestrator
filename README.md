# AI Agent Orchestrator (EON-026)

**Production-ready meta-system that unifies all AI capabilities into a collaborative multi-agent framework**

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange)](https://www.rust-lang.org)
[![React](https://img.shields.io/badge/React-18.2-blue)](https://reactjs.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0-blue)](https://www.typescriptlang.org)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

## Overview

The AI Agent Orchestrator is a meta-system that unifies all your AI capabilities (Intelligent Workflow Assistant, Code Review Assistant, ML Registry, Analytics Aggregator, etc.) into a cohesive, intelligent assistant platform. It provides:

- **Natural Language Task Decomposition**: Parse complex, multi-step user requests into structured execution plans
- **Intelligent Agent Selection**: Automatically match sub-tasks to the most appropriate registered agents based on capabilities
- **Cross-Service Coordination**: Execute tasks across multiple AI agents and aggregate results
- **Learning System**: Track performance metrics to optimize agent selection over time
- **Unified Dashboard**: Visualize system status, agent health, task execution, and analytics

This project completes Daniel's AI ecosystem by making all previous projects (EON-015 through EON-025) work together seamlessly.

---

## ✨ Features

### Core Capabilities

- **Natural Language Parser**: LLM-powered (OpenRouter) task decomposition with structured JSON output
- **Agent Registry**: CRUD operations with capability-based discovery, auto-suggestion from descriptions
- **Intelligent Dispatcher**: Capability matching with future performance-based selection
- **Execution Engine**: Parallel sub-task execution with timeout, error handling, retry logic
- **Result Aggregator**: LLM-powered synthesis of multi-agent outputs into coherent responses
- **Learning System**: Success rate tracking, latency metrics, capability-based performance stats

### API Features

- **20+ REST endpoints** covering agents, tasks, executions, and system status
- **Health checks** with database connectivity monitoring
- **Comprehensive filtering and pagination**
- **Real-time execution monitoring** via WebSocket-ready polling
- **Audit logging** with full execution traces

### Dashboard Features

- **System Status Overview**: Agent count, active tasks, success rates, latency metrics
- **Agent Management**: Register, update, delete agents; view capabilities; performance stats per agent
- **Task Submission**: Natural language input with plan preview before execution
- **Task Monitor**: Real-time status updates, sub-task progress, error visibility
- **Execution Viewer**: Detailed logs including input/output snapshots, latencies, success/failure
- **Analytics**: Agent performance charts, system health metrics

---

## 🚀 Quick Start

### Prerequisites

- **Docker & Docker Compose** (recommended)
  - OR: Rust 1.75+, Node.js 18+, SQLite3

- **OpenRouter API Key**: Get one from [openrouter.ai](https://openrouter.ai)

### One-Command Docker Deployment

```bash
# Clone the repository
git clone https://github.com/EonHermes/ai-agent-orchestrator.git
cd ai-agent-orchestrator

# Set your OpenRouter API key
export OPENROUTER_API_KEY="your-api-key-here"

# Deploy with Docker Compose
docker-compose up -d

# Access the application:
# - Frontend Dashboard: http://localhost:3000
# - Backend API: http://localhost:8081/api/v1
# - Health Check: http://localhost:8081/health
```

That's it! The backend and frontend are both running with Nginx reverse proxy.

---

## 📚 API Documentation

### Base URL

- Development: `http://localhost:8081/api/v1`
- Production: Configure via environment variables

### Authentication

Currently open (no auth). Add API key middleware in production.

### Endpoints

#### System

```
GET /health              - Health check (database connectivity)
GET /api/v1/status       - System status summary (agents, tasks, stats)
```

#### Agents

```
GET    /api/v1/agents               - List all agents (query: ?status=active)
POST   /api/v1/agents               - Register new agent (capabilities auto-suggested if empty)
GET    /api/v1/agents/:id           - Get agent details
PUT    /api/v1/agents/:id           - Update agent (partial update)
DELETE /api/v1/agents/:id           - Delete agent
GET    /api/v1/agents/capabilities  - List all unique capabilities across agents
GET    /api/v1/agents/:id/stats     - Get performance stats for agent
```

**Create Agent Request**:
```json
{
  "name": "Workflow Assistant",
  "description": "AI-powered workflow optimization",
  "endpoint_url": "http://workflow-assistant:8081",
  "capabilities": ["workflow_optimization", "bottleneck_detection"],
  "metadata": {"project": "EON-025"}
}
```

**Note**: If `capabilities` array is empty, the system uses OpenRouter to suggest capabilities based on `description`.

#### Tasks

```
POST /api/v1/tasks                - Create and execute task (async)
GET  /api/v1/tasks               - List tasks (query: ?status=pending&limit=50)
GET  /api/v1/tasks/:id           - Get task with sub-tasks
POST /api/v1/tasks/:id/cancel    - Cancel running task
GET  /api/v1/tasks/:id/plan      - Get execution plan (without executing)
POST /api/v1/parse               - Parse natural language to plan (dry run)
```

**Create Task Request**:
```json
{
  "user_query": "Analyze workflow performance and suggest optimizations"
}
```

**Response**:
```json
{
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "pending"
}
```

#### Executions

```
GET  /api/v1/executions                  - List executions (query: ?limit=100)
GET  /api/v1/executions/:id              - Get single execution details
GET  /api/v1/executions/stats            - Aggregate stats per agent
```

#### Agent Endpoint (for receiving tasks)

```
POST /agent/execute                      - Endpoint agents must implement to receive sub-tasks
```

**Agent Execution Request**:
```json
{
  "task_id": "task-uuid",
  "sub_task_id": "sub-task-uuid",
  "capability": "workflow_optimization",
  "input": {
    "query": "original user query",
    "step_description": "Analyze bottlenecks",
    "capability": "workflow_optimization",
    "step_index": 0,
    "total_steps": 3
  }
}
```

**Agent Execution Response**:
```json
{
  "output": { /* agent-specific JSON */ },
  "success": true,
  "message": "Completed"
}
```

---

## 🎛️ Dashboard Guide

### Registering an Agent

1. Click **"Register Agent"** button
2. Fill in:
   - **Name**: Human-readable name (e.g., "ML Registry")
   - **Endpoint URL**: Where the agent's API is reachable (e.g., `http://ml-registry:8080`)
   - **Description**: What the agent does (used for AI capability suggestion if left blank)
   - **Capabilities**: List of capability strings this agent provides (e.g., `["model_registration", "experiment_tracking"]`)
3. Click **Create Agent**

The agent will appear in the Agents list. Admins can view stats and delete agents.

### Submitting a Task

1. Navigate to **Tasks** tab
2. Type a natural language description in the textarea
3. Click **"Preview Plan"** to see how the system will decompose your request (no execution)
4. Click **"Execute"** to submit the task

The system will:
- Parse your query into a step-by-step plan
- Find agents for each step based on capabilities
- Execute sub-tasks in parallel (respecting concurrency limits)
- Aggregate results into a final response
- Update the task status in real-time

### Monitoring Executions

- **Dashboard tab**: System overview, agent health, recent executions
- **Executions tab**: Full execution log with input/output snapshots, latencies, success/failure indicators
- **Task details**: Click the 👁️ icon next to any task to see sub-task progress and outputs

---

## 🔌 Integrating Existing Agents

The Orchestrator is designed to work with **any existing AI service** (including all your EON projects). To integrate:

### 1. Implement `/agent/execute` endpoint

Your agent needs to respond to POST requests at `/agent/execute` with JSON body:

```json
{
  "task_id": "...",
  "sub_task_id": "...",
  "capability": "your_capability_name",
  "input": {
    "query": "original user query",
    "step_description": "...",
    "capability": "...",
    ...
  }
}
```

Your agent should:
- Perform the requested capability
- Return `{ "output": {...}, "success": true }` on success
- Return `{ "success": false, "error": "message" }` on failure
- Respond within 120 seconds (or the orchestrator will timeout)

### 2. Register the agent via API or Dashboard

Ensure your agent's Docker service is on the same network as the orchestrator (Docker Compose handles this).

### Example: Workflow Assistant Integration

```bash
curl -X POST http://localhost:8081/api/v1/agents \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Workflow Assistant",
    "description": "Analyzes execution patterns and suggests optimizations",
    "endpoint_url": "http://workflow-assistant:8081",
    "capabilities": [
      "workflow_optimization",
      "bottleneck_detection",
      "performance_prediction"
    ]
  }'
```

Once registered, tasks containing these keywords will be automatically dispatched to this agent.

---

## ⚙️ Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `OPENROUTER_API_KEY` | **Required** | OpenRouter API key for LLM parsing/aggregation |
| `OPENROUTER_MODEL` | `anthropic/claude-3-opus` | Model to use (claude, gpt-4, etc. via OpenRouter) |
| `PORT` | `8081` | Backend HTTP server port |
| `DATABASE_URL` | `sqlite:/data/orchestrator.db` | SQLite database path (persist via volume) |
| `MAX_CONCURRENT_TASKS` | `10` | Maximum parallel sub-tasks executing |
| `TASK_TIMEOUT_SECONDS` | `300` | Total task timeout (5 min) |
| `ENABLE_LEARNING` | `true` | Track performance metrics |
| `RUST_LOG` | `info` | Logging level (debug, info, warn, error) |

### Docker Compose Configuration

Edit `docker-compose.yml` to:
- Change port mappings
- Add persistent volumes
- Configure network settings
- Add resource limits (CPU/Memory)

---

## 🗄️ Database Schema

The orchestrator uses SQLite with 5 tables:

- **agents**: Registered AI agents (id, name, endpoint, capabilities JSON, status)
- **tasks**: User tasks with parsing results and status
- **sub_tasks**: Individual agent invocations (linked to task)
- **executions**: Audit log of all system actions (for debugging)
- **agent_performance**: Aggregated success rates and latencies per capability

Database is persisted to `./data/orchestrator.db` (Docker) or configured path.

---

## 🏗️ Architecture

```
┌─────────────────┐
│   Frontend      │  React + TypeScript + Tailwind
│   (Port 3000)   │  Dashboard, Task UI, Agent Management
└────────┬────────┘
         │ HTTP/REST
         ▼
┌─────────────────────────────────────────────┐
│          Backend (Axum)                     │
│  ┌────────────────────────────────────────┐ │
│  │  Handlers (REST API)                   │ │
│  │  - Agents / Tasks / Executions         │ │
│  └──────────────┬─────────────────────────┘ │
│                 │                          │
│  ┌──────────────▼─────────────────────────┐ │
│  │  TaskService (Orchestration Core)      │ │
│  │  - Parse natural language with LLM     │ │
│  │  - Create execution plan               │ │
│  │  - Dispatch to agents (parallel)       │ │
│  │  - Aggregate results                   │ │
│  └──────────────┬─────────────────────────┘ │
│                 │                          │
│  ┌──────────────▼─────────────────────────┐ │
│  │  AgentService                          │ │
│  │  - Capability matching                 │ │
│  │  - Performance tracking                │ │
│  └──────────────┬─────────────────────────┘ │
│                 │                          │
│  ┌──────────────▼─────────────────────────┐ │
│  │  LLMService (OpenRouter)               │ │
│  │  - Parse tasks → plan                  │ │
│  │  - Aggregate multi-agent results       │ │
│  └────────────────────────────────────────┘ │
└─────────────────┬───────────────────────────┘
                  │
         ┌────────▼────────┐
         │   SQLite DB     │  Agents, Tasks, Executions, Performance
         └─────────────────┘
```

### Data Flow

1. User submits query → `POST /api/v1/tasks`
2. TaskService calls LLMService.parse_task() → execution plan (capability steps)
3. For each step: AgentService finds agent(s) with matching capability
4. TaskService dispatches sub-tasks to agent endpoints in parallel
5. Agents process and return results
6. TaskService calls LLMService.aggregate_results() to synthesize final output
7. Task marked completed; performance metrics updated

---

## 🧪 Testing

### Backend

```bash
cd backend
cargo test
```

### Frontend

```bash
cd frontend
npm test
```

### Integration Test

```bash
# With Docker running
curl -X POST http://localhost:8081/api/v1/tasks \
  -H "Content-Type: application/json" \
  -d '{"user_query": "Analyze system health"}'
```

---

## 🔧 Development

### Local Development (no Docker)

**Backend**:
```bash
cd backend
# Install SQLite and set OPENROUTER_API_KEY env var
cargo run
# Server runs on http://localhost:8081
```

**Frontend**:
```bash
cd frontend
npm install
npm run dev
# Frontend runs on http://localhost:3000 (proxied to backend:8081)
```

### Database Migrations

Migrations are in `backend/migrations/`. They run automatically on startup via `sqlx::migrate!()`.

To create a new migration:
```sql
-- backend/migrations/000002_add_xxx.sql
-- +goose Up
CREATE TABLE xxx (...);
-- +goose Down
DROP TABLE xxx;
```

---

## 📊 Performance Considerations

- **SQLite WAL mode** enabled for concurrent reads
- **Connection pooling** via `SqlitePool`
- **Async HTTP** with connection reuse for agent calls
- **Bounded concurrency** (configurable via `MAX_CONCURRENT_TASKS`)
- **Circuit breaker pattern** ready for agent endpoint failures
- **Response caching** for static data (agents list)

---

## 🚧 Future Enhancements

- **WebSocket** for real-time task updates (currently polling via dashboard)
- **Agent auto-discovery** from service mesh (Consul, etc.)
- **Priority-based scheduling** (Urgent vs Best-Effort task queues)
- **Advanced planning**: conditional branches, loops, error recovery workflows
- **Multi-LLM fallback**: different models for different task types (code vs natural language)
- **Distributed execution** across network clusters
- **Authentication & rate limiting** middleware
- **Metrics export** to Analytics Aggregator (self-monitoring ecosystem)
- **A/B testing** framework for planning algorithms
- **Web UI for agent endpoint testing** (test agents directly from dashboard)

---

## 📝 License

MIT License - see LICENSE file for details.

---

## 🙏 Acknowledgments

Built by Eon for Daniel Lindestad as part of the EON project series.
Part of the OpenClaw automation ecosystem.

---

## 📚 Related Projects

- **EON-024**: Automation Workflow Orchestrator (visual workflow designer)
- **EON-025**: Intelligent Workflow Assistant (AI for workflow optimization)
- **EON-022**: Analytics Aggregator (unified metrics dashboard)
- **EON-023**: ML Model Registry & Experiment Tracker
- **EON-016**: AI-Powered Code Review Assistant

All are designed to integrate seamlessly with this orchestrator.
# 🤖 AI Agent Orchestrator (EON-026)

> A production-ready meta-system that unifies all AI capabilities into a collaborative multi-agent framework

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)](https://www.rust-lang.org)
[![React](https://img.shields.io/badge/React-18.2-blue?logo=react)](https://reactjs.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.2-blue?logo=typescript)](https://www.typescriptlang.org)
[![Docker](https://img.shields.io/badge/Docker-Ready-blue?logo=docker)](https://docker.com)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

## ✨ Features

- **Natural Language Task Processing**: Parse complex user queries into structured action plans
- **Intelligent Agent Routing**: Automatically match tasks to agents based on capabilities
- **Multi-Agent Collaboration**: Coordinate multiple AI agents to complete sophisticated workflows
- **Real-time Monitoring**: Dashboard with live task status, agent health, and execution metrics
- **Advanced Planning**: LLM-powered task decomposition with dependency tracking
- **Comprehensive Audit Logging**: Full execution trail with performance metrics
- **RESTful API**: Complete CRUD operations for agents, tasks, and executions
- **Production Ready**: Docker Compose deployment, SQLite with WAL mode, comprehensive error handling

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Frontend (React)                        │
│  Dashboard • Agent Registry • Task Monitor • Analytics     │
└───────────────────────────┬─────────────────────────────────┘
                            │ HTTPS/WS
┌───────────────────────────▼─────────────────────────────────┐
│                    Nginx Reverse Proxy                      │
│              (Load Balancing • Static Files)                │
└───────────────────────────┬─────────────────────────────────┘
                            │
            ┌───────────────▼───────────────┐
            │   AI Agent Orchestrator API    │
            │  ┌──────────────────────────┐ │
            │  │ Task Parser (LLM)        │ │ → Parse natural language
            │  ├──────────────────────────┤ │
            │  │ Execution Planner        │ │ → Create step-by-step plan
            │  ├──────────────────────────┤ │
            │  │ Agent Dispatcher         │ │ → Route to appropriate agents
            │  ├──────────────────────────┤ │
            │  │ Result Aggregator        │ │ → Combine responses
            │  └──────────────────────────┘ │
            └─────────────────────────────────┘
                            │
            ┌───────────────┼───────────────┐
            │               │               │
    ┌───────▼──────┐  ┌────▼─────┐  ┌─────▼──────┐
    │ ML Service   │  │ Code AI  │  │ Analytics  │
    │ Agent (EON-025)│  │ Assistant│  │ Aggregator │
    └──────────────┘  └──────────┘  └────────────┘
```

## 🚀 Quick Start

### Prerequisites

- Docker & Docker Compose (latest)
- Node.js 18+ (for local development)
- Rust 1.75+ (for backend development)
- OpenRouter API key (get one at [openrouter.ai](https://openrouter.ai))

### One-Command Deployment

```bash
# Clone the repository
git clone https://github.com/EONHermes/ai-agent-orchestrator.git
cd ai-agent-orchestrator

# Set your OpenRouter API key
export OPENROUTER_API_KEY="your-openrouter-api-key"

# Deploy with Docker Compose
docker-compose up -d

# Access the application
# Frontend Dashboard: http://localhost:3000
# Backend API: http://localhost:8081
# API Health: http://localhost:8081/health
```

## 📚 API Documentation

### Base URL
```
http://localhost:8081/api/v1
```

### Authentication
Currently, the API is open. For production deployments, add an API gateway (Traefik/Nginx) with JWT validation.

### Endpoints

#### Agents
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/agents` | List all registered agents |
| POST | `/agents` | Register a new agent |
| GET | `/agents/:id` | Get agent details |
| PUT | `/agents/:id` | Update agent status/metadata |
| DELETE | `/agents/:id` | Unregister an agent |
| GET | `/agents/capabilities` | List all unique capabilities |

#### Tasks
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/tasks/submit` | Submit a natural language task (async) |
| POST | `/tasks` | Create a task manually |
| GET | `/tasks` | List tasks with filters |
| GET | `/tasks/:id` | Get task details + sub-tasks |
| POST | `/tasks/:id/cancel` | Cancel a running task |

#### System
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| GET | `/status` | System status summary |
| POST | `/parse` | Parse natural language without executing |

#### Executions
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/executions` | List sub-task executions |
| GET | `/executions/stats` | Aggregate performance metrics |

### Example API Calls

```bash
# Register an agent
curl -X POST http://localhost:8081/api/v1/agents \
  -H "Content-Type: application/json" \
  -d '{
    "name": "ML Registry",
    "description": "Queries model training runs and metrics",
    "endpoint_url": "http://ml-registry:8080",
    "capabilities": ["query_model_runs", "get_metrics", "list_models"],
    "metadata": {"project": "EON-025"}
  }'

# Submit a task
curl -X POST http://localhost:8081/api/v1/tasks/submit \
  -H "Content-Type: application/json" \
  -d '{
    "user_query": "Analyze the performance of all ResNet models trained in March and generate a PDF report"
  }'

# Check task status
curl http://localhost:8081/api/v1/tasks/:task_id

# Get system health
curl http://localhost:8081/health
```

## 🔧 Configuration

### Environment Variables

The backend supports configuration via environment variables (prefixed with `ORCHESTRATOR_`):

| Variable | Default | Description |
|----------|---------|-------------|
| `ORCHESTRATOR_SERVER_HOST` | `0.0.0.0` | Server bind address |
| `ORCHESTRATOR_SERVER_PORT` | `8081` | Server port |
| `ORCHESTRATOR_MAX_CONCURRENT_TASKS` | `10` | Max parallel task execution |
| `ORCHESTRATOR_TASK_TIMEOUT_SECONDS` | `300` | Task timeout (5 min) |
| `ORCHESTRATOR_DATABASE_URL` | `sqlite:/data/orchestrator.db` | SQLite database path |
| `ORCHESTRATOR_LLM_OPENROUTER_API_KEY` | (required) | OpenRouter API key |
| `ORCHESTRATOR_LLM_OPENROUTER_MODEL` | `anthropic/claude-3-opus` | LLM model for planning |
| `ORCHESTRATOR_CORS_ALLOWED_ORIGINS` | `http://localhost:3000` | CORS allowed origins |
| `ORCHESTRATOR_LOGGING_LEVEL` | `info` | Log level: debug/info/warn/error |
| `RUST_LOG` | `info` | Rust logging (augments above) |

### Production Deployment

1. **Using Docker Compose** (recommended):
```bash
# Set production environment variables
export OPENROUTER_API_KEY="your-key"
export ORCHESTRATOR_LOGGING_LEVEL="warn"

# Use production profile
docker-compose --profile production up -d
```

2. **Security Hardening**:
   - Add NGINX rate limiting
   - Configure TLS with Let's Encrypt
   - Set up firewall rules (only expose ports 80/443)
   - Use strong database passwords for Postgres migration
   - Enable JWT authentication middleware

3. **Scaling**:
```bash
# Scale backend instances behind Nginx
docker-compose up -d --scale backend=3
```

## 🛠️ Development

### Local Setup

#### Backend

```bash
cd backend
cargo check
cargo test
cargo run

# With environment variables
OPENROUTER_API_KEY=your_key cargo run
```

#### Frontend

```bash
cd frontend
npm install
npm run dev

# Run tests
npm test

# Build for production
npm run build
```

### Database Migrations

Migrations are auto-applied on startup. For manual execution:

```bash
# View migration files
ls backend/src/db/migrations/

# The system uses SQLite with WAL mode enabled for better concurrency.
# Journal mode: WAL
# Synchronous: NORMAL
# Cache size: 1000 pages
```

## 📊 Database Schema

### Core Tables

**agents**: Registered AI agents
```sql
id TEXT PRIMARY KEY,
name TEXT NOT NULL,
description TEXT,
endpoint_url TEXT NOT NULL,
capabilities JSON NOT NULL,      -- ["capability1", "capability2"]
status TEXT CHECK(status IN ('active', 'inactive', 'error')),
metadata JSON,
created_at TIMESTAMP,
updated_at TIMESTAMP
```

**tasks**: User-submitted tasks
```sql
id TEXT PRIMARY KEY,
user_query TEXT NOT NULL,
parsed_plan JSON,                -- Execution plan
status TEXT CHECK(status IN ('pending', 'dispatched', 'completed', 'failed', 'cancelled')),
error_message TEXT,
created_at TIMESTAMP,
started_at TIMESTAMP,
completed_at TIMESTAMP
```

**sub_tasks**: Individual agent executions
```sql
id TEXT PRIMARY KEY,
task_id TEXT NOT NULL,
agent_id TEXT NOT NULL,
capability TEXT NOT NULL,
input JSON,
output JSON,
error TEXT,
status TEXT CHECK(status IN ('pending', 'running', 'completed', 'failed')),
started_at TIMESTAMP,
completed_at TIMESTAMP,
latency_ms INTEGER,
FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
FOREIGN KEY (agent_id) REFERENCES agents(id) ON DELETE CASCADE
```

**executions**: Audit log
```sql
id TEXT PRIMARY KEY,
task_id TEXT NOT NULL,
agent_id TEXT,
step INTEGER,
action TEXT,
input_snapshot JSON,
output_snapshot JSON,
latency_ms INTEGER,
success BOOLEAN,
timestamp TIMESTAMP
```

**agent_performance**: Learning data (auto-updated via triggers)
```sql
id TEXT PRIMARY KEY,
agent_id TEXT NOT NULL,
capability TEXT NOT NULL,
success_count INTEGER DEFAULT 0,
failure_count INTEGER DEFAULT 0,
avg_latency_ms REAL,
last_updated TIMESTAMP,
UNIQUE(agent_id, capability)
```

## 🔌 Integration Guide

### Registering an Existing EON Service as an Agent

Any EON service can be integrated by exposing a simple HTTP endpoint that accepts:

```json
POST /api/v1/execute
{
  "capability": "analyze_data",
  "input": { /* task-specific data */ },
  "context": { /* optional context from previous steps */ }
}

Response:
{
  "success": true,
  "output": { /* result */ },
  "metadata": { /* optional: timing, resources used, etc */ }
}
```

**Steps**:
1. Ensure your service has an `/api/v1/execute` endpoint (or wrap existing logic)
2. Register it as an agent via the dashboard or API
3. Include the capability strings your service supports
4. The orchestrator will automatically match and dispatch tasks

### Example: EON-025 Intelligent Workflow Assistant

```bash
curl -X POST http://localhost:8081/api/v1/agents \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Workflow Assistant",
    "description": "AI-powered workflow optimization",
    "endpoint_url": "http://workflow-assistant:8081/api/v1/execute",
    "capabilities": [
      "optimize_workflow",
      "detect_bottlenecks",
      "predict_performance"
    ],
    "metadata": {"project": "EON-025"}
  }'
```

## 🧪 Testing

### Backend Tests

```bash
cd backend
cargo test --workspace
cargo test -- --nocapture  # With output
```

### Frontend Tests

```bash
cd frontend
npm test
npm run test:ui  # With Vitest UI
```

### Integration Tests

```bash
# Start the system
docker-compose up -d

# Run the integration test script
./scripts/integration-test.sh
```

## 📈 Performance Considerations

- **Connection Pooling**: SQLite connection pool (r2d2) for concurrent access
- **Async Everything**: Tokio runtime with multi-threaded scheduler
- **WAL Mode**: Write-Ahead Logging for concurrent reads/writes
- **Circuit Breaker**: Ready for integration (pattern implemented in agent_client)
- **Response Caching**: Entity tag (ETag) support ready for static data
- **Compression**: gzip enabled in Nginx for all responses

## 🗺️ Roadmap

- [ ] WebSocket support for real-time task updates
- [ ] Agent auto-discovery from service mesh
- [ ] Priority-based scheduling (Urgent vs Best-Effort)
- [ ] Conditional branching in execution plans
- [ ] Multi-LLM fallback (different models for different task types)
- [ ] A/B testing framework for planning algorithms
- [ ] Distributed execution across network nodes
- [ ] Authentication & rate limiting
- [ ] Metrics export to Analytics Aggregator (self-monitoring)
- [ ] PostgreSQL backend option for horizontal scaling

## 🤝 Contributing

We welcome contributions! Please read our [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Code of Conduct

This project follows the [Contributor Covenant](https://www.contributor-covenant.org/). By participating, you agree to its terms.

## 📄 License

MIT License. See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

Built with:
- [Axum](https://github.com/tokio-rs/axum) - Ergonomic Rust web framework
- [React](https://reactjs.org/) - UI library
- [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS
- [Recharts](https://recharts.org/) - Chart library
- [OpenRouter](https://openrouter.ai/) - LLM aggregation

---

**Made with ❤️ by the EON Team**

*Last updated: 2026-04-05*

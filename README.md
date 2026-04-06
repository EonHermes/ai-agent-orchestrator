# Real-time Task Dashboard 🚀

A full-stack task management application with real-time collaboration features.

**Live Demo:** Coming soon to your own machine!

## Features

- **Task Management**: Create, read, update, delete tasks with priorities and tags
- **Real-time Updates**: WebSocket-based live collaboration indicators and instant updates
- **Filtering & Search**: Filter by status, priority, tags, and full-text search
- **Responsive Design**: Works on desktop and mobile with a modern dark theme
- **Production Ready**: Docker deployment, health checks, CI/CD templates

## Tech Stack

### Backend (Rust)
- **Web Framework**: Axum 0.7
- **Async Runtime**: Tokio
- **Database**: PostgreSQL with SQLx for type-safe queries
- **Real-time**: WebSocket support
- **Serialization**: Serde
- **Testing**: Comprehensive unit and integration tests

### Frontend (React)
- **Framework**: React 18 with TypeScript
- **Build Tool**: Vite
- **Styling**: Tailwind CSS
- **State Management**: React Query + Context
- **WebSocket**: Native WebSocket API with reconnection logic
- **Icons**: Lucide React

## Quick Start

### Prerequisites
- Docker & Docker Compose
- Node.js 20+ (for local development)
- Rust 1.75+

### Using Docker (Recommended)
```bash
git clone <your-repo>
cd task-dashboard
docker-compose up -d
```

Access:
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080
- API Docs: http://localhost:8080/api-docs

### Local Development

**Backend:**
```bash
cd backend
cargo build
cargo run
```

**Frontend:**
```bash
cd frontend
npm install
npm run dev
```

## Project Structure

```
.
├── backend/           # Rust API server
│   ├── src/
│   │   ├── api/       # HTTP routes and WebSocket handlers
│   │   ├── db/        # Database models and queries
│   │   ├── error/     # Error handling
│   │   └── models/    # Data structures
│   ├── migrations/    # SQL migrations
│   └── tests/         # Integration tests
├── frontend/          # React application
│   ├── src/
│   │   ├── components/
│   │   ├── hooks/
│   │   ├── services/  # API and WebSocket clients
│   │   └── types/
│   └── public/
├── docker-compose.yml # Full stack orchestration
├── docker-compose.dev.yml # Development with hot reload
├── .github/workflows/ # CI/CD pipelines
└── docs/              # Additional documentation
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/tasks` | List all tasks with filters |
| POST | `/api/tasks` | Create new task |
| GET | `/api/tasks/:id` | Get task details |
| PUT | `/api/tasks/:id` | Update task |
| DELETE | `/api/tasks/:id` | Delete task |
| WS | `/ws` | WebSocket for real-time updates |

## WebSocket Events

```json
// Client sends
{ "type": "task_updated", "task": { ... } }

// Server broadcasts
{ "type": "task_created", "task": { ... } }
{ "type": "task_updated", "task": { ... } }
{ "type": "task_deleted", "task_id": "..." }
{ "type": "user_active", "user": "..." }
```

## Database Schema

```sql
CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL CHECK (status IN ('todo', 'in_progress', 'done')),
    priority INT NOT NULL DEFAULT 0,
    tags TEXT[] DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_priority ON tasks(priority);
CREATE INDEX idx_tasks_tags ON tasks USING GIN(tags);
```

## Testing

```bash
# Backend
cd backend
cargo test --workspace

# Frontend
cd frontend
npm test

# Integration tests with Docker
docker-compose -f docker-compose.test.yml up
```

## Deployment

See [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) for production deployment guide.

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) first.

## License

MIT

## Credits

Built with ❤️ using Rust and React

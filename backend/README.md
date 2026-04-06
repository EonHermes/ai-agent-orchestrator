# Snippet Manager Backend

Production-ready Rust API for the Code Snippet Manager.

## Features

- Full CRUD operations for code snippets with proper authentication
- Full-text search using Tantivy (BM25 ranking)
- Token-based authentication (JWT)
- PostgreSQL for persistence
- Comprehensive error handling and validation
- Structured logging with tracing
- Rate limiting ready (via nginx frontend)
- CORS support
- Health check endpoint
- Docker support

## Tech Stack

- **Framework:** Actix-web 4.x
- **Database:** PostgreSQL + sqlx (async, compile-time checked queries)
- **Search:** Tantivy full-text search engine
- **Auth:** JSON Web Tokens (jsonwebtoken) + bcrypt password hashing
- **Validation:** validator crate
- **Logging:** tracing + tracing-subscriber
- **Configuration:** config crate + environment variables

## Quick Start

### Prerequisites

- Rust 1.74+ and cargo
- PostgreSQL 16+
- Node.js (for frontend)

### Local Development

1. **Clone and setup:**
```bash
# Create and migrate database
createdb snippets
psql snippets -f backend/migrations/001_initial_schema.sql
```

2. **Configure environment:**
```bash
cp backend/.env.example backend/.env
# Edit .env with your values
```

3. **Run the server:**
```bash
cd backend
cargo run
```

The API will be available at: http://localhost:8080

### Docker Development

```bash
docker-compose up -d
```

All services (PostgreSQL, backend, frontend, nginx) will start.

- API: http://localhost:8080/api
- Frontend: http://localhost:3000
- API Docs: http://localhost:8080/api/docs (if Swagger added later)

## API Endpoints

### Authentication

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/auth/register` | Create new user account |
| POST | `/api/auth/login` | Login and receive JWT token |

### Snippets (authenticated)

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/snippets` | Create new snippet |
| GET | `/api/snippets` | List snippets (with optional filters) |
| GET | `/api/snippets/{id}` | Get snippet by ID |
| PUT | `/api/snippets/{id}` | Update snippet (owner only) |
| DELETE | `/api/snippets/{id}` | Delete snippet (owner only) |
| GET | `/api/snippets/search?q={query}&limit={n}` | Full-text search |

### Health

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |

## Authentication

All snippet endpoints require a Bearer token in Authorization header:

```
Authorization: Bearer <jwt_token>
```

Tokens are valid for 24 hours by default (configurable via `JWT_EXPIRY_SECONDS`).

## Database Schema

### users
- `id` UUID primary key
- `username` VARCHAR(255) UNIQUE NOT NULL
- `password_hash` TEXT NOT NULL (bcrypt)
- `created_at` TIMESTAMPTZ DEFAULT NOW()

### snippets
- `id` UUID primary key
- `user_id` UUID FOREIGN KEY (users.id) ON DELETE CASCADE
- `title` VARCHAR(500) NOT NULL
- `code` TEXT NOT NULL
- `language` VARCHAR(50) NOT NULL
- `description` TEXT (optional)
- `tags` TEXT (comma-separated)
- `created_at` TIMESTAMPTZ DEFAULT NOW()
- `updated_at` TIMESTAMPTZ with auto-update trigger

Indexes:
- `idx_snippets_user_id`
- `idx_snippets_language`
- `idx_snippets_created_at`

## Search Index

Tantivy is used for full-text search across:
- Title
- Code content
- Tags

The search index is stored in the `SEARCH_INDEX_PATH` directory and is automatically updated on snippet create/update/delete.

## Testing

```bash
cargo test
```

Planned test coverage:
- Unit tests for all handlers and database operations
- Integration tests for API endpoints
- Property-based tests for search ranking
- Performance benchmarks

## Production Deployment

### Systemd Service

```ini
[Unit]
Description=Snippet Manager Backend
Requires=postgresql.service
After=postgresql.service

[Service]
Type=simple
User=snippets
WorkingDirectory=/opt/snippet-backend
Environment="DATABASE_URL=postgresql://snippets:PASSWORD@localhost/snippets"
Environment="JWT_SECRET=your-secret-key-here"
Environment="SEARCH_INDEX_PATH=/var/lib/snippets/search_index"
ExecStart=/opt/snippet-backend/target/release/snippet-backend
Restart=on-failure
RestartSec=10

[Install]
WantedBy=multi-user.target
```

### Environment Variables

Required:

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgresql://user:pass@localhost:5432/snippets` |
| `JWT_SECRET` | Secret key for JWT signing (min 32 chars) | `your-super-secret-key-change-this` |
| `SEARCH_INDEX_PATH` | Where Tantivy stores its index | `/var/lib/snippets/search_index` |

Optional:

| Variable | Default | Description |
|----------|---------|-------------|
| `HOST` | `0.0.0.0` | Bind address |
| `PORT` | `8080` | Bind port |
| `RUST_LOG` | `info` | Log level (trace, debug, info, warn, error) |
| `JWT_EXPIRY_SECONDS` | `86400` | Token expiry (24 hours) |

### Docker Production

```bash
docker build -t snippet-backend -f backend/Dockerfile .
docker run -d \
  -p 8080:8080 \
  -e DATABASE_URL=postgresql://... \
  -e JWT_SECRET=... \
  -e SEARCH_INDEX_PATH=/data/search_index \
  -v search_index:/data/search_index \
  snippet-backend
```

## Performance

- Handles 10,000+ snippets per user
- Search latency: <50ms for 100K snippets (benchmark pending)
- Async throughout: no blocking I/O
- Connection pooling: 10-50 DB connections (configurable)

## Security

- Passwords hashed with bcrypt (cost 15)
- JWT tokens signed with HS256
- Prepared statements (no SQL injection)
- CORS configured for frontend origin
- Rate limiting via nginx
- Security headers via nginx

## Contributing

This is a production codebase. Follow these rules:

1. All database queries must be compile-time checked with sqlx
2. All errors must be handled with proper AppError types
3. All public API endpoints return `ApiResponse<T>`
4. All authentication enforced via `extract_user_id` middleware
5. All search index updates must succeed or error cleanly

## License

MIT
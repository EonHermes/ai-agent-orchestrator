# Build stage
FROM rust:1.75-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml ./
COPY sqlx-data.json ./

# Build dependencies (this will be cached if Cargo.toml doesn't change)
RUN cargo build --release --bin task-dashboard-backend --locked || true

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY tests ./tests

# Build application
RUN cargo build --release --bin task-dashboard-backend --locked

# Runtime stage
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/task-dashboard-backend /usr/local/bin/

# Copy migrations
COPY --from=builder /app/migrations ./migrations

# Create non-root user
RUN useradd -m -u 1000 appuser
USER appuser

EXPOSE 8080

CMD ["task-dashboard-backend"]

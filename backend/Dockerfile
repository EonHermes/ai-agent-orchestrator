# Build stage
FROM rust:1.74-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY backend/Cargo.toml ./
COPY backend/src ./src
COPY backend/migrations ./migrations

# Build
RUN cargo build --release

# Run stage
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/snippet-backend /app/snippet-backend
COPY backend/.env.example /app/.env.example

EXPOSE 8080

CMD ["/app/snippet-backend"]
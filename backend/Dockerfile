# Build stage
FROM rust:1.75-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml ./
COPY src ./src

# Build for release
RUN cargo build --release --locked

# Runtime stage
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    tzdata \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r orchestrator && useradd -r -g orchestrator orchestrator

# Copy binary from builder
COPY --from=builder /app/target/release/ai-agent-orchestrator /usr/local/bin/ai-agent-orchestrator

# Create data directory
RUN mkdir -p /data && chown -R orchestrator:orchestrator /data /app

USER orchestrator

EXPOSE 8081

ENV RUST_LOG=info
ENV DATABASE_URL=sqlite:/data/orchestrator.db

CMD ["ai-agent-orchestrator"]

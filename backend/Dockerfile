# Build stage
FROM rust:1.75-slim AS builder

WORKDIR /usr/src/ai-agent-orchestrator

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src/ ./src/
COPY migrations/ ./migrations/

# Build
RUN cargo build --release --locked

# Runtime stage
FROM debian:bookworm-slim AS runtime

WORKDIR /usr/local/bin

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /usr/src/ai-agent-orchestrator/target/release/ai-agent-orchestrator ./ai-agent-orchestrator

# Create data directory
RUN mkdir -p /data
VOLUME /data

# Expose port
EXPOSE 8081

# Run
CMD ["/usr/local/bin/ai-agent-orchestrator"]
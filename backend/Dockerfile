# Build stage
FROM rust:1.87-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only (cached layer)
RUN cargo build --release && rm -rf src

# Copy actual source code
COPY src ./src

# Build the application
RUN touch src/main.rs && cargo build --release

# Runtime stage - minimal image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p /data

# Copy binary from builder
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

# Copy deployments.json
COPY deployments.json /app/deployments.json

# Run as non-root user
RUN useradd -r -s /bin/false backend && chown -R backend:backend /data
USER backend

WORKDIR /app

EXPOSE 3000

CMD ["backend"]

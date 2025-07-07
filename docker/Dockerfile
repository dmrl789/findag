# FinDAG Production Dockerfile
# Multi-stage build for optimized production image

# Stage 1: Build stage
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release

# Remove dummy main.rs and copy actual source code
RUN rm src/main.rs
COPY src/ ./src/
COPY configs/ ./configs/
COPY scripts/ ./scripts/

# Build the application
RUN cargo build --release --bin findag

# Stage 2: Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN groupadd -r findag && useradd -r -g findag findag

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/findag /usr/local/bin/

# Copy configuration files
COPY --from=builder /app/configs/ ./configs/

# Create necessary directories
RUN mkdir -p /app/logs /app/data /app/certs && \
    chown -R findag:findag /app

# Switch to non-root user
USER findag

# Expose ports
EXPOSE 8080 9090

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Default command
CMD ["findag"] 
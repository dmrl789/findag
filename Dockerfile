# ---- Build Stage ----
FROM rust:1.76 as builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source and build
COPY . .
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bullseye-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the statically built binary from the builder stage
COPY --from=builder /app/target/release/findag /app/findag

# Set permissions and entrypoint
RUN chmod +x /app/findag
ENTRYPOINT ["/app/findag"] 
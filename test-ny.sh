#!/bin/bash

echo "Starting FinDAG NY Node on Laptop 1 (192.168.1.44)..."

# Build the project (optional if already built)
cargo build --release

# Start NY node
FINDAG_BIND_ADDR=0.0.0.0:8080 \
FINDAG_PEERS=192.168.1.20:8081 \
./target/release/findag 
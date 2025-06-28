#!/bin/bash

# Test script to run both FinDAG nodes locally
echo "Starting FinDAG Two-Node Test..."

# Build the project
echo "Building FinDAG..."
cargo build --release

# Start NY node in background
echo "Starting NY node on port 8080..."
FINDAG_BIND_ADDR=0.0.0.0:8080 FINDAG_PEERS=127.0.0.1:8081 ./target/release/findag &
NY_PID=$!

# Wait a moment for NY node to start
sleep 2

# Start London node in background
echo "Starting London node on port 8081..."
FINDAG_BIND_ADDR=0.0.0.0:8081 FINDAG_PEERS=127.0.0.1:8080 ./target/release/findag &
LONDON_PID=$!

echo "Both nodes started!"
echo "NY Node PID: $NY_PID"
echo "London Node PID: $LONDON_PID"
echo ""
echo "Test URLs:"
echo "NY Health: http://localhost:8080/health"
echo "NY Info: http://localhost:8080/node/info"
echo "NY Transactions: http://localhost:8080/transactions"
echo "NY Blocks: http://localhost:8080/blocks"
echo ""
echo "London Health: http://localhost:8081/health"
echo "London Info: http://localhost:8081/node/info"
echo "London Transactions: http://localhost:8081/transactions"
echo "London Blocks: http://localhost:8081/blocks"
echo ""
echo "Press Ctrl+C to stop both nodes"

# Wait for interrupt
trap "echo 'Stopping nodes...'; kill $NY_PID $LONDON_PID; exit" INT
wait 
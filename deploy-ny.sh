#!/bin/bash

# FinDAG New York Node Deployment Script
echo "Deploying FinDAG New York Node..."

# Create directories
mkdir -p /opt/findag/data
mkdir -p /opt/findag/logs

# Copy files
cp docker-compose.yml /opt/findag/
cp Dockerfile /opt/findag/

# Create NY-specific docker-compose override
cat > /opt/findag/docker-compose.override.yml << EOF
version: '3.8'
services:
  findag-ny:
    ports:
      - "8080:8080"
      - "9898:9898"
      - "9000:9000"
    environment:
      - FINDAG_BIND_ADDR=0.0.0.0:8080
      - FINDAG_PEERS=YOUR_LONDON_SERVER_IP:8081
      - NODE_ID=ny-node-001
      - NODE_REGION=ny
    volumes:
      - /opt/findag/data:/app/data
      - /opt/findag/logs:/app/logs
EOF

# Build and start
cd /opt/findag
docker-compose up -d findag-ny

echo "FinDAG New York Node deployed!"
echo "HTTP API: http://localhost:8080"
echo "Health check: http://localhost:8080/health"
echo "Node info: http://localhost:8080/node/info" 
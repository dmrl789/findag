#!/bin/bash

# FinDAG London Node Deployment Script
echo "Deploying FinDAG London Node..."

# Create directories
mkdir -p /opt/findag/data
mkdir -p /opt/findag/logs

# Copy files
cp docker-compose.yml /opt/findag/
cp Dockerfile /opt/findag/

# Create London-specific docker-compose override
cat > /opt/findag/docker-compose.override.yml << EOF
version: '3.8'
services:
  findag-london:
    ports:
      - "8081:8080"
      - "9899:9898"
      - "9001:9000"
    environment:
      - FINDAG_BIND_ADDR=0.0.0.0:8080
      - FINDAG_PEERS=YOUR_NY_SERVER_IP:8080
      - NODE_ID=london-node-001
      - NODE_REGION=london
    volumes:
      - /opt/findag/data:/app/data
      - /opt/findag/logs:/app/logs
EOF

# Build and start
cd /opt/findag
docker-compose up -d findag-london

echo "FinDAG London Node deployed!"
echo "HTTP API: http://localhost:8081"
echo "Health check: http://localhost:8081/health"
echo "Node info: http://localhost:8081/node/info" 
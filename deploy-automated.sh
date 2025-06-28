#!/bin/bash

# FinDAG Automated Deployment Script
# This script automatically deploys the two-node setup with transaction bots

set -e  # Exit on any error

echo "🚀 Starting FinDAG Automated Deployment..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
NY_SERVER_IP=${NY_SERVER_IP:-"localhost"}
LONDON_SERVER_IP=${LONDON_SERVER_IP:-"localhost"}
NY_PORT=8080
LONDON_PORT=8081

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Step 1: Build the Rust project
log_info "Building FinDAG Rust project..."
if ! cargo build --release; then
    log_error "Failed to build Rust project"
    exit 1
fi
log_success "Rust project built successfully"

# Step 2: Build Docker image
log_info "Building Docker image..."
if ! docker build -t findag:latest .; then
    log_error "Failed to build Docker image"
    exit 1
fi
log_success "Docker image built successfully"

# Step 3: Update docker-compose with actual server IPs
log_info "Updating docker-compose configuration..."
sed -i.bak "s/YOUR_LONDON_SERVER_IP/$LONDON_SERVER_IP/g" docker-compose.yml
sed -i.bak "s/YOUR_NY_SERVER_IP/$NY_SERVER_IP/g" docker-compose.yml

# Step 4: Start the nodes
log_info "Starting FinDAG nodes..."
if ! docker-compose up -d; then
    log_error "Failed to start Docker containers"
    exit 1
fi
log_success "Docker containers started"

# Step 5: Wait for nodes to be ready
log_info "Waiting for nodes to be ready..."
sleep 10

# Step 6: Health check
log_info "Performing health checks..."

# Check NY node
if curl -f http://localhost:$NY_PORT/health > /dev/null 2>&1; then
    log_success "NY node is healthy"
else
    log_warning "NY node health check failed, but continuing..."
fi

# Check London node
if curl -f http://localhost:$LONDON_PORT/health > /dev/null 2>&1; then
    log_success "London node is healthy"
else
    log_warning "London node health check failed, but continuing..."
fi

# Step 7: Start transaction bots
log_info "Starting transaction bots..."

# Create bot script
cat > transaction_bot.py << 'EOF'
#!/usr/bin/env python3
import requests
import time
import random
import json
from datetime import datetime

class FinDAGTransactionBot:
    def __init__(self, node_url, node_name):
        self.node_url = node_url
        self.node_name = node_name
        self.session = requests.Session()
        
    def send_transaction(self):
        """Send a random transaction to the node"""
        try:
            # Generate random transaction data
            tx_data = {
                "from": f"bot-{self.node_name}-{random.randint(1000, 9999)}",
                "to": f"recipient-{random.randint(1000, 9999)}",
                "amount": random.uniform(0.1, 100.0),
                "timestamp": datetime.now().isoformat(),
                "type": random.choice(["transfer", "swap", "stake"])
            }
            
            # Send transaction
            response = self.session.post(
                f"{self.node_url}/api/transactions",
                json=tx_data,
                timeout=5
            )
            
            if response.status_code == 200:
                print(f"[{self.node_name}] Transaction sent: {tx_data['amount']:.2f} tokens")
                return True
            else:
                print(f"[{self.node_name}] Failed to send transaction: {response.status_code}")
                return False
                
        except Exception as e:
            print(f"[{self.node_name}] Error sending transaction: {e}")
            return False
    
    def run(self, interval=2):
        """Run the bot continuously"""
        print(f"🤖 Starting transaction bot for {self.node_name} node")
        print(f"📡 Sending transactions to: {self.node_url}")
        print(f"⏱️  Interval: {interval} seconds")
        
        while True:
            self.send_transaction()
            time.sleep(interval)

if __name__ == "__main__":
    import sys
    
    if len(sys.argv) != 3:
        print("Usage: python3 transaction_bot.py <node_url> <node_name>")
        sys.exit(1)
    
    node_url = sys.argv[1]
    node_name = sys.argv[2]
    
    bot = FinDAGTransactionBot(node_url, node_name)
    bot.run()
EOF

# Make bot script executable
chmod +x transaction_bot.py

# Start NY bot in background
log_info "Starting NY transaction bot..."
python3 transaction_bot.py http://localhost:$NY_PORT ny &
NY_BOT_PID=$!

# Start London bot in background
log_info "Starting London transaction bot..."
python3 transaction_bot.py http://localhost:$LONDON_PORT london &
LONDON_BOT_PID=$!

# Save bot PIDs for cleanup
echo $NY_BOT_PID > .ny_bot.pid
echo $LONDON_BOT_PID > .london_bot.pid

# Step 8: Create monitoring script
cat > monitor.sh << 'EOF'
#!/bin/bash

echo "📊 FinDAG Network Monitor"
echo "========================"

while true; do
    clear
    echo "📊 FinDAG Network Monitor - $(date)"
    echo "========================"
    
    # Check NY node
    echo "🏙️  NY Node (localhost:8080):"
    if curl -s http://localhost:8080/health > /dev/null; then
        echo "  ✅ Status: Healthy"
        echo "  📈 Blocks: $(curl -s http://localhost:8080/api/stats | jq -r '.total_blocks // "N/A"')"
        echo "  🔄 Transactions: $(curl -s http://localhost:8080/api/stats | jq -r '.total_transactions // "N/A"')"
    else
        echo "  ❌ Status: Unhealthy"
    fi
    
    echo ""
    
    # Check London node
    echo "🇬🇧 London Node (localhost:8081):"
    if curl -s http://localhost:8081/health > /dev/null; then
        echo "  ✅ Status: Healthy"
        echo "  📈 Blocks: $(curl -s http://localhost:8081/api/stats | jq -r '.total_blocks // "N/A"')"
        echo "  🔄 Transactions: $(curl -s http://localhost:8081/api/stats | jq -r '.total_transactions // "N/A"')"
    else
        echo "  ❌ Status: Unhealthy"
    fi
    
    echo ""
    echo "🤖 Transaction Bots:"
    if [ -f .ny_bot.pid ] && kill -0 $(cat .ny_bot.pid) 2>/dev/null; then
        echo "  ✅ NY Bot: Running"
    else
        echo "  ❌ NY Bot: Stopped"
    fi
    
    if [ -f .london_bot.pid ] && kill -0 $(cat .london_bot.pid) 2>/dev/null; then
        echo "  ✅ London Bot: Running"
    else
        echo "  ❌ London Bot: Stopped"
    fi
    
    echo ""
    echo "Press Ctrl+C to exit monitor"
    sleep 5
done
EOF

chmod +x monitor.sh

# Step 9: Create cleanup script
cat > cleanup.sh << 'EOF'
#!/bin/bash

echo "🧹 Cleaning up FinDAG deployment..."

# Stop transaction bots
if [ -f .ny_bot.pid ]; then
    kill $(cat .ny_bot.pid) 2>/dev/null || true
    rm .ny_bot.pid
fi

if [ -f .london_bot.pid ]; then
    kill $(cat .london_bot.pid) 2>/dev/null || true
    rm .london_bot.pid
fi

# Stop Docker containers
docker-compose down

# Remove Docker images
docker rmi findag:latest 2>/dev/null || true

echo "✅ Cleanup completed"
EOF

chmod +x cleanup.sh

# Step 10: Final status
log_success "🎉 FinDAG deployment completed successfully!"
echo ""
echo "📋 Deployment Summary:"
echo "======================"
echo "🏙️  NY Node: http://localhost:$NY_PORT"
echo "🇬🇧 London Node: http://localhost:$LONDON_PORT"
echo "🤖 Transaction bots: Running (PIDs: $NY_BOT_PID, $LONDON_BOT_PID)"
echo ""
echo "📊 Monitor the network: ./monitor.sh"
echo "🧹 Clean up deployment: ./cleanup.sh"
echo ""
echo "🔗 Useful endpoints:"
echo "  - Health: http://localhost:$NY_PORT/health"
echo "  - Stats: http://localhost:$NY_PORT/api/stats"
echo "  - Node Info: http://localhost:$NY_PORT/api/node/info"
echo ""
log_info "Transaction bots are now sending transactions every 2 seconds..."
log_info "Press Ctrl+C to stop the deployment" 
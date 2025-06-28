#!/bin/bash

# FinDAG Home Network Deployment Script
# For two laptops: Laptop 1 (NY) at 192.168.1.44, Laptop 2 (London) at 192.168.1.20

set -e  # Exit on any error

echo "🏠 Starting FinDAG Home Network Deployment..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration for your home network
LAPTOP1_IP="192.168.1.44"  # NY laptop
LAPTOP2_IP="192.168.1.20"  # London laptop
LAPTOP1_PORT=8080
LAPTOP2_PORT=8080
P2P_PORT=9000

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

# Detect which laptop this is
detect_laptop() {
    local current_ip=$(hostname -I | awk '{print $1}')
    if [[ "$current_ip" == "$LAPTOP1_IP" ]]; then
        echo "laptop1"
    elif [[ "$current_ip" == "$LAPTOP2_IP" ]]; then
        echo "laptop2"
    else
        echo "unknown"
    fi
}

LAPTOP_TYPE=$(detect_laptop)

if [[ "$LAPTOP_TYPE" == "unknown" ]]; then
    log_error "Could not detect laptop type. Current IP: $(hostname -I | awk '{print $1}')"
    log_info "Please run this script on either laptop1 ($LAPTOP1_IP) or laptop2 ($LAPTOP2_IP)"
    exit 1
fi

log_info "Detected: $LAPTOP_TYPE"

# Step 1: Build the Rust project
log_info "Building FinDAG Rust project..."
if ! cargo build --release; then
    log_error "Failed to build Rust project"
    exit 1
fi
log_success "Rust project built successfully"

# Step 2: Create configuration based on laptop type
if [[ "$LAPTOP_TYPE" == "laptop1" ]]; then
    # Laptop 1 (NY) configuration
    BIND_ADDR="0.0.0.0:$LAPTOP1_PORT"
    PEERS="$LAPTOP2_IP:$P2P_PORT"
    NODE_NAME="NY-Laptop1"
    HTTP_PORT=$LAPTOP1_PORT
    P2P_BIND="0.0.0.0:$P2P_PORT"
else
    # Laptop 2 (London) configuration
    BIND_ADDR="0.0.0.0:$LAPTOP2_PORT"
    PEERS="$LAPTOP1_IP:$P2P_PORT"
    NODE_NAME="London-Laptop2"
    HTTP_PORT=$LAPTOP2_PORT
    P2P_BIND="0.0.0.0:$P2P_PORT"
fi

# Step 3: Create startup script
cat > start-findag.sh << EOF
#!/bin/bash
# FinDAG startup script for $NODE_NAME

echo "🚀 Starting FinDAG node: $NODE_NAME"
echo "📡 HTTP API: http://0.0.0.0:$HTTP_PORT"
echo "🌐 P2P Bind: $P2P_BIND"
echo "👥 Peers: $PEERS"

# Set environment variables
export FINDAG_BIND_ADDR="$BIND_ADDR"
export FINDAG_PEERS="$PEERS"
export FINDAG_UDP_PORT="$P2P_PORT"
export NODE_ID="$NODE_NAME"

# Start the node
./target/release/findag
EOF

chmod +x start-findag.sh

# Step 4: Create transaction bot script
cat > transaction-bot.py << 'EOF'
#!/usr/bin/env python3
import requests
import time
import random
import json
from datetime import datetime
import sys

class FinDAGTransactionBot:
    def __init__(self, node_url, node_name):
        self.node_url = node_url
        self.node_name = node_name
        self.session = requests.Session()
        self.tx_count = 0
        
    def create_test_transaction(self):
        """Create a test transaction"""
        # Generate random addresses
        from_addr = f"fdg1q{random.randint(100000, 999999)}"
        to_addr = f"fdg1q{random.randint(100000, 999999)}"
        
        # Create transaction data
        tx_data = {
            "from": from_addr,
            "to": to_addr,
            "amount": random.randint(1, 1000),
            "payload": f"Test transaction from {self.node_name}".encode().hex(),
            "findag_time": int(time.time() * 1000000),  # Microseconds
            "hashtimer": bytes([random.randint(0, 255) for _ in range(32)]),
            "shard_id": {"0": 0}
        }
        
        return tx_data
    
    def send_transaction(self):
        """Send a transaction to the node"""
        try:
            tx_data = self.create_test_transaction()
            
            # Send transaction
            response = self.session.post(
                f"{self.node_url}/transactions",
                json=tx_data,
                timeout=5
            )
            
            self.tx_count += 1
            
            if response.status_code == 200:
                print(f"[{self.node_name}] ✅ Transaction #{self.tx_count} sent: {tx_data['amount']} tokens")
                return True
            else:
                print(f"[{self.node_name}] ❌ Failed to send transaction: {response.status_code}")
                return False
                
        except Exception as e:
            print(f"[{self.node_name}] ❌ Error sending transaction: {e}")
            return False
    
    def get_node_info(self):
        """Get node information"""
        try:
            response = self.session.get(f"{self.node_url}/node/info", timeout=5)
            if response.status_code == 200:
                return response.json()
        except:
            pass
        return None
    
    def run(self, interval=3):
        """Run the bot continuously"""
        print(f"🤖 Starting transaction bot for {self.node_name}")
        print(f"📡 Node URL: {self.node_url}")
        print(f"⏱️  Interval: {interval} seconds")
        print(f"🔄 Press Ctrl+C to stop")
        print("-" * 50)
        
        while True:
            # Get and display node info every 10 transactions
            if self.tx_count % 10 == 0:
                info = self.get_node_info()
                if info:
                    print(f"[{self.node_name}] 📊 Node Info: {info.get('block_count', 'N/A')} blocks, {info.get('peers', [])} peers")
            
            self.send_transaction()
            time.sleep(interval)

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python3 transaction-bot.py <node_url> <node_name>")
        print("Example: python3 transaction-bot.py http://192.168.1.44:8080 NY-Laptop1")
        sys.exit(1)
    
    node_url = sys.argv[1]
    node_name = sys.argv[2]
    
    bot = FinDAGTransactionBot(node_url, node_name)
    bot.run()
EOF

chmod +x transaction-bot.py

# Step 5: Create monitoring script
cat > monitor.sh << 'EOF'
#!/bin/bash

echo "📊 FinDAG Home Network Monitor"
echo "=============================="

LAPTOP1_URL="http://192.168.1.44:8080"
LAPTOP2_URL="http://192.168.1.20:8080"

while true; do
    clear
    echo "📊 FinDAG Home Network Monitor - $(date)"
    echo "=============================="
    
    # Check Laptop 1 (NY)
    echo "🏙️  Laptop 1 (NY) - $LAPTOP1_URL:"
    if curl -s "$LAPTOP1_URL/health" > /dev/null 2>&1; then
        echo "  ✅ Status: Healthy"
        INFO1=$(curl -s "$LAPTOP1_URL/node/info" 2>/dev/null)
        if [[ ! -z "$INFO1" ]]; then
            BLOCKS1=$(echo "$INFO1" | jq -r '.block_count // "N/A"' 2>/dev/null || echo "N/A")
            PEERS1=$(echo "$INFO1" | jq -r '.peers | length // "N/A"' 2>/dev/null || echo "N/A")
            echo "  📈 Blocks: $BLOCKS1"
            echo "  👥 Peers: $PEERS1"
        fi
    else
        echo "  ❌ Status: Unhealthy"
    fi
    
    echo ""
    
    # Check Laptop 2 (London)
    echo "🇬🇧 Laptop 2 (London) - $LAPTOP2_URL:"
    if curl -s "$LAPTOP2_URL/health" > /dev/null 2>&1; then
        echo "  ✅ Status: Healthy"
        INFO2=$(curl -s "$LAPTOP2_URL/node/info" 2>/dev/null)
        if [[ ! -z "$INFO2" ]]; then
            BLOCKS2=$(echo "$INFO2" | jq -r '.block_count // "N/A"' 2>/dev/null || echo "N/A")
            PEERS2=$(echo "$INFO2" | jq -r '.peers | length // "N/A"' 2>/dev/null || echo "N/A")
            echo "  📈 Blocks: $BLOCKS2"
            echo "  👥 Peers: $PEERS2"
        fi
    else
        echo "  ❌ Status: Unhealthy"
    fi
    
    echo ""
    echo "🔄 Refreshing in 5 seconds... (Press Ctrl+C to stop)"
    sleep 5
done
EOF

chmod +x monitor.sh

# Step 6: Create instructions file
cat > SETUP_INSTRUCTIONS.md << EOF
# FinDAG Home Network Setup Instructions

## Network Configuration
- **Laptop 1 (NY)**: $LAPTOP1_IP:$LAPTOP1_PORT
- **Laptop 2 (London)**: $LAPTOP2_IP:$LAPTOP2_PORT
- **P2P Port**: $P2P_PORT

## Step-by-Step Setup

### On Both Laptops:

1. **Build the project** (if not already done):
   \`\`\`bash
   cargo build --release
   \`\`\`

2. **Start the FinDAG node**:
   \`\`\`bash
   ./start-findag.sh
   \`\`\`

3. **In a new terminal, start the transaction bot**:
   \`\`\`bash
   # On Laptop 1:
   python3 transaction-bot.py http://$LAPTOP1_IP:$LAPTOP1_PORT NY-Laptop1
   
   # On Laptop 2:
   python3 transaction-bot.py http://$LAPTOP2_IP:$LAPTOP2_PORT London-Laptop2
   \`\`\`

4. **Monitor the network** (optional):
   \`\`\`bash
   ./monitor.sh
   \`\`\`

## Test URLs

### Laptop 1 (NY):
- Health: http://$LAPTOP1_IP:$LAPTOP1_PORT/health
- Node Info: http://$LAPTOP1_IP:$LAPTOP1_PORT/node/info
- Blocks: http://$LAPTOP1_IP:$LAPTOP1_PORT/blocks
- DAG Stats: http://$LAPTOP1_IP:$LAPTOP1_PORT/dag

### Laptop 2 (London):
- Health: http://$LAPTOP2_IP:$LAPTOP2_PORT/health
- Node Info: http://$LAPTOP2_IP:$LAPTOP2_PORT/node/info
- Blocks: http://$LAPTOP2_IP:$LAPTOP2_PORT/blocks
- DAG Stats: http://$LAPTOP2_IP:$LAPTOP2_PORT/dag

## What to Expect

1. **Initial Connection**: Both nodes should discover each other via P2P
2. **Block Production**: Each node will produce blocks every ~1 second
3. **Transaction Propagation**: Transactions sent to one node will appear on the other
4. **DAG Growth**: You should see the DAG structure growing with multiple blocks
5. **Consensus**: Both nodes should have similar block counts and DAG structures

## Troubleshooting

- **Connection Issues**: Ensure both laptops can ping each other
- **Port Conflicts**: Make sure ports $LAPTOP1_PORT and $P2P_PORT are not in use
- **Firewall**: Allow UDP traffic on port $P2P_PORT between laptops
- **Logs**: Check the terminal output for any error messages

## Web Interface

You can also access the web interface by opening the node URLs in your browser!
EOF

log_success "Setup complete for $NODE_NAME!"
log_info "Configuration created:"
log_info "  - HTTP API: http://0.0.0.0:$HTTP_PORT"
log_info "  - P2P Bind: $P2P_BIND"
log_info "  - Peers: $PEERS"
log_info ""
log_info "Next steps:"
log_info "1. Run: ./start-findag.sh"
log_info "2. In another terminal: python3 transaction-bot.py http://$(hostname -I | awk '{print $1}'):$HTTP_PORT $NODE_NAME"
log_info "3. Check SETUP_INSTRUCTIONS.md for detailed instructions" 
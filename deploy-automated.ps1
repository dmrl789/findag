# FinDAG Automated Deployment Script for Windows
# This script automatically deploys the two-node setup with transaction bots

param(
    [string]$NY_SERVER_IP = "localhost",
    [string]$LONDON_SERVER_IP = "localhost"
)

# Configuration
$NY_PORT = 8080
$LONDON_PORT = 8081

Write-Host "🚀 Starting FinDAG Automated Deployment..." -ForegroundColor Green

# Function to write colored output
function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

# Step 1: Build the Rust project
Write-Info "Building FinDAG Rust project..."
try {
    cargo build --release
    Write-Success "Rust project built successfully"
} catch {
    Write-Error "Failed to build Rust project"
    exit 1
}

# Step 2: Build Docker image
Write-Info "Building Docker image..."
try {
    docker build -t findag:latest .
    Write-Success "Docker image built successfully"
} catch {
    Write-Error "Failed to build Docker image"
    exit 1
}

# Step 3: Update docker-compose with actual server IPs
Write-Info "Updating docker-compose configuration..."
$dockerComposeContent = Get-Content docker-compose.yml -Raw
$dockerComposeContent = $dockerComposeContent -replace "YOUR_LONDON_SERVER_IP", $LONDON_SERVER_IP
$dockerComposeContent = $dockerComposeContent -replace "YOUR_NY_SERVER_IP", $NY_SERVER_IP
$dockerComposeContent | Set-Content docker-compose.yml

# Step 4: Start the nodes
Write-Info "Starting FinDAG nodes..."
try {
    docker-compose up -d
    Write-Success "Docker containers started"
} catch {
    Write-Error "Failed to start Docker containers"
    exit 1
}

# Step 5: Wait for nodes to be ready
Write-Info "Waiting for nodes to be ready..."
Start-Sleep -Seconds 10

# Step 6: Health check
Write-Info "Performing health checks..."

# Check NY node
try {
    $response = Invoke-WebRequest -Uri "http://localhost:$NY_PORT/health" -UseBasicParsing -TimeoutSec 5
    if ($response.StatusCode -eq 200) {
        Write-Success "NY node is healthy"
    } else {
        Write-Warning "NY node health check failed, but continuing..."
    }
} catch {
    Write-Warning "NY node health check failed, but continuing..."
}

# Check London node
try {
    $response = Invoke-WebRequest -Uri "http://localhost:$LONDON_PORT/health" -UseBasicParsing -TimeoutSec 5
    if ($response.StatusCode -eq 200) {
        Write-Success "London node is healthy"
    } else {
        Write-Warning "London node health check failed, but continuing..."
    }
} catch {
    Write-Warning "London node health check failed, but continuing..."
}

# Step 7: Start transaction bots
Write-Info "Starting transaction bots..."

# Create bot script
$botScript = @'
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
'@

$botScript | Out-File -FilePath "transaction_bot.py" -Encoding UTF8

# Start NY bot in background
Write-Info "Starting NY transaction bot..."
$nyBotJob = Start-Job -ScriptBlock {
    param($nodeUrl, $nodeName)
    python3 transaction_bot.py $nodeUrl $nodeName
} -ArgumentList "http://localhost:$NY_PORT", "ny"

# Start London bot in background
Write-Info "Starting London transaction bot..."
$londonBotJob = Start-Job -ScriptBlock {
    param($nodeUrl, $nodeName)
    python3 transaction_bot.py $nodeUrl $nodeName
} -ArgumentList "http://localhost:$LONDON_PORT", "london"

# Save job IDs for cleanup
$nyBotJob.Id | Out-File -FilePath ".ny_bot.pid"
$londonBotJob.Id | Out-File -FilePath ".london_bot.pid"

# Step 8: Create monitoring script
$monitorScript = @'
# FinDAG Network Monitor for Windows
Write-Host "📊 FinDAG Network Monitor"
Write-Host "========================"

while ($true) {
    Clear-Host
    Write-Host "📊 FinDAG Network Monitor - $(Get-Date)"
    Write-Host "========================"
    
    # Check NY node
    Write-Host "NY Node: localhost:8080"
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:8080/health" -UseBasicParsing -TimeoutSec 5
        if ($response.StatusCode -eq 200) {
            Write-Host "  ✅ Status: Healthy" -ForegroundColor Green
            try {
                $stats = Invoke-WebRequest -Uri "http://localhost:8080/api/stats" -UseBasicParsing | ConvertFrom-Json
                Write-Host "  📈 Blocks: $($stats.total_blocks)"
                Write-Host "  🔄 Transactions: $($stats.total_transactions)"
            } catch {
                Write-Host "  📈 Blocks: N/A"
                Write-Host "  🔄 Transactions: N/A"
            }
        } else {
            Write-Host "  ❌ Status: Unhealthy" -ForegroundColor Red
        }
    } catch {
        Write-Host "  ❌ Status: Unhealthy" -ForegroundColor Red
    }
    
    Write-Host ""
    
    # Check London node
    Write-Host "London Node: localhost:8081"
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:8081/health" -UseBasicParsing -TimeoutSec 5
        if ($response.StatusCode -eq 200) {
            Write-Host "  ✅ Status: Healthy" -ForegroundColor Green
            try {
                $stats = Invoke-WebRequest -Uri "http://localhost:8081/api/stats" -UseBasicParsing | ConvertFrom-Json
                Write-Host "  📈 Blocks: $($stats.total_blocks)"
                Write-Host "  🔄 Transactions: $($stats.total_transactions)"
            } catch {
                Write-Host "  📈 Blocks: N/A"
                Write-Host "  🔄 Transactions: N/A"
            }
        } else {
            Write-Host "  ❌ Status: Unhealthy" -ForegroundColor Red
        }
    } catch {
        Write-Host "  ❌ Status: Unhealthy" -ForegroundColor Red
    }
    
    Write-Host ""
    Write-Host "🤖 Transaction bots:"
    
    if (Test-Path ".ny_bot.pid") {
        $nyJobId = Get-Content ".ny_bot.pid"
        $nyJob = Get-Job -Id $nyJobId -ErrorAction SilentlyContinue
        if ($nyJob -and $nyJob.State -eq "Running") {
            Write-Host "  ✅ NY Bot: Running" -ForegroundColor Green
        } else {
            Write-Host "  ❌ NY Bot: Stopped" -ForegroundColor Red
        }
    } else {
        Write-Host "  ❌ NY Bot: Not found" -ForegroundColor Red
    }
    
    if (Test-Path ".london_bot.pid") {
        $londonJobId = Get-Content ".london_bot.pid"
        $londonJob = Get-Job -Id $londonJobId -ErrorAction SilentlyContinue
        if ($londonJob -and $londonJob.State -eq "Running") {
            Write-Host "  ✅ London Bot: Running" -ForegroundColor Green
        } else {
            Write-Host "  ❌ London Bot: Stopped" -ForegroundColor Red
        }
    } else {
        Write-Host "  ❌ London Bot: Not found" -ForegroundColor Red
    }
    
    Write-Host ""
    Write-Host "Press Ctrl+C to exit monitor"
    Start-Sleep -Seconds 5
}
'@

$monitorScript | Out-File -FilePath "monitor.ps1" -Encoding UTF8

# Step 9: Create cleanup script
$cleanupScript = @'
# FinDAG Cleanup Script for Windows
Write-Host "🧹 Cleaning up FinDAG deployment..."

# Stop transaction bots
if (Test-Path ".ny_bot.pid") {
    $nyJobId = Get-Content ".ny_bot.pid"
    Stop-Job -Id $nyJobId -ErrorAction SilentlyContinue
    Remove-Job -Id $nyJobId -ErrorAction SilentlyContinue
    Remove-Item ".ny_bot.pid" -ErrorAction SilentlyContinue
}

if (Test-Path ".london_bot.pid") {
    $londonJobId = Get-Content ".london_bot.pid"
    Stop-Job -Id $londonJobId -ErrorAction SilentlyContinue
    Remove-Job -Id $londonJobId -ErrorAction SilentlyContinue
    Remove-Item ".london_bot.pid" -ErrorAction SilentlyContinue
}

# Stop Docker containers
docker-compose down

# Remove Docker images
docker rmi findag:latest 2>$null

Write-Host "✅ Cleanup completed" -ForegroundColor Green
'@

$cleanupScript | Out-File -FilePath "cleanup.ps1" -Encoding UTF8

# Step 10: Final status
Write-Success "FinDAG deployment completed successfully!"
Write-Host ""
Write-Host "Deployment Summary:"
Write-Host "======================"
Write-Host "NY Node: http://localhost:$NY_PORT"
Write-Host "London Node: http://localhost:$LONDON_PORT"
Write-Host "Transaction bots: Running (Job IDs: $($nyBotJob.Id), $($londonBotJob.Id))"
Write-Host ""
Write-Host "Monitor the network: .\monitor.ps1"
Write-Host "Clean up deployment: .\cleanup.ps1"
Write-Host ""
Write-Host "Useful endpoints:"
Write-Host "  - Health: http://localhost:$NY_PORT/health"
Write-Host "  - Stats: http://localhost:$NY_PORT/api/stats"
Write-Host "  - Node Info: http://localhost:$NY_PORT/api/node/info"
Write-Host "" 
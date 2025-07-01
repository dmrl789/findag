# Single Node FinDAG Testnet Setup
Write-Host "=== FinDAG Single Node Testnet Setup ===" -ForegroundColor Magenta
Write-Host ""

# Clean up any existing state
Write-Host "Cleaning up existing state..." -ForegroundColor Yellow
if (Test-Path "state_db") {
    Remove-Item -Recurse -Force "state_db"
    Write-Host "Removed existing state_db directory" -ForegroundColor Green
}

# Step 1: Start the node
Write-Host "Step 1: Starting FinDAG node..." -ForegroundColor Green
Write-Host ""

# Set environment variables for the node
$env:FINDAG_HTTP_PORT = "3001"
$env:FINDAG_UDP_PORT = "9001"
$env:FINDAG_DATA_DIR = "state_db"

# Start the node in the background
Write-Host "Starting FinDAG node on port 3001..." -ForegroundColor Yellow
$nodeProcess = Start-Process -FilePath "cargo" -ArgumentList "run", "--release" -PassThru -WindowStyle Hidden

# Wait for node to start
Write-Host "Waiting for node to start..." -ForegroundColor Yellow
Start-Sleep -Seconds 10

# Step 2: Initialize genesis
Write-Host ""
Write-Host "Step 2: Initializing genesis..." -ForegroundColor Green

# Initialize genesis with faucet account
Write-Host "Creating faucet account with 1,000,000 USD..." -ForegroundColor Yellow
cargo run --bin initialize_genesis -- --node-url http://127.0.0.1:3001 --faucet-amount 1000000

# Fund the bot account
Write-Host "Funding bot account with 10,000 USD..." -ForegroundColor Yellow
cargo run --bin fund_accounts -- --node-url http://127.0.0.1:3001 --amount 10000

# Step 3: Start transaction bot
Write-Host ""
Write-Host "Step 3: Starting transaction bot..." -ForegroundColor Green

# Start the transaction bot in the background
Write-Host "Starting transaction bot..." -ForegroundColor Yellow
$botProcess = Start-Process -FilePath "cargo" -ArgumentList "run", "--bin", "transaction_bot", "--", "start", "--node-url", "http://127.0.0.1:3001", "--interval-ms", "1000" -PassThru -WindowStyle Hidden

# Step 4: Monitor
Write-Host ""
Write-Host "Step 4: Monitoring..." -ForegroundColor Green
Write-Host "FinDAG single node testnet is running!" -ForegroundColor Magenta
Write-Host ""
Write-Host "Node Information:" -ForegroundColor Cyan
Write-Host "  HTTP API: http://localhost:3001" -ForegroundColor White
Write-Host "  P2P Port: 9001" -ForegroundColor White
Write-Host "  Data Directory: state_db" -ForegroundColor White
Write-Host ""
Write-Host "Available endpoints:" -ForegroundColor Cyan
Write-Host "  Health: http://localhost:3001/health" -ForegroundColor White
Write-Host "  Node Info: http://localhost:3001/node" -ForegroundColor White
Write-Host "  DAG Stats: http://localhost:3001/dag" -ForegroundColor White
Write-Host "  Mempool: http://localhost:3001/mempool-status" -ForegroundColor White
Write-Host ""
Write-Host "Press Ctrl+C to stop all processes" -ForegroundColor Red

# Wait for user to stop
try {
    while ($true) {
        Start-Sleep -Seconds 1
    }
}
finally {
    Write-Host ""
    Write-Host "Stopping FinDAG testnet..." -ForegroundColor Yellow
    
    # Stop the processes
    if ($nodeProcess -and !$nodeProcess.HasExited) {
        Stop-Process -Id $nodeProcess.Id -Force
        Write-Host "Stopped FinDAG node" -ForegroundColor Green
    }
    
    if ($botProcess -and !$botProcess.HasExited) {
        Stop-Process -Id $botProcess.Id -Force
        Write-Host "Stopped transaction bot" -ForegroundColor Green
    }
    
    Write-Host "FinDAG testnet stopped" -ForegroundColor Green
} 
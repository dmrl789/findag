# FinDAG Transaction Viewer Script
# Simple script to view transactions in your FinDAG node

Write-Host "FinDAG Transaction Viewer" -ForegroundColor Green
Write-Host "=============================" -ForegroundColor Green

$NODE_URL = "http://127.0.0.1:3000"

# Helper to safely run a web request and print error if it fails
function Safe-Invoke {
    param(
        [string]$Uri,
        [string]$Description
    )
    try {
        $response = Invoke-WebRequest -Uri $Uri -Method GET -TimeoutSec 10 -UseBasicParsing -ErrorAction Stop
        return $response.Content | ConvertFrom-Json
    } catch {
        Write-Host "Failed to get $($Description): $($_.Exception.Message)" -ForegroundColor Red
        return $null
    }
}

# Test node health
Write-Host "\nChecking node health..." -ForegroundColor Cyan
try {
    $response = Invoke-WebRequest -Uri "$NODE_URL/health" -Method GET -TimeoutSec 5 -UseBasicParsing
    Write-Host "Node is healthy!" -ForegroundColor Green
} catch {
    Write-Host "Node is not responding. Please start the FinDAG node first." -ForegroundColor Red
    Write-Host "Run: cargo run --bin findag" -ForegroundColor Yellow
    exit 1
}

# Get mempool status
Write-Host "\nGetting mempool status..." -ForegroundColor Cyan
$mempool_status = Safe-Invoke -Uri "$NODE_URL/mempool-status" -Description "mempool status"
if ($mempool_status) {
    Write-Host "Mempool size: $($mempool_status.mempool_size)" -ForegroundColor Green
    Write-Host "TX pool size: $($mempool_status.tx_pool_size)" -ForegroundColor Green
    Write-Host "Timestamp: $($mempool_status.timestamp)" -ForegroundColor Gray
}

# Get mempool transactions
Write-Host "\nGetting mempool transactions..." -ForegroundColor Cyan
$mempool_txs = Safe-Invoke -Uri "$NODE_URL/mempool-transactions" -Description "mempool transactions"
if ($mempool_txs) {
    Write-Host "Found $($mempool_txs.transactions.Count) transactions in mempool" -ForegroundColor Green
    if ($mempool_txs.transactions.Count -gt 0) {
        Write-Host "\nSample transactions:" -ForegroundColor Cyan
        $mempool_txs.transactions | Select-Object -First 5 | ForEach-Object {
            Write-Host "   From: $($_.from) -> To: $($_.to) Amount: $($_.amount)" -ForegroundColor White
        }
    }
}

# Get simple transactions
Write-Host "\nGetting simple transactions..." -ForegroundColor Cyan
$simple_txs = Safe-Invoke -Uri "$NODE_URL/simple-transactions" -Description "simple transactions"
if ($simple_txs) {
    Write-Host "Found $($simple_txs.transactions.Count) transactions" -ForegroundColor Green
    if ($simple_txs.transactions.Count -gt 0) {
        Write-Host "\nSample transactions:" -ForegroundColor Cyan
        $simple_txs.transactions | Select-Object -First 5 | ForEach-Object {
            Write-Host "   From: $($_.from) -> To: $($_.to) Amount: $($_.amount) Status: $($_.status)" -ForegroundColor White
        }
    }
}

Write-Host "\nTransaction viewing complete!" -ForegroundColor Green
Write-Host "To see real-time transactions, open test-transactions.html in your browser" -ForegroundColor Yellow 
# FinDAG Windows Startup Script
# For two laptops: Laptop 1 (NY) at 192.168.1.44, Laptop 2 (London) at 192.168.1.20

# Configuration
$LAPTOP1_IP = "192.168.1.44"
$LAPTOP2_IP = "192.168.1.20"
$HTTP_PORT = 8080
$P2P_PORT = 9000

# Get current laptop IP
$currentIP = (Get-NetIPAddress -AddressFamily IPv4 | Where-Object {$_.IPAddress -like "192.168.1.*"}).IPAddress
if (-not $currentIP) {
    $currentIP = (Get-NetIPAddress -AddressFamily IPv4 | Where-Object {$_.IPAddress -notlike "127.*" -and $_.IPAddress -notlike "169.*"}).IPAddress[0]
}

Write-Host "🚀 Starting FinDAG Node..." -ForegroundColor Green
Write-Host "📍 Current IP: $currentIP" -ForegroundColor Yellow

# Determine which laptop this is and set configuration
if ($currentIP -eq $LAPTOP1_IP) {
    $NODE_NAME = "NY-Laptop1"
    $PEERS = "$LAPTOP2_IP`:$P2P_PORT"
    Write-Host "🏙️  Detected: Laptop 1 (NY)" -ForegroundColor Cyan
} elseif ($currentIP -eq $LAPTOP2_IP) {
    $NODE_NAME = "London-Laptop2"
    $PEERS = "$LAPTOP1_IP`:$P2P_PORT"
    Write-Host "🇬🇧 Detected: Laptop 2 (London)" -ForegroundColor Cyan
} else {
    Write-Host "⚠️  Warning: IP $currentIP doesn't match expected laptop IPs" -ForegroundColor Red
    Write-Host "   Expected: $LAPTOP1_IP or $LAPTOP2_IP" -ForegroundColor Red
    Write-Host "   Continuing with default configuration..." -ForegroundColor Yellow
    $NODE_NAME = "Unknown-Node"
    $PEERS = "$LAPTOP1_IP`:$P2P_PORT,$LAPTOP2_IP`:$P2P_PORT"
}

Write-Host "📡 HTTP API: http://0.0.0.0:$HTTP_PORT" -ForegroundColor Green
Write-Host "🌐 P2P Bind: 0.0.0.0:$P2P_PORT" -ForegroundColor Green
Write-Host "👥 Peers: $PEERS" -ForegroundColor Green
Write-Host "🏷️  Node Name: $NODE_NAME" -ForegroundColor Green
Write-Host ""

# Check if FinDAG binary exists
$findagPath = ".\target\release\findag.exe"
if (-not (Test-Path $findagPath)) {
    Write-Host "❌ FinDAG binary not found at $findagPath" -ForegroundColor Red
    Write-Host "   Please run 'cargo build --release' first" -ForegroundColor Yellow
    exit 1
}

# Set environment variables
$env:FINDAG_HTTP_PORT = $HTTP_PORT
$env:FINDAG_PEERS = $PEERS
$env:FINDAG_UDP_PORT = $P2P_PORT
$env:NODE_ID = $NODE_NAME

Write-Host "✅ Environment variables set:" -ForegroundColor Green
Write-Host "   FINDAG_HTTP_PORT = $env:FINDAG_HTTP_PORT" -ForegroundColor White
Write-Host "   FINDAG_PEERS = $env:FINDAG_PEERS" -ForegroundColor White
Write-Host "   FINDAG_UDP_PORT = $env:FINDAG_UDP_PORT" -ForegroundColor White
Write-Host "   NODE_ID = $env:NODE_ID" -ForegroundColor White
Write-Host ""

Write-Host "🚀 Starting FinDAG node..." -ForegroundColor Green
Write-Host "   Press Ctrl+C to stop" -ForegroundColor Yellow
Write-Host ""

# Start FinDAG
try {
    & $findagPath
} catch {
    Write-Host "❌ Failed to start FinDAG: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
} 
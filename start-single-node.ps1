# Start a single FinDAG node
Write-Host "Starting single FinDAG node..." -ForegroundColor Green

# Set environment variables for the node
$env:FINDAG_HTTP_PORT = "3001"
$env:FINDAG_UDP_PORT = "9001"
$env:FINDAG_DATA_DIR = "state_db"

# Build and run the node
Write-Host "Building and starting FinDAG node on port 3001..." -ForegroundColor Yellow
cargo run --release

Write-Host "Node started successfully!" -ForegroundColor Green
Write-Host "HTTP API: http://localhost:3001" -ForegroundColor Cyan
Write-Host "P2P Port: 9001" -ForegroundColor Cyan
Write-Host "Data Directory: state_db" -ForegroundColor Cyan 
# Start transaction bot for single node
Write-Host "Starting transaction bot..." -ForegroundColor Green

# Set environment variables
$env:FINDAG_HTTP_PORT = "3001"

# Start the transaction bot
Write-Host "Starting transaction bot..." -ForegroundColor Yellow
cargo run --bin transaction_bot -- start --node-url http://127.0.0.1:3001 --interval-ms 1000

Write-Host "Transaction bot started!" -ForegroundColor Green
Write-Host "Sending transactions to http://127.0.0.1:3001" -ForegroundColor Cyan
Write-Host "Interval: 1000ms" -ForegroundColor Cyan 
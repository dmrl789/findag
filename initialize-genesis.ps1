# Initialize genesis and fund bot account for single node
Write-Host "Initializing genesis and funding bot account..." -ForegroundColor Green

# Set environment variables
$env:FINDAG_HTTP_PORT = "3001"
$env:FINDAG_DATA_DIR = "state_db"

# Wait for node to be ready
Write-Host "Waiting for node to be ready..." -ForegroundColor Yellow
Start-Sleep -Seconds 3

# Initialize genesis with faucet account
Write-Host "Initializing genesis..." -ForegroundColor Yellow
cargo run --bin initialize_genesis -- --node-url http://127.0.0.1:3001 --faucet-amount 1000000

# Fund the bot account
Write-Host "Funding bot account..." -ForegroundColor Yellow
cargo run --bin fund_accounts -- --node-url http://127.0.0.1:3001 --amount 10000

Write-Host "Genesis initialization complete!" -ForegroundColor Green
Write-Host "Faucet account funded with 1,000,000 USD" -ForegroundColor Cyan
Write-Host "Bot account funded with 10,000 USD" -ForegroundColor Cyan 
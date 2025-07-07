# FinDAG Security Test Script
# This script runs the security test suite against a test server

param(
    [string]$ServerPort = "8080",
    [switch]$StartServer,
    [switch]$RunTests
)

Write-Host "🔒 FinDAG Security Test Suite" -ForegroundColor Green
Write-Host "==============================" -ForegroundColor Green

# Set up environment variables for testing
$env:ADMIN_USERNAME = "admin"
$env:ADMIN_PASSWORD_HASH = "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8"  # admin123
$env:JWT_SECRET = "test_jwt_secret_for_testing_only_change_in_production"
$env:AUDIT_LOG_PATH = "logs\audit.log"

# Create logs directory if it doesn't exist
if (!(Test-Path "logs")) {
    New-Item -ItemType Directory -Path "logs" | Out-Null
}

if ($StartServer) {
    Write-Host "🚀 Starting test server on port $ServerPort..." -ForegroundColor Cyan
    
    # Start the server in background
    $serverProcess = Start-Process -FilePath "cargo" -ArgumentList "run", "--bin", "findag" -PassThru -WindowStyle Hidden
    
    # Wait for server to start
    Write-Host "⏳ Waiting for server to start..." -ForegroundColor Yellow
    Start-Sleep -Seconds 5
    
    Write-Host "✅ Server started (PID: $($serverProcess.Id))" -ForegroundColor Green
    Write-Host "Server URL: http://localhost:$ServerPort" -ForegroundColor Green
}

if ($RunTests) {
    Write-Host "🧪 Running security tests..." -ForegroundColor Cyan
    
    # Build the security test
    Write-Host "Building security test..." -ForegroundColor Yellow
    cargo build --bin security_test
    
    if ($LASTEXITCODE -eq 0) {
        # Run the security test
        Write-Host "Running security test suite..." -ForegroundColor Yellow
        cargo run --bin security_test
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ Security tests completed successfully!" -ForegroundColor Green
        } else {
            Write-Host "❌ Security tests failed!" -ForegroundColor Red
        }
    } else {
        Write-Host "❌ Failed to build security test!" -ForegroundColor Red
    }
}

# If both flags are provided, run tests then stop server
if ($StartServer -and $RunTests) {
    Write-Host "🛑 Stopping test server..." -ForegroundColor Cyan
    if (Get-Variable -Name "serverProcess" -ErrorAction SilentlyContinue) {
        Stop-Process -Id $serverProcess.Id -Force
        Write-Host "✅ Server stopped" -ForegroundColor Green
    }
}

Write-Host "`n📊 Security Test Summary:" -ForegroundColor Cyan
Write-Host "  - Authentication: JWT-based with replay protection" -ForegroundColor White
Write-Host "  - Rate Limiting: Login and API endpoints" -ForegroundColor White
Write-Host "  - Input Validation: Address, amount, currency, XSS protection" -ForegroundColor White
Write-Host "  - Protected Endpoints: Admin and validator operations" -ForegroundColor White
Write-Host "  - CORS Protection: Cross-origin request handling" -ForegroundColor White
Write-Host "  - Request Size Limits: 1MB maximum payload" -ForegroundColor White
Write-Host "  - Audit Logging: All security events logged" -ForegroundColor White

Write-Host "`n📝 Check audit.log for detailed security events" -ForegroundColor Yellow 
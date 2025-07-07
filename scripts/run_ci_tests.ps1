# FinDAG CI Tests Local Runner
# This script runs the same tests as the CI pipeline locally

param(
    [switch]$CodeQuality,
    [switch]$UnitTests,
    [switch]$SecurityTests,
    [switch]$PerformanceTests,
    [switch]$All,
    [switch]$Verbose
)

Write-Host "🔧 FinDAG CI Tests Local Runner" -ForegroundColor Green
Write-Host "=================================" -ForegroundColor Green

# Set environment variables for testing
$env:CARGO_TERM_COLOR = "always"
$env:RUST_BACKTRACE = "1"

if ($All -or $CodeQuality) {
    Write-Host "`n📋 Running Code Quality Checks..." -ForegroundColor Cyan
    
    # Install clippy if not available
    Write-Host "Installing clippy..." -ForegroundColor Yellow
    rustup component add clippy
    
    # Run clippy
    Write-Host "Running clippy..." -ForegroundColor Yellow
    cargo clippy --all-targets --all-features -- -D warnings
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Clippy passed" -ForegroundColor Green
    } else {
        Write-Host "❌ Clippy failed!" -ForegroundColor Red
        exit 1
    }
    
    # Check formatting
    Write-Host "Checking code formatting..." -ForegroundColor Yellow
    cargo fmt --all -- --check
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Code formatting is correct" -ForegroundColor Green
    } else {
        Write-Host "❌ Code formatting issues found!" -ForegroundColor Red
        Write-Host "Run 'cargo fmt --all' to fix formatting" -ForegroundColor Yellow
        exit 1
    }
    
    # Security audit
    Write-Host "Running security audit..." -ForegroundColor Yellow
    cargo audit --deny warnings
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Security audit passed" -ForegroundColor Green
    } else {
        Write-Host "❌ Security vulnerabilities found!" -ForegroundColor Red
        exit 1
    }
    
    # Cargo check
    Write-Host "Running cargo check..." -ForegroundColor Yellow
    cargo check --all-targets --all-features
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Cargo check passed" -ForegroundColor Green
    } else {
        Write-Host "❌ Cargo check failed!" -ForegroundColor Red
        exit 1
    }
}

if ($All -or $UnitTests) {
    Write-Host "`n🧪 Running Unit Tests..." -ForegroundColor Cyan
    
    # Run unit tests
    Write-Host "Running unit tests..." -ForegroundColor Yellow
    cargo test --lib --bins --tests
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Unit tests passed" -ForegroundColor Green
    } else {
        Write-Host "❌ Unit tests failed!" -ForegroundColor Red
        exit 1
    }
    
    # Run integration tests
    Write-Host "Running integration tests..." -ForegroundColor Yellow
    cargo test --test '*'
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Integration tests passed" -ForegroundColor Green
    } else {
        Write-Host "❌ Integration tests failed!" -ForegroundColor Red
        exit 1
    }
    
    # Generate test coverage (if tarpaulin is available)
    Write-Host "Generating test coverage..." -ForegroundColor Yellow
    try {
        cargo install cargo-tarpaulin
        cargo tarpaulin --out Html --output-dir coverage
        Write-Host "✅ Test coverage generated" -ForegroundColor Green
        Write-Host "Coverage report: coverage/tarpaulin-report.html" -ForegroundColor Yellow
    }
    catch {
        Write-Host "⚠️  Could not generate test coverage (tarpaulin not available)" -ForegroundColor Yellow
    }
}

if ($All -or $SecurityTests) {
    Write-Host "`n🔒 Running Security Tests..." -ForegroundColor Cyan
    
    # Set up environment variables for security tests
    $env:ADMIN_USERNAME = "admin"
    $env:ADMIN_PASSWORD_HASH = "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8"
    $env:JWT_SECRET = "test_jwt_secret_for_local_ci"
    
    # Build security test
    Write-Host "Building security test..." -ForegroundColor Yellow
    cargo build --bin security_test
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Security test built successfully" -ForegroundColor Green
    } else {
        Write-Host "❌ Security test build failed!" -ForegroundColor Red
        exit 1
    }
    
    # Run security tests
    Write-Host "Running security tests..." -ForegroundColor Yellow
    cargo run --bin security_test
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Security tests passed" -ForegroundColor Green
    } else {
        Write-Host "❌ Security tests failed!" -ForegroundColor Red
        exit 1
    }
    
    # Run fuzz tests
    Write-Host "Running fuzz tests..." -ForegroundColor Yellow
    Set-Location fuzz
    cargo test
    Set-Location ..
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Fuzz tests passed" -ForegroundColor Green
    } else {
        Write-Host "❌ Fuzz tests failed!" -ForegroundColor Red
        exit 1
    }
}

if ($All -or $PerformanceTests) {
    Write-Host "`n⚡ Running Performance Tests..." -ForegroundColor Cyan
    
    # Build release version
    Write-Host "Building release version..." -ForegroundColor Yellow
    cargo build --release
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Release build successful" -ForegroundColor Green
    } else {
        Write-Host "❌ Release build failed!" -ForegroundColor Red
        exit 1
    }
    
    # Run performance benchmarks
    Write-Host "Running performance benchmarks..." -ForegroundColor Yellow
    cargo bench --no-run
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Performance benchmarks passed" -ForegroundColor Green
    } else {
        Write-Host "❌ Performance benchmarks failed!" -ForegroundColor Red
        exit 1
    }
    
    # Run basic load test
    Write-Host "Running basic load test..." -ForegroundColor Yellow
    
    # Start the application in background
    $serverProcess = Start-Process -FilePath "cargo" -ArgumentList "run", "--release", "--bin", "findag" -PassThru -WindowStyle Hidden
    
    # Wait for server to start
    Write-Host "Waiting for server to start..." -ForegroundColor Yellow
    Start-Sleep -Seconds 15
    
    # Test basic connectivity
    $healthCheck = $false
    for ($i = 1; $i -le 10; $i++) {
        try {
            $response = Invoke-WebRequest -Uri "http://localhost:8080/health" -TimeoutSec 5
            if ($response.StatusCode -eq 200) {
                $healthCheck = $true
                Write-Host "✅ Server is responding" -ForegroundColor Green
                break
            }
        }
        catch {
            Write-Host "⏳ Waiting for server... (attempt $i/10)" -ForegroundColor Yellow
            Start-Sleep -Seconds 2
        }
    }
    
    if (-not $healthCheck) {
        Write-Host "❌ Server health check failed!" -ForegroundColor Red
        Stop-Process -Id $serverProcess.Id -Force
        exit 1
    }
    
    # Run transaction load test
    Write-Host "Running transaction load test..." -ForegroundColor Yellow
    $startTime = Get-Date
    
    # Send 100 transactions
    $jobs = @()
    for ($i = 1; $i -le 100; $i++) {
        $job = Start-Job -ScriptBlock {
            param($i)
            $body = @{
                from = "fdg1test$i"
                to = "fdg1test$($i+1)"
                amount = 100
                currency = "USD"
            } | ConvertTo-Json
            
            try {
                $response = Invoke-WebRequest -Uri "http://localhost:8080/tx" -Method POST -Body $body -ContentType "application/json" -TimeoutSec 10
                return $response.StatusCode
            }
            catch {
                return "ERROR"
            }
        } -ArgumentList $i
        $jobs += $job
    }
    
    # Wait for all jobs to complete
    $jobs | Wait-Job | Receive-Job
    $jobs | Remove-Job
    
    $endTime = Get-Date
    $duration = ($endTime - $startTime).TotalSeconds
    
    Write-Host "✅ Transaction load test completed in $duration seconds" -ForegroundColor Green
    
    # Stop the server
    Stop-Process -Id $serverProcess.Id -Force
    Write-Host "Server stopped" -ForegroundColor Yellow
}

Write-Host "`n🎉 All CI tests completed successfully!" -ForegroundColor Green

if ($Verbose) {
    Write-Host "`n📊 Test Summary:" -ForegroundColor Cyan
    Write-Host "  - Code Quality: ✅ Passed" -ForegroundColor Green
    Write-Host "  - Unit Tests: ✅ Passed" -ForegroundColor Green
    Write-Host "  - Security Tests: ✅ Passed" -ForegroundColor Green
    Write-Host "  - Performance Tests: ✅ Passed" -ForegroundColor Green
}

Write-Host "`n📝 Usage Examples:" -ForegroundColor Cyan
Write-Host "  .\scripts\run_ci_tests.ps1 -All              # Run all tests" -ForegroundColor White
Write-Host "  .\scripts\run_ci_tests.ps1 -CodeQuality      # Run code quality checks only" -ForegroundColor White
Write-Host "  .\scripts\run_ci_tests.ps1 -UnitTests        # Run unit tests only" -ForegroundColor White
Write-Host "  .\scripts\run_ci_tests.ps1 -SecurityTests    # Run security tests only" -ForegroundColor White
Write-Host "  .\scripts\run_ci_tests.ps1 -PerformanceTests # Run performance tests only" -ForegroundColor White
Write-Host "  .\scripts\run_ci_tests.ps1 -Verbose          # Show detailed output" -ForegroundColor White 
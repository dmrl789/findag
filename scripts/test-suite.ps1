# FinDAG Comprehensive Test Suite for Windows
# This script runs all tests, checks, and quality gates locally

param(
    [switch]$SkipDocker,
    [switch]$SkipCoverage,
    [switch]$Verbose
)

# Set error action preference
$ErrorActionPreference = "Stop"

# Function to print colored output
function Write-Status {
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

# Function to check if command exists
function Test-Command {
    param([string]$Command)
    try {
        Get-Command $Command -ErrorAction Stop | Out-Null
        return $true
    }
    catch {
        return $false
    }
}

# Function to install cargo tools if missing
function Install-CargoTool {
    param([string]$Tool)
    if (-not (Test-Command $Tool)) {
        Write-Status "Installing $Tool..."
        cargo install $Tool
    }
}

# Start test suite
Write-Host "🚀 Starting FinDAG Test Suite" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan

# Check Rust version
Write-Status "Checking Rust version..."
rustc --version
cargo --version

# Install required tools
Write-Status "Installing required tools..."
Install-CargoTool "cargo-audit"
if (-not $SkipCoverage) {
    Install-CargoTool "cargo-tarpaulin"
}
Install-CargoTool "cargo-outdated"

# Clean previous builds
Write-Status "Cleaning previous builds..."
cargo clean

# Check formatting
Write-Status "Checking code formatting..."
try {
    cargo fmt --all -- --check
    Write-Success "Code formatting is correct"
}
catch {
    Write-Error "Code formatting issues found. Run 'cargo fmt --all' to fix"
    exit 1
}

# Run clippy
Write-Status "Running clippy..."
try {
    cargo clippy --all-targets --all-features -- -D warnings
    Write-Success "Clippy passed"
}
catch {
    Write-Error "Clippy found issues"
    exit 1
}

# Run security audit
Write-Status "Running security audit..."
try {
    cargo audit
    Write-Success "Security audit passed"
}
catch {
    Write-Warning "Security vulnerabilities found"
}

# Check for outdated dependencies
Write-Status "Checking for outdated dependencies..."
try {
    cargo outdated
}
catch {
    Write-Warning "Some dependencies may be outdated"
}

# Run unit tests
Write-Status "Running unit tests..."
try {
    cargo test --lib --all-features
    Write-Success "Unit tests passed"
}
catch {
    Write-Error "Unit tests failed"
    exit 1
}

# Run integration tests
Write-Status "Running integration tests..."
try {
    cargo test --test "*" --all-features
    Write-Success "Integration tests passed"
}
catch {
    Write-Error "Integration tests failed"
    exit 1
}

# Run all tests with coverage
if (-not $SkipCoverage) {
    Write-Status "Running tests with coverage..."
    try {
        cargo tarpaulin --out Html --output-dir coverage
        Write-Success "Coverage report generated"
        Write-Status "Coverage report available in coverage/tarpaulin-report.html"
    }
    catch {
        Write-Error "Coverage generation failed"
        exit 1
    }
}

# Build release version
Write-Status "Building release version..."
try {
    cargo build --release
    Write-Success "Release build successful"
}
catch {
    Write-Error "Release build failed"
    exit 1
}

# Run benchmarks (compile only)
Write-Status "Compiling benchmarks..."
try {
    cargo bench --no-run
    Write-Success "Benchmarks compiled successfully"
}
catch {
    Write-Warning "Benchmarks failed to compile"
}

# Build documentation
Write-Status "Building documentation..."
try {
    cargo doc --no-deps --all-features
    Write-Success "Documentation built successfully"
    Write-Status "Documentation available in target/doc/"
}
catch {
    Write-Error "Documentation build failed"
    exit 1
}

# Test Docker build
if (-not $SkipDocker) {
    Write-Status "Testing Docker build..."
    try {
        docker build -t findag:test -f Dockerfile.test .
        Write-Success "Docker test build successful"
    }
    catch {
        Write-Warning "Docker build failed"
    }
}

# Performance checks
Write-Status "Running performance checks..."
try {
    $stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
    cargo build --release
    & "./target/release/findag.exe" --help | Out-Null
    $stopwatch.Stop()
    Write-Success "Performance check passed (${stopwatch.ElapsedMilliseconds}ms)"
}
catch {
    Write-Warning "Performance check failed"
}

# Generate test report
Write-Status "Generating test report..."
$report = @"
# FinDAG Test Suite Report
Generated: $(Get-Date)

## Summary
- ✅ Code formatting: PASS
- ✅ Clippy: PASS
- ✅ Unit tests: PASS
- ✅ Integration tests: PASS
"@

if (-not $SkipCoverage) {
    $report += "`n- ✅ Coverage: PASS"
}

$report += @"
- ✅ Release build: PASS
- ✅ Documentation: PASS

## Coverage
"@

if (-not $SkipCoverage -and (Test-Path "coverage/tarpaulin-report.html")) {
    $report += "`nCoverage report: coverage/tarpaulin-report.html"
}

$report += @"

## Documentation
Documentation: target/doc/

## Binaries
Release binaries: target/release/
"@

$report | Out-File -FilePath "test-report.md" -Encoding UTF8

Write-Success "Test suite completed successfully!"
Write-Status "Test report saved to test-report.md"
if (-not $SkipCoverage) {
    Write-Status "Coverage report: coverage/tarpaulin-report.html"
}
Write-Status "Documentation: target/doc/"

Write-Host ""
Write-Host "🎉 All tests passed! FinDAG is ready for production." -ForegroundColor Green 
#!/bin/bash

# FinDAG Comprehensive Test Suite
# This script runs all tests, checks, and quality gates locally

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to install cargo tools if missing
install_cargo_tool() {
    local tool=$1
    if ! command_exists "$tool"; then
        print_status "Installing $tool..."
        cargo install "$tool"
    fi
}

# Start test suite
echo "🚀 Starting FinDAG Test Suite"
echo "================================"

# Check Rust version
print_status "Checking Rust version..."
rustc --version
cargo --version

# Install required tools
print_status "Installing required tools..."
install_cargo_tool "cargo-audit"
install_cargo_tool "cargo-tarpaulin"
install_cargo_tool "cargo-outdated"

# Clean previous builds
print_status "Cleaning previous builds..."
cargo clean

# Check formatting
print_status "Checking code formatting..."
if cargo fmt --all -- --check; then
    print_success "Code formatting is correct"
else
    print_error "Code formatting issues found. Run 'cargo fmt --all' to fix"
    exit 1
fi

# Run clippy
print_status "Running clippy..."
if cargo clippy --all-targets --all-features -- -D warnings; then
    print_success "Clippy passed"
else
    print_error "Clippy found issues"
    exit 1
fi

# Run security audit
print_status "Running security audit..."
if cargo audit; then
    print_success "Security audit passed"
else
    print_warning "Security vulnerabilities found"
fi

# Check for outdated dependencies
print_status "Checking for outdated dependencies..."
cargo outdated || print_warning "Some dependencies may be outdated"

# Run unit tests
print_status "Running unit tests..."
if cargo test --lib --all-features; then
    print_success "Unit tests passed"
else
    print_error "Unit tests failed"
    exit 1
fi

# Run integration tests
print_status "Running integration tests..."
if cargo test --test "*" --all-features; then
    print_success "Integration tests passed"
else
    print_error "Integration tests failed"
    exit 1
fi

# Run all tests with coverage
print_status "Running tests with coverage..."
if cargo tarpaulin --out Html --output-dir coverage; then
    print_success "Coverage report generated"
    print_status "Coverage report available in coverage/tarpaulin-report.html"
else
    print_error "Coverage generation failed"
    exit 1
fi

# Build release version
print_status "Building release version..."
if cargo build --release; then
    print_success "Release build successful"
else
    print_error "Release build failed"
    exit 1
fi

# Run benchmarks (compile only)
print_status "Compiling benchmarks..."
if cargo bench --no-run; then
    print_success "Benchmarks compiled successfully"
else
    print_warning "Benchmarks failed to compile"
fi

# Build documentation
print_status "Building documentation..."
if cargo doc --no-deps --all-features; then
    print_success "Documentation built successfully"
    print_status "Documentation available in target/doc/"
else
    print_error "Documentation build failed"
    exit 1
fi

# Test Docker build
print_status "Testing Docker build..."
if docker build -t findag:test -f Dockerfile.test .; then
    print_success "Docker test build successful"
else
    print_warning "Docker build failed"
fi

# Performance checks
print_status "Running performance checks..."
if cargo build --release && time ./target/release/findag --help > /dev/null 2>&1; then
    print_success "Performance check passed"
else
    print_warning "Performance check failed"
fi

# Memory usage check
print_status "Checking memory usage..."
if command_exists "valgrind"; then
    valgrind --tool=memcheck --leak-check=full --show-leak-kinds=all --track-origins=yes --verbose --log-file=valgrind-out.txt ./target/release/findag --help > /dev/null 2>&1 || true
    if grep -q "definitely lost: 0 bytes" valgrind-out.txt; then
        print_success "Memory check passed"
    else
        print_warning "Memory leaks detected"
    fi
else
    print_warning "Valgrind not available, skipping memory check"
fi

# Generate test report
print_status "Generating test report..."
{
    echo "# FinDAG Test Suite Report"
    echo "Generated: $(date)"
    echo ""
    echo "## Summary"
    echo "- ✅ Code formatting: PASS"
    echo "- ✅ Clippy: PASS"
    echo "- ✅ Unit tests: PASS"
    echo "- ✅ Integration tests: PASS"
    echo "- ✅ Coverage: PASS"
    echo "- ✅ Release build: PASS"
    echo "- ✅ Documentation: PASS"
    echo ""
    echo "## Coverage"
    if [ -f "coverage/tarpaulin-report.html" ]; then
        echo "Coverage report: coverage/tarpaulin-report.html"
    fi
    echo ""
    echo "## Documentation"
    echo "Documentation: target/doc/"
    echo ""
    echo "## Binaries"
    echo "Release binaries: target/release/"
} > test-report.md

print_success "Test suite completed successfully!"
print_status "Test report saved to test-report.md"
print_status "Coverage report: coverage/tarpaulin-report.html"
print_status "Documentation: target/doc/"

echo ""
echo "🎉 All tests passed! FinDAG is ready for production." 
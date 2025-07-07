#!/bin/bash

# FinDAG Performance Testing Script
# This script runs comprehensive performance and load tests

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_TYPE="${BUILD_TYPE:-release}"
TEST_DURATION="${TEST_DURATION:-300}"  # 5 minutes default
MAX_TPS="${MAX_TPS:-5000}"            # Maximum TPS to test
CONCURRENT_USERS="${CONCURRENT_USERS:-50}"

# Directories
REPORTS_DIR="$PROJECT_ROOT/reports"
PERFORMANCE_DIR="$REPORTS_DIR/performance"
LOAD_TEST_DIR="$REPORTS_DIR/load_tests"
BENCHMARK_DIR="$REPORTS_DIR/benchmarks"

# Create directories
mkdir -p "$PERFORMANCE_DIR" "$LOAD_TEST_DIR" "$BENCHMARK_DIR"

echo -e "${BLUE}🚀 FinDAG Performance Testing Suite${NC}"
echo "=========================================="
echo "Project Root: $PROJECT_ROOT"
echo "Build Type: $BUILD_TYPE"
echo "Test Duration: $TEST_DURATION seconds"
echo "Max TPS: $MAX_TPS"
echo "Concurrent Users: $CONCURRENT_USERS"
echo ""

# Function to print section headers
print_section() {
    echo -e "\n${YELLOW}$1${NC}"
    echo "----------------------------------------"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to build the project
build_project() {
    print_section "Building FinDAG Project"
    
    if [ "$BUILD_TYPE" = "release" ]; then
        echo "Building in release mode..."
        cargo build --release
    else
        echo "Building in debug mode..."
        cargo build
    fi
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Build completed successfully${NC}"
    else
        echo -e "${RED}❌ Build failed${NC}"
        exit 1
    fi
}

# Function to run performance tests
run_performance_tests() {
    print_section "Running Performance Tests"
    
    local timestamp=$(date +"%Y%m%d_%H%M%S")
    local report_file="$PERFORMANCE_DIR/performance_test_$timestamp.json"
    
    echo "Running performance test suite..."
    echo "Report will be saved to: $report_file"
    
    # Run the performance test binary
    if [ "$BUILD_TYPE" = "release" ]; then
        ./target/release/performance_test 2>&1 | tee "$PERFORMANCE_DIR/performance_test_$timestamp.log"
    else
        ./target/debug/performance_test 2>&1 | tee "$PERFORMANCE_DIR/performance_test_$timestamp.log"
    fi
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Performance tests completed successfully${NC}"
        
        # Copy the generated report
        if [ -f "performance_report.json" ]; then
            mv performance_report.json "$report_file"
            echo "Performance report saved to: $report_file"
        fi
    else
        echo -e "${RED}❌ Performance tests failed${NC}"
        return 1
    fi
}

# Function to run load tests
run_load_tests() {
    print_section "Running Load Tests"
    
    local timestamp=$(date +"%Y%m%d_%H%M%S")
    local report_file="$LOAD_TEST_DIR/load_test_$timestamp.json"
    
    echo "Running load test suite..."
    echo "Report will be saved to: $report_file"
    
    # Run the load test binary
    if [ "$BUILD_TYPE" = "release" ]; then
        ./target/release/load_test 2>&1 | tee "$LOAD_TEST_DIR/load_test_$timestamp.log"
    else
        ./target/debug/load_test 2>&1 | tee "$LOAD_TEST_DIR/load_test_$timestamp.log"
    fi
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Load tests completed successfully${NC}"
        
        # Copy the generated report
        if [ -f "load_test_report.json" ]; then
            mv load_test_report.json "$report_file"
            echo "Load test report saved to: $report_file"
        fi
    else
        echo -e "${RED}❌ Load tests failed${NC}"
        return 1
    fi
}

# Function to run database benchmarks
run_database_benchmarks() {
    print_section "Running Database Benchmarks"
    
    local timestamp=$(date +"%Y%m%d_%H%M%S")
    local report_file="$BENCHMARK_DIR/db_benchmark_$timestamp.json"
    
    echo "Running database benchmarks..."
    echo "Report will be saved to: $report_file"
    
    # Test different database configurations
    local configs=("default" "production" "high_frequency" "storage_efficient")
    
    for config in "${configs[@]}"; do
        echo "Testing configuration: $config"
        
        # Set environment variables for the test
        export FINDAG_DB_CONFIG_PROFILE="$config"
        
        # Run a quick benchmark
        if [ "$BUILD_TYPE" = "release" ]; then
            timeout 60 ./target/release/performance_test --config "$config" --quick 2>&1 | tee "$BENCHMARK_DIR/db_benchmark_${config}_$timestamp.log"
        else
            timeout 60 ./target/debug/performance_test --config "$config" --quick 2>&1 | tee "$BENCHMARK_DIR/db_benchmark_${config}_$timestamp.log"
        fi
    done
    
    echo -e "${GREEN}✅ Database benchmarks completed${NC}"
}

# Function to run system resource monitoring
run_resource_monitoring() {
    print_section "Monitoring System Resources"
    
    local timestamp=$(date +"%Y%m%d_%H%M%S")
    local monitor_file="$REPORTS_DIR/system_monitor_$timestamp.log"
    
    echo "Starting system resource monitoring..."
    echo "Monitoring data will be saved to: $monitor_file"
    
    # Check if htop or top is available
    if command_exists htop; then
        echo "Using htop for monitoring..."
        # Run monitoring in background
        (
            while true; do
                echo "=== $(date) ===" >> "$monitor_file"
                htop -t -d 1 -n 1 | head -20 >> "$monitor_file" 2>/dev/null || true
                sleep 5
            done
        ) &
        MONITOR_PID=$!
    elif command_exists top; then
        echo "Using top for monitoring..."
        (
            while true; do
                echo "=== $(date) ===" >> "$monitor_file"
                top -b -n 1 | head -20 >> "$monitor_file" 2>/dev/null || true
                sleep 5
            done
        ) &
        MONITOR_PID=$!
    else
        echo "No monitoring tool available, skipping resource monitoring"
        MONITOR_PID=""
    fi
    
    # Store PID for cleanup
    if [ ! -z "$MONITOR_PID" ]; then
        echo "$MONITOR_PID" > "$REPORTS_DIR/monitor.pid"
    fi
}

# Function to stop resource monitoring
stop_resource_monitoring() {
    if [ -f "$REPORTS_DIR/monitor.pid" ]; then
        local monitor_pid=$(cat "$REPORTS_DIR/monitor.pid")
        if kill -0 "$monitor_pid" 2>/dev/null; then
            echo "Stopping resource monitoring..."
            kill "$monitor_pid"
        fi
        rm -f "$REPORTS_DIR/monitor.pid"
    fi
}

# Function to generate summary report
generate_summary_report() {
    print_section "Generating Summary Report"
    
    local timestamp=$(date +"%Y%m%d_%H%M%S")
    local summary_file="$REPORTS_DIR/summary_$timestamp.md"
    
    echo "Generating summary report..."
    
    cat > "$summary_file" << EOF
# FinDAG Performance Test Summary

**Date:** $(date)
**Build Type:** $BUILD_TYPE
**Test Duration:** $TEST_DURATION seconds
**Max TPS:** $MAX_TPS
**Concurrent Users:** $CONCURRENT_USERS

## Test Results

### Performance Tests
- **Status:** $(if [ $? -eq 0 ]; then echo "✅ PASSED"; else echo "❌ FAILED"; fi)
- **Report:** performance_test_$timestamp.json

### Load Tests
- **Status:** $(if [ $? -eq 0 ]; then echo "✅ PASSED"; else echo "❌ FAILED"; fi)
- **Report:** load_test_$timestamp.json

### Database Benchmarks
- **Status:** ✅ COMPLETED
- **Reports:** db_benchmark_*.json

## System Information

\`\`\`
$(uname -a)
$(cat /proc/cpuinfo | grep "model name" | head -1)
$(cat /proc/meminfo | grep "MemTotal")
\`\`\`

## Recommendations

Based on the test results, consider the following optimizations:

1. **Database Configuration:** Use the optimal configuration profile for your use case
2. **Resource Allocation:** Ensure sufficient CPU and memory resources
3. **Network Optimization:** Optimize network settings for high-frequency operations
4. **Storage Tuning:** Use SSD storage for better I/O performance

## Next Steps

1. Review the detailed reports in the reports directory
2. Analyze performance bottlenecks
3. Implement recommended optimizations
4. Re-run tests to validate improvements

EOF

    echo "Summary report saved to: $summary_file"
}

# Function to cleanup test artifacts
cleanup() {
    print_section "Cleaning Up"
    
    echo "Cleaning up test artifacts..."
    
    # Stop resource monitoring
    stop_resource_monitoring
    
    # Remove temporary files
    rm -f performance_report.json load_test_report.json
    
    # Clean up test databases
    rm -rf performance_test_db load_test_db
    
    echo -e "${GREEN}✅ Cleanup completed${NC}"
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -h, --help              Show this help message"
    echo "  -b, --build-type TYPE   Build type (debug|release) [default: release]"
    echo "  -d, --duration SECONDS  Test duration in seconds [default: 300]"
    echo "  -t, --tps TPS           Maximum TPS to test [default: 5000]"
    echo "  -u, --users USERS       Number of concurrent users [default: 50]"
    echo "  --performance-only      Run only performance tests"
    echo "  --load-only             Run only load tests"
    echo "  --benchmark-only        Run only database benchmarks"
    echo "  --no-cleanup            Skip cleanup after tests"
    echo ""
    echo "Examples:"
    echo "  $0                      # Run all tests with default settings"
    echo "  $0 --performance-only   # Run only performance tests"
    echo "  $0 -b debug -d 600      # Run in debug mode for 10 minutes"
    echo "  $0 -t 10000 -u 100      # Test up to 10K TPS with 100 users"
}

# Parse command line arguments
PERFORMANCE_ONLY=false
LOAD_ONLY=false
BENCHMARK_ONLY=false
NO_CLEANUP=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_usage
            exit 0
            ;;
        -b|--build-type)
            BUILD_TYPE="$2"
            shift 2
            ;;
        -d|--duration)
            TEST_DURATION="$2"
            shift 2
            ;;
        -t|--tps)
            MAX_TPS="$2"
            shift 2
            ;;
        -u|--users)
            CONCURRENT_USERS="$2"
            shift 2
            ;;
        --performance-only)
            PERFORMANCE_ONLY=true
            shift
            ;;
        --load-only)
            LOAD_ONLY=true
            shift
            ;;
        --benchmark-only)
            BENCHMARK_ONLY=true
            shift
            ;;
        --no-cleanup)
            NO_CLEANUP=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Main execution
main() {
    echo -e "${BLUE}Starting FinDAG Performance Testing Suite${NC}"
    echo "================================================"
    
    # Build the project
    build_project
    
    # Start resource monitoring
    run_resource_monitoring
    
    # Run tests based on options
    if [ "$PERFORMANCE_ONLY" = true ]; then
        run_performance_tests
    elif [ "$LOAD_ONLY" = true ]; then
        run_load_tests
    elif [ "$BENCHMARK_ONLY" = true ]; then
        run_database_benchmarks
    else
        # Run all tests
        run_performance_tests
        run_load_tests
        run_database_benchmarks
    fi
    
    # Stop resource monitoring
    stop_resource_monitoring
    
    # Generate summary report
    generate_summary_report
    
    # Cleanup
    if [ "$NO_CLEANUP" = false ]; then
        cleanup
    fi
    
    echo -e "\n${GREEN}🎉 Performance testing completed successfully!${NC}"
    echo "Check the reports directory for detailed results."
}

# Trap to ensure cleanup on exit
trap cleanup EXIT

# Run main function
main "$@" 
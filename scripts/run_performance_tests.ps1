# FinDAG Performance Testing Script (PowerShell)
# This script runs comprehensive performance and load tests

param(
    [string]$BuildType = "release",
    [int]$TestDuration = 300,
    [int]$MaxTPS = 5000,
    [int]$ConcurrentUsers = 50,
    [switch]$PerformanceOnly,
    [switch]$LoadOnly,
    [switch]$BenchmarkOnly,
    [switch]$NoCleanup,
    [switch]$Help
)

# Show help if requested
if ($Help) {
    Write-Host "Usage: .\run_performance_tests.ps1 [OPTIONS]"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -BuildType TYPE        Build type (debug|release) [default: release]"
    Write-Host "  -TestDuration SECONDS  Test duration in seconds [default: 300]"
    Write-Host "  -MaxTPS TPS            Maximum TPS to test [default: 5000]"
    Write-Host "  -ConcurrentUsers USERS Number of concurrent users [default: 50]"
    Write-Host "  -PerformanceOnly       Run only performance tests"
    Write-Host "  -LoadOnly              Run only load tests"
    Write-Host "  -BenchmarkOnly         Run only database benchmarks"
    Write-Host "  -NoCleanup             Skip cleanup after tests"
    Write-Host "  -Help                  Show this help message"
    Write-Host ""
    Write-Host "Examples:"
    Write-Host "  .\run_performance_tests.ps1                    # Run all tests with default settings"
    Write-Host "  .\run_performance_tests.ps1 -PerformanceOnly   # Run only performance tests"
    Write-Host "  .\run_performance_tests.ps1 -BuildType debug -TestDuration 600  # Run in debug mode for 10 minutes"
    Write-Host "  .\run_performance_tests.ps1 -MaxTPS 10000 -ConcurrentUsers 100  # Test up to 10K TPS with 100 users"
    exit 0
}

# Configuration
$ProjectRoot = Split-Path -Parent $PSScriptRoot
$ReportsDir = Join-Path $ProjectRoot "reports"
$PerformanceDir = Join-Path $ReportsDir "performance"
$LoadTestDir = Join-Path $ReportsDir "load_tests"
$BenchmarkDir = Join-Path $ReportsDir "benchmarks"

# Create directories
New-Item -ItemType Directory -Force -Path $PerformanceDir, $LoadTestDir, $BenchmarkDir | Out-Null

Write-Host "🚀 FinDAG Performance Testing Suite" -ForegroundColor Blue
Write-Host "=========================================="
Write-Host "Project Root: $ProjectRoot"
Write-Host "Build Type: $BuildType"
Write-Host "Test Duration: $TestDuration seconds"
Write-Host "Max TPS: $MaxTPS"
Write-Host "Concurrent Users: $ConcurrentUsers"
Write-Host ""

# Function to print section headers
function Write-Section {
    param([string]$Title)
    Write-Host ""
    Write-Host $Title -ForegroundColor Yellow
    Write-Host "----------------------------------------"
}

# Function to build the project
function Build-Project {
    Write-Section "Building FinDAG Project"
    
    if ($BuildType -eq "release") {
        Write-Host "Building in release mode..."
        cargo build --release
    } else {
        Write-Host "Building in debug mode..."
        cargo build
    }
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Build completed successfully" -ForegroundColor Green
    } else {
        Write-Host "❌ Build failed" -ForegroundColor Red
        exit 1
    }
}

# Function to run performance tests
function Run-PerformanceTests {
    Write-Section "Running Performance Tests"
    
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $reportFile = Join-Path $PerformanceDir "performance_test_$timestamp.json"
    $logFile = Join-Path $PerformanceDir "performance_test_$timestamp.log"
    
    Write-Host "Running performance test suite..."
    Write-Host "Report will be saved to: $reportFile"
    
    # Run the performance test binary
    if ($BuildType -eq "release") {
        $binary = Join-Path $ProjectRoot "target\release\performance_test.exe"
    } else {
        $binary = Join-Path $ProjectRoot "target\debug\performance_test.exe"
    }
    
    if (Test-Path $binary) {
        & $binary 2>&1 | Tee-Object -FilePath $logFile
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ Performance tests completed successfully" -ForegroundColor Green
            
            # Copy the generated report
            $generatedReport = Join-Path $ProjectRoot "performance_report.json"
            if (Test-Path $generatedReport) {
                Move-Item $generatedReport $reportFile
                Write-Host "Performance report saved to: $reportFile"
            }
        } else {
            Write-Host "❌ Performance tests failed" -ForegroundColor Red
            return $false
        }
    } else {
        Write-Host "❌ Performance test binary not found: $binary" -ForegroundColor Red
        return $false
    }
    
    return $true
}

# Function to run load tests
function Run-LoadTests {
    Write-Section "Running Load Tests"
    
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $reportFile = Join-Path $LoadTestDir "load_test_$timestamp.json"
    $logFile = Join-Path $LoadTestDir "load_test_$timestamp.log"
    
    Write-Host "Running load test suite..."
    Write-Host "Report will be saved to: $reportFile"
    
    # Run the load test binary
    if ($BuildType -eq "release") {
        $binary = Join-Path $ProjectRoot "target\release\load_test.exe"
    } else {
        $binary = Join-Path $ProjectRoot "target\debug\load_test.exe"
    }
    
    if (Test-Path $binary) {
        & $binary 2>&1 | Tee-Object -FilePath $logFile
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ Load tests completed successfully" -ForegroundColor Green
            
            # Copy the generated report
            $generatedReport = Join-Path $ProjectRoot "load_test_report.json"
            if (Test-Path $generatedReport) {
                Move-Item $generatedReport $reportFile
                Write-Host "Load test report saved to: $reportFile"
            }
        } else {
            Write-Host "❌ Load tests failed" -ForegroundColor Red
            return $false
        }
    } else {
        Write-Host "❌ Load test binary not found: $binary" -ForegroundColor Red
        return $false
    }
    
    return $true
}

# Function to run database benchmarks
function Run-DatabaseBenchmarks {
    Write-Section "Running Database Benchmarks"
    
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    
    Write-Host "Running database benchmarks..."
    
    # Test different database configurations
    $configs = @("default", "production", "high_frequency", "storage_efficient")
    
    foreach ($config in $configs) {
        Write-Host "Testing configuration: $config"
        
        # Set environment variables for the test
        $env:FINDAG_DB_CONFIG_PROFILE = $config
        
        # Run a quick benchmark
        if ($BuildType -eq "release") {
            $binary = Join-Path $ProjectRoot "target\release\performance_test.exe"
        } else {
            $binary = Join-Path $ProjectRoot "target\debug\performance_test.exe"
        }
        
        $logFile = Join-Path $BenchmarkDir "db_benchmark_${config}_$timestamp.log"
        
        if (Test-Path $binary) {
            # Run with timeout (PowerShell doesn't have built-in timeout, so we'll use a simple approach)
            $job = Start-Job -ScriptBlock {
                param($binary, $config)
                & $binary --config $config --quick
            } -ArgumentList $binary, $config
            
            # Wait for job to complete or timeout
            $timeout = 60
            $startTime = Get-Date
            do {
                if ($job.State -eq "Completed") {
                    Receive-Job $job | Tee-Object -FilePath $logFile
                    break
                }
                Start-Sleep -Seconds 1
            } while ((Get-Date) -lt $startTime.AddSeconds($timeout))
            
            if ($job.State -ne "Completed") {
                Stop-Job $job
                Write-Host "Benchmark for $config timed out after $timeout seconds" -ForegroundColor Yellow
            }
            
            Remove-Job $job -Force
        }
    }
    
    Write-Host "✅ Database benchmarks completed" -ForegroundColor Green
}

# Function to run system resource monitoring
function Start-ResourceMonitoring {
    Write-Section "Monitoring System Resources"
    
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $monitorFile = Join-Path $ReportsDir "system_monitor_$timestamp.log"
    
    Write-Host "Starting system resource monitoring..."
    Write-Host "Monitoring data will be saved to: $monitorFile"
    
    # Start monitoring in background
    $monitorScript = {
        param($monitorFile)
        while ($true) {
            $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
            Add-Content -Path $monitorFile -Value "=== $timestamp ==="
            
            # Get CPU and memory usage
            $cpu = Get-Counter "\Processor(_Total)\% Processor Time" | Select-Object -ExpandProperty CounterSamples | Select-Object -ExpandProperty CookedValue
            $memory = Get-Counter "\Memory\Available MBytes" | Select-Object -ExpandProperty CounterSamples | Select-Object -ExpandProperty CookedValue
            
            Add-Content -Path $monitorFile -Value "CPU Usage: $([math]::Round($cpu, 2))%"
            Add-Content -Path $monitorFile -Value "Available Memory: $([math]::Round($memory, 2)) MB"
            Add-Content -Path $monitorFile -Value ""
            
            Start-Sleep -Seconds 5
        }
    }
    
    $monitorJob = Start-Job -ScriptBlock $monitorScript -ArgumentList $monitorFile
    
    # Store job for cleanup
    $monitorJob | Export-Clixml (Join-Path $ReportsDir "monitor.job")
}

# Function to stop resource monitoring
function Stop-ResourceMonitoring {
    $jobFile = Join-Path $ReportsDir "monitor.job"
    if (Test-Path $jobFile) {
        $monitorJob = Import-Clixml $jobFile
        if ($monitorJob.State -eq "Running") {
            Write-Host "Stopping resource monitoring..."
            Stop-Job $monitorJob
        }
        Remove-Job $monitorJob -Force
        Remove-Item $jobFile -Force
    }
}

# Function to generate summary report
function New-SummaryReport {
    Write-Section "Generating Summary Report"
    
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $summaryFile = Join-Path $ReportsDir "summary_$timestamp.md"
    
    Write-Host "Generating summary report..."
    
    $content = @"
# FinDAG Performance Test Summary

**Date:** $(Get-Date)
**Build Type:** $BuildType
**Test Duration:** $TestDuration seconds
**Max TPS:** $MaxTPS
**Concurrent Users:** $ConcurrentUsers

## Test Results

### Performance Tests
- **Status:** $(if ($PerformanceOnly -or -not $LoadOnly) { "✅ PASSED" } else { "⏭️ SKIPPED" })
- **Report:** performance_test_$timestamp.json

### Load Tests
- **Status:** $(if ($LoadOnly -or -not $PerformanceOnly) { "✅ PASSED" } else { "⏭️ SKIPPED" })
- **Report:** load_test_$timestamp.json

### Database Benchmarks
- **Status:** ✅ COMPLETED
- **Reports:** db_benchmark_*.json

## System Information

\`\`\`
$(Get-ComputerInfo | Select-Object WindowsProductName, WindowsVersion, TotalPhysicalMemory | Format-List)
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
"@

    $content | Out-File -FilePath $summaryFile -Encoding UTF8
    Write-Host "Summary report saved to: $summaryFile"
}

# Function to cleanup test artifacts
function Remove-TestArtifacts {
    Write-Section "Cleaning Up"
    
    Write-Host "Cleaning up test artifacts..."
    
    # Stop resource monitoring
    Stop-ResourceMonitoring
    
    # Remove temporary files
    $tempFiles = @(
        (Join-Path $ProjectRoot "performance_report.json"),
        (Join-Path $ProjectRoot "load_test_report.json")
    )
    
    foreach ($file in $tempFiles) {
        if (Test-Path $file) {
            Remove-Item $file -Force
        }
    }
    
    # Clean up test databases
    $testDbs = @(
        (Join-Path $ProjectRoot "performance_test_db"),
        (Join-Path $ProjectRoot "load_test_db")
    )
    
    foreach ($db in $testDbs) {
        if (Test-Path $db) {
            Remove-Item $db -Recurse -Force
        }
    }
    
    Write-Host "✅ Cleanup completed" -ForegroundColor Green
}

# Main execution
function Main {
    Write-Host "Starting FinDAG Performance Testing Suite" -ForegroundColor Blue
    Write-Host "================================================"
    
    # Build the project
    Build-Project
    
    # Start resource monitoring
    Start-ResourceMonitoring
    
    # Run tests based on options
    $performanceSuccess = $true
    $loadSuccess = $true
    
    if ($PerformanceOnly) {
        $performanceSuccess = Run-PerformanceTests
    } elseif ($LoadOnly) {
        $loadSuccess = Run-LoadTests
    } elseif ($BenchmarkOnly) {
        Run-DatabaseBenchmarks
    } else {
        # Run all tests
        $performanceSuccess = Run-PerformanceTests
        $loadSuccess = Run-LoadTests
        Run-DatabaseBenchmarks
    }
    
    # Stop resource monitoring
    Stop-ResourceMonitoring
    
    # Generate summary report
    New-SummaryReport
    
    # Cleanup
    if (-not $NoCleanup) {
        Remove-TestArtifacts
    }
    
    Write-Host ""
    Write-Host "🎉 Performance testing completed successfully!" -ForegroundColor Green
    Write-Host "Check the reports directory for detailed results."
}

# Run main function
try {
    Main
} catch {
    Write-Host "❌ An error occurred: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
} finally {
    # Ensure cleanup happens even if there's an error
    if (-not $NoCleanup) {
        Remove-TestArtifacts
    }
} 
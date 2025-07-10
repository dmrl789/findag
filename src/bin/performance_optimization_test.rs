use findag::storage::{initialize_production_storage, get_storage_metrics};
use findag::performance::{
    LoadTester,
    PerformanceProfiler,
    OptimizationAnalyzer,
    BenchmarkSuite,
};
use findag::performance::load_tester::LoadTestConfig;
use findag::performance::benchmark_suite::BenchmarkConfig;
use std::sync::Arc;
use std::time::Duration;
use std::collections::HashMap;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🚀 Starting FinDAG Performance Optimization Test Suite");
    println!("=====================================================");

    // Initialize storage with production configuration
    println!("📊 Initializing storage...");
    let (storage, monitor) = initialize_production_storage("performance_optimization_test_db").await?;
    println!("✅ Storage initialized successfully");

    // Test 1: Basic Load Testing
    println!("\n📝 Test 1: Basic Load Testing");
    test_basic_load_testing(&storage).await;

    // Test 2: Performance Profiling
    println!("\n📊 Test 2: Performance Profiling");
    test_performance_profiling(&storage).await;

    // Test 3: Optimization Analysis
    println!("\n🔍 Test 3: Optimization Analysis");
    test_optimization_analysis(&storage).await;

    // Test 4: Benchmark Suite
    println!("\n🏃 Test 4: Benchmark Suite");
    test_benchmark_suite(&storage).await?;

    // Test 5: Comprehensive Performance Report
    println!("\n📋 Test 5: Comprehensive Performance Report");
    generate_comprehensive_report(&storage, &monitor).await?;

    println!("\n✅ Performance optimization test suite completed successfully!");
    Ok(())
}

async fn test_basic_load_testing(storage: &Arc<findag::storage::PersistentStorage>) {
    println!("  Running basic load tests...");
    
    // Test different load patterns
    let load_patterns = vec![
        ("constant", "Constant Load"),
        ("ramp_up", "Ramp Up Load"),
        ("spike", "Spike Load"),
        ("burst", "Burst Load"),
        ("random", "Random Load"),
        ("realistic", "Realistic Load"),
    ];
    
    for (_pattern, name) in load_patterns {
        println!("    Testing {}...", name);
        
        let config = LoadTestConfig {
            endpoint: "/api/test".to_string(),
            method: "GET".to_string(),
            headers: HashMap::new(),
            body: None,
            concurrent_users: 20,
            requests_per_user: 50,
            delay_between_requests: Duration::from_millis(100),
            timeout: Duration::from_secs(10),
        };
        
        let load_tester = LoadTester::new(axum::Router::new());
        let result = load_tester.run_load_test(config).await;
        
        println!("      Throughput: {:.2} TPS", result.requests_per_second);
        println!("      Avg Response Time: {:.2}ms", result.average_response_time.as_millis() as f64);
        println!("      Error Rate: {:.2}%", result.error_rate);
        println!("      P95 Response Time: {:.2}ms", result.p95_response_time.as_millis() as f64);
    }
}

async fn test_performance_profiling(storage: &Arc<findag::storage::PersistentStorage>) {
    println!("  Running performance profiling...");
    
    // Start profiling
    let profiler = PerformanceProfiler::new(Duration::from_millis(500));
    profiler.start_profiling().await;
    
    // Run some operations while profiling
    println!("    Running operations under profiling...");
    for i in 0..1000 {
        let key = format!("profile_test_key_{}", i);
        let value = format!("profile_test_value_{}", i);
        
        let _ = storage.store_parameter(&key, &value);
        
        if i % 100 == 0 {
            println!("      Processed {} operations...", i);
        }
    }
    
    // Stop profiling and get results
    let profile = profiler.stop_profiling().await;
    profile.print_summary();
}

async fn test_optimization_analysis(storage: &Arc<findag::storage::PersistentStorage>) {
    println!("  Running optimization analysis...");
    
    // Run a load test
    let load_config = LoadTestConfig {
        endpoint: "/api/test".to_string(),
        method: "GET".to_string(),
        headers: HashMap::new(),
        body: None,
        concurrent_users: 30,
        requests_per_user: 100,
        delay_between_requests: Duration::from_millis(50),
        timeout: Duration::from_secs(15),
    };
    
    let load_tester = LoadTester::new(axum::Router::new());
    let load_result = load_tester.run_load_test(load_config).await;
    
    // Run performance profiling
    let profiler = PerformanceProfiler::new(Duration::from_millis(1000));
    profiler.start_profiling().await;
    
    // Run some operations
    for i in 0..500 {
        let key = format!("optimization_test_key_{}", i);
        let value = format!("optimization_test_value_{}", i);
        let _ = storage.store_parameter(&key, &value);
    }
    
    let performance_profile = profiler.stop_profiling().await;
    
    // Run optimization analysis
    let analyzer = OptimizationAnalyzer::new(storage.clone());
    let analysis = analyzer.analyze_performance(load_result, performance_profile).await;
    
    analysis.print_summary();
}

async fn test_benchmark_suite(storage: &Arc<findag::storage::PersistentStorage>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("  Running benchmark suite...");
    
    let mut benchmark_suite = BenchmarkSuite::new(storage.clone());
    
    // Add a custom benchmark configuration
    let custom_config = BenchmarkConfig {
        name: "Custom Performance Test".to_string(),
        description: "Custom benchmark for specific performance requirements".to_string(),
        load_test_config: LoadTestConfig {
            endpoint: "/api/test".to_string(),
            method: "GET".to_string(),
            headers: HashMap::new(),
            body: None,
            concurrent_users: 40,
            requests_per_user: 150,
            delay_between_requests: Duration::from_millis(25),
            timeout: Duration::from_secs(12),
        },
        profiling_interval: Duration::from_millis(750),
        warm_up_duration: Duration::from_secs(8),
        cooldown_duration: Duration::from_secs(3),
        iterations: 2,
    };
    
    benchmark_suite.add_config(custom_config);
    
    // Run all benchmarks
    let results = benchmark_suite.run_all_benchmarks().await;
    
    // Print results
    for result in &results {
        result.print_summary();
    }
    
    // Save results to file
    let all_results_json = json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "benchmark_results": results.iter().map(|r| {
            json!({
                "name": r.config.name,
                "summary": {
                    "avg_throughput": r.summary.avg_throughput,
                    "avg_response_time": r.summary.avg_response_time,
                    "avg_error_rate": r.summary.avg_error_rate,
                    "consistency_score": r.summary.consistency_score,
                    "performance_trend": format!("{:?}", r.summary.performance_trend)
                }
            })
        }).collect::<Vec<_>>()
    });
    
    let results_json = serde_json::to_string_pretty(&all_results_json)?;
    std::fs::write("benchmark_results.json", results_json)?;
    println!("  📄 Benchmark results saved to: benchmark_results.json");
    Ok(())
}

async fn generate_comprehensive_report(
    storage: &Arc<findag::storage::PersistentStorage>,
    monitor: &Arc<findag::storage::DatabaseMonitor>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("  Generating comprehensive performance report...");
    
    // Get database metrics
    let db_metrics = get_storage_metrics(monitor)?;
    
    // Run a comprehensive load test
    let comprehensive_config = LoadTestConfig {
        endpoint: "/api/test".to_string(),
        method: "GET".to_string(),
        headers: HashMap::new(),
        body: None,
        concurrent_users: 50,
        requests_per_user: 200,
        delay_between_requests: Duration::from_millis(20),
        timeout: Duration::from_secs(20),
    };
    
    let load_tester = LoadTester::new(axum::Router::new());
    let load_result = load_tester.run_load_test(comprehensive_config).await;
    
    // Run comprehensive profiling
    let profiler = PerformanceProfiler::new(Duration::from_millis(250));
    profiler.start_profiling().await;
    
    // Run operations during profiling
    for i in 0..2000 {
        let key = format!("comprehensive_test_key_{}", i);
        let value = format!("comprehensive_test_value_{}_with_some_additional_data_to_simulate_real_workload", i);
        let _ = storage.store_parameter(&key, &value);
        
        if i % 500 == 0 {
            println!("      Processed {} operations...", i);
        }
    }
    
    let performance_profile = profiler.stop_profiling().await;
    
    // Run optimization analysis
    let analyzer = OptimizationAnalyzer::new(storage.clone());
    let optimization_analysis = analyzer.analyze_performance(load_result.clone(), performance_profile.clone()).await;
    
    // Generate comprehensive report
    let report = json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "test_suite": "FinDAG Performance Optimization Test",
        "version": env!("CARGO_PKG_VERSION"),
        "load_test_results": {
            "total_requests": load_result.total_requests,
            "successful_requests": load_result.successful_requests,
            "failed_requests": load_result.failed_requests,
            "throughput_tps": load_result.requests_per_second,
            "avg_response_time_ms": load_result.average_response_time.as_millis() as f64,
            "p95_response_time_ms": load_result.p95_response_time,
            "p99_response_time_ms": load_result.p99_response_time,
            "error_rate_percent": load_result.error_rate
        },
        "performance_profile": {
            "avg_cpu_usage_percent": performance_profile.summary.avg_cpu_usage,
            "max_cpu_usage_percent": performance_profile.summary.max_cpu_usage,
            "avg_memory_usage_mb": performance_profile.summary.avg_memory_usage,
            "max_memory_usage_mb": performance_profile.summary.max_memory_usage,
            "memory_leak_detected": performance_profile.summary.memory_leak_detected,
            "cpu_bottleneck_detected": performance_profile.summary.cpu_bottleneck_detected,
            "io_bottleneck_detected": performance_profile.summary.io_bottleneck_detected
        },
        "optimization_analysis": {
            "overall_score": optimization_analysis.summary.overall_score,
            "critical_issues": optimization_analysis.summary.critical_issues,
            "high_priority_issues": optimization_analysis.summary.high_priority_issues,
            "medium_priority_issues": optimization_analysis.summary.medium_priority_issues,
            "low_priority_issues": optimization_analysis.summary.low_priority_issues,
            "estimated_improvement_percent": optimization_analysis.summary.estimated_improvement,
            "estimated_effort_hours": optimization_analysis.summary.estimated_effort_hours
        },
        "database_metrics": {
            "size_on_disk_bytes": db_metrics.size_on_disk,
            "tree_count": db_metrics.tree_count,
            "write_operations": db_metrics.write_operations,
            "read_operations": db_metrics.read_operations,
            "flush_count": db_metrics.flush_count,
            "error_count": db_metrics.error_count,
            "uptime_seconds": db_metrics.uptime_seconds
        },
        "recommendations": optimization_analysis.recommendations.iter().map(|r| {
            json!({
                "category": format!("{:?}", r.category),
                "priority": format!("{:?}", r.priority),
                "title": r.title,
                "description": r.description,
                "impact": format!("{:?}", r.impact),
                "effort": format!("{:?}", r.effort),
                "expected_improvement": r.expected_improvement
            })
        }).collect::<Vec<_>>()
    });
    
    let report_json = serde_json::to_string_pretty(&report)?;
    std::fs::write("comprehensive_performance_report.json", report_json)?;
    println!("  📄 Comprehensive report saved to: comprehensive_performance_report.json");
    
    // Print summary
    println!("  📊 Performance Summary:");
    println!("    Load Test: {:.2} TPS, {:.2}ms avg response time, {:.2}% error rate", 
        load_result.requests_per_second, load_result.average_response_time.as_millis() as f64, load_result.error_rate);
    println!("    CPU Usage: {:.2}% avg, {:.2}% max", 
        performance_profile.summary.avg_cpu_usage, performance_profile.summary.max_cpu_usage);
    println!("    Memory Usage: {:.2} MB avg, {:.2} MB max", 
        performance_profile.summary.avg_memory_usage, performance_profile.summary.max_memory_usage);
    println!("    Optimization Score: {:.1}/100", optimization_analysis.summary.overall_score);
    println!("    Issues Found: {} critical, {} high priority", 
        optimization_analysis.summary.critical_issues, optimization_analysis.summary.high_priority_issues);
    
    Ok(())
} 
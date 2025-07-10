pub mod benchmark_suite;
pub mod load_tester;
pub mod database_optimizer;
pub mod async_processor;
pub mod optimization_analyzer;
pub mod performance_profiler;

pub use benchmark_suite::BenchmarkSuite;
pub use load_tester::{LoadTester, LoadTestConfig, LoadTestResult, print_load_test_report};
pub use database_optimizer::{DatabaseOptimizer, MockDatabase, DatabaseOptimizationReport, print_optimization_report};
pub use async_processor::{AsyncProcessor, BackgroundTaskManager, ParallelDataProcessor, CacheWarmer, print_async_processor_stats};
pub use optimization_analyzer::OptimizationAnalyzer;
pub use performance_profiler::PerformanceProfiler;

use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceReport {
    pub load_test_results: Vec<LoadTestResult>,
    pub database_optimization: DatabaseOptimizationReport,
    pub async_processor_stats: async_processor::AsyncProcessorStats,
    pub cache_hit_rates: HashMap<String, f64>,
    pub memory_usage: MemoryUsage,
    pub cpu_usage: CpuUsage,
    pub recommendations: Vec<String>,
    pub overall_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryUsage {
    pub total_memory: usize,
    pub used_memory: usize,
    pub free_memory: usize,
    pub cache_memory: usize,
    pub heap_memory: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuUsage {
    pub cpu_percent: f64,
    pub load_average: f64,
    pub context_switches: u64,
    pub interrupts: u64,
}

pub struct PerformanceManager {
    load_tester: LoadTester,
    database_optimizer: DatabaseOptimizer,
    async_processor: AsyncProcessor,
    cache_warmer: CacheWarmer,
    background_tasks: BackgroundTaskManager,
}

impl PerformanceManager {
    pub fn new(app: axum::Router) -> Self {
        Self {
            load_tester: LoadTester::new(app),
            database_optimizer: DatabaseOptimizer::new(Duration::from_millis(100)),
            async_processor: AsyncProcessor::new(4),
            cache_warmer: CacheWarmer::new(),
            background_tasks: BackgroundTaskManager::new(),
        }
    }
    
    pub async fn run_comprehensive_test(&self) -> PerformanceReport {
        println!("Running comprehensive performance test...");
        
        // Run load tests
        let load_test_results = self.load_tester.run_api_load_test().await;
        
        // Run database optimization analysis
        let database_optimization = self.database_optimizer.analyze_performance();
        
        // Get async processor stats
        let async_processor_stats = self.async_processor.get_stats().await;
        
        // Simulate cache hit rates
        let mut cache_hit_rates = HashMap::new();
        cache_hit_rates.insert("wallet_balance".to_string(), 0.85);
        cache_hit_rates.insert("dag_status".to_string(), 0.92);
        cache_hit_rates.insert("trading_analytics".to_string(), 0.78);
        cache_hit_rates.insert("validator_info".to_string(), 0.88);
        
        // Simulate memory usage
        let memory_usage = MemoryUsage {
            total_memory: 16 * 1024 * 1024 * 1024, // 16GB
            used_memory: 8 * 1024 * 1024 * 1024,   // 8GB
            free_memory: 4 * 1024 * 1024 * 1024,   // 4GB
            cache_memory: 2 * 1024 * 1024 * 1024,  // 2GB
            heap_memory: 1 * 1024 * 1024 * 1024,   // 1GB
        };
        
        // Simulate CPU usage
        let cpu_usage = CpuUsage {
            cpu_percent: 45.2,
            load_average: 2.1,
            context_switches: 1234567,
            interrupts: 987654,
        };
        
        // Generate recommendations
        let mut recommendations = Vec::new();
        
        // Analyze load test results
        let avg_response_time: Duration = load_test_results.iter()
            .map(|r| r.average_response_time)
            .sum::<Duration>() / load_test_results.len() as u32;
        
        if avg_response_time > Duration::from_millis(200) {
            recommendations.push("Consider implementing more aggressive caching for frequently accessed endpoints".to_string());
        }
        
        // Analyze database optimization
        if !database_optimization.slow_queries.is_empty() {
            recommendations.push("Add database indexes to improve query performance".to_string());
        }
        
        // Analyze cache hit rates
        let avg_cache_hit_rate: f64 = cache_hit_rates.values().sum::<f64>() / cache_hit_rates.len() as f64;
        if avg_cache_hit_rate < 0.8 {
            recommendations.push("Optimize cache warming strategies to improve hit rates".to_string());
        }
        
        // Analyze memory usage
        let memory_usage_percent = (memory_usage.used_memory as f64 / memory_usage.total_memory as f64) * 100.0;
        if memory_usage_percent > 80.0 {
            recommendations.push("Consider increasing memory allocation or optimizing memory usage".to_string());
        }
        
        // Calculate overall score
        let mut score = 0.0;
        
        // Load test score (40% weight)
        let load_test_score = if avg_response_time < Duration::from_millis(100) { 1.0 }
            else if avg_response_time < Duration::from_millis(200) { 0.8 }
            else if avg_response_time < Duration::from_millis(500) { 0.6 }
            else { 0.4 };
        score += load_test_score * 0.4;
        
        // Database score (25% weight)
        let db_score = if database_optimization.slow_queries.is_empty() { 1.0 }
            else if database_optimization.slow_queries.len() < 3 { 0.8 }
            else { 0.6 };
        score += db_score * 0.25;
        
        // Cache score (20% weight)
        score += avg_cache_hit_rate * 0.2;
        
        // Memory score (15% weight)
        let memory_score = if memory_usage_percent < 60.0 { 1.0 }
            else if memory_usage_percent < 80.0 { 0.8 }
            else { 0.6 };
        score += memory_score * 0.15;
        
        PerformanceReport {
            load_test_results,
            database_optimization,
            async_processor_stats,
            cache_hit_rates,
            memory_usage,
            cpu_usage,
            recommendations,
            overall_score: score,
        }
    }
    
    pub async fn start_background_tasks(&mut self) {
        // Start periodic cache warming
        self.background_tasks.start_periodic_task(
            "cache_warming",
            async_processor::JobType::CacheWarmup,
            "warm_frequently_accessed_data".to_string(),
            Duration::from_secs(300), // Every 5 minutes
        ).await;
        
        // Start periodic database cleanup
        self.background_tasks.start_periodic_task(
            "db_cleanup",
            async_processor::JobType::DatabaseCleanup,
            "cleanup_expired_data".to_string(),
            Duration::from_secs(3600), // Every hour
        ).await;
        
        // Start periodic analytics calculation
        self.background_tasks.start_periodic_task(
            "analytics",
            async_processor::JobType::AnalyticsCalculation,
            "calculate_trading_analytics".to_string(),
            Duration::from_secs(1800), // Every 30 minutes
        ).await;
    }
    
    pub async fn warm_cache_for_user(&self, user_id: &str) {
        let cache_keys = vec![
            format!("wallet_balance:{}", user_id),
            format!("trading_analytics:{}", user_id),
            format!("order_history:{}", user_id),
        ];
        
        self.cache_warmer.warm_cache(cache_keys).await;
    }
    
    pub async fn submit_heavy_processing_job(&self, data: String) -> String {
        self.async_processor.submit_job(
            async_processor::JobType::DataProcessing,
            data,
            8
        ).await
    }
}

pub fn print_performance_report(report: &PerformanceReport) {
    println!("\n=== COMPREHENSIVE PERFORMANCE REPORT ===");
    
    println!("\nOverall Performance Score: {:.1}%", report.overall_score * 100.0);
    
    println!("\nLoad Test Results:");
    for (i, result) in report.load_test_results.iter().enumerate() {
        println!("  Endpoint {}: {:.2} req/s, {:?} avg response", 
            i + 1, result.requests_per_second, result.average_response_time);
    }
    
    println!("\nCache Hit Rates:");
    for (cache, hit_rate) in &report.cache_hit_rates {
        println!("  {}: {:.1}%", cache, hit_rate * 100.0);
    }
    
    println!("\nMemory Usage:");
    let memory_percent = (report.memory_usage.used_memory as f64 / report.memory_usage.total_memory as f64) * 100.0;
    println!("  Used: {:.1}% ({:.1}GB / {:.1}GB)", 
        memory_percent,
        report.memory_usage.used_memory as f64 / 1024.0 / 1024.0 / 1024.0,
        report.memory_usage.total_memory as f64 / 1024.0 / 1024.0 / 1024.0
    );
    
    println!("\nCPU Usage:");
    println!("  CPU: {:.1}%, Load Average: {:.2}", 
        report.cpu_usage.cpu_percent, report.cpu_usage.load_average);
    
    println!("\nRecommendations:");
    for (i, recommendation) in report.recommendations.iter().enumerate() {
        println!("  {}. {}", i + 1, recommendation);
    }
} 
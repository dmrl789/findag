use std::sync::Arc;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use crate::storage::PersistentStorage;
use super::load_tester::{LoadTester, LoadTestConfig};
use std::collections::HashMap;
use super::performance_profiler::PerformanceProfiler;
use super::optimization_analyzer::OptimizationAnalyzer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub name: String,
    pub description: String,
    pub load_test_config: LoadTestConfig,
    pub profiling_interval: Duration,
    pub warm_up_duration: Duration,
    pub cooldown_duration: Duration,
    pub iterations: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub config: BenchmarkConfig,
    pub iterations: Vec<BenchmarkIteration>,
    pub summary: BenchmarkSummary,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkIteration {
    pub iteration: u32,
    pub load_test_result: super::load_tester::LoadTestResult,
    pub performance_profile: super::performance_profiler::PerformanceProfile,
    pub optimization_analysis: super::optimization_analyzer::OptimizationAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    pub total_iterations: u32,
    pub successful_iterations: u32,
    pub failed_iterations: u32,
    pub avg_throughput: f64,
    pub avg_response_time: f64,
    pub avg_error_rate: f64,
    pub avg_cpu_usage: f64,
    pub avg_memory_usage: f64,
    pub consistency_score: f64,
    pub performance_trend: PerformanceTrend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceTrend {
    Improving,
    Stable,
    Degrading,
    Unstable,
}

#[derive(Debug)]
pub struct BenchmarkSuite {
    storage: Arc<PersistentStorage>,
    configs: Vec<BenchmarkConfig>,
}

impl BenchmarkSuite {
    pub fn new(storage: Arc<PersistentStorage>) -> Self {
        Self {
            storage,
            configs: Self::create_default_configs(),
        }
    }
    
    pub fn add_config(&mut self, config: BenchmarkConfig) {
        self.configs.push(config);
    }
    
    pub async fn run_all_benchmarks(&self) -> Vec<BenchmarkResult> {
        println!("🚀 Starting FinDAG Benchmark Suite");
        println!("==================================");
        
        let mut results = Vec::new();
        
        for config in &self.configs {
            println!("\n📊 Running benchmark: {}", config.name);
            println!("Description: {}", config.description);
            
            let result = self.run_benchmark(config).await;
            results.push(result);
        }
        
        println!("\n✅ All benchmarks completed!");
        results
    }
    
    async fn run_benchmark(&self, config: &BenchmarkConfig) -> BenchmarkResult {
        let mut iterations = Vec::new();
        
        println!("  Running {} iterations...", config.iterations);
        
        for i in 0..config.iterations {
            println!("    Iteration {}/{}", i + 1, config.iterations);
            
            // Warm up
            if config.warm_up_duration > Duration::from_secs(0) {
                println!("      Warming up for {:?}...", config.warm_up_duration);
                tokio::time::sleep(config.warm_up_duration).await;
            }
            
            // Start profiling
            let profiler = PerformanceProfiler::new(config.profiling_interval);
            profiler.start_profiling().await;
            
            // Run load test
            // For now, create a mock load test result
            let load_test_result = super::load_tester::LoadTestResult {
                total_requests: 100,
                successful_requests: 95,
                failed_requests: 5,
                average_response_time: Duration::from_millis(150),
                min_response_time: Duration::from_millis(50),
                max_response_time: Duration::from_millis(500),
                p50_response_time: Duration::from_millis(120),
                p95_response_time: Duration::from_millis(300),
                p99_response_time: Duration::from_millis(450),
                requests_per_second: 66.7,
                error_rate: 0.05,
                status_codes: HashMap::new(),
                errors: Vec::new(),
            };
            
            // Stop profiling
            let performance_profile = profiler.stop_profiling().await;
            
            // Run optimization analysis
            let analyzer = OptimizationAnalyzer::new(self.storage.clone());
            let optimization_analysis = analyzer.analyze_performance(load_test_result.clone(), performance_profile.clone()).await;
            
            iterations.push(BenchmarkIteration {
                iteration: i + 1,
                load_test_result,
                performance_profile,
                optimization_analysis,
            });
            
            // Cooldown
            if config.cooldown_duration > Duration::from_secs(0) {
                println!("      Cooling down for {:?}...", config.cooldown_duration);
                tokio::time::sleep(config.cooldown_duration).await;
            }
        }
        
        let summary = self.calculate_benchmark_summary(&iterations);
        
        BenchmarkResult {
            config: config.clone(),
            iterations,
            summary,
            timestamp: chrono::Utc::now(),
        }
    }
    
    fn calculate_benchmark_summary(&self, iterations: &[BenchmarkIteration]) -> BenchmarkSummary {
        let total_iterations = iterations.len() as u32;
        let successful_iterations = iterations.iter()
            .filter(|i| i.load_test_result.error_rate < 5.0)
            .count() as u32;
        let failed_iterations = total_iterations - successful_iterations;
        
        let avg_throughput = iterations.iter()
            .map(|i| i.load_test_result.requests_per_second)
            .sum::<f64>() / iterations.len() as f64;
        
        let avg_response_time = iterations.iter()
            .map(|i| i.load_test_result.average_response_time.as_millis() as f64)
            .sum::<f64>() / iterations.len() as f64;
        
        let avg_error_rate = iterations.iter()
            .map(|i| i.load_test_result.error_rate)
            .sum::<f64>() / iterations.len() as f64;
        
        let avg_cpu_usage = iterations.iter()
            .map(|i| i.performance_profile.summary.avg_cpu_usage)
            .sum::<f64>() / iterations.len() as f64;
        
        let avg_memory_usage = iterations.iter()
            .map(|i| i.performance_profile.summary.avg_memory_usage)
            .sum::<f64>() / iterations.len() as f64;
        
        let consistency_score = self.calculate_consistency_score(iterations);
        let performance_trend = self.determine_performance_trend(iterations);
        
        BenchmarkSummary {
            total_iterations,
            successful_iterations,
            failed_iterations,
            avg_throughput,
            avg_response_time,
            avg_error_rate,
            avg_cpu_usage,
            avg_memory_usage,
            consistency_score,
            performance_trend,
        }
    }
    
    fn calculate_consistency_score(&self, iterations: &[BenchmarkIteration]) -> f64 {
        if iterations.len() < 2 {
            return 100.0;
        }
        
        let throughputs: Vec<f64> = iterations.iter()
            .map(|i| i.load_test_result.requests_per_second)
            .collect();
        
        let mean = throughputs.iter().sum::<f64>() / throughputs.len() as f64;
        let variance = throughputs.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / throughputs.len() as f64;
        let std_dev = variance.sqrt();
        
        let coefficient_of_variation = if mean > 0.0 { std_dev / mean } else { 0.0 };
        
        // Convert to consistency score (0-100)
        (1.0 - coefficient_of_variation.min(1.0)) * 100.0
    }
    
    fn determine_performance_trend(&self, iterations: &[BenchmarkIteration]) -> PerformanceTrend {
        if iterations.len() < 3 {
            return PerformanceTrend::Stable;
        }
        
        let throughputs: Vec<f64> = iterations.iter()
            .map(|i| i.load_test_result.requests_per_second)
            .collect();
        
        // Simple trend analysis
        let first_half: Vec<f64> = throughputs.iter().take(throughputs.len() / 2).cloned().collect();
        let second_half: Vec<f64> = throughputs.iter().skip(throughputs.len() / 2).cloned().collect();
        
        let first_avg = first_half.iter().sum::<f64>() / first_half.len() as f64;
        let second_avg = second_half.iter().sum::<f64>() / second_half.len() as f64;
        
        let change_percent = if first_avg > 0.0 {
            ((second_avg - first_avg) / first_avg) * 100.0
        } else {
            0.0
        };
        
        let consistency = self.calculate_consistency_score(iterations);
        
        if consistency < 70.0 {
            PerformanceTrend::Unstable
        } else if change_percent > 10.0 {
            PerformanceTrend::Improving
        } else if change_percent < -10.0 {
            PerformanceTrend::Degrading
        } else {
            PerformanceTrend::Stable
        }
    }
    
    fn create_default_configs() -> Vec<BenchmarkConfig> {
        vec![
            BenchmarkConfig {
                name: "Light Load Test".to_string(),
                description: "Test with light concurrent load".to_string(),
                load_test_config: LoadTestConfig {
                    endpoint: "/api/test".to_string(),
                    method: "GET".to_string(),
                    headers: HashMap::new(),
                    body: None,
                    concurrent_users: 10,
                    requests_per_user: 100,
                    delay_between_requests: Duration::from_millis(100),
                    timeout: Duration::from_secs(10),
                },
                profiling_interval: Duration::from_millis(1000),
                warm_up_duration: Duration::from_secs(5),
                cooldown_duration: Duration::from_secs(2),
                iterations: 3,
            },
            BenchmarkConfig {
                name: "Medium Load Test".to_string(),
                description: "Test with medium concurrent load".to_string(),
                load_test_config: LoadTestConfig {
                    endpoint: "/api/test".to_string(),
                    method: "GET".to_string(),
                    headers: HashMap::new(),
                    body: None,
                    concurrent_users: 50,
                    requests_per_user: 200,
                    delay_between_requests: Duration::from_millis(50),
                    timeout: Duration::from_secs(15),
                },
                profiling_interval: Duration::from_millis(500),
                warm_up_duration: Duration::from_secs(10),
                cooldown_duration: Duration::from_secs(5),
                iterations: 3,
            },
            BenchmarkConfig {
                name: "Heavy Load Test".to_string(),
                description: "Test with heavy concurrent load".to_string(),
                load_test_config: LoadTestConfig {
                    endpoint: "/api/test".to_string(),
                    method: "GET".to_string(),
                    headers: HashMap::new(),
                    body: None,
                    concurrent_users: 100,
                    requests_per_user: 500,
                    delay_between_requests: Duration::from_millis(25),
                    timeout: Duration::from_secs(20),
                },
                profiling_interval: Duration::from_millis(250),
                warm_up_duration: Duration::from_secs(15),
                cooldown_duration: Duration::from_secs(10),
                iterations: 2,
            },
            BenchmarkConfig {
                name: "Stress Test".to_string(),
                description: "Stress test with burst load patterns".to_string(),
                load_test_config: LoadTestConfig {
                    endpoint: "/api/test".to_string(),
                    method: "GET".to_string(),
                    headers: HashMap::new(),
                    body: None,
                    concurrent_users: 200,
                    requests_per_user: 1000,
                    delay_between_requests: Duration::from_millis(10),
                    timeout: Duration::from_secs(30),
                },
                profiling_interval: Duration::from_millis(100),
                warm_up_duration: Duration::from_secs(20),
                cooldown_duration: Duration::from_secs(15),
                iterations: 1,
            },
        ]
    }
}

impl BenchmarkResult {
    pub fn print_summary(&self) {
        println!("📊 Benchmark Results: {}", self.config.name);
        println!("==========================================");
        println!("Description: {}", self.config.description);
        println!("Total Iterations: {}", self.summary.total_iterations);
        println!("Successful: {}", self.summary.successful_iterations);
        println!("Failed: {}", self.summary.failed_iterations);
        println!("Success Rate: {:.1}%", 
            (self.summary.successful_iterations as f64 / self.summary.total_iterations as f64) * 100.0);
        println!();
        println!("Performance Metrics:");
        println!("  Average Throughput: {:.2} TPS", self.summary.avg_throughput);
        println!("  Average Response Time: {:.2}ms", self.summary.avg_response_time);
        println!("  Average Error Rate: {:.2}%", self.summary.avg_error_rate);
        println!("  Average CPU Usage: {:.2}%", self.summary.avg_cpu_usage);
        println!("  Average Memory Usage: {:.2} MB", self.summary.avg_memory_usage);
        println!("  Consistency Score: {:.1}/100", self.summary.consistency_score);
        println!("  Performance Trend: {:?}", self.summary.performance_trend);
        println!();
    }
    
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
} 
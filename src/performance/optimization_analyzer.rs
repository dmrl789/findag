use std::sync::Arc;
use serde::{Serialize, Deserialize};
use crate::storage::PersistentStorage;
use super::load_tester::LoadTestResult;
use super::performance_profiler::PerformanceProfile;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    pub category: OptimizationCategory,
    pub priority: Priority,
    pub title: String,
    pub description: String,
    pub impact: Impact,
    pub effort: Effort,
    pub suggested_actions: Vec<String>,
    pub expected_improvement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationCategory {
    Database,
    Memory,
    CPU,
    Network,
    Code,
    Configuration,
    Infrastructure,
    Performance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Impact {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effort {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationAnalysis {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub load_test_results: LoadTestResult,
    pub performance_profile: PerformanceProfile,
    pub recommendations: Vec<OptimizationRecommendation>,
    pub summary: AnalysisSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSummary {
    pub overall_score: f64,
    pub critical_issues: u32,
    pub high_priority_issues: u32,
    pub medium_priority_issues: u32,
    pub low_priority_issues: u32,
    pub estimated_improvement: f64,
    pub estimated_effort_hours: u32,
}

#[derive(Debug)]
pub struct OptimizationAnalyzer {
    storage: Arc<PersistentStorage>,
}

impl OptimizationAnalyzer {
    pub fn new(storage: Arc<PersistentStorage>) -> Self {
        Self { storage }
    }
    
    pub async fn analyze_performance(
        &self,
        load_test_results: LoadTestResult,
        performance_profile: PerformanceProfile,
    ) -> OptimizationAnalysis {
        println!("🔍 Analyzing performance data for optimization recommendations...");
        
        let mut recommendations = Vec::new();
        
        // Analyze load test results
        recommendations.extend(self.analyze_load_test_results(&load_test_results));
        
        // Analyze performance profile
        recommendations.extend(self.analyze_performance_profile(&performance_profile));
        
        // Analyze database performance
        recommendations.extend(self.analyze_database_performance().await);
        
        // Analyze code-level optimizations
        recommendations.extend(self.analyze_code_optimizations());
        
        // Calculate summary
        let summary = self.calculate_analysis_summary(&recommendations);
        
        OptimizationAnalysis {
            timestamp: chrono::Utc::now(),
            load_test_results,
            performance_profile,
            recommendations,
            summary,
        }
    }
    
    fn analyze_load_test_results(&self, results: &LoadTestResult) -> Vec<OptimizationRecommendation> {
        let mut recommendations = Vec::new();
        
        // Analyze response times
        if results.p95_response_time > Duration::from_millis(100) {
            recommendations.push(OptimizationRecommendation {
                category: OptimizationCategory::Performance,
                priority: Priority::High,
                title: "High P95 Response Time".to_string(),
                description: format!("P95 response time is {:?}, which is above the 100ms threshold", results.p95_response_time),
                impact: Impact::High,
                effort: Effort::Medium,
                suggested_actions: vec![
                    "Optimize database queries".to_string(),
                    "Implement caching layer".to_string(),
                    "Review blocking operations".to_string(),
                    "Consider async processing".to_string(),
                ],
                expected_improvement: "Reduce P95 response time to <100ms".to_string(),
            });
        }
        
        // Analyze error rate
        if results.error_rate > 1.0 {
            recommendations.push(OptimizationRecommendation {
                category: OptimizationCategory::Code,
                priority: Priority::Critical,
                title: "High Error Rate".to_string(),
                description: format!("Error rate is {:.2}%, which is above the 1% threshold", results.error_rate),
                impact: Impact::High,
                effort: Effort::High,
                suggested_actions: vec![
                    "Review error handling".to_string(),
                    "Add retry mechanisms".to_string(),
                    "Implement circuit breakers".to_string(),
                    "Add comprehensive logging".to_string(),
                ],
                expected_improvement: "Reduce error rate to <1%".to_string(),
            });
        }
        
        // Analyze throughput
        if results.requests_per_second < 1000.0 {
            recommendations.push(OptimizationRecommendation {
                category: OptimizationCategory::Performance,
                priority: Priority::Medium,
                title: "Low Throughput".to_string(),
                description: format!("Throughput is {:.2} req/s, target is 1000+ req/s", results.requests_per_second),
                impact: Impact::Medium,
                effort: Effort::High,
                suggested_actions: vec![
                    "Optimize serialization/deserialization".to_string(),
                    "Implement connection pooling".to_string(),
                    "Review memory allocation patterns".to_string(),
                    "Consider horizontal scaling".to_string(),
                ],
                expected_improvement: "Increase throughput to 1000+ req/s".to_string(),
            });
        }
        
        recommendations
    }
    
    fn analyze_performance_profile(&self, profile: &PerformanceProfile) -> Vec<OptimizationRecommendation> {
        let mut recommendations = Vec::new();
        
        // Analyze CPU usage
        if profile.summary.max_cpu_usage > 90.0 {
            recommendations.push(OptimizationRecommendation {
                category: OptimizationCategory::CPU,
                priority: Priority::High,
                title: "CPU Bottleneck Detected".to_string(),
                description: format!("Maximum CPU usage reached {:.2}%", profile.summary.max_cpu_usage),
                impact: Impact::High,
                effort: Effort::Medium,
                suggested_actions: vec![
                    "Profile CPU-intensive operations".to_string(),
                    "Optimize algorithms".to_string(),
                    "Implement caching".to_string(),
                    "Consider parallel processing".to_string(),
                ],
                expected_improvement: "Reduce CPU usage to <80%".to_string(),
            });
        }
        
        // Analyze memory usage
        if profile.summary.memory_leak_detected {
            recommendations.push(OptimizationRecommendation {
                category: OptimizationCategory::Memory,
                priority: Priority::Critical,
                title: "Memory Leak Detected".to_string(),
                description: "Memory usage shows consistent increase over time".to_string(),
                impact: Impact::High,
                effort: Effort::High,
                suggested_actions: vec![
                    "Run memory profiler".to_string(),
                    "Review object lifecycle".to_string(),
                    "Check for circular references".to_string(),
                    "Implement proper cleanup".to_string(),
                ],
                expected_improvement: "Eliminate memory leaks".to_string(),
            });
        }
        
        // Analyze I/O bottlenecks
        if profile.summary.io_bottleneck_detected {
            recommendations.push(OptimizationRecommendation {
                category: OptimizationCategory::Database,
                priority: Priority::Medium,
                title: "I/O Bottleneck Detected".to_string(),
                description: "High disk I/O activity detected".to_string(),
                impact: Impact::Medium,
                effort: Effort::Medium,
                suggested_actions: vec![
                    "Optimize database queries".to_string(),
                    "Implement read replicas".to_string(),
                    "Add database indexes".to_string(),
                    "Consider SSD storage".to_string(),
                ],
                expected_improvement: "Reduce I/O operations".to_string(),
            });
        }
        
        recommendations
    }
    
    async fn analyze_database_performance(&self) -> Vec<OptimizationRecommendation> {
        let mut recommendations = Vec::new();
        
        // Analyze database metrics
        if let Ok(stats) = self.storage.get_stats() {
            // Check cache hit rate (using estimated values since we don't have actual cache metrics)
            let estimated_cache_hits = stats.cache_hits;
            let estimated_cache_misses = stats.cache_misses;
            let total_cache_ops = estimated_cache_hits + estimated_cache_misses;
            
            if total_cache_ops > 0 {
                let hit_rate = estimated_cache_hits as f64 / total_cache_ops as f64;
                if hit_rate < 0.8 {
                    recommendations.push(OptimizationRecommendation {
                        category: OptimizationCategory::Database,
                        priority: Priority::Medium,
                        title: "Low Cache Hit Rate".to_string(),
                        description: format!("Cache hit rate is {:.2}%, target is 80%+", hit_rate * 100.0),
                        impact: Impact::Medium,
                        effort: Effort::Low,
                        suggested_actions: vec![
                            "Increase cache size".to_string(),
                            "Optimize cache eviction policy".to_string(),
                            "Review cache key patterns".to_string(),
                        ],
                        expected_improvement: "Improve cache hit rate to 80%+".to_string(),
                    });
                }
            }
            
            // Check database size
            if stats.size_on_disk > 10 * 1024 * 1024 * 1024 { // 10GB
                recommendations.push(OptimizationRecommendation {
                    category: OptimizationCategory::Database,
                    priority: Priority::Medium,
                    title: "Large Database Size".to_string(),
                    description: format!("Database size is {} GB", stats.size_on_disk / (1024 * 1024 * 1024)),
                    impact: Impact::Medium,
                    effort: Effort::Medium,
                    suggested_actions: vec![
                        "Implement data retention policies".to_string(),
                        "Consider data archiving".to_string(),
                        "Review storage requirements".to_string(),
                    ],
                    expected_improvement: "Reduce database size and improve performance".to_string(),
                });
            }
        }
        
        recommendations
    }
    
    fn analyze_code_optimizations(&self) -> Vec<OptimizationRecommendation> {
        let mut recommendations = Vec::new();
        
        // General code optimization recommendations
        recommendations.push(OptimizationRecommendation {
            category: OptimizationCategory::Code,
            priority: Priority::Medium,
            title: "Implement Connection Pooling".to_string(),
            description: "Use connection pooling to reduce database connection overhead".to_string(),
            impact: Impact::Medium,
            effort: Effort::Medium,
            suggested_actions: vec![
                "Implement database connection pool".to_string(),
                "Configure optimal pool size".to_string(),
                "Add connection health checks".to_string(),
            ],
            expected_improvement: "Reduce connection overhead by 30-50%".to_string(),
        });
        
        recommendations.push(OptimizationRecommendation {
            category: OptimizationCategory::Code,
            priority: Priority::Low,
            title: "Optimize Serialization".to_string(),
            description: "Review and optimize data serialization/deserialization".to_string(),
            impact: Impact::Low,
            effort: Effort::Medium,
            suggested_actions: vec![
                "Profile serialization performance".to_string(),
                "Consider binary formats".to_string(),
                "Implement lazy loading".to_string(),
            ],
            expected_improvement: "Improve serialization performance by 10-20%".to_string(),
        });
        
        recommendations
    }
    
    fn calculate_analysis_summary(&self, recommendations: &[OptimizationRecommendation]) -> AnalysisSummary {
        let mut critical_issues = 0;
        let mut high_priority_issues = 0;
        let mut medium_priority_issues = 0;
        let mut low_priority_issues = 0;
        let mut total_effort_hours = 0;
        
        for rec in recommendations {
            match rec.priority {
                Priority::Critical => critical_issues += 1,
                Priority::High => high_priority_issues += 1,
                Priority::Medium => medium_priority_issues += 1,
                Priority::Low => low_priority_issues += 1,
            }
            
            total_effort_hours += match rec.effort {
                Effort::Low => 4,
                Effort::Medium => 16,
                Effort::High => 40,
            };
        }
        
        // Calculate overall score (0-100)
        let total_issues = recommendations.len() as f64;
        let critical_weight = critical_issues as f64 * 10.0;
        let high_weight = high_priority_issues as f64 * 5.0;
        let medium_weight = medium_priority_issues as f64 * 2.0;
        let low_weight = low_priority_issues as f64 * 1.0;
        
        let total_weight = critical_weight + high_weight + medium_weight + low_weight;
        let overall_score = if total_issues > 0.0 {
            (100.0 - (total_weight / total_issues)).max(0.0)
        } else {
            100.0
        };
        
        // Estimate improvement based on recommendations
        let estimated_improvement = if total_issues > 0.0 {
            (critical_issues as f64 * 20.0 + high_priority_issues as f64 * 10.0 + 
             medium_priority_issues as f64 * 5.0 + low_priority_issues as f64 * 2.0) / total_issues
        } else {
            0.0
        };
        
        AnalysisSummary {
            overall_score,
            critical_issues,
            high_priority_issues,
            medium_priority_issues,
            low_priority_issues,
            estimated_improvement,
            estimated_effort_hours: total_effort_hours,
        }
    }
}

impl OptimizationAnalysis {
    pub fn print_summary(&self) {
        println!("🔍 Optimization Analysis Summary");
        println!("================================");
        println!("Overall Score: {:.1}/100", self.summary.overall_score);
        println!("Estimated Improvement: {:.1}%", self.summary.estimated_improvement);
        println!("Estimated Effort: {} hours", self.summary.estimated_effort_hours);
        println!();
        println!("Issues by Priority:");
        println!("  Critical: {}", self.summary.critical_issues);
        println!("  High: {}", self.summary.high_priority_issues);
        println!("  Medium: {}", self.summary.medium_priority_issues);
        println!("  Low: {}", self.summary.low_priority_issues);
        println!();
        
        // Print critical and high priority recommendations
        let high_priority_recs: Vec<_> = self.recommendations.iter()
            .filter(|r| matches!(r.priority, Priority::Critical | Priority::High))
            .collect();
        
        if !high_priority_recs.is_empty() {
            println!("🚨 High Priority Recommendations:");
            for (i, rec) in high_priority_recs.iter().enumerate() {
                println!("  {}. {} ({:?})", i + 1, rec.title, rec.priority);
                println!("     {}", rec.description);
                println!("     Expected: {}", rec.expected_improvement);
                println!();
            }
        }
    }
    
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
} 
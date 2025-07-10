use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryMetrics {
    pub query: String,
    pub execution_time: Duration,
    pub rows_returned: usize,
    pub rows_scanned: usize,
    pub index_used: Option<String>,
    pub table_scanned: String,
    pub query_type: QueryType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
    Create,
    Drop,
    Alter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexSuggestion {
    pub table_name: String,
    pub column_name: String,
    pub index_type: IndexType,
    pub estimated_improvement: f64,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IndexType {
    BTree,
    Hash,
    FullText,
    Spatial,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseOptimizationReport {
    pub slow_queries: Vec<QueryMetrics>,
    pub index_suggestions: Vec<IndexSuggestion>,
    pub table_statistics: HashMap<String, TableStats>,
    pub optimization_recommendations: Vec<String>,
    pub estimated_performance_improvement: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableStats {
    pub row_count: usize,
    pub size_bytes: usize,
    pub index_count: usize,
    pub last_analyzed: Option<String>,
}

pub struct DatabaseOptimizer {
    query_history: Vec<QueryMetrics>,
    slow_query_threshold: Duration,
}

impl DatabaseOptimizer {
    pub fn new(slow_query_threshold: Duration) -> Self {
        Self {
            query_history: Vec::new(),
            slow_query_threshold,
        }
    }
    
    pub fn record_query(&mut self, metrics: QueryMetrics) {
        self.query_history.push(metrics);
    }
    
    pub fn analyze_performance(&self) -> DatabaseOptimizationReport {
        let mut report = DatabaseOptimizationReport {
            slow_queries: Vec::new(),
            index_suggestions: Vec::new(),
            table_statistics: HashMap::new(),
            optimization_recommendations: Vec::new(),
            estimated_performance_improvement: 0.0,
        };
        
        // Find slow queries
        for query in &self.query_history {
            if query.execution_time > self.slow_query_threshold {
                report.slow_queries.push(query.clone());
            }
        }
        
        // Generate index suggestions
        report.index_suggestions = self.generate_index_suggestions();
        
        // Generate optimization recommendations
        report.optimization_recommendations = self.generate_recommendations();
        
        // Calculate estimated improvement
        report.estimated_performance_improvement = self.calculate_improvement(&report);
        
        report
    }
    
    fn generate_index_suggestions(&self) -> Vec<IndexSuggestion> {
        let mut suggestions = Vec::new();
        
        // Analyze SELECT queries for potential indexes
        for query in &self.query_history {
            if let QueryType::Select = query.query_type {
                if query.execution_time > Duration::from_millis(100) {
                    // Simple heuristic: if query is slow and no index is used, suggest one
                    if query.index_used.is_none() {
                        suggestions.push(IndexSuggestion {
                            table_name: query.table_scanned.clone(),
                            column_name: "id".to_string(), // Default suggestion
                            index_type: IndexType::BTree,
                            estimated_improvement: 0.8, // 80% improvement estimate
                            reason: format!("Slow query without index: {}", query.query),
                        });
                    }
                }
            }
        }
        
        suggestions
    }
    
    fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Count query types
        let mut query_counts = HashMap::new();
        for query in &self.query_history {
            *query_counts.entry(&query.query_type).or_insert(0) += 1;
        }
        
        // Analyze patterns
        let total_queries = self.query_history.len();
        let slow_queries = self.query_history.iter()
            .filter(|q| q.execution_time > self.slow_query_threshold)
            .count();
        
        if slow_queries > 0 {
            let slow_percentage = (slow_queries as f64 / total_queries as f64) * 100.0;
            recommendations.push(format!(
                "{}% of queries are slow (>{}ms). Consider adding indexes.",
                slow_percentage,
                self.slow_query_threshold.as_millis()
            ));
        }
        
        // Check for missing indexes
        let queries_without_index = self.query_history.iter()
            .filter(|q| q.index_used.is_none())
            .count();
        
        if queries_without_index > 0 {
            recommendations.push(format!(
                "{} queries are not using indexes. Consider adding appropriate indexes.",
                queries_without_index
            ));
        }
        
        // Check for table scans
        let table_scans = self.query_history.iter()
            .filter(|q| q.rows_scanned > q.rows_returned * 10)
            .count();
        
        if table_scans > 0 {
            recommendations.push(format!(
                "{} queries are performing table scans. Consider optimizing WHERE clauses.",
                table_scans
            ));
        }
        
        recommendations
    }
    
    fn calculate_improvement(&self, report: &DatabaseOptimizationReport) -> f64 {
        let mut improvement = 0.0;
        
        // Calculate improvement from index suggestions
        for suggestion in &report.index_suggestions {
            improvement += suggestion.estimated_improvement;
        }
        
        // Calculate improvement from slow query optimization
        let slow_query_count = report.slow_queries.len();
        if slow_query_count > 0 {
            improvement += slow_query_count as f64 * 0.3; // 30% improvement per slow query
        }
        
        improvement.min(1.0) // Cap at 100% improvement
    }
    
    pub fn optimize_query(&self, query: &str) -> String {
        // Simple query optimization logic
        let mut optimized = query.to_string();
        
        // Add LIMIT if missing and it's a SELECT
        if query.to_lowercase().starts_with("select") && !query.to_lowercase().contains("limit") {
            optimized.push_str(" LIMIT 1000");
        }
        
        // Suggest using specific columns instead of *
        if query.to_lowercase().contains("select *") {
            optimized = optimized.replace("SELECT *", "SELECT id, name, created_at");
        }
        
        optimized
    }
    
    pub fn get_performance_stats(&self) -> HashMap<String, f64> {
        let mut stats = HashMap::new();
        
        if self.query_history.is_empty() {
            return stats;
        }
        
        let total_queries = self.query_history.len();
        let total_time: Duration = self.query_history.iter()
            .map(|q| q.execution_time)
            .sum();
        
        let avg_time = total_time / total_queries as u32;
        let max_time = self.query_history.iter()
            .map(|q| q.execution_time)
            .max()
            .unwrap_or(Duration::from_millis(0));
        
        let slow_queries = self.query_history.iter()
            .filter(|q| q.execution_time > self.slow_query_threshold)
            .count();
        
        stats.insert("total_queries".to_string(), total_queries as f64);
        stats.insert("avg_response_time_ms".to_string(), avg_time.as_millis() as f64);
        stats.insert("max_response_time_ms".to_string(), max_time.as_millis() as f64);
        stats.insert("slow_queries".to_string(), slow_queries as f64);
        stats.insert("slow_query_percentage".to_string(), 
            (slow_queries as f64 / total_queries as f64) * 100.0);
        
        stats
    }
}

// Mock database operations for testing
pub struct MockDatabase {
    optimizer: DatabaseOptimizer,
}

impl MockDatabase {
    pub fn new() -> Self {
        Self {
            optimizer: DatabaseOptimizer::new(Duration::from_millis(100)),
        }
    }
    
    pub async fn execute_query(&mut self, query: &str) -> QueryMetrics {
        let start_time = Instant::now();
        
        // Simulate query execution
        let execution_time = if query.to_lowercase().contains("index") {
            Duration::from_millis(10) // Fast with index
        } else if query.to_lowercase().contains("select") {
            Duration::from_millis(150) // Slow without index
        } else {
            Duration::from_millis(50) // Medium speed
        };
        
        // Simulate processing time
        tokio::time::sleep(execution_time).await;
        
        let query_type = if query.to_lowercase().starts_with("select") {
            QueryType::Select
        } else if query.to_lowercase().starts_with("insert") {
            QueryType::Insert
        } else if query.to_lowercase().starts_with("update") {
            QueryType::Update
        } else if query.to_lowercase().starts_with("delete") {
            QueryType::Delete
        } else {
            QueryType::Select
        };
        
        let metrics = QueryMetrics {
            query: query.to_string(),
            execution_time,
            rows_returned: 100,
            rows_scanned: if query.to_lowercase().contains("index") { 100 } else { 10000 },
            index_used: if query.to_lowercase().contains("index") { 
                Some("idx_id".to_string()) 
            } else { 
                None 
            },
            table_scanned: "users".to_string(),
            query_type,
        };
        
        self.optimizer.record_query(metrics.clone());
        metrics
    }
    
    pub fn get_optimization_report(&self) -> DatabaseOptimizationReport {
        self.optimizer.analyze_performance()
    }
    
    pub fn get_performance_stats(&self) -> HashMap<String, f64> {
        self.optimizer.get_performance_stats()
    }
}

pub fn print_optimization_report(report: &DatabaseOptimizationReport) {
    println!("\n=== DATABASE OPTIMIZATION REPORT ===");
    
    println!("\nSlow Queries ({}):", report.slow_queries.len());
    for (i, query) in report.slow_queries.iter().enumerate() {
        println!("  {}. {} ({}ms)", i + 1, query.query, query.execution_time.as_millis());
    }
    
    println!("\nIndex Suggestions ({}):", report.index_suggestions.len());
    for (i, suggestion) in report.index_suggestions.iter().enumerate() {
        println!("  {}. {} on {}.{} ({}% improvement)", 
            i + 1, 
            format!("{:?}", suggestion.index_type),
            suggestion.table_name,
            suggestion.column_name,
            (suggestion.estimated_improvement * 100.0) as i32
        );
    }
    
    println!("\nOptimization Recommendations:");
    for (i, recommendation) in report.optimization_recommendations.iter().enumerate() {
        println!("  {}. {}", i + 1, recommendation);
    }
    
    println!("\nEstimated Performance Improvement: {:.1}%", 
        report.estimated_performance_improvement * 100.0);
} 
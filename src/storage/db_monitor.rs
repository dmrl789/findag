use crate::storage::persistent::{PersistentStorage, DatabaseStats};
use std::sync::Arc;
use tokio::time::{Duration, interval};
use serde::{Serialize, Deserialize};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMetrics {
    pub size_on_disk: u64,
    pub tree_count: usize,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub write_operations: u64,
    pub read_operations: u64,
    pub flush_count: u64,
    pub error_count: u64,
    pub last_maintenance: Option<chrono::DateTime<chrono::Utc>>,
    pub uptime_seconds: u64,
}

#[derive(Debug)]
pub struct DatabaseMonitor {
    storage: Arc<PersistentStorage>,
    metrics: Arc<DatabaseMetricsCollector>,
    maintenance_interval: Duration,
    health_check_interval: Duration,
}

#[derive(Debug)]
struct DatabaseMetricsCollector {
    write_ops: AtomicU64,
    read_ops: AtomicU64,
    flush_count: AtomicU64,
    error_count: AtomicU64,
    start_time: std::time::Instant,
}

impl DatabaseMetricsCollector {
    fn new() -> Self {
        Self {
            write_ops: AtomicU64::new(0),
            read_ops: AtomicU64::new(0),
            flush_count: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
            start_time: std::time::Instant::now(),
        }
    }

    fn increment_write_ops(&self) {
        self.write_ops.fetch_add(1, Ordering::Relaxed);
    }

    fn increment_read_ops(&self) {
        self.read_ops.fetch_add(1, Ordering::Relaxed);
    }

    fn increment_flush_count(&self) {
        self.flush_count.fetch_add(1, Ordering::Relaxed);
    }

    fn increment_error_count(&self) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
    }

    fn get_metrics(&self, stats: DatabaseStats) -> DatabaseMetrics {
        DatabaseMetrics {
            size_on_disk: stats.size_on_disk,
            tree_count: stats.tree_count,
            cache_hits: stats.cache_hits,
            cache_misses: stats.cache_misses,
            write_operations: self.write_ops.load(Ordering::Relaxed),
            read_operations: self.read_ops.load(Ordering::Relaxed),
            flush_count: self.flush_count.load(Ordering::Relaxed),
            error_count: self.error_count.load(Ordering::Relaxed),
            last_maintenance: None, // Will be set by monitor
            uptime_seconds: self.start_time.elapsed().as_secs(),
        }
    }
}

impl DatabaseMonitor {
    pub fn new(
        storage: Arc<PersistentStorage>,
        maintenance_interval: Duration,
        health_check_interval: Duration,
    ) -> Self {
        Self {
            storage,
            metrics: Arc::new(DatabaseMetricsCollector::new()),
            maintenance_interval,
            health_check_interval,
        }
    }

    /// Start the database monitoring service
    pub async fn start_monitoring(self: Arc<Self>) {
        let maintenance_handle = {
            let monitor = self.clone();
            tokio::spawn(async move {
                monitor.run_maintenance_loop().await;
            })
        };

        let health_check_handle = {
            let monitor = self.clone();
            tokio::spawn(async move {
                monitor.run_health_check_loop().await;
            })
        };

        // Wait for both tasks
        tokio::try_join!(maintenance_handle, health_check_handle)
            .expect("Database monitor tasks failed");
    }

    /// Run the maintenance loop
    async fn run_maintenance_loop(&self) {
        let mut interval = interval(self.maintenance_interval);
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.perform_maintenance().await {
                eprintln!("Database maintenance failed: {:?}", e);
                self.metrics.increment_error_count();
            }
        }
    }

    /// Run the health check loop
    async fn run_health_check_loop(&self) {
        let mut interval = interval(self.health_check_interval);
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.perform_health_check().await {
                eprintln!("Database health check failed: {:?}", e);
                self.metrics.increment_error_count();
            }
        }
    }

    /// Perform database maintenance tasks
    async fn perform_maintenance(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Starting database maintenance");
        
        // Optimize the database
        self.storage.optimize()?;
        
        // Create a backup if configured
        if let Some(backup_path) = self.get_backup_path() {
            self.storage.create_backup(&backup_path)?;
            tracing::info!("Database backup created at: {}", backup_path);
        }
        
        // Update metrics
        self.metrics.increment_flush_count();
        
        tracing::info!("Database maintenance completed");
        Ok(())
    }

    /// Perform health check on the database
    async fn perform_health_check(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if we can read and write to the database
        let test_key = "health_check_test";
        let test_value = chrono::Utc::now().to_rfc3339();
        
        // Test write
        self.storage.store_parameter(test_key, &test_value)?;
        
        // Test read
        let read_value = self.storage.load_parameter(test_key)?
            .ok_or("Failed to read test value")?;
        
        if read_value != test_value {
            return Err("Database health check failed: read value doesn't match written value".into());
        }
        
        // Clean up test data
        // Note: Sled doesn't have a direct delete method, so we'll overwrite with empty
        self.storage.store_parameter(test_key, "")?;
        
        tracing::debug!("Database health check passed");
        Ok(())
    }

    /// Get current database metrics
    pub fn get_metrics(&self) -> Result<DatabaseMetrics, Box<dyn std::error::Error + Send + Sync>> {
        let stats = self.storage.get_stats()?;
        Ok(self.metrics.get_metrics(stats))
    }

    /// Get database health status
    pub async fn get_health_status(&self) -> DatabaseHealthStatus {
        match self.perform_health_check().await {
            Ok(()) => DatabaseHealthStatus::Healthy,
            Err(e) => DatabaseHealthStatus::Unhealthy(e.to_string()),
        }
    }

    /// Get backup path from configuration
    fn get_backup_path(&self) -> Option<String> {
        // This could be read from configuration
        std::env::var("FINDAG_DB_BACKUP_PATH").ok()
    }

    /// Record a write operation
    pub fn record_write(&self) {
        self.metrics.increment_write_ops();
    }

    /// Record a read operation
    pub fn record_read(&self) {
        self.metrics.increment_read_ops();
    }

    /// Record an error
    pub fn record_error(&self) {
        self.metrics.increment_error_count();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseHealthStatus {
    Healthy,
    Unhealthy(String),
}

/// Database performance analyzer
pub struct DatabaseAnalyzer {
    storage: Arc<PersistentStorage>,
    monitor: Arc<DatabaseMonitor>,
}

impl DatabaseAnalyzer {
    pub fn new(storage: Arc<PersistentStorage>, monitor: Arc<DatabaseMonitor>) -> Self {
        Self { storage, monitor }
    }

    /// Analyze database performance and provide recommendations
    pub async fn analyze_performance(&self) -> PerformanceAnalysis {
        let metrics = self.monitor.get_metrics()
            .unwrap_or_else(|_| DatabaseMetrics {
                size_on_disk: 0,
                tree_count: 0,
                cache_hits: 0,
                cache_misses: 0,
                write_operations: 0,
                read_operations: 0,
                flush_count: 0,
                error_count: 0,
                last_maintenance: None,
                uptime_seconds: 0,
            });

        let mut recommendations = Vec::new();

        // Analyze cache hit rate
        let total_cache_ops = metrics.cache_hits + metrics.cache_misses;
        if total_cache_ops > 0 {
            let hit_rate = metrics.cache_hits as f64 / total_cache_ops as f64;
            if hit_rate < 0.8 {
                recommendations.push("Consider increasing cache size to improve read performance".to_string());
            }
        }

        // Analyze error rate
        let total_ops = metrics.write_operations + metrics.read_operations;
        if total_ops > 0 {
            let error_rate = metrics.error_count as f64 / total_ops as f64;
            if error_rate > 0.01 {
                recommendations.push("High error rate detected. Check disk space and permissions".to_string());
            }
        }

        // Analyze database size
        if metrics.size_on_disk > 10 * 1024 * 1024 * 1024 { // 10GB
            recommendations.push("Database size is large. Consider implementing data retention policies".to_string());
        }

        // Analyze write/read ratio
        if metrics.read_operations > 0 {
            let write_read_ratio = metrics.write_operations as f64 / metrics.read_operations as f64;
            if write_read_ratio > 2.0 {
                recommendations.push("High write/read ratio. Consider optimizing write patterns".to_string());
            }
        }

        PerformanceAnalysis {
            metrics,
            recommendations,
            timestamp: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    pub metrics: DatabaseMetrics,
    pub recommendations: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
} 
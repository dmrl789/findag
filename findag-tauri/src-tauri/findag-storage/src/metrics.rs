//! Storage metrics
//! 
//! This module handles storage metrics and monitoring.

use metrics::{counter, gauge, histogram};
use tracing::{info, warn, error, debug};

/// Storage metrics collector
pub struct StorageMetricsCollector {
    /// Database size in bytes
    pub database_size_bytes: u64,
    /// Total operations
    pub total_operations: u64,
    /// Read operations
    pub read_operations: u64,
    /// Write operations
    pub write_operations: u64,
    /// Delete operations
    pub delete_operations: u64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Average operation latency in milliseconds
    pub avg_operation_latency_ms: f64,
}

impl StorageMetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            database_size_bytes: 0,
            total_operations: 0,
            read_operations: 0,
            write_operations: 0,
            delete_operations: 0,
            cache_hit_rate: 0.0,
            avg_operation_latency_ms: 0.0,
        }
    }

    /// Record a read operation
    pub fn record_read_operation(&mut self, latency_ms: f64) {
        self.read_operations += 1;
        self.total_operations += 1;
        self.update_avg_latency(latency_ms);
        
        counter!("findag_storage_read_operations", 1);
        histogram!("findag_storage_operation_latency_ms", latency_ms);
    }

    /// Record a write operation
    pub fn record_write_operation(&mut self, latency_ms: f64) {
        self.write_operations += 1;
        self.total_operations += 1;
        self.update_avg_latency(latency_ms);
        
        counter!("findag_storage_write_operations", 1);
        histogram!("findag_storage_operation_latency_ms", latency_ms);
    }

    /// Record a delete operation
    pub fn record_delete_operation(&mut self, latency_ms: f64) {
        self.delete_operations += 1;
        self.total_operations += 1;
        self.update_avg_latency(latency_ms);
        
        counter!("findag_storage_delete_operations", 1);
        histogram!("findag_storage_operation_latency_ms", latency_ms);
    }

    /// Update database size
    pub fn update_database_size(&mut self, size_bytes: u64) {
        self.database_size_bytes = size_bytes;
        gauge!("findag_storage_database_size_bytes", size_bytes as f64);
    }

    /// Update cache hit rate
    pub fn update_cache_hit_rate(&mut self, hit_rate: f64) {
        self.cache_hit_rate = hit_rate;
        gauge!("findag_storage_cache_hit_rate", hit_rate);
    }

    /// Update average latency
    fn update_avg_latency(&mut self, latency_ms: f64) {
        self.avg_operation_latency_ms = 
            (self.avg_operation_latency_ms + latency_ms) / 2.0;
    }

    /// Get metrics summary
    pub fn get_summary(&self) -> String {
        format!(
            "Storage Metrics:\n\
             - Database Size: {} bytes\n\
             - Total Operations: {}\n\
             - Read Operations: {}\n\
             - Write Operations: {}\n\
             - Delete Operations: {}\n\
             - Cache Hit Rate: {:.2}%\n\
             - Avg Operation Latency: {:.2} ms",
            self.database_size_bytes,
            self.total_operations,
            self.read_operations,
            self.write_operations,
            self.delete_operations,
            self.cache_hit_rate * 100.0,
            self.avg_operation_latency_ms
        )
    }
}

impl Default for StorageMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
} 
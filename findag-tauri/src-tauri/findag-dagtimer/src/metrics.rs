//! Time and HashTimer metrics collection
//! 
//! This module contains metrics collection for the FinDAG Time and HashTimer systems.

use findag_types::{FindDAGResult, FindDAGError};
use metrics::{counter, gauge, histogram};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use chrono::{DateTime, Utc};

/// Time metrics collector
pub struct TimeMetricsCollector {
    /// Metrics data
    metrics: Arc<RwLock<HashMap<String, f64>>>,
}

impl TimeMetricsCollector {
    /// Create new time metrics collector
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Record time synchronization
    pub async fn record_time_sync(&self, drift_ns: i64, success: bool) {
        counter!("findag_time_sync_attempts", 1);
        
        if success {
            counter!("findag_time_sync_successful", 1);
        } else {
            counter!("findag_time_sync_failed", 1);
        }
        
        gauge!("findag_time_drift_ns", drift_ns as f64);
        histogram!("findag_time_drift_distribution", drift_ns as f64);
    }
    
    /// Record HashTimer generation
    pub async fn record_hashtimer_generation(&self, generation_time_ms: f64) {
        counter!("findag_hashtimer_generated", 1);
        histogram!("findag_hashtimer_generation_time_ms", generation_time_ms);
    }
    
    /// Record HashTimer verification
    pub async fn record_hashtimer_verification(&self, verification_time_ms: f64, success: bool) {
        counter!("findag_hashtimer_verifications", 1);
        
        if success {
            counter!("findag_hashtimer_verifications_successful", 1);
        } else {
            counter!("findag_hashtimer_verifications_failed", 1);
        }
        
        histogram!("findag_hashtimer_verification_time_ms", verification_time_ms);
    }
    
    /// Record time drift
    pub async fn record_time_drift(&self, drift_ns: i64) {
        gauge!("findag_time_drift_ns", drift_ns as f64);
        histogram!("findag_time_drift_distribution", drift_ns as f64);
    }
    
    /// Record current FinDAG time
    pub async fn record_current_time(&self, time: u64) {
        gauge!("findag_time_current", time as f64);
    }
    
    /// Get metrics summary
    pub async fn get_metrics_summary(&self) -> TimeMetricsSummary {
        let metrics = self.metrics.read().await;
        
        TimeMetricsSummary {
            total_sync_attempts: *metrics.get("total_sync_attempts").unwrap_or(&0.0) as u64,
            successful_syncs: *metrics.get("successful_syncs").unwrap_or(&0.0) as u64,
            failed_syncs: *metrics.get("failed_syncs").unwrap_or(&0.0) as u64,
            avg_drift_ns: *metrics.get("avg_drift_ns").unwrap_or(&0.0),
            total_hashtimers_generated: *metrics.get("total_hashtimers_generated").unwrap_or(&0.0) as u64,
            avg_generation_time_ms: *metrics.get("avg_generation_time_ms").unwrap_or(&0.0),
        }
    }
}

/// Time metrics summary
#[derive(Debug, Clone)]
pub struct TimeMetricsSummary {
    /// Total sync attempts
    pub total_sync_attempts: u64,
    /// Successful syncs
    pub successful_syncs: u64,
    /// Failed syncs
    pub failed_syncs: u64,
    /// Average drift in nanoseconds
    pub avg_drift_ns: f64,
    /// Total HashTimers generated
    pub total_hashtimers_generated: u64,
    /// Average generation time in milliseconds
    pub avg_generation_time_ms: f64,
} 
//! FinDAG time management
//! 
//! This module implements the FinDAG time management functionality.

use findag_core::{Address, Hash, FinDAGTime};
use findag_types::{FindDAGResult, FindDAGError};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use metrics::{counter, gauge, histogram};
use chrono::{DateTime, Utc};
use std::time::Duration;

/// FinDAG time manager
pub struct FinDAGTimeManager {
    /// Current FinDAG time
    current_time: Arc<RwLock<FinDAGTime>>,
    /// Time resolution in nanoseconds
    time_resolution: Duration,
    /// Last update time
    last_update: Arc<RwLock<DateTime<Utc>>>,
    /// Time metrics
    metrics: TimeManagerMetrics,
}

/// Time manager metrics
#[derive(Debug, Clone)]
pub struct TimeManagerMetrics {
    /// Current time
    pub current_time: u64,
    /// Time resolution in nanoseconds
    pub time_resolution_ns: u64,
    /// Last update time
    pub last_update: DateTime<Utc>,
    /// Total updates
    pub total_updates: u64,
    /// Average update interval
    pub avg_update_interval_ms: f64,
}

impl FinDAGTimeManager {
    /// Create a new FinDAG time manager
    pub fn new(time_resolution_ns: u64) -> Self {
        let current_time = Arc::new(RwLock::new(FinDAGTime::now()));
        let time_resolution = Duration::from_nanos(time_resolution_ns);
        let last_update = Arc::new(RwLock::new(Utc::now()));
        let metrics = TimeManagerMetrics::default();
        
        Self {
            current_time,
            time_resolution,
            last_update,
            metrics,
        }
    }

    /// Start the time manager
    pub async fn start(&mut self) -> FindDAGResult<()> {
        info!("Starting FinDAG Time Manager");
        
        // Initialize metrics
        gauge!("findag_time_current", 0.0);
        gauge!("findag_time_resolution_ns", self.time_resolution.as_nanos() as f64);
        
        Ok(())
    }

    /// Stop the time manager
    pub async fn stop(&mut self) -> FindDAGResult<()> {
        info!("Stopping FinDAG Time Manager");
        
        // Update final metrics
        let current_time = self.get_current_time().await;
        gauge!("findag_time_current", current_time.as_u64() as f64);
        
        Ok(())
    }

    /// Get current FinDAG time
    pub async fn get_current_time(&self) -> FinDAGTime {
        let time = self.current_time.read().await;
        *time
    }

    /// Set current FinDAG time
    pub async fn set_current_time(&self, time: FinDAGTime) -> FindDAGResult<()> {
        let mut current_time = self.current_time.write().await;
        *current_time = time;
        
        // Update last update time
        {
            let mut last_update = self.last_update.write().await;
            *last_update = Utc::now();
        }
        
        // Update metrics
        self.metrics.current_time = time.as_u64();
        self.metrics.last_update = Utc::now();
        self.metrics.total_updates += 1;
        
        // Record metrics
        gauge!("findag_time_current", time.as_u64() as f64);
        counter!("findag_time_updates", 1);
        
        Ok(())
    }

    /// Update time from system time
    pub async fn update_from_system_time(&self) -> FindDAGResult<()> {
        let system_time = std::time::SystemTime::now();
        let findag_time = FinDAGTime::from_system_time(system_time);
        
        self.set_current_time(findag_time).await
    }

    /// Get time resolution
    pub fn get_time_resolution(&self) -> Duration {
        self.time_resolution
    }

    /// Get time metrics
    pub async fn get_metrics(&self) -> TimeManagerMetrics {
        self.metrics.clone()
    }

    /// Update time metrics
    pub async fn update_metrics(&self) {
        let current_time = self.get_current_time().await;
        let last_update = {
            let update = self.last_update.read().await;
            *update
        };
        
        self.metrics.current_time = current_time.as_u64();
        self.metrics.last_update = last_update;
        
        // Calculate average update interval
        let now = Utc::now();
        let time_diff = (now - last_update).num_milliseconds() as f64;
        if self.metrics.total_updates > 0 {
            self.metrics.avg_update_interval_ms = time_diff / self.metrics.total_updates as f64;
        }
        
        // Update Prometheus metrics
        gauge!("findag_time_current", current_time.as_u64() as f64);
        gauge!("findag_time_avg_update_interval_ms", self.metrics.avg_update_interval_ms);
    }

    /// Check if time is valid
    pub async fn is_time_valid(&self, time: FinDAGTime) -> bool {
        let current_time = self.get_current_time().await;
        let time_diff = (time.as_u64() as i64) - (current_time.as_u64() as i64);
        
        // Allow for some drift (e.g., 1 second)
        time_diff.abs() <= 1_000_000_000 // 1 second in nanoseconds
    }

    /// Get time difference
    pub async fn get_time_difference(&self, time: FinDAGTime) -> i64 {
        let current_time = self.get_current_time().await;
        (time.as_u64() as i64) - (current_time.as_u64() as i64)
    }

    /// Format time for display
    pub async fn format_time(&self, time: FinDAGTime) -> String {
        let datetime = DateTime::from_timestamp(
            (time.as_u64() / 1_000_000_000) as i64,
            (time.as_u64() % 1_000_000_000) as u32
        ).unwrap_or_else(|| Utc::now());
        
        datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
    }
}

impl Default for TimeManagerMetrics {
    fn default() -> Self {
        Self {
            current_time: 0,
            time_resolution_ns: 100, // 100ns default resolution
            last_update: Utc::now(),
            total_updates: 0,
            avg_update_interval_ms: 0.0,
        }
    }
}

/// Time manager configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeManagerConfig {
    /// Time resolution in nanoseconds
    pub time_resolution_ns: u64,
    /// Update interval in milliseconds
    pub update_interval_ms: u64,
    /// Enable automatic updates
    pub enable_auto_updates: bool,
    /// Maximum time drift in nanoseconds
    pub max_drift_ns: u64,
}

impl Default for TimeManagerConfig {
    fn default() -> Self {
        Self {
            time_resolution_ns: 100, // 100ns resolution
            update_interval_ms: 1000, // 1 second updates
            enable_auto_updates: true,
            max_drift_ns: 1_000_000, // 1ms max drift
        }
    }
} 
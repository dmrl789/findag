//! FinDAG Time and HashTimer Implementation
//! 
//! This crate implements the FinDAG Time system and HashTimer functionality
//! for precise timestamping and deterministic ordering in the FinDAG blockchain.

pub mod time_manager;
pub mod hashtimer;
pub mod synchronization;
pub mod metrics;
pub mod errors;

pub use time_manager::*;
pub use hashtimer::*;
pub use synchronization::*;
pub use metrics::*;
pub use errors::*;

use findag_core::{Address, Hash, FinDAGTime};
use findag_types::{FindDAGResult, FindDAGError};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use metrics::{counter, gauge, histogram};
use chrono::{DateTime, Utc};

/// FinDAG Time Manager
pub struct FinDAGTimeManager {
    /// Current FinDAG time
    current_time: Arc<RwLock<FinDAGTime>>,
    /// Time synchronization manager
    sync_manager: Arc<TimeSyncManager>,
    /// HashTimer manager
    hashtimer_manager: Arc<HashTimerManager>,
    /// Configuration
    config: TimeManagerConfig,
    /// Metrics
    metrics: TimeMetrics,
}

/// Time manager configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeManagerConfig {
    /// Time resolution in nanoseconds
    pub time_resolution_ns: u64,
    /// Synchronization interval in milliseconds
    pub sync_interval_ms: u64,
    /// Maximum time drift in nanoseconds
    pub max_drift_ns: u64,
    /// Enable time synchronization
    pub enable_sync: bool,
    /// Enable HashTimer generation
    pub enable_hashtimer: bool,
    /// HashTimer nonce range
    pub hashtimer_nonce_range: u32,
}

impl Default for TimeManagerConfig {
    fn default() -> Self {
        Self {
            time_resolution_ns: 100, // 100ns resolution
            sync_interval_ms: 1000,  // 1 second sync interval
            max_drift_ns: 1000000,   // 1ms max drift
            enable_sync: true,
            enable_hashtimer: true,
            hashtimer_nonce_range: 1000000,
        }
    }
}

/// Time metrics
#[derive(Debug, Clone)]
pub struct TimeMetrics {
    /// Current FinDAG time
    pub current_time: u64,
    /// Time drift in nanoseconds
    pub time_drift_ns: i64,
    /// HashTimer generation rate
    pub hashtimer_rate: f64,
    /// Synchronization status
    pub sync_status: String,
    /// Last synchronization time
    pub last_sync: DateTime<Utc>,
}

impl FinDAGTimeManager {
    /// Create a new FinDAG Time Manager
    pub fn new(config: TimeManagerConfig) -> Self {
        let current_time = Arc::new(RwLock::new(FinDAGTime::now()));
        let sync_manager = Arc::new(TimeSyncManager::new(config.clone()));
        let hashtimer_manager = Arc::new(HashTimerManager::new(config.clone()));
        let metrics = TimeMetrics::default();
        
        Self {
            current_time,
            sync_manager,
            hashtimer_manager,
            config,
            metrics,
        }
    }

    /// Start the time manager
    pub async fn start(&mut self) -> FindDAGResult<()> {
        info!("Starting FinDAG Time Manager");
        
        // Start time synchronization
        if self.config.enable_sync {
            self.sync_manager.start().await?;
        }
        
        // Start HashTimer manager
        if self.config.enable_hashtimer {
            self.hashtimer_manager.start().await?;
        }
        
        info!("FinDAG Time Manager started");
        
        Ok(())
    }

    /// Stop the time manager
    pub async fn stop(&mut self) -> FindDAGResult<()> {
        info!("Stopping FinDAG Time Manager");
        
        // Stop time synchronization
        if self.config.enable_sync {
            self.sync_manager.stop().await?;
        }
        
        // Stop HashTimer manager
        if self.config.enable_hashtimer {
            self.hashtimer_manager.stop().await?;
        }
        
        info!("FinDAG Time Manager stopped");
        
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
        
        // Update metrics
        self.metrics.current_time = time.as_u64();
        
        // Record metrics
        gauge!("findag_time_current", time.as_u64() as f64);
        
        Ok(())
    }

    /// Generate HashTimer
    pub async fn generate_hashtimer(&self, content_hash: Hash) -> FindDAGResult<HashTimer> {
        let hashtimer = self.hashtimer_manager.generate_hashtimer(content_hash).await?;
        
        // Record metrics
        counter!("findag_hashtimer_generated", 1);
        
        Ok(hashtimer)
    }

    /// Verify HashTimer
    pub async fn verify_hashtimer(&self, hashtimer: &HashTimer) -> FindDAGResult<bool> {
        let is_valid = self.hashtimer_manager.verify_hashtimer(hashtimer).await?;
        
        // Record metrics
        if is_valid {
            counter!("findag_hashtimer_verified", 1);
        } else {
            counter!("findag_hashtimer_invalid", 1);
        }
        
        Ok(is_valid)
    }

    /// Get time metrics
    pub async fn get_metrics(&self) -> TimeMetrics {
        self.metrics.clone()
    }

    /// Update time metrics
    pub async fn update_metrics(&self) {
        let current_time = self.get_current_time().await;
        let drift = self.sync_manager.get_time_drift().await;
        
        self.metrics.current_time = current_time.as_u64();
        self.metrics.time_drift_ns = drift;
        self.metrics.last_sync = Utc::now();
        
        // Update Prometheus metrics
        gauge!("findag_time_current", current_time.as_u64() as f64);
        gauge!("findag_time_drift_ns", drift as f64);
    }

    /// Get time synchronization status
    pub async fn get_sync_status(&self) -> SyncStatus {
        self.sync_manager.get_status().await
    }

    /// Force time synchronization
    pub async fn force_sync(&self) -> FindDAGResult<()> {
        self.sync_manager.force_sync().await
    }
}

impl Default for TimeMetrics {
    fn default() -> Self {
        Self {
            current_time: 0,
            time_drift_ns: 0,
            hashtimer_rate: 0.0,
            sync_status: "unknown".to_string(),
            last_sync: Utc::now(),
        }
    }
} 
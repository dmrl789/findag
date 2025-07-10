//! Consensus metrics
//! 
//! This module handles consensus metrics collection and reporting.

use findag_core::{Address, Hash, HashTimer, FinDAGTime};
use findag_types::{ConsensusMetrics, FindDAGResult};

/// Metrics collector
pub struct MetricsCollector {
    /// Consensus metrics
    metrics: ConsensusMetrics,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            metrics: ConsensusMetrics::default(),
        }
    }

    /// Update metrics
    pub fn update_metrics(&mut self, new_metrics: ConsensusMetrics) {
        self.metrics = new_metrics;
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> &ConsensusMetrics {
        &self.metrics
    }

    /// Record round finalization
    pub fn record_round_finalization(&mut self, latency_ms: f64) {
        self.metrics.rounds_per_sec += 1.0;
        self.metrics.avg_round_latency_ms = 
            (self.metrics.avg_round_latency_ms + latency_ms) / 2.0;
    }

    /// Record block finalization
    pub fn record_block_finalization(&mut self, latency_ms: f64) {
        self.metrics.blocks_per_sec += 1.0;
        self.metrics.avg_finalization_time_ms = 
            (self.metrics.avg_finalization_time_ms + latency_ms) / 2.0;
    }
} 
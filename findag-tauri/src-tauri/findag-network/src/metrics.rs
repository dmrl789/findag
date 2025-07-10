//! Network metrics
//! 
//! This module handles network metrics collection and reporting.

use findag_types::{NetworkMetrics, FindDAGResult};

/// Network metrics collector
pub struct NetworkMetricsCollector {
    /// Network metrics
    metrics: NetworkMetrics,
}

impl NetworkMetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            metrics: NetworkMetrics::default(),
        }
    }

    /// Update metrics
    pub fn update_metrics(&mut self, new_metrics: NetworkMetrics) {
        self.metrics = new_metrics;
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> &NetworkMetrics {
        &self.metrics
    }

    /// Record peer connection
    pub fn record_peer_connected(&mut self) {
        self.metrics.connected_peers += 1;
        self.metrics.total_peers_discovered += 1;
    }

    /// Record peer disconnection
    pub fn record_peer_disconnected(&mut self) {
        if self.metrics.connected_peers > 0 {
            self.metrics.connected_peers -= 1;
        }
    }

    /// Record message sent
    pub fn record_message_sent(&mut self, bytes: usize) {
        self.metrics.messages_sent += 1;
        self.metrics.bytes_sent += bytes as u64;
    }

    /// Record message received
    pub fn record_message_received(&mut self, bytes: usize) {
        self.metrics.messages_received += 1;
        self.metrics.bytes_received += bytes as u64;
    }

    /// Record latency
    pub fn record_latency(&mut self, latency_ms: f64) {
        // Update average latency
        let total_latency = self.metrics.avg_latency_ms * self.metrics.messages_received as f64;
        let new_total = total_latency + latency_ms;
        self.metrics.avg_latency_ms = new_total / (self.metrics.messages_received + 1) as f64;
    }
} 
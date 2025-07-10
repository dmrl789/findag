//! API metrics collection
//! 
//! This module contains metrics collection for the FinDAG API.

use findag_types::{FindDAGResult, FindDAGError};
use metrics::{counter, gauge, histogram};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};

/// API metrics collector
pub struct APIMetricsCollector {
    /// Metrics data
    metrics: Arc<RwLock<HashMap<String, f64>>>,
}

impl APIMetricsCollector {
    /// Create new metrics collector
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Record request
    pub async fn record_request(&self, endpoint: &str, method: &str, status: u16, duration_ms: f64) {
        counter!("findag_api_requests_total", 1, "endpoint" => endpoint.to_string(), "method" => method.to_string());
        histogram!("findag_api_request_duration_ms", duration_ms, "endpoint" => endpoint.to_string(), "method" => method.to_string());
        
        if status < 400 {
            counter!("findag_api_successful_requests", 1, "endpoint" => endpoint.to_string());
        } else {
            counter!("findag_api_failed_requests", 1, "endpoint" => endpoint.to_string(), "status" => status.to_string());
        }
    }
    
    /// Record WebSocket connection
    pub async fn record_websocket_connection(&self) {
        counter!("findag_websocket_connections_total", 1);
        gauge!("findag_websocket_active_connections", 1.0);
    }
    
    /// Record WebSocket disconnection
    pub async fn record_websocket_disconnection(&self) {
        counter!("findag_websocket_disconnections_total", 1);
        gauge!("findag_websocket_active_connections", -1.0);
    }
    
    /// Record error
    pub async fn record_error(&self, error_type: &str, error_message: &str) {
        counter!("findag_api_errors_total", 1, "type" => error_type.to_string());
        error!("API Error [{}]: {}", error_type, error_message);
    }
    
    /// Get metrics summary
    pub async fn get_metrics_summary(&self) -> MetricsSummary {
        let metrics = self.metrics.read().await;
        
        MetricsSummary {
            total_requests: *metrics.get("total_requests").unwrap_or(&0.0) as u64,
            successful_requests: *metrics.get("successful_requests").unwrap_or(&0.0) as u64,
            failed_requests: *metrics.get("failed_requests").unwrap_or(&0.0) as u64,
            avg_response_time_ms: *metrics.get("avg_response_time_ms").unwrap_or(&0.0),
            active_connections: *metrics.get("active_connections").unwrap_or(&0.0) as u64,
        }
    }
}

/// Metrics summary
#[derive(Debug, Clone)]
pub struct MetricsSummary {
    /// Total requests
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Active connections
    pub active_connections: u64,
} 
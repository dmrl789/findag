use prometheus::{IntGauge, IntCounter, IntCounterVec, Histogram, HistogramOpts, Registry, Encoder, TextEncoder};
use std::sync::Once;

static INIT: Once = Once::new();

// Global registry
lazy_static::lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    
    // Basic metrics
    pub static ref BLOCK_TOTAL: IntCounter = IntCounter::new("findag_blocks_total", "Total blocks produced").unwrap();
    pub static ref TRANSACTION_TOTAL: IntCounter = IntCounter::new("findag_transactions_total", "Total transactions processed").unwrap();
    pub static ref ROUND_TOTAL: IntCounter = IntCounter::new("findag_rounds_total", "Total rounds completed").unwrap();
    
    // Performance metrics
    pub static ref BLOCK_LATENCY: Histogram = Histogram::with_opts(
        HistogramOpts::new("findag_block_latency_seconds", "Block production latency")
    ).unwrap();
    
    pub static ref ROUND_LATENCY: Histogram = Histogram::with_opts(
        HistogramOpts::new("findag_round_latency_seconds", "Round completion latency")
    ).unwrap();
    
    // Network metrics
    pub static ref PEER_COUNT: IntGauge = IntGauge::new("findag_peers_total", "Number of connected peers").unwrap();
    pub static ref API_CALLS: IntCounter = IntCounter::new("findag_api_calls_total", "Total API calls").unwrap();
    
    // Error metrics
    pub static ref ERROR_COUNT: IntCounter = IntCounter::new("findag_errors_total", "Total errors").unwrap();
    
    // Cross-shard metrics
    pub static ref CROSS_SHARD_TX_ATTEMPTS: IntCounter = IntCounter::new("findag_cross_shard_tx_attempts_total", "Cross-shard transaction attempts").unwrap();
    pub static ref CROSS_SHARD_TX_SUCCESS: IntCounter = IntCounter::new("findag_cross_shard_tx_success_total", "Successful cross-shard transactions").unwrap();
    pub static ref CROSS_SHARD_TX_FAILURE: IntCounter = IntCounter::new("findag_cross_shard_tx_failure_total", "Failed cross-shard transactions").unwrap();
    pub static ref CROSS_SHARD_TX_LATENCY: Histogram = Histogram::with_opts(
        HistogramOpts::new("findag_cross_shard_tx_latency_seconds", "Cross-shard transaction latency")
    ).unwrap();
    
    // Bridge metrics
    pub static ref BRIDGE_ATTEMPTS: IntCounter = IntCounter::new("findag_bridge_attempts_total", "Cross-chain bridge attempts").unwrap();
    pub static ref BRIDGE_SUCCESS: IntCounter = IntCounter::new("findag_bridge_success_total", "Successful bridge operations").unwrap();
    pub static ref BRIDGE_FAILURE: IntCounter = IntCounter::new("findag_bridge_failure_total", "Failed bridge operations").unwrap();
    pub static ref BRIDGE_LATENCY: Histogram = Histogram::with_opts(
        HistogramOpts::new("findag_bridge_latency_seconds", "Bridge operation latency")
    ).unwrap();
    
    // Confidential transaction metrics
    pub static ref CONFIDENTIAL_TX_ATTEMPTS: IntCounter = IntCounter::new("findag_confidential_tx_attempts_total", "Confidential transaction attempts").unwrap();
    pub static ref CONFIDENTIAL_TX_SUCCESS: IntCounter = IntCounter::new("findag_confidential_tx_success_total", "Successful confidential transactions").unwrap();
    pub static ref CONFIDENTIAL_TX_FAILURE: IntCounter = IntCounter::new("findag_confidential_tx_failure_total", "Failed confidential transactions").unwrap();
    pub static ref CONFIDENTIAL_TX_LATENCY: Histogram = Histogram::with_opts(
        HistogramOpts::new("findag_confidential_tx_latency_seconds", "Confidential transaction latency")
    ).unwrap();
    
    // Identity operation metrics
    pub static ref IDENTITY_OP_ATTEMPTS: IntCounter = IntCounter::new("findag_identity_op_attempts_total", "Identity operation attempts").unwrap();
    pub static ref IDENTITY_OP_SUCCESS: IntCounter = IntCounter::new("findag_identity_op_success_total", "Successful identity operations").unwrap();
    pub static ref IDENTITY_OP_FAILURE: IntCounter = IntCounter::new("findag_identity_op_failure_total", "Failed identity operations").unwrap();
    pub static ref IDENTITY_OP_LATENCY: Histogram = Histogram::with_opts(
        HistogramOpts::new("findag_identity_op_latency_seconds", "Identity operation latency")
    ).unwrap();
}

pub fn register_metrics() {
    INIT.call_once(|| {
        // Register all metrics
        REGISTRY.register(Box::new(BLOCK_TOTAL.clone())).ok();
        REGISTRY.register(Box::new(TRANSACTION_TOTAL.clone())).ok();
        REGISTRY.register(Box::new(ROUND_TOTAL.clone())).ok();
        REGISTRY.register(Box::new(BLOCK_LATENCY.clone())).ok();
        REGISTRY.register(Box::new(ROUND_LATENCY.clone())).ok();
        REGISTRY.register(Box::new(PEER_COUNT.clone())).ok();
        REGISTRY.register(Box::new(API_CALLS.clone())).ok();
        REGISTRY.register(Box::new(ERROR_COUNT.clone())).ok();
        REGISTRY.register(Box::new(CROSS_SHARD_TX_ATTEMPTS.clone())).ok();
        REGISTRY.register(Box::new(CROSS_SHARD_TX_SUCCESS.clone())).ok();
        REGISTRY.register(Box::new(CROSS_SHARD_TX_FAILURE.clone())).ok();
        REGISTRY.register(Box::new(CROSS_SHARD_TX_LATENCY.clone())).ok();
        REGISTRY.register(Box::new(BRIDGE_ATTEMPTS.clone())).ok();
        REGISTRY.register(Box::new(BRIDGE_SUCCESS.clone())).ok();
        REGISTRY.register(Box::new(BRIDGE_FAILURE.clone())).ok();
        REGISTRY.register(Box::new(BRIDGE_LATENCY.clone())).ok();
        REGISTRY.register(Box::new(CONFIDENTIAL_TX_ATTEMPTS.clone())).ok();
        REGISTRY.register(Box::new(CONFIDENTIAL_TX_SUCCESS.clone())).ok();
        REGISTRY.register(Box::new(CONFIDENTIAL_TX_FAILURE.clone())).ok();
        REGISTRY.register(Box::new(CONFIDENTIAL_TX_LATENCY.clone())).ok();
        REGISTRY.register(Box::new(IDENTITY_OP_ATTEMPTS.clone())).ok();
        REGISTRY.register(Box::new(IDENTITY_OP_SUCCESS.clone())).ok();
        REGISTRY.register(Box::new(IDENTITY_OP_FAILURE.clone())).ok();
        REGISTRY.register(Box::new(IDENTITY_OP_LATENCY.clone())).ok();
    });
}

pub fn gather_metrics() -> Vec<u8> {
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    buffer
} 
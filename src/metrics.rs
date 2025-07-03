use prometheus::{Histogram, HistogramOpts, IntCounter, IntGauge, IntCounterVec, Registry};
use std::sync::Once;
use lazy_static::lazy_static;
use prometheus::TextEncoder;
use prometheus::Encoder;

static INIT: Once = Once::new();

// Global registry
lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    
    // Basic metrics
    pub static ref BLOCK_TOTAL: IntCounter = IntCounter::new("block_total", "Total blocks produced").unwrap();
    pub static ref BLOCKS_PER_SEC: IntGauge = IntGauge::new("blocks_per_sec", "Blocks per second").unwrap();
    pub static ref TPS: IntGauge = IntGauge::new("tps", "Transactions per second").unwrap();
    pub static ref TX_TOTAL: IntCounter = IntCounter::new("tx_total", "Total transactions").unwrap();
    
    // Performance metrics
    pub static ref BLOCK_LATENCY: Histogram = Histogram::with_opts(HistogramOpts::new("block_latency", "Block production latency")).unwrap();
    
    // Network metrics
    pub static ref PEER_COUNT: IntGauge = IntGauge::new("peer_count", "Peer count").unwrap();
    
    // Error metrics
    pub static ref ERROR_COUNT: IntCounterVec = IntCounterVec::new(prometheus::Opts::new("error_count", "Error count"), &["type"]).unwrap();
    
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
    
    pub static ref MEMPOOL_SIZE: IntGauge = IntGauge::new("mempool_size", "Mempool size").unwrap();
    
    pub static ref API_LATENCY: Histogram = Histogram::with_opts(
        HistogramOpts::new("api_latency_seconds", "API endpoint latency")
            .buckets(vec![0.01, 0.05, 0.1, 0.5, 1.0, 2.5, 5.0, 10.0])
    ).unwrap();
    
    pub static ref MEMORY_USAGE: IntGauge = IntGauge::new("memory_usage_bytes", "Process memory usage").unwrap();
    
    pub static ref CONFIG_RELOADS: IntCounter = IntCounter::new("config_reloads_total", "Configuration reload count").unwrap();
    
    pub static ref TLS_HANDSHAKES: IntCounterVec = IntCounterVec::new(
        prometheus::Opts::new("tls_handshakes_total", "TLS handshake count"), 
        &["status"]
    ).unwrap();
}

pub fn register_metrics() {
    INIT.call_once(|| {
        // Register all metrics
        REGISTRY.register(Box::new(BLOCK_TOTAL.clone())).ok();
        REGISTRY.register(Box::new(BLOCKS_PER_SEC.clone())).ok();
        REGISTRY.register(Box::new(TPS.clone())).ok();
        REGISTRY.register(Box::new(TX_TOTAL.clone())).ok();
        REGISTRY.register(Box::new(BLOCK_LATENCY.clone())).ok();
        REGISTRY.register(Box::new(PEER_COUNT.clone())).ok();
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
        REGISTRY.register(Box::new(MEMPOOL_SIZE.clone())).ok();
        REGISTRY.register(Box::new(API_LATENCY.clone())).ok();
        REGISTRY.register(Box::new(MEMORY_USAGE.clone())).ok();
        REGISTRY.register(Box::new(CONFIG_RELOADS.clone())).ok();
        REGISTRY.register(Box::new(TLS_HANDSHAKES.clone())).ok();
    });
}

pub fn gather_metrics() -> Vec<u8> {
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    buffer
} 
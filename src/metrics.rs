use prometheus::{IntGauge, IntCounter, IntCounterVec, Histogram, HistogramOpts, Registry, Encoder, TextEncoder};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref TPS: IntGauge = IntGauge::new("findag_tps", "Transactions per second").unwrap();
    pub static ref BLOCKS_PER_SEC: IntGauge = IntGauge::new("findag_blocks_per_sec", "Blocks per second").unwrap();
    pub static ref MEMPOOL_SIZE: IntGauge = IntGauge::new("findag_mempool_size", "Current mempool size").unwrap();
    pub static ref PEER_COUNT: IntGauge = IntGauge::new("findag_peer_count", "Current peer count").unwrap();
    pub static ref TX_TOTAL: IntCounter = IntCounter::new("findag_tx_total", "Total transactions processed").unwrap();
    pub static ref BLOCK_TOTAL: IntCounter = IntCounter::new("findag_block_total", "Total blocks produced").unwrap();
    pub static ref PER_ASSET_TPS: IntCounterVec = IntCounterVec::new(
        prometheus::Opts::new("findag_per_asset_tps", "Transactions per second per asset"),
        &["asset"]
    ).unwrap();
    pub static ref BLOCK_LATENCY: Histogram = Histogram::with_opts(
        HistogramOpts::new("findag_block_latency_seconds", "Block production latency in seconds")
    ).unwrap();
    pub static ref ROUND_LATENCY: Histogram = Histogram::with_opts(
        HistogramOpts::new("findag_round_latency_seconds", "Round checkpoint latency in seconds")
    ).unwrap();
    pub static ref API_CALLS: IntCounterVec = IntCounterVec::new(
        prometheus::Opts::new("findag_api_calls", "API call count by endpoint"),
        &["endpoint"]
    ).unwrap();
    pub static ref ERROR_COUNT: IntCounterVec = IntCounterVec::new(
        prometheus::Opts::new("findag_error_count", "Error count by type"),
        &["type"]
    ).unwrap();
    pub static ref CROSS_SHARD_TX_ATTEMPTS: IntCounter = IntCounter::new("findag_cross_shard_tx_attempts", "Cross-shard transaction attempts").unwrap();
    pub static ref CROSS_SHARD_TX_SUCCESS: IntCounter = IntCounter::new("findag_cross_shard_tx_success", "Cross-shard transaction successes").unwrap();
    pub static ref CROSS_SHARD_TX_FAILURE: IntCounter = IntCounter::new("findag_cross_shard_tx_failure", "Cross-shard transaction failures").unwrap();
    pub static ref CROSS_SHARD_TX_LATENCY: Histogram = Histogram::with_opts(prometheus::opts!("findag_cross_shard_tx_latency_seconds", "Cross-shard transaction latency in seconds")).unwrap();
    pub static ref BRIDGE_ATTEMPTS: IntCounter = IntCounter::new("findag_bridge_attempts", "Bridge operation attempts").unwrap();
    pub static ref BRIDGE_SUCCESS: IntCounter = IntCounter::new("findag_bridge_success", "Bridge operation successes").unwrap();
    pub static ref BRIDGE_FAILURE: IntCounter = IntCounter::new("findag_bridge_failure", "Bridge operation failures").unwrap();
    pub static ref BRIDGE_LATENCY: Histogram = Histogram::with_opts(prometheus::opts!("findag_bridge_latency_seconds", "Bridge operation latency in seconds")).unwrap();
    pub static ref CONFIDENTIAL_TX_ATTEMPTS: IntCounter = IntCounter::new("findag_confidential_tx_attempts", "Confidential transaction attempts").unwrap();
    pub static ref CONFIDENTIAL_TX_SUCCESS: IntCounter = IntCounter::new("findag_confidential_tx_success", "Confidential transaction successes").unwrap();
    pub static ref CONFIDENTIAL_TX_FAILURE: IntCounter = IntCounter::new("findag_confidential_tx_failure", "Confidential transaction failures").unwrap();
    pub static ref CONFIDENTIAL_TX_LATENCY: Histogram = Histogram::with_opts(prometheus::opts!("findag_confidential_tx_latency_seconds", "Confidential transaction latency in seconds")).unwrap();
    pub static ref IDENTITY_OP_ATTEMPTS: IntCounter = IntCounter::new("findag_identity_op_attempts", "Identity/KYC operation attempts").unwrap();
    pub static ref IDENTITY_OP_SUCCESS: IntCounter = IntCounter::new("findag_identity_op_success", "Identity/KYC operation successes").unwrap();
    pub static ref IDENTITY_OP_FAILURE: IntCounter = IntCounter::new("findag_identity_op_failure", "Identity/KYC operation failures").unwrap();
    pub static ref IDENTITY_OP_LATENCY: Histogram = Histogram::with_opts(prometheus::opts!("findag_identity_op_latency_seconds", "Identity/KYC operation latency in seconds")).unwrap();
}

pub fn register_metrics() {
    REGISTRY.register(Box::new(TPS.clone())).ok();
    REGISTRY.register(Box::new(BLOCKS_PER_SEC.clone())).ok();
    REGISTRY.register(Box::new(MEMPOOL_SIZE.clone())).ok();
    REGISTRY.register(Box::new(PEER_COUNT.clone())).ok();
    REGISTRY.register(Box::new(TX_TOTAL.clone())).ok();
    REGISTRY.register(Box::new(BLOCK_TOTAL.clone())).ok();
    REGISTRY.register(Box::new(PER_ASSET_TPS.clone())).ok();
    REGISTRY.register(Box::new(BLOCK_LATENCY.clone())).ok();
    REGISTRY.register(Box::new(ROUND_LATENCY.clone())).ok();
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
}

pub fn gather_metrics() -> String {
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
} 
//! FinDAG common type definitions
//! 
//! This crate provides shared type definitions used across the FinDAG system,
//! including blockchain types, consensus types, and network types.

pub mod blockchain;
pub mod consensus;
pub mod network;
pub mod trading;
pub mod wallet;

pub use blockchain::*;
pub use consensus::*;
pub use network::*;
pub use trading::*;
pub use wallet::*;

/// Re-export common types from findag-core
pub use findag_core::{Address, Hash, HashTimer, FinDAGTime};

/// Common error types
#[derive(Debug, thiserror::Error)]
pub enum FindDAGError {
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    
    #[error("Invalid block: {0}")]
    InvalidBlock(String),
    
    #[error("Invalid round: {0}")]
    InvalidRound(String),
    
    #[error("Consensus error: {0}")]
    ConsensusError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Result type for FinDAG operations
pub type FindDAGResult<T> = Result<T, FindDAGError>;

/// Common status types
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Status {
    /// Node is starting up
    Starting,
    /// Node is running normally
    Running,
    /// Node is stopping
    Stopping,
    /// Node has stopped
    Stopped,
    /// Node encountered an error
    Error,
    /// Node is syncing with the network
    Syncing,
    /// Node is validating blocks
    Validating,
}

/// Node configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeConfig {
    /// Node ID
    pub node_id: String,
    /// Network port
    pub port: u16,
    /// Data directory
    pub data_dir: String,
    /// Log level
    pub log_level: String,
    /// Maximum peers
    pub max_peers: usize,
    /// Block interval in milliseconds
    pub block_interval_ms: u64,
    /// Round interval in milliseconds
    pub round_interval_ms: u64,
    /// Enable metrics
    pub enable_metrics: bool,
    /// Metrics port
    pub metrics_port: u16,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            node_id: "node-1".to_string(),
            port: 8080,
            data_dir: "./data".to_string(),
            log_level: "info".to_string(),
            max_peers: 50,
            block_interval_ms: 50,
            round_interval_ms: 250,
            enable_metrics: true,
            metrics_port: 9898,
        }
    }
}

/// System information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemInfo {
    /// Node status
    pub status: Status,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Version
    pub version: String,
    /// Build timestamp
    pub build_timestamp: String,
    /// Platform
    pub platform: String,
    /// Architecture
    pub architecture: String,
}

/// Performance metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceMetrics {
    /// Transactions per second
    pub tps: f64,
    /// Blocks per second
    pub blocks_per_sec: f64,
    /// Rounds per second
    pub rounds_per_sec: f64,
    /// Average block latency in milliseconds
    pub avg_block_latency_ms: f64,
    /// Average round latency in milliseconds
    pub avg_round_latency_ms: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Network bytes received
    pub network_bytes_received: u64,
    /// Network bytes sent
    pub network_bytes_sent: u64,
    /// Active connections
    pub active_connections: usize,
    /// Mempool size
    pub mempool_size: usize,
}

/// Network peer information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PeerInfo {
    /// Peer ID
    pub peer_id: String,
    /// Peer address
    pub address: String,
    /// Connection status
    pub status: PeerStatus,
    /// Last seen timestamp
    pub last_seen: chrono::DateTime<chrono::Utc>,
    /// Latency in milliseconds
    pub latency_ms: Option<u64>,
    /// Version
    pub version: Option<String>,
}

/// Peer connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PeerStatus {
    /// Connected
    Connected,
    /// Connecting
    Connecting,
    /// Disconnected
    Disconnected,
    /// Failed
    Failed,
}

/// API response wrapper
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse<T> {
    /// Success status
    pub success: bool,
    /// Response data
    pub data: Option<T>,
    /// Error message
    pub error: Option<String>,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    /// Create a successful response
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create an error response
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: chrono::Utc::now(),
        }
    }
} 
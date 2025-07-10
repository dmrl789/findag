pub mod address;
pub mod types;
pub mod dag_engine;
pub mod tx_pool;
pub mod block_producer;
pub mod block_production_loop;
pub mod round_checkpoint_loop;
pub mod wallet;
pub mod handle_registry;
pub mod bridge;
pub mod confidential;
pub mod identity;

pub use address::Address;
pub use types::{Transaction, Block, Round, ShardId, AssetRecord, AssetHistory};
pub use dag_engine::DagEngine;
pub use tx_pool::{TxPool, ShardedTxPool};
pub use block_producer::BlockProducer;
pub use wallet::Wallet;
pub use handle_registry::HandleRegistry;

// Re-export commonly used types
pub use ed25519_dalek::{Signature, VerifyingKey, SigningKey};
pub use serde::{Serialize, Deserialize};

/// Supported assets for FinDAG transactions
pub const SUPPORTED_ASSETS: &[&str] = &[
    "EUR", "USD", "GBP", "JPY", "CHF", "SGD", "AED", "CNY",
    "BUND", "OAT", "BTP", "GILT", "UST", "JGB", "T-BILL",
    "CP", "CD", "XAU", "XAG", "XPT", "XPD",
    "XS1234567890", "FR0000120271", "BE0003796134", "DE0001135275",
    "ETF1", "UCITS1", "BTC", "ETH", "USDT", "USDC"
];

/// FinDAG-specific error types
#[derive(thiserror::Error, Debug)]
pub enum FinDAGError {
    #[error("Invalid address format: {0}")]
    InvalidAddress(String),
    
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    
    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u64, available: u64 },
    
    #[error("Asset not supported: {0}")]
    UnsupportedAsset(String),
    
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    
    #[error("Block validation failed: {0}")]
    BlockValidationFailed(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Consensus error: {0}")]
    ConsensusError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Result type for FinDAG operations
pub type FinDAGResult<T> = Result<T, FinDAGError>;

/// Core configuration for FinDAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinDAGConfig {
    /// Block production interval in milliseconds
    pub block_interval_ms: u64,
    /// Round checkpoint interval in milliseconds
    pub round_interval_ms: u64,
    /// Maximum transactions per block
    pub max_transactions_per_block: usize,
    /// Maximum mempool size per shard
    pub max_mempool_size: usize,
    /// Number of shards
    pub shard_count: u16,
    /// Data directory path
    pub data_dir: String,
    /// Network port for P2P communication
    pub p2p_port: u16,
    /// HTTP API port
    pub api_port: u16,
    /// Metrics port
    pub metrics_port: u16,
    /// Enable debug logging
    pub debug_logging: bool,
    /// Enable performance profiling
    pub enable_profiling: bool,
}

impl Default for FinDAGConfig {
    fn default() -> Self {
        Self {
            block_interval_ms: 50,
            round_interval_ms: 200,
            max_transactions_per_block: 1000,
            max_mempool_size: 10000,
            shard_count: 4,
            data_dir: "findag_data".to_string(),
            p2p_port: 9001,
            api_port: 8080,
            metrics_port: 9898,
            debug_logging: false,
            enable_profiling: false,
        }
    }
}

/// FinDAG application state
pub struct FinDAGApp {
    pub config: FinDAGConfig,
    pub dag_engine: DagEngine,
    pub tx_pool: ShardedTxPool,
    pub wallet: Wallet,
    pub handle_registry: HandleRegistry,
}

impl FinDAGApp {
    /// Create a new FinDAG application instance
    pub async fn new(config: FinDAGConfig) -> FinDAGResult<Self> {
        let dag_engine = DagEngine::new().await;
        let tx_pool = ShardedTxPool::new_with_whitelist_per_shard(
            config.max_mempool_size,
            std::sync::Arc::new(std::sync::Mutex::new(
                SUPPORTED_ASSETS.iter().map(|s| s.to_string()).collect()
            )),
            config.shard_count as usize,
        );
        let wallet = Wallet::new(&config.data_dir)?;
        let handle_registry = HandleRegistry::new(&config.data_dir)?;

        Ok(Self {
            config,
            dag_engine,
            tx_pool,
            wallet,
            handle_registry,
        })
    }

    /// Start the FinDAG application
    pub async fn start(&mut self) -> FinDAGResult<()> {
        log::info!("Starting FinDAG application...");
        
        // Initialize components
        self.initialize_components().await?;
        
        // Start background tasks
        self.start_background_tasks().await?;
        
        log::info!("FinDAG application started successfully");
        Ok(())
    }

    /// Stop the FinDAG application
    pub async fn stop(&mut self) -> FinDAGResult<()> {
        log::info!("Stopping FinDAG application...");
        
        // Cleanup and shutdown
        self.cleanup().await?;
        
        log::info!("FinDAG application stopped successfully");
        Ok(())
    }

    async fn initialize_components(&mut self) -> FinDAGResult<()> {
        // Initialize DAG engine
        log::debug!("Initializing DAG engine...");
        
        // Initialize transaction pool
        log::debug!("Initializing transaction pool...");
        
        // Initialize wallet
        log::debug!("Initializing wallet...");
        
        // Initialize handle registry
        log::debug!("Initializing handle registry...");
        
        Ok(())
    }

    async fn start_background_tasks(&mut self) -> FinDAGResult<()> {
        // Start block production loop
        log::debug!("Starting block production loop...");
        
        // Start round checkpoint loop
        log::debug!("Starting round checkpoint loop...");
        
        // Start network propagation
        log::debug!("Starting network propagation...");
        
        Ok(())
    }

    async fn cleanup(&mut self) -> FinDAGResult<()> {
        // Stop background tasks
        log::debug!("Stopping background tasks...");
        
        // Flush storage
        log::debug!("Flushing storage...");
        
        // Close connections
        log::debug!("Closing connections...");
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_findag_app_creation() {
        let config = FinDAGConfig::default();
        let app = FinDAGApp::new(config).await;
        assert!(app.is_ok());
    }

    #[test]
    fn test_supported_assets() {
        assert!(SUPPORTED_ASSETS.contains(&"USD"));
        assert!(SUPPORTED_ASSETS.contains(&"EUR"));
        assert!(SUPPORTED_ASSETS.contains(&"BTC"));
    }
} 
//! FinDAG Sled-based persistent storage
//! 
//! This crate implements persistent storage for FinDAG using Sled,
//! providing crash-safe database operations for blockchain state.

pub mod database;
pub mod blocks;
pub mod transactions;
pub mod rounds;
pub mod assets;
pub mod wallets;
pub mod validators;
pub mod governance;
pub mod metrics;

pub use database::*;
pub use blocks::*;
pub use transactions::*;
pub use rounds::*;
pub use assets::*;
pub use wallets::*;
pub use validators::*;
pub use governance::*;
pub use metrics::*;

use findag_core::{Address, Hash, HashTimer, FinDAGTime};
use findag_types::{
    Block, Round, Transaction, Asset, Balance, Wallet, Validator, GovernanceProposal,
    FindDAGResult, FindDAGError,
};

use sled::{Db, Tree};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tracing::{info, warn, error, debug};
use metrics::{counter, gauge, histogram};

/// Storage manager
pub struct StorageManager {
    /// Sled database
    db: Db,
    /// Database trees
    trees: HashMap<String, Tree>,
    /// Configuration
    config: StorageConfig,
    /// Event sender
    event_sender: mpsc::Sender<StorageEvent>,
    /// Command receiver
    command_receiver: mpsc::Receiver<StorageCommand>,
    /// Metrics
    metrics: StorageMetrics,
}

/// Storage configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// Database path
    pub db_path: String,
    /// Cache size in bytes
    pub cache_size: usize,
    /// Flush interval in milliseconds
    pub flush_interval_ms: u64,
    /// Enable compression
    pub enable_compression: bool,
    /// Enable metrics
    pub enable_metrics: bool,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            db_path: "./data".to_string(),
            cache_size: 1024 * 1024 * 100, // 100MB
            flush_interval_ms: 1000,
            enable_compression: true,
            enable_metrics: true,
        }
    }
}

/// Storage metrics
#[derive(Debug, Clone)]
pub struct StorageMetrics {
    /// Total operations
    pub total_operations: u64,
    /// Read operations
    pub read_operations: u64,
    /// Write operations
    pub write_operations: u64,
    /// Delete operations
    pub delete_operations: u64,
    /// Database size in bytes
    pub database_size_bytes: u64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Average operation latency in milliseconds
    pub avg_operation_latency_ms: f64,
    /// Storage uptime in seconds
    pub uptime_seconds: u64,
}

/// Storage event
#[derive(Debug, Clone)]
pub enum StorageEvent {
    /// Data written
    DataWritten {
        key: String,
        size: usize,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    /// Data read
    DataRead {
        key: String,
        size: usize,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    /// Data deleted
    DataDeleted {
        key: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    /// Database flushed
    DatabaseFlushed {
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    /// Storage error
    StorageError {
        error: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
}

/// Storage command
#[derive(Debug, Clone)]
pub enum StorageCommand {
    /// Write data
    WriteData {
        tree: String,
        key: Vec<u8>,
        value: Vec<u8>,
    },
    /// Read data
    ReadData {
        tree: String,
        key: Vec<u8>,
    },
    /// Delete data
    DeleteData {
        tree: String,
        key: Vec<u8>,
    },
    /// Flush database
    FlushDatabase,
    /// Get metrics
    GetMetrics,
    /// Update configuration
    UpdateConfig(StorageConfig),
}

impl StorageManager {
    /// Create a new storage manager
    pub fn new(
        config: StorageConfig,
        event_sender: mpsc::Sender<StorageEvent>,
        command_receiver: mpsc::Receiver<StorageCommand>,
    ) -> FindDAGResult<Self> {
        // Open database
        let db = sled::open(&config.db_path)?;
        
        // Initialize trees
        let mut trees = HashMap::new();
        let tree_names = vec![
            "blocks", "rounds", "transactions", "assets", "balances",
            "wallets", "validators", "governance", "metadata",
        ];
        
        for tree_name in tree_names {
            let tree = db.open_tree(tree_name)?;
            trees.insert(tree_name.to_string(), tree);
        }
        
        let metrics = StorageMetrics::default();
        
        Ok(Self {
            db,
            trees,
            config,
            event_sender,
            command_receiver,
            metrics,
        })
    }

    /// Start the storage manager
    pub async fn start(&mut self) -> FindDAGResult<()> {
        info!("Starting storage manager");
        
        // Initialize metrics
        self.initialize_metrics();
        
        // Start storage loop
        self.storage_loop().await?;
        
        Ok(())
    }

    /// Stop the storage manager
    pub async fn stop(&mut self) -> FindDAGResult<()> {
        info!("Stopping storage manager");
        
        // Flush database
        self.db.flush()?;
        
        Ok(())
    }

    /// Get storage metrics
    pub async fn get_metrics(&self) -> StorageMetrics {
        self.metrics.clone()
    }

    /// Write data to storage
    pub async fn write_data(&mut self, tree: &str, key: Vec<u8>, value: Vec<u8>) -> FindDAGResult<()> {
        let start_time = std::time::Instant::now();
        
        if let Some(tree) = self.trees.get(tree) {
            tree.insert(key.clone(), value.clone())?;
            
            // Update metrics
            self.metrics.write_operations += 1;
            self.metrics.total_operations += 1;
            
            // Send event
            let _ = self.event_sender.send(StorageEvent::DataWritten {
                key: String::from_utf8_lossy(&key).to_string(),
                size: value.len(),
                timestamp: chrono::Utc::now(),
            }).await;
            
            // Record latency
            let latency = start_time.elapsed().as_millis() as f64;
            self.metrics.avg_operation_latency_ms = 
                (self.metrics.avg_operation_latency_ms + latency) / 2.0;
            
            counter!("findag_storage_write_operations", 1);
            histogram!("findag_storage_operation_latency_ms", latency);
        }
        
        Ok(())
    }

    /// Read data from storage
    pub async fn read_data(&self, tree: &str, key: Vec<u8>) -> FindDAGResult<Option<Vec<u8>>> {
        let start_time = std::time::Instant::now();
        
        if let Some(tree) = self.trees.get(tree) {
            let result = tree.get(key.clone())?;
            
            // Update metrics
            self.metrics.read_operations += 1;
            self.metrics.total_operations += 1;
            
            if let Some(value) = &result {
                // Send event
                let _ = self.event_sender.send(StorageEvent::DataRead {
                    key: String::from_utf8_lossy(&key).to_string(),
                    size: value.len(),
                    timestamp: chrono::Utc::now(),
                }).await;
            }
            
            // Record latency
            let latency = start_time.elapsed().as_millis() as f64;
            self.metrics.avg_operation_latency_ms = 
                (self.metrics.avg_operation_latency_ms + latency) / 2.0;
            
            counter!("findag_storage_read_operations", 1);
            histogram!("findag_storage_operation_latency_ms", latency);
            
            return Ok(result.map(|v| v.to_vec()));
        }
        
        Ok(None)
    }

    /// Delete data from storage
    pub async fn delete_data(&mut self, tree: &str, key: Vec<u8>) -> FindDAGResult<()> {
        let start_time = std::time::Instant::now();
        
        if let Some(tree) = self.trees.get(tree) {
            tree.remove(key.clone())?;
            
            // Update metrics
            self.metrics.delete_operations += 1;
            self.metrics.total_operations += 1;
            
            // Send event
            let _ = self.event_sender.send(StorageEvent::DataDeleted {
                key: String::from_utf8_lossy(&key).to_string(),
                timestamp: chrono::Utc::now(),
            }).await;
            
            // Record latency
            let latency = start_time.elapsed().as_millis() as f64;
            self.metrics.avg_operation_latency_ms = 
                (self.metrics.avg_operation_latency_ms + latency) / 2.0;
            
            counter!("findag_storage_delete_operations", 1);
            histogram!("findag_storage_operation_latency_ms", latency);
        }
        
        Ok(())
    }

    /// Flush database
    pub async fn flush_database(&mut self) -> FindDAGResult<()> {
        self.db.flush()?;
        
        // Send event
        let _ = self.event_sender.send(StorageEvent::DatabaseFlushed {
            timestamp: chrono::Utc::now(),
        }).await;
        
        Ok(())
    }

    /// Initialize metrics
    fn initialize_metrics(&self) {
        gauge!("findag_storage_database_size_bytes", 0.0);
        gauge!("findag_storage_cache_hit_rate", 0.0);
        counter!("findag_storage_total_operations", 0);
        counter!("findag_storage_read_operations", 0);
        counter!("findag_storage_write_operations", 0);
        counter!("findag_storage_delete_operations", 0);
        histogram!("findag_storage_operation_latency_ms", 0.0);
    }

    /// Main storage loop
    async fn storage_loop(&mut self) -> FindDAGResult<()> {
        info!("Starting storage loop");
        
        while let Some(command) = self.command_receiver.recv().await {
            match command {
                StorageCommand::WriteData { tree, key, value } => {
                    self.write_data(&tree, key, value).await?;
                }
                StorageCommand::ReadData { tree, key } => {
                    let _result = self.read_data(&tree, key).await?;
                }
                StorageCommand::DeleteData { tree, key } => {
                    self.delete_data(&tree, key).await?;
                }
                StorageCommand::FlushDatabase => {
                    self.flush_database().await?;
                }
                StorageCommand::GetMetrics => {
                    // Metrics are updated in real-time
                }
                StorageCommand::UpdateConfig(config) => {
                    info!("Updating storage configuration");
                    self.config = config;
                }
            }
        }
        
        Ok(())
    }
}

impl Default for StorageMetrics {
    fn default() -> Self {
        Self {
            total_operations: 0,
            read_operations: 0,
            write_operations: 0,
            delete_operations: 0,
            database_size_bytes: 0,
            cache_hit_rate: 0.0,
            avg_operation_latency_ms: 0.0,
            uptime_seconds: 0,
        }
    }
} 
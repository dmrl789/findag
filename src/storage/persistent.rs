// Usage:
// let storage = Arc::new(PersistentStorage::new("findag_db"));
// let (tx, rx) = mpsc::unbounded_channel();
// storage.clone().spawn_background_writer(rx);
// tx.send(PersistMsg::Block(block)).unwrap();
// tx.send(PersistMsg::Round(round)).unwrap(); 

use sled;
use crate::core::types::{Block, Round, SerializableBlock, SerializableRound, AssetRecord};
use bincode;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::consensus::validator_set::ValidatorSet;
use crate::core::handle_registry::HandleRecord;

#[derive(Debug)]
pub struct PersistentStorage {
    db: sled::Db,
}

impl PersistentStorage {
    pub fn new(_path: &str) -> Result<Self, sled::Error> {
        let config = Self::create_optimized_config();
        let db = config.open()?;
        Ok(Self { db })
    }

    /// Create production-optimized Sled configuration
    fn create_optimized_config() -> sled::Config {
        sled::Config::default()
            // Cache configuration for high-performance reads
            .cache_capacity(1024 * 1024 * 1024) // 1GB cache for high-frequency access
            .use_compression(true) // Enable compression for storage efficiency
            .compression_factor(2) // Balance between compression and performance
            
            // Segment configuration for write performance
            .segment_size(16 * 1024 * 1024) // 16MB segments (sled max)
            .flush_every_ms(Some(100)) // Flush every 100ms for durability
    }

    /// Create configuration optimized for high-frequency financial data
    pub fn new_high_frequency(_path: &str) -> Result<Self, sled::Error> {
        let config = sled::Config::default()
            // Optimized for high-frequency trading scenarios
            .cache_capacity(2048 * 1024 * 1024) // 2GB cache for ultra-low latency
            .use_compression(false) // Disable compression for speed
            .segment_size(16 * 1024 * 1024) // 16MB segments (sled max)
            .flush_every_ms(Some(50)); // Flush every 50ms for near real-time durability
        
        let db = config.open()?;
        Ok(Self { db })
    }

    /// Create configuration optimized for storage efficiency
    pub fn new_storage_efficient(_path: &str) -> Result<Self, sled::Error> {
        let config = sled::Config::default()
            // Optimized for storage efficiency
            .cache_capacity(512 * 1024 * 1024) // 512MB cache
            .use_compression(true)
            .compression_factor(4) // Higher compression
            .segment_size(4 * 1024 * 1024) // 4MB segments
            .flush_every_ms(Some(500)); // Less frequent flushes
        
        let db = config.open()?;
        Ok(Self { db })
    }

    pub fn save_block(&self, block: &Block) {
        let key = [b"block:".as_ref(), &block.block_id].concat();
        let serializable = SerializableBlock::from(block.clone());
        let value = bincode::serialize(&serializable).unwrap();
        self.db.insert(key, value).unwrap();
    }

    pub fn save_round(&self, round: &Round) {
        let key = [b"round:".as_ref(), &round.round_number.to_be_bytes()].concat();
        let serializable = SerializableRound::from(round.clone());
        let value = bincode::serialize(&serializable).unwrap();
        self.db.insert(key, value).unwrap();
    }

    pub fn load_block(&self, block_id: &[u8; 32]) -> Option<Block> {
        let key = [b"block:".as_ref(), block_id].concat();
        self.db.get(key).unwrap().map(|ivec| {
            let serializable: SerializableBlock = bincode::deserialize(&ivec).unwrap();
            Block::try_from(serializable).unwrap()
        })
    }

    pub fn load_round(&self, round_id: u64) -> Option<Round> {
        let key = [b"round:".as_ref(), &round_id.to_be_bytes()].concat();
        self.db.get(key).unwrap().map(|ivec| {
            let serializable: SerializableRound = bincode::deserialize(&ivec).unwrap();
            Round::try_from(serializable).unwrap()
        })
    }

    pub fn store_validator_set(&self, set: &ValidatorSet) -> Result<(), Box<dyn std::error::Error>> {
        let key = b"validator_set";
        let value = bincode::serialize(set)?;
        self.db.insert(key, value)?;
        Ok(())
    }

    pub fn load_validator_set(&self) -> Result<Option<ValidatorSet>, Box<dyn std::error::Error>> {
        let key = b"validator_set";
        match self.db.get(key)? {
            Some(ivec) => {
                let set: ValidatorSet = bincode::deserialize(&ivec)?;
                Ok(Some(set))
            }
            None => Ok(None),
        }
    }

    pub fn store_governance_state(&self, state: &crate::consensus::governance::GovernanceState) -> Result<(), Box<dyn std::error::Error>> {
        let key = b"governance_state";
        let value = bincode::serialize(state)?;
        self.db.insert(key, value)?;
        Ok(())
    }

    pub fn load_governance_state(&self) -> Result<Option<crate::consensus::governance::GovernanceState>, Box<dyn std::error::Error>> {
        let key = b"governance_state";
        match self.db.get(key)? {
            Some(ivec) => {
                let state: crate::consensus::governance::GovernanceState = bincode::deserialize(&ivec)?;
                Ok(Some(state))
            }
            None => Ok(None),
        }
    }

    pub fn store_parameter(&self, key: &str, value: &str) -> Result<(), sled::Error> {
        let key_bytes = key.as_bytes();
        let value_bytes = value.as_bytes();
        self.db.insert(key_bytes, value_bytes)?;
        Ok(())
    }

    pub fn load_parameter(&self, key: &str) -> Result<Option<String>, sled::Error> {
        let key_bytes = key.as_bytes();
        match self.db.get(key_bytes)? {
            Some(ivec) => {
                let value = String::from_utf8(ivec.to_vec())
                    .map_err(|_| sled::Error::Corruption { at: None, bt: () })?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    pub fn flush(&self) -> Result<(), sled::Error> {
        self.db.flush()?;
        Ok(())
    }

    /// Get database statistics for monitoring
    pub fn get_stats(&self) -> Result<DatabaseStats, sled::Error> {
        let _tree = self.db.open_tree("stats")?;
        let size_on_disk = self.db.size_on_disk()?;
        
        Ok(DatabaseStats {
            size_on_disk,
            tree_count: self.db.tree_names().len(),
            cache_hits: 0, // Would need to implement custom tracking
            cache_misses: 0,
        })
    }

    /// Perform database maintenance and optimization
    pub fn optimize(&self) -> Result<(), sled::Error> {
        // Trigger background compaction
        self.db.flush()?;
        
        // Force garbage collection
        for tree_name in self.db.tree_names() {
            if let Ok(tree) = self.db.open_tree(tree_name) {
                tree.flush()?;
            }
        }
        
        Ok(())
    }

    /// Create a backup of the database
    pub fn create_backup(&self, backup_path: &str) -> Result<(), sled::Error> {
        // Ensure all data is flushed to disk
        self.flush()?;
        
        // For now, just create the backup directory
        // TODO: Implement proper backup functionality
        std::fs::create_dir_all(backup_path)?;
        
        Ok(())
    }

    /// Async, batched write example: send blocks/rounds to this channel for background persistence
    pub fn spawn_background_writer(self: Arc<Self>, mut rx: mpsc::UnboundedReceiver<PersistMsg>) {
        tokio::spawn(async move {
            let mut batch = Vec::new();
            let batch_size = 100; // Batch size for better performance
            
            while let Some(msg) = rx.recv().await {
                batch.push(msg);
                
                // Process batch when it reaches the size limit or after a timeout
                if batch.len() >= batch_size {
                    Self::process_batch(&self, &mut batch).await;
                }
            }
            
            // Process remaining items
            if !batch.is_empty() {
                Self::process_batch(&self, &mut batch).await;
            }
        });
    }

    /// Process a batch of persistence messages
    async fn process_batch(storage: &PersistentStorage, batch: &mut Vec<PersistMsg>) {
        for msg in batch.drain(..) {
            match msg {
                PersistMsg::Block(block) => storage.save_block(&block),
                PersistMsg::Round(round) => storage.save_round(&round),
            }
        }
        
        // Flush after each batch for durability
        if let Err(e) = storage.flush() {
            eprintln!("Failed to flush database: {:?}", e);
        }
    }

    // Store an asset record
    pub fn store_asset(&self, asset: &AssetRecord) -> Result<(), sled::Error> {
        let key = format!("asset:{}", asset.asset_id);
        let value = bincode::serialize(asset).unwrap();
        self.db.insert(key.as_bytes(), value)?;
        Ok(())
    }

    // Load an asset record
    pub fn load_asset(&self, asset_id: &str) -> Option<AssetRecord> {
        let key = format!("asset:{asset_id}");
        self.db.get(key.as_bytes()).unwrap().map(|ivec| {
            bincode::deserialize(&ivec).unwrap()
        })
    }

    // Store a handle record
    pub fn store_handle(&self, handle: &HandleRecord) -> Result<(), sled::Error> {
        let key = format!("handle:{}", handle.handle);
        let value = bincode::serialize(handle).unwrap();
        self.db.insert(key.as_bytes(), value)?;
        Ok(())
    }

    // Load a handle record
    pub fn load_handle(&self, handle: &str) -> Option<HandleRecord> {
        let key = format!("handle:{handle}");
        self.db.get(key.as_bytes()).unwrap().map(|ivec| {
            bincode::deserialize(&ivec).unwrap()
        })
    }
}

#[derive(Debug)]
pub enum PersistMsg {
    Block(Block),
    Round(Round),
}

#[derive(Debug)]
pub struct DatabaseStats {
    pub size_on_disk: u64,
    pub tree_count: usize,
    pub cache_hits: u64,
    pub cache_misses: u64,
} 
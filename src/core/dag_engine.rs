use crate::core::types::{Block, Transaction, ShardId};
use crate::core::address::Address;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DAGVertex {
    pub block: Block,
    pub parents: Vec<[u8; 32]>,
    pub timestamp: u64,
}

impl DAGVertex {
    pub fn new(block: Block, parents: Vec<[u8; 32]>, timestamp: u64) -> Self {
        Self {
            block,
            parents,
            timestamp,
        }
    }
}

/// DAG statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagStats {
    pub total_blocks: usize,
    pub tips_count: usize,
    pub max_depth: usize,
    pub avg_txs_per_block: f64,
}

/// DAG engine for managing the directed acyclic graph of blocks
pub struct DagEngine {
    vertices: Arc<Mutex<HashMap<[u8; 32], DAGVertex>>>,
    tips: Arc<Mutex<HashSet<[u8; 32]>>>,
    genesis_blocks: Arc<Mutex<Vec<[u8; 32]>>>,
    max_depth: u64,
    shard_tips: Arc<Mutex<HashMap<ShardId, Vec<[u8; 32]>>>>,
    stats: Arc<Mutex<DagStats>>,
}

impl DagEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            vertices: Arc::new(Mutex::new(HashMap::new())),
            tips: Arc::new(Mutex::new(HashSet::new())),
            genesis_blocks: Arc::new(Mutex::new(Vec::new())),
            max_depth: 0,
            shard_tips: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(DagStats {
                total_blocks: 0,
                tips_count: 0,
                max_depth: 0,
                avg_txs_per_block: 0.0,
            })),
        };
        engine.create_genesis_blocks();
        engine.update_stats();
        engine
    }

    fn create_genesis_blocks(&mut self) {
        let genesis_blocks = vec![
            self.create_genesis_block(ShardId(0)),
            self.create_genesis_block(ShardId(1)),
            self.create_genesis_block(ShardId(2)),
        ];
        for genesis_block in genesis_blocks {
            let vertex = DAGVertex::new(genesis_block.clone(), Vec::new(), 0);
            self.vertices.lock().unwrap().insert(genesis_block.block_id, vertex);
            self.tips.lock().unwrap().insert(genesis_block.block_id);
            self.genesis_blocks.lock().unwrap().push(genesis_block.block_id);
            let mut shard_tips = self.shard_tips.lock().unwrap();
            shard_tips.entry(ShardId(0)).or_default().push(genesis_block.block_id);
        }
    }

    fn create_genesis_block(&self, shard_id: ShardId) -> Block {
        let mut hasher = Sha256::new();
        hasher.update(b"genesis");
        hasher.update(shard_id.0.to_le_bytes());
        let block_hash = hasher.finalize();
        let mut block_id = [0u8; 32];
        block_id.copy_from_slice(&block_hash);
        Block {
            block_id,
            parent_blocks: Vec::new(),
            transactions: Vec::new(),
            findag_time: 0,
            hashtimer: block_id,
            proposer: Address("genesis".to_string()),
            signature: ed25519_dalek::Signature::from_bytes(&[0u8; 64]).unwrap(),
            public_key: ed25519_dalek::PublicKey::from_bytes(&[0u8; 32]).unwrap(),
            shard_id,
            merkle_root: Some(block_id),
        }
    }

    /// Add a new block to the DAG
    pub fn add_block(&self, block: Block) -> Result<(), String> {
        let block_id = self.compute_block_id(&block);
        let parents = self.select_parents(&block_id);
        let timestamp = self.get_current_timestamp();
        
        let vertex = DAGVertex::new(block, parents, timestamp);
        
        let mut vertices = self.vertices.lock().unwrap();
        vertices.insert(block_id, vertex);
        
        let mut tips = self.tips.lock().unwrap();
        tips.insert(block_id);
        
        Ok(())
    }

    /// Get all tip blocks (blocks with no children)
    pub fn get_tips(&self) -> Vec<[u8; 32]> {
        let tips = self.tips.lock().unwrap();
        tips.iter().copied().collect()
    }

    /// Get all blocks in the DAG
    pub fn get_all_blocks(&self) -> Vec<Block> {
        let vertices = self.vertices.lock().unwrap();
        vertices.values().map(|vertex| vertex.block.clone()).collect()
    }

    /// Get block count
    pub fn block_count(&self) -> usize {
        let vertices = self.vertices.lock().unwrap();
        vertices.len()
    }

    /// Get DAG statistics
    pub fn get_stats(&self) -> DagStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }

    /// Update DAG statistics
    fn update_stats(&mut self) {
        let vertices = self.vertices.lock().unwrap();
        let tips = self.tips.lock().unwrap();
        
        let total_blocks = vertices.len();
        let tips_count = tips.len();
        
        // Calculate average transactions per block
        let total_txs: usize = vertices.values().map(|vertex| vertex.block.transactions.len()).sum();
        let avg_txs_per_block = if total_blocks > 0 {
            total_txs as f64 / total_blocks as f64
        } else {
            0.0
        };

        // Calculate max depth (simplified - could be improved with proper depth calculation)
        let max_depth = total_blocks; // Placeholder

        let mut stats = self.stats.lock().unwrap();
        stats.total_blocks = total_blocks;
        stats.tips_count = tips_count;
        stats.max_depth = max_depth;
        stats.avg_txs_per_block = avg_txs_per_block;
    }

    pub fn get_shard_tips(&self, shard_id: ShardId) -> Vec<[u8; 32]> {
        self.shard_tips.lock().unwrap()
            .get(&shard_id)
            .cloned()
            .unwrap_or_default()
    }
    pub fn get_block(&self, block_id: &[u8; 32]) -> Option<Block> {
        self.vertices.lock().unwrap()
            .get(block_id)
            .map(|vertex| vertex.block.clone())
    }
    pub fn get_parents(&self, block_id: &[u8; 32]) -> Vec<Block> {
        let vertices = self.vertices.lock().unwrap();
        vertices.get(block_id)
            .map(|v| v.parents.iter().filter_map(|p| vertices.get(p).map(|v| v.block.clone())).collect())
            .unwrap_or_default()
    }
    pub fn topological_sort(&self) -> Vec<[u8; 32]> {
        self.vertices.lock().unwrap().keys().cloned().collect()
    }
    pub fn add_round(&mut self, _round: crate::core::types::Round) {
        // TODO: Implement round addition logic
    }
    pub fn block_tips(&self) -> Vec<&crate::core::types::Block> {
        // TODO: Return block tips if needed
        vec![]
    }

    pub fn get_parent_blocks(&self) -> Vec<[u8; 32]> {
        self.get_tips()
    }

    fn compute_block_id(&self, block: &Block) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&block.shard_id.0.to_be_bytes());
        hasher.update(&block.findag_time.to_be_bytes());
        hasher.update(&block.hashtimer);
        for tx in &block.transactions {
            hasher.update(&tx.from.0.as_bytes());
            hasher.update(&tx.to.0.as_bytes());
            hasher.update(&tx.amount.to_be_bytes());
        }
        let result = hasher.finalize();
        let mut block_id = [0u8; 32];
        block_id.copy_from_slice(&result);
        block_id
    }

    fn select_parents(&self, _block_id: &[u8; 32]) -> Vec<[u8; 32]> {
        // Simple parent selection - use all current tips
        let tips = self.tips.lock().unwrap();
        tips.iter().copied().collect()
    }

    fn get_current_timestamp(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl Default for DagEngine {
    fn default() -> Self {
        Self::new()
    }
} 
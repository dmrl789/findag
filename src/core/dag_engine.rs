use crate::core::types::{Block, Transaction, ShardId};
use crate::core::address::Address;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex as TokioMutex;

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
    vertices: Arc<TokioMutex<HashMap<[u8; 32], DAGVertex>>>,
    tips: Arc<TokioMutex<HashSet<[u8; 32]>>>,
    genesis_blocks: Arc<TokioMutex<Vec<[u8; 32]>>>,
    max_depth: u64,
    shard_tips: Arc<TokioMutex<HashMap<ShardId, Vec<[u8; 32]>>>>,
    stats: Arc<TokioMutex<DagStats>>,
}

impl DagEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            vertices: Arc::new(TokioMutex::new(HashMap::new())),
            tips: Arc::new(TokioMutex::new(HashSet::new())),
            genesis_blocks: Arc::new(TokioMutex::new(Vec::new())),
            max_depth: 0,
            shard_tips: Arc::new(TokioMutex::new(HashMap::new())),
            stats: Arc::new(TokioMutex::new(DagStats {
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

    async fn create_genesis_blocks(&mut self) {
        let genesis_blocks = vec![
            self.create_genesis_block(ShardId(0)),
            self.create_genesis_block(ShardId(1)),
            self.create_genesis_block(ShardId(2)),
        ];
        for genesis_block in genesis_blocks {
            let vertex = DAGVertex::new(genesis_block.clone(), Vec::new(), 0);
            self.vertices.lock().await.insert(genesis_block.block_id, vertex);
            self.tips.lock().await.insert(genesis_block.block_id);
            self.genesis_blocks.lock().await.push(genesis_block.block_id);
            let mut shard_tips = self.shard_tips.lock().await;
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
    pub async fn add_block(&self, block: Block) -> Result<(), String> {
        let block_id = self.compute_block_id(&block);
        let parents = self.select_parents(&block_id).await;
        let timestamp = self.get_current_timestamp();
        
        let vertex = DAGVertex::new(block, parents, timestamp);
        
        let mut vertices = self.vertices.lock().await;
        vertices.insert(block_id, vertex);
        
        let mut tips = self.tips.lock().await;
        tips.insert(block_id);
        
        Ok(())
    }

    /// Get all tip blocks (blocks with no children)
    pub async fn get_tips(&self) -> Vec<[u8; 32]> {
        let tips = self.tips.lock().await;
        tips.iter().copied().collect()
    }

    /// Get all blocks in the DAG
    pub async fn get_all_blocks(&self) -> Vec<Block> {
        let vertices = self.vertices.lock().await;
        vertices.values().map(|vertex| vertex.block.clone()).collect()
    }

    /// Get block count
    pub async fn block_count(&self) -> usize {
        let vertices = self.vertices.lock().await;
        vertices.len()
    }

    /// Get DAG statistics
    pub async fn get_stats(&self) -> DagStats {
        let stats = self.stats.lock().await;
        stats.clone()
    }

    /// Update DAG statistics
    async fn update_stats(&mut self) {
        let vertices = self.vertices.lock().await;
        let tips = self.tips.lock().await;
        
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

        let mut stats = self.stats.lock().await;
        stats.total_blocks = total_blocks;
        stats.tips_count = tips_count;
        stats.max_depth = max_depth;
        stats.avg_txs_per_block = avg_txs_per_block;
    }

    pub async fn get_shard_tips(&self, shard_id: ShardId) -> Vec<[u8; 32]> {
        self.shard_tips.lock().await
            .get(&shard_id)
            .cloned()
            .unwrap_or_default()
    }
    pub async fn get_block(&self, block_id: &[u8; 32]) -> Option<Block> {
        self.vertices.lock().await
            .get(block_id)
            .map(|vertex| vertex.block.clone())
    }
    pub async fn get_parents(&self, block_id: &[u8; 32]) -> Vec<Block> {
        let vertices = self.vertices.lock().await;
        vertices.get(block_id)
            .map(|v| v.parents.iter().filter_map(|p| vertices.get(p).map(|v| v.block.clone())).collect())
            .unwrap_or_default()
    }
    pub async fn topological_sort(&self) -> Vec<[u8; 32]> {
        self.vertices.lock().await.keys().cloned().collect()
    }
    pub async fn add_round(&mut self, _round: crate::core::types::Round) {
        // TODO: Implement round addition logic
    }
    pub async fn block_tips(&self) -> Vec<&crate::core::types::Block> {
        // TODO: Return block tips if needed
        vec![]
    }

    pub async fn get_parent_blocks(&self) -> Vec<[u8; 32]> {
        self.get_tips().await
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

    async fn select_parents(&self, _block_id: &[u8; 32]) -> Vec<[u8; 32]> {
        // Simple parent selection - use all current tips
        let tips = self.tips.lock().await;
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
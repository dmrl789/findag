use crate::core::{
    dag_engine::DagEngine,
    tx_pool::ShardedTxPool,
    types::{Block, ShardId},
};
use crate::core::address::Address;
use crate::dagtimer::findag_time_manager::FinDAGTimeManager;
use ed25519_dalek::{Keypair, Signature, Signer};
use sha2::{Digest, Sha256};
use bincode;

/// Configuration for block production
#[derive(Clone)]
pub struct BlockProducerConfig {
    pub max_txs_per_block: usize,
    pub target_block_time_ms: u64,
    pub shard_id: ShardId,
}

/// Block production logic for FinDAG
pub struct BlockProducer<'a> {
    pub dag: &'a mut DagEngine,
    pub tx_pool: &'a ShardedTxPool,
    pub proposer: Address,
    pub keypair: &'a Keypair,
    pub config: BlockProducerConfig,
    pub time_manager: &'a FinDAGTimeManager,
    pub current_round: u64,
    pub transaction_count: usize,
}

impl<'a> BlockProducer<'a> {
    pub fn new(
        dag: &'a mut DagEngine,
        tx_pool: &'a ShardedTxPool,
        proposer: Address,
        keypair: &'a Keypair,
        config: BlockProducerConfig,
        time_manager: &'a FinDAGTimeManager,
    ) -> Self {
        Self {
            dag,
            tx_pool,
            proposer,
            keypair,
            config,
            time_manager,
            current_round: 0,
            transaction_count: 0,
        }
    }

    /// Generate a unique block with real content
    pub async fn produce_block(&mut self) -> Option<Block> {
        let max_txs = self.config.max_txs_per_block;
        
        // DEBUG: Log transaction sources
        println!("[DEBUG] BlockProducer: Starting block production with max_txs={}", max_txs);
        
        // Get transactions from tx_pool (where HTTP API transactions are stored)
        let transactions = self.tx_pool.get_transactions(max_txs, self.config.shard_id.0);
        println!("[DEBUG] BlockProducer: Retrieved {} transactions from tx_pool", transactions.len());
        
        // Early return if no transactions
        if transactions.is_empty() {
            println!("[DEBUG] BlockProducer: No transactions available, skipping block production");
            return None;
        }
        
        // Update transaction count
        self.transaction_count += transactions.len();
        
        // Get current FinDAG Time
        let findag_time = self.time_manager.get_findag_time();
        
        // Generate HashTimer (simplified for now)
        let mut hashtimer = [0u8; 32];
        let time_bytes = findag_time.to_le_bytes();
        hashtimer[..8].copy_from_slice(&time_bytes);
        
        // Get parent blocks (tips)
        let parent_blocks = self.dag.get_tips().await;
        
        // Create block
        let mut block = Block {
            transactions,
            findag_time,
            hashtimer,
            proposer: self.proposer.clone(),
            parent_blocks,
            signature: Signature::from_bytes(&[0u8; 64]).unwrap(), // Placeholder
            block_id: [0u8; 32], // Will be computed
            public_key: self.keypair.public,
            shard_id: self.config.shard_id,
            merkle_root: None,
        };
        
        // Compute block ID
        let block_id = self.compute_block_id(&block);
        block.block_id = block_id;
        
        // Sign the block
        let block_signature = self.sign_block(&block);
        block.signature = block_signature;
        
        println!("[DEBUG] BlockProducer: Successfully produced block with {} transactions", block.transactions.len());
        Some(block)
    }
    
    /// Compute block ID from block content
    fn compute_block_id(&self, block: &Block) -> [u8; 32] {
        let block_bytes = bincode::serialize(block).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&block_bytes);
        let hash = hasher.finalize();
        let mut block_id = [0u8; 32];
        block_id.copy_from_slice(&hash);
        block_id
    }
    
    /// Sign the block
    fn sign_block(&self, block: &Block) -> Signature {
        self.keypair.sign(&block.block_id)
    }
    
    /// Get current round
    pub fn get_current_round(&self) -> u64 {
        self.current_round
    }
    
    /// Get transaction count
    pub fn get_transaction_count(&self) -> usize {
        self.transaction_count
    }
} 
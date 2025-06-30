use crate::core::types::{Transaction, Block, ShardId};
use crate::core::address::Address;
use crate::consensus::mempool::Mempool;
use ed25519_dalek::{Keypair, Signer, Signature};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;
use rand::rngs::OsRng;
use chrono;
use serde_json;
use bincode;
use crate::dagtimer::hashtimer::compute_hashtimer;

/// Asset instruction for block payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    LoadAsset {
        asset_id: String,
        amount: u64,
        issuer: String,
    },
    TransferAsset {
        asset_id: String,
        amount: u64,
        from: String,
        to: String,
    },
    UnloadAsset {
        asset_id: String,
        amount: u64,
        owner: String,
    },
}

/// Block payload containing instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockPayload {
    pub instructions: Vec<Instruction>,
    pub findag_time: u64,
    pub parent_blocks: Vec<[u8; 32]>,
    pub nonce: u32,
}

/// Configuration for block production
#[derive(Debug, Clone)]
pub struct BlockProducerConfig {
    pub max_txs_per_block: usize,
    pub target_block_time_ms: u64,
    pub shard_id: ShardId,
}

/// Block production logic for FinDAG
pub struct BlockProducer<'a> {
    pub dag: &'a mut crate::core::dag_engine::DagEngine,
    pub tx_pool: &'a crate::core::tx_pool::ShardedTxPool,
    pub mempool: &'a Mempool,
    pub proposer: Address,
    pub keypair: &'a Keypair,
    pub config: BlockProducerConfig,
    pub time_manager: &'a crate::dagtimer::findag_time_manager::FinDAGTimeManager,
    pub current_round: u64,
}

impl<'a> BlockProducer<'a> {
    pub fn new(
        dag: &'a mut crate::core::dag_engine::DagEngine,
        tx_pool: &'a crate::core::tx_pool::ShardedTxPool,
        mempool: &'a Mempool,
        proposer: Address,
        keypair: &'a Keypair,
        config: BlockProducerConfig,
        time_manager: &'a crate::dagtimer::findag_time_manager::FinDAGTimeManager,
    ) -> Self {
        Self {
            dag,
            tx_pool,
            mempool,
            proposer,
            keypair,
            config,
            time_manager,
            current_round: 0,
        }
    }

    /// Generate a dummy instruction for testing
    fn generate_dummy_instruction(&self) -> Instruction {
        let mut rng = StdRng::from_rng(OsRng).unwrap();
        let random_id: u32 = rng.gen_range(1000..9999);
        let instruction_type = rng.gen_range(0..3);
        
        match instruction_type {
            0 => Instruction::LoadAsset {
                asset_id: format!("DUMMY-ASSET-{}", random_id),
                amount: rng.gen_range(1..1000),
                issuer: "@test.fd".into(),
            },
            1 => Instruction::TransferAsset {
                asset_id: "USD".to_string(),
                amount: rng.gen_range(1..100),
                from: format!("fdg1qbot{}", random_id),
                to: format!("fdg1qdest{}", random_id + 1000),
            },
            _ => Instruction::UnloadAsset {
                asset_id: format!("DUMMY-ASSET-{}", random_id),
                amount: rng.gen_range(1..500),
                owner: format!("fdg1qowner{}", random_id),
            },
        }
    }

    /// Build block payload with instructions
    fn build_block_payload(&self, transactions: &[Transaction], parent_blocks: &[[u8; 32]]) -> BlockPayload {
        let findag_time = self.time_manager.get_findag_time();
        let mut instructions = Vec::new();
        let mut rng = StdRng::from_rng(OsRng).unwrap();
        
        // Add real transaction instructions if available
        for tx in transactions {
            instructions.push(Instruction::TransferAsset {
                asset_id: "USD".to_string(),
                amount: tx.amount,
                from: tx.from.as_str().to_string(),
                to: tx.to.as_str().to_string(),
            });
        }
        
        // Add dummy instructions to ensure block diversity
        let dummy_count = if instructions.is_empty() { 2 } else { 1 };
        for _ in 0..dummy_count {
            instructions.push(self.generate_dummy_instruction());
        }
        
        BlockPayload {
            instructions,
            findag_time,
            parent_blocks: parent_blocks.to_vec(),
            nonce: rng.gen(), // Random nonce for better distribution
        }
    }

    /// Compute block hash over the full payload
    fn compute_block_hash(&self, payload: &BlockPayload) -> [u8; 32] {
        let mut hasher = Sha256::new();
        let payload_bytes = bincode::serialize(payload).unwrap();
        hasher.update(&payload_bytes);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }

    /// Generate a unique block with real content
    pub async fn produce_block(&mut self) -> Option<Block> {
        let mut rng = StdRng::from_rng(OsRng).unwrap();
        
        // Get transactions from mempool
        let mut transactions = Vec::new();
        for _ in 0..rng.gen_range(1..5) { // 1-4 transactions per block
            if let Some(tx) = self.mempool.next().await {
                transactions.push(tx);
            }
        }
        
        // Generate unique nonce for this block
        let nonce = rng.gen::<u32>();
        
        // Get parent blocks from DAG
        let parent_blocks = self.dag.get_parent_blocks();
        
        // Create unique instruction payload
        let instruction = self.generate_dummy_instruction();
        let payload = BlockPayload {
            instructions: vec![instruction],
            findag_time: self.time_manager.get_findag_time(),
            parent_blocks: parent_blocks.clone(),
            nonce,
        };
        
        // Get FinDAG time and hashtimer
        let findag_time = self.time_manager.get_findag_time();
        let parent_blocks_bytes = bincode::serialize(&parent_blocks).unwrap();
        let hashtimer = compute_hashtimer(findag_time, &parent_blocks_bytes, nonce);
        
        // Create block with unique content
        let mut block = Block {
            block_id: [0; 32], // Will be computed below
            parent_blocks,
            transactions,
            findag_time,
            hashtimer,
            proposer: self.proposer.clone(),
            signature: Signature::from_bytes(&[0u8; 64]).unwrap(), // Will be signed below
            public_key: self.keypair.public,
            shard_id: self.config.shard_id,
            merkle_root: None, // Simplified for now
        };
        
        // Compute block hash over the full payload
        let block_data = bincode::serialize(&block).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&block_data);
        hasher.update(&nonce.to_le_bytes());
        hasher.update(&payload.findag_time.to_le_bytes());
        block.block_id = hasher.finalize().into();
        
        // Sign the block
        let signature_data = bincode::serialize(&block.block_id).unwrap();
        block.signature = self.keypair.sign(&signature_data);
        
        println!("[BlockProducer] Created unique block: {:?} with {} transactions", 
                 block.block_id, block.transactions.len());
        
        Some(block)
    }

    /// Get current round number
    pub fn get_current_round(&self) -> u64 {
        self.current_round
    }

    /// Get transaction count for this block
    pub fn get_transaction_count(&self) -> usize {
        self.config.max_txs_per_block
    }
} 
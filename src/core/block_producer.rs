use crate::core::types::{Block, Transaction};
use crate::core::dag_engine::DagEngine;
use crate::core::tx_pool::ShardedTxPool;
use crate::core::address::Address;
use ed25519_dalek::{Keypair, Signer};

/// Block production logic for FinDAG
pub struct BlockProducer<'a> {
    pub dag: &'a mut DagEngine,
    pub tx_pool: &'a ShardedTxPool,
    pub proposer: Address,
    pub keypair: &'a Keypair,
    pub max_block_txs: usize,
}

impl<'a> BlockProducer<'a> {
    /// Produce a new block from the transaction pool and insert into the DAG
    pub fn produce_block(&mut self, parent_blocks: Vec<[u8; 32]>, findag_time: u64, hashtimer: [u8; 32]) -> Option<Block> {
        // Fetch transactions from the pool
        let txs = self.tx_pool.get_transactions(self.max_block_txs);
        if txs.is_empty() {
            return None; // Skip-when-empty
        }
        // Clone transactions for block
        let block_txs: Vec<Transaction> = txs.iter().cloned().collect();
        // Compute block_id (hash of contents, simplified here)
        let mut hasher = sha2::Sha256::new();
        for tx in &block_txs {
            hasher.update(&tx.hashtimer);
        }
        hasher.update(&findag_time.to_be_bytes());
        hasher.update(&hashtimer);
        let block_id = {
            let hash = hasher.finalize();
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&hash[..32]);
            arr
        };
        // Sign the block (simplified: sign block_id)
        let signature = self.keypair.sign(&block_id);
        let public_key = self.keypair.public;
        let block = Block {
            block_id,
            parent_blocks,
            transactions: block_txs.clone(),
            findag_time,
            hashtimer,
            proposer: self.proposer.clone(),
            signature,
            public_key,
        };
        // Insert block into DAG
        self.dag.add_block(block.clone());
        // Remove included transactions from the pool
        for tx in &block_txs {
            self.tx_pool.remove_transaction(&tx.hashtimer, tx.findag_time);
        }
        Some(block)
    }
} 
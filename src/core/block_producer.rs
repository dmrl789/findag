use crate::core::types::{Block, Transaction};
use crate::core::dag_engine::DagEngine;
use crate::core::tx_pool::ShardedTxPool;
use crate::core::address::Address;
use ed25519_dalek::{Keypair, Signer};
use storage::state::StateDB;

/// Block production logic for FinDAG
pub struct BlockProducer<'a> {
    pub dag: &'a mut DagEngine,
    pub tx_pool: &'a ShardedTxPool,
    pub proposer: Address,
    pub keypair: &'a Keypair,
    pub max_block_txs: usize,
    pub state_db: &'a StateDB,
    pub shard_id: u16,
}

impl<'a> BlockProducer<'a> {
    /// Produce a new block from the transaction pool and insert into the DAG
    pub fn produce_block(&mut self, parent_blocks: Vec<[u8; 32]>, findag_time: u64, hashtimer: [u8; 32]) -> Option<Block> {
        // Fetch transactions from the pool for this shard
        let txs = self.tx_pool.get_transactions(self.max_block_txs, self.shard_id);
        if txs.is_empty() {
            return None; // Skip-when-empty
        }
        // Validate and apply transactions to state
        let mut block_txs: Vec<Transaction> = Vec::new();
        for tx in txs.iter().cloned() {
            let from = tx.from.as_str();
            let to = tx.to.as_str();
            let asset = tx.currency.as_str();
            let amount = tx.amount;
            if self.state_db.transfer(self.shard_id, from, to, asset, amount) {
                block_txs.push(tx);
            } else {
                println!("[BlockProducer] Skipped tx: insufficient funds for {} ({} {})", from, amount, asset);
            }
        }
        if block_txs.is_empty() {
            return None;
        }
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
        // Remove included transactions from the pool for this shard
        for tx in &block_txs {
            self.tx_pool.remove_transaction(&tx.hashtimer, tx.findag_time, self.shard_id);
        }
        Some(block)
    }
} 
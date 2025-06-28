use crate::core::types::{Transaction, Block, ShardId};
use crate::core::address::Address;
use ed25519_dalek::{Keypair, Signer};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::str;
use hex;

/// Block production logic for FinDAG
pub struct BlockProducer<'a> {
    pub dag: &'a mut crate::core::dag_engine::DagEngine,
    pub tx_pool: &'a crate::core::tx_pool::ShardedTxPool,
    pub proposer_address: Address,
    pub proposer_keypair: &'a Keypair,
    pub max_txs_per_block: usize,
    pub target_block_time_ms: u64,
    pub time_manager: &'a crate::dagtimer::findag_time_manager::FinDAGTimeManager,
    pub shard_id: ShardId,
}

impl<'a> BlockProducer<'a> {
    pub fn new(
        dag: &'a mut crate::core::dag_engine::DagEngine,
        tx_pool: &'a crate::core::tx_pool::ShardedTxPool,
        proposer_address: Address,
        proposer_keypair: &'a Keypair,
        max_txs_per_block: usize,
        target_block_time_ms: u64,
        time_manager: &'a crate::dagtimer::findag_time_manager::FinDAGTimeManager,
        shard_id: ShardId,
    ) -> Self {
        Self {
            dag,
            tx_pool,
            proposer_address,
            proposer_keypair,
            max_txs_per_block,
            target_block_time_ms,
            time_manager,
            shard_id,
        }
    }

    /// Produce a new block from the transaction pool and insert into the DAG
    pub fn produce_block(&mut self) -> Option<Block> {
        // Get transactions from the pool for this shard
        let txs = self.tx_pool.get_transactions_for_shard(self.shard_id.0 as usize, self.max_txs_per_block);
        if txs.is_empty() {
            return None;
        }

        // Group transactions by asset for asset-specific processing
        let mut asset_groups: HashMap<String, Vec<&Transaction>> = HashMap::new();
        for tx in &txs {
            let asset = "USD"; // Default asset for now
            asset_groups.entry(asset.to_string()).or_default().push(tx);
        }

        // Create block content
        let mut block_content = Vec::new();
        for tx in &txs {
            block_content.extend_from_slice(&tx.hashtimer);
        }

        // Compute block hash
        let mut hasher = Sha256::new();
        hasher.update(&block_content);
        let block_hash = hasher.finalize();
        let mut block_id = [0u8; 32];
        block_id.copy_from_slice(&block_hash);

        // Get FinDAG Time and compute HashTimer
        let findag_time = self.time_manager.get_findag_time();
        let hashtimer = crate::dagtimer::hashtimer::compute_hashtimer(findag_time, &block_content, 0);

        // Get parent blocks from DAG
        let parent_blocks = self.dag.get_tips().iter().map(|tip| tip.clone()).collect();

        // Create block
        let block = Block {
            block_id,
            parent_blocks,
            transactions: txs.into_iter().cloned().collect(),
            findag_time,
            hashtimer,
            proposer: self.proposer_address.clone(),
            signature: self.proposer_keypair.sign(&block_id),
            public_key: self.proposer_keypair.public,
            shard_id: self.shard_id,
            merkle_root: Some(block_id), // Simplified for now
        };

        // Insert block into DAG
        self.dag.add_block(block.clone());

        Some(block)
    }
} 
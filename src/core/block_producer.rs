use crate::core::types::{Transaction, Block, ShardId};
use crate::core::address::Address;
use ed25519_dalek::{Keypair, Signer};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

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
    pub proposer_address: Address,
    pub proposer_keypair: &'a Keypair,
    pub config: BlockProducerConfig,
    pub time_manager: &'a crate::dagtimer::findag_time_manager::FinDAGTimeManager,
}

impl<'a> BlockProducer<'a> {
    pub fn new(
        dag: &'a mut crate::core::dag_engine::DagEngine,
        tx_pool: &'a crate::core::tx_pool::ShardedTxPool,
        proposer_address: Address,
        proposer_keypair: &'a Keypair,
        config: BlockProducerConfig,
        time_manager: &'a crate::dagtimer::findag_time_manager::FinDAGTimeManager,
    ) -> Self {
        Self {
            dag,
            tx_pool,
            proposer_address,
            proposer_keypair,
            config,
            time_manager,
        }
    }

    /// Produce a new block from the transaction pool and insert into the DAG
    pub fn produce_block(&mut self) -> Option<Block> {
        // Get transactions from the pool for this shard
        let txs = self.tx_pool.get_transactions(self.config.max_txs_per_block, self.config.shard_id.0);
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
        let parent_blocks = self.dag.get_tips().iter().copied().collect();

        // Create block
        let block = Block {
            block_id,
            parent_blocks,
            transactions: txs.into_iter().collect(),
            findag_time,
            hashtimer,
            proposer: self.proposer_address.clone(),
            signature: self.proposer_keypair.sign(&block_id),
            public_key: self.proposer_keypair.public,
            shard_id: self.config.shard_id,
            merkle_root: Some(block_id), // Simplified for now
        };

        // Insert block into DAG
        let _ = self.dag.add_block(block.clone());

        Some(block)
    }
} 
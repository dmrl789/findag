use crate::core::block_producer::BlockProducer;
use crate::core::dag_engine::DagEngine;
use crate::core::tx_pool::ShardedTxPool;
use crate::core::address::Address;
use crate::dagtimer::findag_time_manager::FinDAGTimeManager;
use crate::dagtimer::hashtimer::compute_hashtimer;
use ed25519_dalek::Keypair;
use tokio::time::{sleep, Duration};
use crate::storage::persistent::PersistMsg;
use tokio::sync::mpsc::UnboundedSender;
use crate::metrics;
use std::time::Instant;
use crate::core::types::ShardId;
use crate::consensus::round_finalizer::RoundFinalizer;

/// Runs the block production loop for a specific shard
pub async fn run_block_production_loop(
    dag: &mut DagEngine,
    tx_pool: &ShardedTxPool,
    proposer: Address,
    keypair: &Keypair,
    max_block_txs: usize,
    interval_ms: u64,
    time_manager: &FinDAGTimeManager,
    persist_tx: UnboundedSender<PersistMsg>,
    shard_id: u16,
    _round_finalizer: RoundFinalizer<'_>,
) {
    loop {
        let block_start = Instant::now();
        
        // Get parent blocks before any mutable borrow
        let parent_blocks: Vec<[u8; 32]> = dag.get_tips();
        
        // Create block producer after getting tips
        let mut block_producer = BlockProducer::new(
            dag,
            tx_pool,
            proposer.clone(),
            keypair,
            max_block_txs,
            interval_ms,
            time_manager,
            ShardId(shard_id),
        );
        
        // Get real FinDAG Time
        let findag_time = time_manager.get_findag_time();
        
        // Compute HashTimer for the block (using FinDAG Time, parent block ids, and a nonce)
        let mut block_content = Vec::new();
        for parent in &parent_blocks {
            block_content.extend_from_slice(parent);
        }
        let nonce = 0u32; // You may want to increment or randomize this
        let _hashtimer = compute_hashtimer(findag_time, &block_content, nonce);
        
        // Only fetch transactions for this shard
        let produced = block_producer.produce_block();
        if let Some(block) = produced {
            println!("[Shard {}] Produced block: {:?} at FinDAG Time {}", shard_id, block.block_id, findag_time);
            // TODO: Use round_finalizer for consensus/finality in this shard
            // Persist the block asynchronously
            let _ = persist_tx.send(PersistMsg::Block(block.clone()));
            // --- Metrics instrumentation ---
            metrics::BLOCK_TOTAL.inc();
            metrics::BLOCKS_PER_SEC.inc();
            metrics::TPS.set(block.transactions.len() as i64);
            metrics::TX_TOTAL.inc_by(block.transactions.len() as u64);
            metrics::BLOCK_LATENCY.observe(block_start.elapsed().as_secs_f64());
            // metrics::PER_ASSET_TPS.with_label_values(&[&tx.currency]).inc();
        }
        sleep(Duration::from_millis(interval_ms)).await;
    }
}

// Example usage (in your main):
//
// #[tokio::main]
// async fn main() {
//     let mut dag = DagEngine::new();
//     let tx_pool = ShardedTxPool::new(100_000);
//     let (keypair, address) = generate_address();
//     let time_manager = FinDAGTimeManager::new();
//     run_block_production_loop(&mut dag, &tx_pool, address, &keypair, 100, 20, &time_manager).await;
// } 
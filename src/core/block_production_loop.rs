use crate::core::block_producer::BlockProducer;
use crate::core::dag_engine::DagEngine;
use crate::core::tx_pool::ShardedTxPool;
use crate::core::address::Address;
use crate::dagtimer::findag_time_manager::FinDAGTimeManager;
use crate::dagtimer::hashtimer::compute_hashtimer;
use ed25519_dalek::Keypair;
use tokio::time::{sleep, Duration};

/// Runs the block production loop at the given interval (ms)
pub async fn run_block_production_loop(
    dag: &mut DagEngine,
    tx_pool: &ShardedTxPool,
    proposer: Address,
    keypair: &Keypair,
    max_block_txs: usize,
    interval_ms: u64,
    time_manager: &FinDAGTimeManager,
) {
    let mut block_producer = BlockProducer {
        dag,
        tx_pool,
        proposer,
        keypair,
        max_block_txs,
    };
    loop {
        // Set parent_blocks from current DAG tips
        let parent_blocks = block_producer.dag.block_tips().iter().map(|b| b.block_id).collect();
        // Get real FinDAG Time
        let findag_time = time_manager.get_findag_time();
        // Compute HashTimer for the block (using FinDAG Time, parent block ids, and a nonce)
        let mut block_content = Vec::new();
        for parent in &parent_blocks {
            block_content.extend_from_slice(parent);
        }
        let nonce = 0u32; // You may want to increment or randomize this
        let hashtimer = compute_hashtimer(findag_time, &block_content, nonce);
        let produced = block_producer.produce_block(parent_blocks, findag_time, hashtimer);
        if let Some(block) = produced {
            println!("Produced block: {:?} at FinDAG Time {}", block.block_id, findag_time);
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
use crate::core::{
    block_producer::{BlockProducer, BlockProducerConfig},
    dag_engine::DagEngine,
    tx_pool::ShardedTxPool,
    types::{Block, ShardId},
};
use crate::core::address::Address;
use crate::consensus::round_finalizer::RoundFinalizer;
use crate::dagtimer::findag_time_manager::FinDAGTimeManager;
use crate::storage::persistent::PersistMsg;
use crate::metrics;
use ed25519_dalek::Keypair;
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::{sleep, Duration, Instant};
use hex;

/// Configuration for block production loop
#[derive(Debug, Clone)]
pub struct BlockProductionConfig {
    pub max_block_txs: usize,
    pub interval_ms: u64,
    pub shard_id: u16,
}

/// Runs the block production loop for a specific shard
pub async fn run_block_production_loop(
    dag: &mut DagEngine,
    tx_pool: &ShardedTxPool,
    proposer: Address,
    keypair: &Keypair,
    config: BlockProductionConfig,
    time_manager: &FinDAGTimeManager,
    persist_tx: UnboundedSender<PersistMsg>,
    _round_finalizer: RoundFinalizer<'_>,
) {
    loop {
        let block_start = Instant::now();
        
        // DEBUG: Log mempool sizes
        let tx_pool_size = tx_pool.size(config.shard_id);
        println!("[DEBUG] TxPool size: {}", tx_pool_size);
        
        // Get parent blocks before any mutable borrow
        let parent_blocks: Vec<[u8; 32]> = dag.get_tips().await;
        let tips_time = block_start.elapsed();
        
        // DEBUG: Log tips contents
        println!("[DEBUG] Tips count: {}, tips: {:?}", parent_blocks.len(), 
                 parent_blocks.iter().map(|h| format!("0x{}", hex::encode(h))).collect::<Vec<_>>());
        
        // Create block producer without holding mempool lock
        let mut block_producer = BlockProducer::new(
            dag,
            tx_pool,
            proposer.clone(),
            keypair,
            BlockProducerConfig {
                max_txs_per_block: config.max_block_txs,
                target_block_time_ms: config.interval_ms,
                shard_id: ShardId(config.shard_id),
            },
            time_manager,
        );
        let producer_time = block_start.elapsed();
        
        // Get real FinDAG Time
        let findag_time = time_manager.get_findag_time();
        
        // Calculate round number from FinDAG Time (assuming 16 second rounds)
        let round_duration_ns = 16_000_000_000u64; // 16 seconds in nanoseconds
        let current_round = findag_time / round_duration_ns;
        
        // Only fetch transactions for this shard
        let produced = block_producer.produce_block().await;
        let production_time = block_start.elapsed();
        
        if let Some(block) = produced {
            println!("[Round {}][Shard {}] Produced block: 0x{} at FinDAG Time {} with HashTimer: 0x{} ({} txs)", 
                current_round, config.shard_id, 
                hex::encode(block.block_id),
                findag_time, 
                hex::encode(block.hashtimer),
                block.transactions.len());
            // Log block producer stats
            println!("[BlockProducer] Round {}: {} transactions", 
                     block_producer.get_current_round(), block_producer.get_transaction_count());
            // TODO: Use round_finalizer for consensus/finality in this shard
            // Persist the block asynchronously
            let _ = persist_tx.send(PersistMsg::Block(block.clone()));
            // --- Metrics instrumentation ---
            metrics::BLOCK_TOTAL.inc();
            metrics::BLOCKS_PER_SEC.inc();
            metrics::TPS.set(block.transactions.len() as i64);
            metrics::TX_TOTAL.inc_by(block.transactions.len() as u64);
            metrics::BLOCK_LATENCY.observe(block_start.elapsed().as_secs_f64());
        } else {
            println!("[Round {}][Shard {}] No block produced - no transactions available | Timing: tips={:?}, producer={:?}, production={:?}", 
                current_round, config.shard_id, tips_time, producer_time, production_time);
            
            // DEBUG: Additional diagnostics for why no block was produced
            if tx_pool_size > 0 {
                println!("[DEBUG] WARNING: TxPool has {} transactions but no block was produced!", tx_pool_size);
                println!("[DEBUG] This suggests a bug in the block producer logic");
            }
        }
        
        // Calculate proper sleep time to maintain the intended interval
        let elapsed = block_start.elapsed();
        let sleep_duration = if elapsed.as_millis() < config.interval_ms as u128 {
            Duration::from_millis(config.interval_ms - elapsed.as_millis() as u64)
        } else {
            Duration::from_millis(1) // Minimum sleep if we're already over the interval
        };
        
        sleep(sleep_duration).await;
        let total_time = block_start.elapsed();
        println!("[Shard {}] Loop timing: total={:?}, sleep={:?}, interval={}ms", 
            config.shard_id, total_time, sleep_duration, config.interval_ms);
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
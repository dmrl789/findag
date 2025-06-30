use crate::core::block_producer::{BlockProducer, BlockProducerConfig};
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
use sha2::{Sha256, Digest};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use crate::consensus::mempool::Mempool;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Configuration for block production loop
#[derive(Debug, Clone)]
pub struct BlockProductionConfig {
    pub max_block_txs: usize,
    pub interval_ms: u64,
    pub shard_id: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    LoadAsset {
        asset_id: String,
        amount: u64,
        issuer: String,
    },
}

fn generate_dummy_instruction() -> Instruction {
    let mut rng = StdRng::from_rng(OsRng).unwrap();
    let random_id: u32 = rng.gen_range(1000, 9999);
    Instruction::LoadAsset {
        asset_id: format!("DUMMY-ASSET-{}", random_id),
        amount: 1,
        issuer: "@test.fd".into(),
    }
}

fn build_block_payload() -> Vec<u8> {
    let dummy = generate_dummy_instruction();
    bincode::serialize(&vec![dummy]).unwrap()
}

fn compute_block_hash(payload: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

/// Runs the block production loop for a specific shard
pub async fn run_block_production_loop(
    dag: &mut DagEngine,
    tx_pool: &ShardedTxPool,
    mempool: Arc<Mutex<Mempool>>,
    proposer: Address,
    keypair: &Keypair,
    config: BlockProductionConfig,
    time_manager: &FinDAGTimeManager,
    persist_tx: UnboundedSender<PersistMsg>,
    _round_finalizer: RoundFinalizer<'_>,
) {
    loop {
        let block_start = Instant::now();
        
        // Get parent blocks before any mutable borrow
        let parent_blocks: Vec<[u8; 32]> = dag.get_tips();
        let tips_time = block_start.elapsed();
        
        // Create block producer without holding mempool lock
        let mut block_producer = BlockProducer::new(
            dag,
            tx_pool,
            mempool.clone(), // Pass the Arc<Mutex<Mempool>> directly
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
        
        // Build block payload with dummy instruction
        let payload = build_block_payload();
        let block_hash = compute_block_hash(&payload);
        
        // Compute HashTimer for the block (using FinDAG Time, parent block ids, and a nonce)
        let mut block_content = Vec::new();
        for parent in &parent_blocks {
            block_content.extend_from_slice(parent);
        }
        let nonce = 0u32; // You may want to increment or randomize this
        let hashtimer = compute_hashtimer(findag_time, &block_content, nonce);
        
        // Only fetch transactions for this shard
        let produced = block_producer.produce_block().await;
        let production_time = block_start.elapsed();
        
        if let Some(block) = produced {
            println!("[Round {}][Shard {}] Produced block: 0x{} at FinDAG Time {} with HashTimer: 0x{} (Payload: {} bytes)", 
                current_round, config.shard_id, 
                block_hash.iter().map(|b| format!("{:02x}", b)).collect::<String>(),
                findag_time, 
                hashtimer.iter().map(|b| format!("{:02x}", b)).collect::<String>(),
                payload.len());
            
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
            // metrics::PER_ASSET_TPS.with_label_values(&[&tx.currency]).inc();
        } else {
            println!("[Round {}][Shard {}] No block produced - no transactions available | Timing: tips={:?}, producer={:?}, production={:?}", 
                current_round, config.shard_id, tips_time, producer_time, production_time);
        }
        
        let sleep_start = Instant::now();
        sleep(Duration::from_millis(config.interval_ms)).await;
        let total_time = block_start.elapsed();
        println!("[Shard {}] Loop timing: total={:?}, sleep={:?}, interval={}ms", 
            config.shard_id, total_time, sleep_start.elapsed(), config.interval_ms);
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
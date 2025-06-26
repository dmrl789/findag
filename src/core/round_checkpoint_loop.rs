use crate::core::dag_engine::DagEngine;
use crate::core::types::{Round, Block};
use crate::core::address::Address;
use crate::dagtimer::findag_time_manager::FinDAGTimeManager;
use crate::dagtimer::hashtimer::compute_hashtimer;
use ed25519_dalek::{Keypair, Signer};
use tokio::time::{sleep, Duration};
use std::collections::HashSet;

/// Runs the round checkpointing loop at the given interval (ms)
pub async fn run_round_checkpoint_loop(
    dag: &mut DagEngine,
    proposer: Address,
    keypair: &Keypair,
    interval_ms: u64,
    time_manager: &FinDAGTimeManager,
) {
    let mut last_round_id = 0u64;
    let mut last_block_set: HashSet<[u8; 32]> = HashSet::new();
    loop {
        // Collect all new blocks since last round
        let all_blocks: Vec<_> = dag.blocks.values().collect();
        let mut new_block_ids = Vec::new();
        for block in &all_blocks {
            if !last_block_set.contains(&block.block_id) {
                new_block_ids.push(block.block_id);
            }
        }
        if !new_block_ids.is_empty() {
            // Prepare round data
            let round_id = last_round_id + 1;
            let parent_rounds = if last_round_id > 0 { vec![last_round_id] } else { vec![] };
            let findag_time = time_manager.get_findag_time();
            // HashTimer for the round (using FinDAG Time, block ids, and a nonce)
            let mut round_content = Vec::new();
            for block_id in &new_block_ids {
                round_content.extend_from_slice(block_id);
            }
            let nonce = 0u32; // You may want to increment or randomize this
            let hashtimer = compute_hashtimer(findag_time, &round_content, nonce);
            // Sign the round (simplified: sign round_id)
            let mut round_id_bytes = [0u8; 8];
            round_id_bytes.copy_from_slice(&round_id.to_be_bytes());
            let signature = keypair.sign(&round_id_bytes);
            let public_key = keypair.public;
            let round = Round {
                round_id,
                parent_rounds,
                block_ids: new_block_ids.clone(),
                findag_time,
                hashtimer,
                proposer: proposer.clone(),
                signature,
                public_key,
            };
            dag.add_round(round);
            last_round_id = round_id;
            last_block_set.extend(new_block_ids);
            println!("Created round checkpoint: {} with {} blocks", round_id, last_block_set.len());
        }
        sleep(Duration::from_millis(interval_ms)).await;
    }
}

// Example usage (in your main):
//
// #[tokio::main]
// async fn main() {
//     let mut dag = DagEngine::new();
//     let (keypair, address) = generate_address();
//     let time_manager = FinDAGTimeManager::new();
//     run_round_checkpoint_loop(&mut dag, address, &keypair, 200, &time_manager).await;
// } 
use crate::core::dag_engine::DagEngine;
use crate::core::types::Round;
use crate::core::address::Address;
use crate::dagtimer::findag_time_manager::FinDAGTimeManager;
use ed25519_dalek::SigningKey;
use tokio::time::{sleep, Duration};
use std::collections::HashSet;
use crate::storage::persistent::PersistMsg;
use tokio::sync::mpsc::UnboundedSender;
use crate::consensus::roundchain::RoundChain;

/// Runs the round checkpointing loop at the given interval (ms)
/// Uses simple linear RoundChain for deterministic finality
pub async fn run_round_checkpoint_loop(
    dag: &mut DagEngine,
    proposer: Address,
    keypair: &SigningKey,
    interval_ms: u64,
    time_manager: &FinDAGTimeManager,
    persist_tx: UnboundedSender<PersistMsg>,
    roundchain: &mut RoundChain,
) {
    let mut last_round_number = 0u64;
    let mut last_block_set: HashSet<[u8; 32]> = HashSet::new();
    
    loop {
        // Collect all new blocks since last round
        let all_blocks: Vec<_> = dag.get_all_blocks().await;
        let mut new_blocks = Vec::new();
        for block in &all_blocks {
            if !last_block_set.contains(&block.block_id) {
                new_blocks.push(block.clone());
            }
        }
        
        if !new_blocks.is_empty() {
            // Create new round with finalized blocks
            let round_number = last_round_number + 1;
            let findag_time = time_manager.get_findag_time();
            
            // Create the round using RoundChain
            let round = roundchain.create_round(
                round_number,
                new_blocks.clone(),
                findag_time,
                keypair,
                proposer.clone(),
            ).expect("Failed to create round");
            
            // Add round to RoundChain
            roundchain.add_round(round.clone()).expect("Failed to add round to chain");
            
            // Convert RoundChain Round to core Round for compatibility
            let core_round = Round {
                round_number: round.round_number,
                parent_round_hash: round.parent_round_hash,
                finalized_block_hashes: round.finalized_block_hashes,
                block_hashtimers: round.block_hashtimers,
                quorum_signature: round.quorum_signature,
                findag_time: round.findag_time,
                proposer: round.proposer,
                proposer_signature: round.proposer_signature,
                proposer_public_key: round.proposer_public_key,
            };
            dag.add_round(core_round.clone()).await;
            
            last_round_number = round_number;
            for block in &new_blocks {
                last_block_set.insert(block.block_id);
            }
            
            println!("Created round checkpoint: {} with {} blocks", round_number, new_blocks.len());
            
            // Persist the round asynchronously
            let _ = persist_tx.send(PersistMsg::Round(core_round));
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
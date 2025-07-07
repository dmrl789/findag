use crate::consensus::round_finalizer::{RoundFinalizer, RoundCommitment};
use crate::consensus::validator_set::ValidatorSet;
use crate::core::types::Block;
use std::collections::HashMap;

/// Aggregates blocks for a round and manages finalization
pub struct RoundAggregator<'a> {
    pub current_round: u64,
    pub block_pool: HashMap<[u8; 32], Block>, // block_hash => Block
    pub commitments: Vec<RoundCommitment>,
    pub finalizer: RoundFinalizer<'a>,
}

impl<'a> RoundAggregator<'a> {
    pub fn new(validator_set: &'a mut ValidatorSet) -> Self {
        let finalizer = RoundFinalizer::dummy(validator_set);
        Self {
            current_round: 0,
            block_pool: HashMap::new(),
            commitments: Vec::new(),
            finalizer,
        }
    }

    /// Add a block to the current round
    pub fn add_block(&mut self, round_number: u64, block: Block) {
        let hash = block.hashtimer; // Using hashtimer as block hash
        self.block_pool.insert(hash, block);

        println!("[RoundAggregator] Added block to round {} (pool size: {})", 
                 round_number, self.block_pool.len());

        // Finalize round if threshold reached
        if self.commitments.len() >= self.finalizer.get_quorum_threshold() {
            println!("[RoundAggregator] Round {round_number} finalization threshold met!");
            // Finalize and reset pool
            self.block_pool.clear();
            self.commitments.clear();
            self.current_round += 1;
        }
    }

    /// Collect a commitment from a validator
    pub fn add_commitment(&mut self, commitment: RoundCommitment) {
        if self.finalizer.verify_commitment(&commitment) {
            self.commitments.push(commitment);
            println!("[RoundAggregator] Added commitment (total: {})", self.commitments.len());
        } else {
            println!("[RoundAggregator] Invalid commitment rejected");
        }
    }

    /// Get current round number
    pub fn get_current_round(&self) -> u64 {
        self.current_round
    }

    /// Get number of blocks in current round
    pub fn get_block_count(&self) -> usize {
        self.block_pool.len()
    }

    /// Get number of commitments in current round
    pub fn get_commitment_count(&self) -> usize {
        self.commitments.len()
    }

    /// Check if round is ready for finalization
    pub fn is_ready_for_finalization(&self) -> bool {
        self.commitments.len() >= self.finalizer.get_quorum_threshold()
    }
} 
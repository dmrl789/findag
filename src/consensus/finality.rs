use crate::types::finality::{FinalityVote, Justification};
use std::collections::{HashMap, HashSet};
use hex;

#[derive(Default)]
pub struct FinalityManager {
    pub votes: HashMap<String, Vec<FinalityVote>>,
    pub finalized_blocks: HashSet<String>,
    pub justifications: HashMap<String, Justification>,
    pub finality_threshold: usize,
}

impl FinalityManager {
    pub fn new(threshold: usize) -> Self {
        Self {
            votes: HashMap::new(),
            finalized_blocks: HashSet::new(),
            justifications: HashMap::new(),
            finality_threshold: threshold,
        }
    }

    pub fn submit_vote(&mut self, vote: FinalityVote) {
        self.votes
            .entry(hex::encode(&vote.block_hash))
            .or_insert_with(Vec::new)
            .push(vote.clone());
    }

    pub fn is_finalized(&self, block_hash: &str) -> bool {
        self.finalized_blocks.contains(block_hash)
    }

    pub fn try_finalize(&mut self, block_hash: &str) -> Option<Justification> {
        let votes = self.votes.get(block_hash)?;
        if votes.len() >= self.finality_threshold {
            let justification = Justification {
                block_hash: block_hash.to_string(),
                signers: votes.iter().map(|v| hex::encode(&v.validator)).collect(),
            };
            self.finalized_blocks.insert(block_hash.to_string());
            self.justifications.insert(block_hash.to_string(), justification.clone());
            Some(justification)
        } else {
            None
        }
    }

    pub fn get_justification(&self, block_hash: &str) -> Option<&Justification> {
        self.justifications.get(block_hash)
    }
}

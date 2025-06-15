use crate::types::vote::VoteType;
use crate::types::finality::{FinalityVote, Justification};
use std::collections::{HashMap, HashSet};
use hex;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Default)]
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
            let (for_votes, against_votes) = votes.iter().fold((Vec::new(), Vec::new()), |(mut for_vec, mut against_vec), vote| {
                match vote.vote_type {
                    VoteType::For => for_vec.push(vote),
                    VoteType::Against => against_vec.push(vote),
                    _ => {}
                }
                (for_vec, against_vec)
            });

            if for_votes.len() >= self.finality_threshold {
                let justification = Justification::new(
                    block_hash,
                    for_votes.iter().map(|v| v.validator.clone()).collect(),
                    VoteType::For,
                    for_votes.iter().map(|v| v.signature.clone()).collect(),
                );
                self.finalized_blocks.insert(block_hash.to_string());
                self.justifications.insert(block_hash.to_string(), justification.clone());
                Some(justification)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_justification(&self, block_hash: &str) -> Option<&Justification> {
        self.justifications.get(block_hash)
    }

    pub fn get_vote_counts(&self, block_hash: &str) -> Option<(usize, usize)> {
        self.votes.get(block_hash).map(|votes| {
            votes.iter().fold((0, 0), |(for_count, against_count), vote| {
                match vote.vote_type {
                    VoteType::For => (for_count + 1, against_count),
                    VoteType::Against => (for_count, against_count + 1),
                    _ => (for_count, against_count),
                }
            })
        })
    }
}

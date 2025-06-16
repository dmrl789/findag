use crate::blockchain::block::Block;
use crate::blockchain::error::BlockchainError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct FinalityManager {
    votes: Arc<Mutex<HashMap<String, Vec<bool>>>>,
}

impl FinalityManager {
    pub fn new() -> Self {
        Self {
            votes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn vote(&self, block_hash: String, vote: bool) -> Result<(), BlockchainError> {
        let mut votes = self.votes.lock().await;
        let block_votes = votes.entry(block_hash).or_insert_with(Vec::new);
        block_votes.push(vote);
        Ok(())
    }

    pub async fn is_finalized(&self, block_hash: &str, threshold: f32) -> bool {
        let votes = self.votes.lock().await;
        if let Some(block_votes) = votes.get(block_hash) {
            let for_votes = block_votes.iter().filter(|&&v| v).count();
            let total_votes = block_votes.len();
            if total_votes > 0 {
                (for_votes as f32 / total_votes as f32) >= threshold
            } else {
                false
            }
        } else {
            false
        }
    }
} 
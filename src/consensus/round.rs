use crate::blockchain::block::Block;
use crate::utils::time::get_findag_time_micro;
use serde::{Serialize, Deserialize};
use blake3::Hasher;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Round {
    pub round_id: u64,
    pub timestamp: u64,
    pub parent_rounds: Vec<String>,  // Round hash links
    pub blocks_included: Vec<Block>,
    pub validator: String,
    pub round_hash: String,
}

impl Round {
    pub fn new(round_id: u64, parent_rounds: Vec<String>, blocks: Vec<Block>, validator: String) -> Self {
        let timestamp = get_findag_time_micro();
        let hash = Self::compute_hash(round_id, &parent_rounds, &blocks, &validator, timestamp);
        Round {
            round_id,
            timestamp,
            parent_rounds,
            blocks_included: blocks,
            validator,
            round_hash: hash,
        }
    }

    fn compute_hash(
        round_id: u64,
        parent_rounds: &Vec<String>,
        blocks: &Vec<Block>,
        validator: &str,
        timestamp: u64,
    ) -> String {
        let mut hasher = Hasher::new();
        hasher.update(&round_id.to_le_bytes());
        for parent in parent_rounds {
            hasher.update(parent.as_bytes());
        }
        for block in blocks {
            let block_bytes = serde_json::to_vec(block).unwrap();
            hasher.update(&block_bytes);
        }
        hasher.update(validator.as_bytes());
        hasher.update(&timestamp.to_le_bytes());
        hasher.finalize().to_hex().to_string()
    }
}

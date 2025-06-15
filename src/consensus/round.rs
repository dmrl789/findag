use crate::types::block::Block;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Round {
    pub round_id: String,
    pub validator: String,
    pub blocks_included: Vec<Block>,
}

impl Round {
    pub fn new(round_id: String, validator: String, blocks: Vec<Block>) -> Self {
        Round {
            round_id,
            validator,
            blocks_included: blocks,
        }
    }

    pub fn validate_blocks(&self) -> bool {
        // Placeholder validation logic: in production, verify signatures, timestamps, etc.
        !self.blocks_included.is_empty()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    pub fn from_bytes(data: &[u8]) -> Self {
        serde_json::from_slice(data).unwrap()
    }
}

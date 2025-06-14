use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub hash: String,
    pub parents: Vec<String>,
    pub timestamp: u64,
    pub data: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundHash {
    pub round_id: u64,
    pub block_hashes: Vec<String>,
    pub timestamp: u64,
    pub validator_signature: String,
}


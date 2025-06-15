use crate::types::transaction::Transaction;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub hash: Vec<u8>,
    pub parents: Vec<Vec<u8>>,
    pub timestamp: u64,
    pub data: Vec<Transaction>,
    pub signature: Vec<u8>,
    pub justification: Option<Vec<u8>>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            hash: Vec::new(),
            parents: Vec::new(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            data: Vec::new(),
            signature: Vec::new(),
            justification: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundHash {
    pub round_id: u64,
    pub block_hashes: Vec<String>,
    pub timestamp: u64,
    pub validator_signature: String,
}


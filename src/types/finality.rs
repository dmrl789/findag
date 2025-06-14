use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalityVote {
    pub block_hash: Vec<u8>,
    pub validator: Vec<u8>,
    pub round_id: u64,
    pub vote_type: VoteType,
    pub signature: Vec<u8>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteType {
    PrePrepare,
    Prepare,
    Commit,
}

impl FinalityVote {
    pub fn new(
        block_hash: Vec<u8>,
        validator: Vec<u8>,
        round_id: u64,
        vote_type: VoteType,
    ) -> Self {
        Self {
            block_hash,
            validator,
            round_id,
            vote_type,
            signature: Vec::new(),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn sign(&mut self, private_key: &[u8]) -> Result<(), Box<dyn Error>> {
        // TODO: Implement signing
        Ok(())
    }

    pub fn verify(&self) -> Result<bool, Box<dyn Error>> {
        // TODO: Implement verification
        Ok(true)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Justification {
    pub block_hash: String,
    pub signers: Vec<String>,
}

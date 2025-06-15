use serde::{Serialize, Deserialize};
use std::error::Error;
use chrono;
use crate::types::vote::VoteType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalityVote {
    pub block_hash: String,
    pub validator: String,
    pub vote_type: VoteType,
    pub signature: Vec<u8>,
    pub timestamp: i64,
    pub justification: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Justification {
    pub block_hash: String,
    pub validators: Vec<String>,
    pub vote_type: VoteType,
    pub timestamp: i64,
    pub signatures: Vec<Vec<u8>>,
}

impl FinalityVote {
    pub fn new(block_hash: &str, validator: &str, vote_type: VoteType, signature: Vec<u8>) -> Self {
        Self {
            block_hash: block_hash.to_string(),
            validator: validator.to_string(),
            vote_type,
            signature,
            timestamp: chrono::Utc::now().timestamp(),
            justification: None,
        }
    }

    pub fn with_justification(mut self, justification: String) -> Self {
        self.justification = Some(justification);
        self
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

impl Justification {
    pub fn new(block_hash: &str, validators: Vec<String>, vote_type: VoteType, signatures: Vec<Vec<u8>>) -> Self {
        Self {
            block_hash: block_hash.to_string(),
            validators,
            vote_type,
            timestamp: chrono::Utc::now().timestamp(),
            signatures,
        }
    }

    pub fn verify(&self) -> Result<bool, Box<dyn Error>> {
        // TODO: Implement verification of all signatures
        Ok(true)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalityProof {
    pub block_hash: String,
    pub validator_signatures: Vec<(String, Vec<u8>)>,
    pub vote_type: VoteType,
    pub timestamp: i64,
}

impl FinalityProof {
    pub fn new(block_hash: &str, validator_signatures: Vec<(String, Vec<u8>)>, vote_type: VoteType) -> Self {
        Self {
            block_hash: block_hash.to_string(),
            validator_signatures,
            vote_type,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn verify(&self) -> Result<bool, Box<dyn Error>> {
        // TODO: Implement verification of all signatures
        Ok(true)
    }
}

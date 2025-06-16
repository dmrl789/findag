use serde::{Serialize, Deserialize};
use crate::types::{Justification, Transaction};
use blake3::Hash;
use hex;
use std::time::{SystemTime, UNIX_EPOCH};
use bincode;
use crate::blockchain::error::BlockchainError;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockAnalysis {
    pub relevance_score: f32,
    pub category: String,
    pub key_topics: Vec<String>,
    pub summary: String,
    pub suggested_improvements: Vec<String>,
}

impl Default for BlockAnalysis {
    fn default() -> Self {
        Self {
            relevance_score: 0.5,
            category: "general".to_string(),
            key_topics: vec![],
            summary: "".to_string(),
            suggested_improvements: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub hash: Vec<u8>,
    pub parents: Vec<Vec<u8>>,
    pub timestamp: u64,
    pub data: Vec<Transaction>,
    pub signature: Vec<u8>,
    pub justification: Option<Vec<u8>>,
}

impl Block {
    pub fn new(id: String, content: String, parent_hash: Option<String>) -> Result<Self, BlockchainError> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| BlockchainError::TimestampError(e.to_string()))?
            .as_secs();
        let hash = Self::calculate_hash(&id, &content, &timestamp, &parent_hash);
        Ok(Self {
            hash: hash.as_bytes().to_vec(),
            parents: vec![],
            timestamp,
            data: vec![],
            signature: vec![],
            justification: None,
        })
    }

    pub fn calculate_hash(id: &str, content: &str, timestamp: &u64, parent_hash: &Option<String>) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(id.as_bytes());
        hasher.update(content.as_bytes());
        hasher.update(&timestamp.to_le_bytes());
        if let Some(parent) = parent_hash {
            hasher.update(parent.as_bytes());
        }
        hex::encode(hasher.finalize().as_bytes())
    }

    pub fn analyze(&self) -> BlockAnalysis {
        BlockAnalysis::default()
    }

    pub fn get_relevance_score(&self) -> Option<f32> {
        None // TODO: Implement this
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = blake3::Hasher::new();
        for tx in &self.data {
            hasher.update(&bincode::serialize(tx).unwrap());
        }
        if let Some(just) = &self.justification {
            hasher.update(just);
        }
        hasher.update(&self.timestamp.to_le_bytes());
        for parent in &self.parents {
            hasher.update(parent);
        }
        hasher.finalize().as_bytes().to_vec()
    }

    pub fn create(data: Vec<Transaction>, parents: Vec<Vec<u8>>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut block = Block {
            hash: vec![0u8; 32],
            parents,
            timestamp,
            data,
            signature: vec![],
            justification: None,
        };
        
        block.hash = block.hash();
        block
    }

    pub fn verify_hash(&self) -> bool {
        self.hash == self.hash()
    }
}

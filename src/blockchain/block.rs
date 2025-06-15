use serde::{Serialize, Deserialize};
use crate::types::finality::Justification;
use crate::types::transaction::Transaction;
use crate::blockchain::block::Block;
use blake3::Hash;
use hex;
use std::time::{SystemTime, UNIX_EPOCH};
use bincode;

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

impl Block {
    pub async fn analyze(&mut self) -> Result<BlockAnalysis, Box<dyn std::error::Error>> {
        Ok(BlockAnalysis::default())
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

    pub fn create(data: Vec<crate::types::transaction::Transaction>, parents: Vec<Vec<u8>>) -> Self {
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
}

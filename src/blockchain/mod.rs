pub mod block;
pub mod transaction;
pub mod state;

use std::error::Error;
use crate::storage::Storage;
use block::Block;

pub struct Blockchain {
    storage: Storage,
}

impl Blockchain {
    pub fn new(storage: Storage) -> Self {
        Blockchain { storage }
    }

    pub async fn add_block(&self, mut block: Block) -> Result<(), Box<dyn Error>> {
        // Analyze block content before adding
        if let Err(e) = block.analyze().await {
            return Err(format!("Failed to analyze block: {}", e).into());
        }

        // Check relevance score
        if let Some(score) = block.get_relevance_score() {
            if score < 0.5 {
                return Err("Block content has low relevance score".into());
            }
        }

        println!("Adding block to chain");
        Ok(())
    }

    pub fn get_block(&self, hash: &[u8]) -> Result<Option<Block>, Box<dyn Error>> {
        println!("Getting block from chain");
        Ok(None)
    }

    pub fn get_latest_block(&self) -> Result<Option<Block>, Box<dyn Error>> {
        println!("Getting latest block from chain");
        Ok(None)
    }

    pub async fn analyze_block_content(&self, content: &str) -> Result<block::BlockAnalysis, Box<dyn Error>> {
        let mut block = Block::new(content.to_string());
        block.analyze().await?;
        block.ai_analysis.ok_or_else(|| "Block analysis failed".into())
    }

    pub async fn get_blocks_by_category(&self, category: &str) -> Result<Vec<Block>, Box<dyn Error>> {
        // This would typically query the storage for blocks with matching category
        // For now, return empty vector
        Ok(Vec::new())
    }

    pub async fn get_blocks_by_relevance(&self, min_score: f32) -> Result<Vec<Block>, Box<dyn Error>> {
        // This would typically query the storage for blocks with relevance score >= min_score
        // For now, return empty vector
        Ok(Vec::new())
    }
}

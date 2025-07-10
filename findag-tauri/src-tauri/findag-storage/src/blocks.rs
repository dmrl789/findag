//! Block storage operations
//! 
//! This module handles storage operations for blockchain blocks.

use sled::Tree;
use findag_core::{Hash, FinDAGTime};
use findag_types::{Block, FindDAGResult};

/// Block storage manager
pub struct BlockStorage {
    /// Blocks tree
    blocks_tree: Tree,
}

impl BlockStorage {
    /// Create a new block storage manager
    pub fn new(blocks_tree: Tree) -> Self {
        Self { blocks_tree }
    }

    /// Store a block
    pub fn store_block(&self, block: &Block) -> FindDAGResult<()> {
        let key = block.header.hash.to_string();
        let value = bincode::serialize(block)?;
        
        self.blocks_tree.insert(key.as_bytes(), value)?;
        Ok(())
    }

    /// Get a block by hash
    pub fn get_block(&self, hash: &Hash) -> FindDAGResult<Option<Block>> {
        let key = hash.to_string();
        let result = self.blocks_tree.get(key.as_bytes())?;
        
        if let Some(value) = result {
            let block: Block = bincode::deserialize(&value)?;
            Ok(Some(block))
        } else {
            Ok(None)
        }
    }

    /// Get block by number
    pub fn get_block_by_number(&self, number: u64) -> FindDAGResult<Option<Block>> {
        let key = format!("block_number:{}", number);
        let result = self.blocks_tree.get(key.as_bytes())?;
        
        if let Some(value) = result {
            let block: Block = bincode::deserialize(&value)?;
            Ok(Some(block))
        } else {
            Ok(None)
        }
    }

    /// Get latest block
    pub fn get_latest_block(&self) -> FindDAGResult<Option<Block>> {
        let key = "latest_block";
        let result = self.blocks_tree.get(key.as_bytes())?;
        
        if let Some(value) = result {
            let block: Block = bincode::deserialize(&value)?;
            Ok(Some(block))
        } else {
            Ok(None)
        }
    }

    /// Set latest block
    pub fn set_latest_block(&self, block: &Block) -> FindDAGResult<()> {
        let key = "latest_block";
        let value = bincode::serialize(block)?;
        
        self.blocks_tree.insert(key.as_bytes(), value)?;
        Ok(())
    }

    /// Get blocks in range
    pub fn get_blocks_in_range(&self, start: u64, end: u64) -> FindDAGResult<Vec<Block>> {
        let mut blocks = Vec::new();
        
        for number in start..=end {
            if let Some(block) = self.get_block_by_number(number)? {
                blocks.push(block);
            }
        }
        
        Ok(blocks)
    }

    /// Get block count
    pub fn get_block_count(&self) -> FindDAGResult<u64> {
        let count = self.blocks_tree.len();
        Ok(count)
    }

    /// Delete block
    pub fn delete_block(&self, hash: &Hash) -> FindDAGResult<()> {
        let key = hash.to_string();
        self.blocks_tree.remove(key.as_bytes())?;
        Ok(())
    }

    /// Check if block exists
    pub fn block_exists(&self, hash: &Hash) -> FindDAGResult<bool> {
        let key = hash.to_string();
        let result = self.blocks_tree.get(key.as_bytes())?;
        Ok(result.is_some())
    }

    /// Get all blocks
    pub fn get_all_blocks(&self) -> FindDAGResult<Vec<Block>> {
        let mut blocks = Vec::new();
        
        for result in self.blocks_tree.iter() {
            let (key, value) = result?;
            let key_str = String::from_utf8_lossy(&key);
            
            // Skip metadata keys
            if !key_str.starts_with("block_number:") && key_str != "latest_block" {
                if let Ok(block) = bincode::deserialize::<Block>(&value) {
                    blocks.push(block);
                }
            }
        }
        
        Ok(blocks)
    }

    /// Get blocks by producer
    pub fn get_blocks_by_producer(&self, producer: &str) -> FindDAGResult<Vec<Block>> {
        let mut blocks = Vec::new();
        
        for result in self.blocks_tree.iter() {
            let (_, value) = result?;
            if let Ok(block) = bincode::deserialize::<Block>(&value) {
                if block.header.producer.to_string() == producer {
                    blocks.push(block);
                }
            }
        }
        
        Ok(blocks)
    }

    /// Get blocks by timestamp range
    pub fn get_blocks_by_timestamp_range(&self, start: FinDAGTime, end: FinDAGTime) -> FindDAGResult<Vec<Block>> {
        let mut blocks = Vec::new();
        
        for result in self.blocks_tree.iter() {
            let (_, value) = result?;
            if let Ok(block) = bincode::deserialize::<Block>(&value) {
                if block.header.timestamp >= start && block.header.timestamp <= end {
                    blocks.push(block);
                }
            }
        }
        
        Ok(blocks)
    }
} 
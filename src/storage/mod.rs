pub mod db;
pub mod sled;

pub mod snapshot;

use std::sync::Arc;
use crate::blockchain::block::Block;

#[derive(Clone)]
pub struct Storage {
    // your underlying storage backend, e.g., sled, sqlite, etc.
    // for placeholder purposes:
    pub db: Arc<sled::Db>,
}

impl Storage {
    pub fn init(path: &str) -> Self {
        let db = sled::open(path).expect("Failed to open DB");
        Storage { db: Arc::new(db) }
    }

    pub fn get_all_blocks_sorted(&self) -> Vec<Block> {
        // placeholder logic for fetching blocks in order
        vec![]
    }

    pub fn get_all_block_hashes_sorted(&self) -> Vec<String> {
        // placeholder logic for fetching ordered block hashes
        vec![]
    }

    pub fn load_blocks_by_hashes(&self, _hashes: &[String]) -> Result<(), String> {
        // placeholder: load block data from hash list
        Ok(())
    }
}

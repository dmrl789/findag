pub mod db;
pub mod round_index;
pub mod asset_index;
pub mod snapshot;

use std::sync::Arc;
use crate::blockchain::block::Block;
use std::error::Error;
use std::path::Path;
use sled::Db;

#[derive(Clone)]
pub struct Storage {
    pub db: Arc<Db>,
    db_path: String,
}

impl Storage {
    pub fn init(db_path: &str) -> Self {
        println!("Initializing storage at {}", db_path);
        let db = sled::open(db_path).expect("Failed to open DB");
        Storage {
            db: Arc::new(db),
            db_path: db_path.to_string(),
        }
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

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        println!("Getting value for key: {:?}", key);
        Ok(self.db.get(key)?.map(|v| v.to_vec()))
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), Box<dyn Error>> {
        println!("Putting value for key: {:?}", key);
        self.db.insert(key, value)?;
        Ok(())
    }

    pub fn delete(&self, key: &[u8]) -> Result<(), Box<dyn Error>> {
        println!("Deleting value for key: {:?}", key);
        self.db.remove(key)?;
        Ok(())
    }
}

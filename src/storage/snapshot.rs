use crate::sync::replay::ReplayState;
use crate::storage::Storage;
use std::fs;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub blocks: Vec<String>,  // block hashes in order
    pub state: ReplayState,   // balances, metadata, etc.
    pub timestamp: u64,
}

impl Snapshot {
    pub fn create(storage: &Storage, state: &ReplayState) -> Result<Self, String> {
        let blocks = storage.get_all_block_hashes_sorted();
        let timestamp = crate::utils::time::now_timestamp();
        Ok(Snapshot { blocks, state: state.clone(), timestamp })
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        let bytes = bincode::serialize(self).map_err(|e| e.to_string())?;
        let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
        file.write_all(&bytes).map_err(|e| e.to_string())
    }

    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).map_err(|e| e.to_string())?;
        bincode::deserialize(&buf).map_err(|e| e.to_string())
    }

    pub fn restore(self, storage: &Storage, replay_state: &mut ReplayState) -> Result<(), String> {
        storage.load_blocks_by_hashes(&self.blocks)?;
        *replay_state = self.state;
        Ok(())
    }
}

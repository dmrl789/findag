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
    pub version: u32,         // snapshot version for compatibility
    pub checksum: Vec<u8>,    // integrity verification
    pub metadata: SnapshotMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct SnapshotMetadata {
    pub block_height: u64,
    pub total_transactions: u64,
    pub last_block_hash: String,
    pub incremental_from: Option<String>, // previous snapshot hash if incremental
    pub compression_type: String,         // compression algorithm used
}

impl Snapshot {
    pub fn create(storage: &Storage, state: &ReplayState) -> Result<Self, String> {
        let blocks = storage.get_all_block_hashes_sorted();
        let timestamp = crate::utils::time::now_timestamp();
        Ok(Snapshot {
            blocks,
            state: state.clone(),
            timestamp,
            version: 1,
            checksum: vec![],
            metadata: SnapshotMetadata {
                block_height: 0,
                total_transactions: 0,
                last_block_hash: String::new(),
                incremental_from: None,
                compression_type: String::new(),
            },
        })
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

    pub fn create_incremental(storage: &Storage, state: &ReplayState, base_snapshot: &str) -> Result<Self, String> {
        let base = Self::load_from_file(base_snapshot)?;
        let new_blocks = storage.get_blocks_since_snapshot(&base.metadata.last_block_hash)?;
        
        let mut snapshot = Self::create(storage, state)?;
        snapshot.metadata.incremental_from = Some(base_snapshot.to_string());
        snapshot.blocks = new_blocks;
        
        Ok(snapshot)
    }

    pub fn verify_integrity(&self) -> Result<bool, String> {
        let computed_checksum = self.compute_checksum()?;
        Ok(computed_checksum == self.checksum)
    }

    fn compute_checksum(&self) -> Result<Vec<u8>, String> {
        // Implement checksum calculation
        Ok(vec![])
    }
}

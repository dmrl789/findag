use crate::types::finality::{FinalityVote, Justification};
use sled::{Db, IVec};
use serde::{Serialize, Deserialize};
use bincode::{self, Options};

pub struct FinalityStore {
    db: Db,
}

impl FinalityStore {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub fn store_vote(&self, vote: &FinalityVote) -> Result<(), String> {
        let key = format!("vote:{}", vote.block_hash);
        let val = bincode::serialize(vote)
            .map_err(|e| format!("Failed to serialize vote: {}", e))?;
        self.db.insert(key, val)
            .map_err(|e| format!("Failed to store vote: {}", e))?;
        Ok(())
    }

    pub fn store_justification(&self, justification: &Justification) -> Result<(), String> {
        let key = format!("justification:{}", justification.block_hash);
        let val = bincode::serialize(justification)
            .map_err(|e| format!("Failed to serialize justification: {}", e))?;
        self.db.insert(key, val)
            .map_err(|e| format!("Failed to store justification: {}", e))?;
        Ok(())
    }
}

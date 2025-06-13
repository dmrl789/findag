use crate::types::finality::{FinalityVote, Justification};
use sled::{Db, IVec};
use serde::{Serialize, Deserialize};
use bincode;

pub struct FinalityStore {
    db: Db,
}

impl FinalityStore {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub fn store_vote(&self, vote: &FinalityVote) {
        let key = format!("vote:{}", vote.block_hash);
        let val = bincode::serialize(vote).unwrap();
        self.db.insert(key, val).unwrap();
    }

    pub fn store_justification(&self, justification: &Justification) {
        let key = format!("justification:{}", justification.block_hash);
        let val = bincode::serialize(justification).unwrap();
        self.db.insert(key, val).unwrap();
    }
}

//! Round storage operations
//! 
//! This module handles storage operations for consensus rounds.

use sled::Tree;
use findag_core::{Hash, FinDAGTime};
use findag_types::{Round, FindDAGResult};

/// Round storage manager
pub struct RoundStorage {
    /// Rounds tree
    rounds_tree: Tree,
}

impl RoundStorage {
    /// Create a new round storage manager
    pub fn new(rounds_tree: Tree) -> Self {
        Self { rounds_tree }
    }

    /// Store a round
    pub fn store_round(&self, round: &Round) -> FindDAGResult<()> {
        let key = format!("round:{}", round.header.number);
        let value = bincode::serialize(round)?;
        
        self.rounds_tree.insert(key.as_bytes(), value)?;
        Ok(())
    }

    /// Get a round by number
    pub fn get_round(&self, number: u64) -> FindDAGResult<Option<Round>> {
        let key = format!("round:{}", number);
        let result = self.rounds_tree.get(key.as_bytes())?;
        
        if let Some(value) = result {
            let round: Round = bincode::deserialize(&value)?;
            Ok(Some(round))
        } else {
            Ok(None)
        }
    }

    /// Get latest round
    pub fn get_latest_round(&self) -> FindDAGResult<Option<Round>> {
        let key = "latest_round";
        let result = self.rounds_tree.get(key.as_bytes())?;
        
        if let Some(value) = result {
            let round: Round = bincode::deserialize(&value)?;
            Ok(Some(round))
        } else {
            Ok(None)
        }
    }

    /// Set latest round
    pub fn set_latest_round(&self, round: &Round) -> FindDAGResult<()> {
        let key = "latest_round";
        let value = bincode::serialize(round)?;
        
        self.rounds_tree.insert(key.as_bytes(), value)?;
        Ok(())
    }
} 
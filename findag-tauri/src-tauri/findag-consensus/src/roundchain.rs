//! RoundChain implementation
//! 
//! This module implements the RoundChain consensus algorithm,
//! providing deterministic finality with parallel block processing.

use findag_core::{Address, Hash, HashTimer, FinDAGTime};
use findag_types::{Block, Round, ConsensusState, FindDAGResult};

/// RoundChain implementation
pub struct RoundChain {
    /// Current round number
    current_round: u64,
    /// Latest finalized round
    latest_finalized_round: u64,
    /// Round history
    rounds: Vec<Round>,
}

impl RoundChain {
    /// Create a new RoundChain
    pub fn new() -> Self {
        Self {
            current_round: 0,
            latest_finalized_round: 0,
            rounds: vec![],
        }
    }

    /// Get current round number
    pub fn current_round(&self) -> u64 {
        self.current_round
    }

    /// Get latest finalized round
    pub fn latest_finalized_round(&self) -> u64 {
        self.latest_finalized_round
    }

    /// Finalize a round
    pub fn finalize_round(&mut self, round: Round) -> FindDAGResult<()> {
        // Validate round
        self.validate_round(&round)?;
        
        // Add to history
        self.rounds.push(round);
        
        // Update state
        self.latest_finalized_round = self.current_round;
        self.current_round += 1;
        
        Ok(())
    }

    /// Validate a round
    fn validate_round(&self, round: &Round) -> FindDAGResult<()> {
        // Check round number
        if round.header.number != self.current_round {
            return Err(findag_types::FindDAGError::ConsensusError(
                format!("Invalid round number: expected {}, got {}", 
                    self.current_round, round.header.number)
            ));
        }
        
        // Check previous round reference
        if self.latest_finalized_round > 0 {
            if round.header.previous_round != self.rounds.last().unwrap().header.hash {
                return Err(findag_types::FindDAGError::ConsensusError(
                    "Invalid previous round reference".to_string()
                ));
            }
        }
        
        Ok(())
    }
} 
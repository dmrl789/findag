pub mod ai_security;
pub mod finality;
pub mod reputation;
pub mod round;
pub mod scheduler;
pub mod validators;

pub use round::*;
pub use validators::ValidatorSet;
pub use reputation::*;
// pub use finality::{FinalityManager, FinalityConfig};
pub use finality::FinalityManager;
// pub use ai_security::AISecurityManager;
// use crate::consensus::ai_security::AISecureConsensus;
pub use scheduler::*;

use crate::types::vote::VoteType;
use crate::types::finality::{FinalityVote, Justification};
use crate::types::block::Block;
use crate::security::SecurityManager;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct ConsensusConfig {
    pub finality_threshold: usize,
    pub min_validators: usize,
    pub max_validators: usize,
    pub block_time: u64,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            finality_threshold: 2,
            min_validators: 3,
            max_validators: 100,
            block_time: 1000, // 1 second
        }
    }
}

#[derive(Debug)]
pub struct ConsensusManager {
    config: ConsensusConfig,
    finality_manager: FinalityManager,
    // TODO: AISecureConsensus is unresolved, field commented out
    // ai_consensus: Option<AISecureConsensus>,
    validator_set: ValidatorSet,
}

impl ConsensusManager {
    pub fn new(config: ConsensusConfig) -> Self {
        Self {
            finality_manager: FinalityManager::new(config.finality_threshold),
            // TODO: AISecureConsensus is unresolved, usage commented out
            // ai_consensus: None,
            validator_set: ValidatorSet::new(),
            config,
        }
    }

    pub fn enable_ai_security(&mut self, security_config: crate::security::SecurityConfig) {
        // TODO: AISecureConsensus is unresolved, usage commented out
        // self.ai_consensus = Some(AISecureConsensus::new(
        //     security_config,
        //     self.config.finality_threshold,
        // ));
    }

    pub async fn submit_vote(&mut self, vote: FinalityVote) -> Result<(), String> {
        if !self.validator_set.is_authorized(&vote.validator) {
            return Err("Unauthorized validator".to_string());
        }
        self.finality_manager.submit_vote(vote);
        Ok(())
    }

    pub async fn try_finalize(&mut self, block_hash: &str) -> Option<Justification> {
        // TODO: ai_consensus is unresolved, code commented out
        // if let Some(ai_consensus) = &mut self.ai_consensus {
        //     ai_consensus.try_finalize(block_hash).await
        // } else {
        self.finality_manager.try_finalize(block_hash)
        // }
    }

    pub fn is_finalized(&self, block_hash: &str) -> bool {
        // TODO: ai_consensus is unresolved, code commented out
        // if let Some(ai_consensus) = &self.ai_consensus {
        //     ai_consensus.is_finalized(block_hash)
        // } else {
        self.finality_manager.is_finalized(block_hash)
        // }
    }

    pub fn get_justification(&self, block_hash: &str) -> Option<&Justification> {
        // TODO: ai_consensus is unresolved, code commented out
        // if let Some(ai_consensus) = &self.ai_consensus {
        //     ai_consensus.get_justification(block_hash)
        // } else {
        self.finality_manager.get_justification(block_hash)
        // }
    }

    pub fn get_vote_counts(&self, block_hash: &str) -> Option<(usize, usize)> {
        // TODO: ai_consensus is unresolved, code commented out
        // if let Some(ai_consensus) = &self.ai_consensus {
        //     ai_consensus.get_vote_counts(block_hash)
        // } else {
        self.finality_manager.get_vote_counts(block_hash)
        // }
    }

    pub fn add_validator(&mut self, validator: &str) -> bool {
        self.validator_set.authorize(validator)
    }

    pub fn remove_validator(&mut self, validator: &str) -> bool {
        self.validator_set.revoke(validator)
    }
}

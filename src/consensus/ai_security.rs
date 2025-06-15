use crate::types::finality::{FinalityVote, Justification};
use crate::types::vote::VoteType;
use crate::security::{SecurityManager, SecurityConfig, SecurityAuditLog};
use crate::security::audit::SecuritySeverity;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug)]
pub struct AISecureConsensus {
    security_manager: Arc<Mutex<SecurityManager>>,
    votes: HashMap<String, Vec<FinalityVote>>,
    finalized_blocks: HashSet<String>,
    justifications: HashMap<String, Justification>,
    finality_threshold: usize,
}

impl AISecureConsensus {
    pub fn new(security_config: SecurityConfig, threshold: usize) -> Self {
        Self {
            security_manager: Arc::new(Mutex::new(SecurityManager::new(security_config))),
            votes: HashMap::new(),
            finalized_blocks: HashSet::new(),
            justifications: HashMap::new(),
            finality_threshold: threshold,
        }
    }

    pub async fn submit_vote(&mut self, vote: FinalityVote) -> Result<(), String> {
        // Security checks
        {
            let mut manager = self.security_manager.lock().await;
            manager.check_rate_limit(&vote.validator).await?;
        }
        // Store vote
        self.votes.entry(vote.block_hash.clone())
            .or_default()
            .push(vote.clone());
        // Log audit
        {
            let mut manager = self.security_manager.lock().await;
            manager.log_audit_event(
                vote.validator.clone(),
                SecuritySeverity::Low,
                format!("Vote submitted by validator {} for block {} with type {}", 
                    vote.validator, vote.block_hash, vote.vote_type),
            ).await;
        }
        Ok(())
    }

    pub async fn try_finalize(&mut self, block_hash: &str) -> Option<Justification> {
        let votes = match self.votes.get(block_hash) {
            Some(v) => v,
            None => return None,
        };

        let (for_votes, against_votes) = votes.iter().fold((Vec::new(), Vec::new()), |(mut for_vec, mut against_vec), vote| {
            match vote.vote_type {
                VoteType::For => for_vec.push(vote),
                VoteType::Against => against_vec.push(vote),
                _ => {}
            }
            (for_vec, against_vec)
        });

        if for_votes.len() >= self.finality_threshold {
            let justification = Justification::new(
                block_hash,
                for_votes.iter().map(|v| v.validator.clone()).collect(),
                VoteType::For,
                for_votes.iter().map(|v| v.signature.clone()).collect(),
            );
            self.finalized_blocks.insert(block_hash.to_string());
            self.justifications.insert(block_hash.to_string(), justification.clone());
            Some(justification)
        } else {
            None
        }
    }

    pub fn is_finalized(&self, block_hash: &str) -> bool {
        self.finalized_blocks.contains(block_hash)
    }

    pub fn get_justification(&self, block_hash: &str) -> Option<&Justification> {
        self.justifications.get(block_hash)
    }

    pub fn get_vote_counts(&self, block_hash: &str) -> Option<(usize, usize)> {
        self.votes.get(block_hash).map(|votes| {
            votes.iter().fold((0, 0), |(for_count, against_count), vote| {
                match vote.vote_type {
                    VoteType::For => (for_count + 1, against_count),
                    VoteType::Against => (for_count, against_count + 1),
                    _ => (for_count, against_count),
                }
            })
        })
    }
} 
use std::path::Path;
use sled;
use serde::{Serialize, Deserialize};
use crate::types::{
    governance::{Proposal, ProposalStatus, ProposalType},
    vote::{Vote, VoteType},
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use crate::utils::governance::{get_current_timestamp, validate_proposal_parameters};
use thiserror::Error;

pub struct GovernanceConfig {
    pub required_quorum_percentage: u64,
    pub required_majority_percentage: u64,
    pub voting_period_days: u64,
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self {
            required_quorum_percentage: 30,
            required_majority_percentage: 51,
            voting_period_days: 7,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GovernanceError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Proposal not found")]
    ProposalNotFound,
    #[error("Vote already cast")]
    VoteAlreadyCast,
    #[error("Proposal already closed")]
    ProposalAlreadyClosed,
    #[error("Insufficient voting power")]
    InsufficientVotingPower,
    #[error("Invalid vote type")]
    InvalidVoteType,
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub struct GovernanceRegistry {
    db: sled::Db,
    proposals: Arc<Mutex<HashMap<u64, Proposal>>>,
    votes: Arc<Mutex<HashMap<u64, Vec<Vote>>>>,
    config: GovernanceConfig,
}

impl GovernanceRegistry {
    pub fn new(path: &Path, config: Option<GovernanceConfig>) -> Result<Self, GovernanceError> {
        let db = sled::open(path).map_err(|e| GovernanceError::InternalError(e.to_string()))?;
        Ok(Self {
            db,
            proposals: Arc::new(Mutex::new(HashMap::new())),
            votes: Arc::new(Mutex::new(HashMap::new())),
            config: config.unwrap_or_default(),
        })
    }

    pub async fn create_proposal(
        &self,
        title: String,
        description: String,
        proposal_type: ProposalType,
    ) -> Result<u64, GovernanceError> {
        validate_proposal_parameters(&title, &description, &proposal_type)
            .map_err(|e| GovernanceError::ValidationError(e))?;

        let mut proposals = self.proposals.lock().await;
        let id = proposals.len() as u64 + 1;
        let proposal = Proposal::new(
            id,
            title,
            description,
            proposal_type,
        );
        proposals.insert(id, proposal);
        Ok(id)
    }

    pub async fn cast_vote(
        &self,
        proposal_id: u64,
        voter: String,
        vote_type: VoteType,
    ) -> Result<(), GovernanceError> {
        let mut proposals = self.proposals.lock().await;
        let proposal = proposals.get_mut(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?;

        if proposal.status != ProposalStatus::Active {
            return Err(GovernanceError::ProposalAlreadyClosed);
        }
        if proposal.is_expired() {
            proposal.status = ProposalStatus::Expired;
            return Err(GovernanceError::ProposalAlreadyClosed);
        }

        let mut votes = self.votes.lock().await;
        let proposal_votes = votes.entry(proposal_id).or_insert_with(Vec::new);
        if proposal_votes.iter().any(|v| v.voter == voter) {
            return Err(GovernanceError::VoteAlreadyCast);
        }
        let vote = Vote::new(proposal_id, voter, vote_type);
        proposal_votes.push(vote);
        Ok(())
    }

    pub async fn finalize_proposal(&self, proposal_id: u64) -> Result<(), GovernanceError> {
        let mut proposals = self.proposals.lock().await;
        let mut votes = self.votes.lock().await;
        let proposal = proposals.get_mut(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?;
        let proposal_votes = votes.get(&proposal_id).cloned().unwrap_or_default();
        let total_votes = proposal_votes.len() as u64;
        let for_votes = proposal_votes.iter().filter(|v| v.vote_type == VoteType::For).count() as u64;
        let quorum_met = total_votes * 100 / self.config.required_quorum_percentage >= 100;
        let majority_met = if total_votes > 0 {
            for_votes * 100 / total_votes >= self.config.required_majority_percentage
        } else {
            false
        };
        proposal.status = if quorum_met && majority_met {
            ProposalStatus::Passed
        } else {
            ProposalStatus::Rejected
        };
        Ok(())
    }

    pub async fn get_proposal(&self, proposal_id: u64) -> Result<Option<Proposal>, GovernanceError> {
        let proposals = self.proposals.lock().await;
        Ok(proposals.get(&proposal_id).cloned())
    }

    pub async fn get_votes(&self, proposal_id: u64) -> Result<Vec<Vote>, GovernanceError> {
        let votes = self.votes.lock().await;
        Ok(votes.get(&proposal_id).cloned().unwrap_or_default())
    }
}

pub struct GovernanceManager {
    config: GovernanceConfig,
    proposals: Arc<RwLock<HashMap<u64, Proposal>>>,
    votes: Arc<RwLock<HashMap<u64, Vec<Vote>>>>,
}

impl GovernanceManager {
    pub fn new(config: GovernanceConfig) -> Self {
        Self {
            config,
            proposals: Arc::new(RwLock::new(HashMap::new())),
            votes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_proposal(
        &self,
        title: String,
        description: String,
        proposal_type: ProposalType,
    ) -> Result<u64, String> {
        let mut proposals = self.proposals.write().await;
        let id = proposals.len() as u64 + 1;
        
        let proposal = Proposal::new(
            id,
            title,
            description,
            proposal_type,
        );

        proposals.insert(id, proposal);
        Ok(id)
    }

    pub async fn get_proposal(&self, id: u64) -> Result<Option<Proposal>, String> {
        let proposals = self.proposals.read().await;
        Ok(proposals.get(&id).cloned())
    }

    pub async fn add_vote(&self, proposal_id: u64, voter: String, vote_type: VoteType) -> Result<(), String> {
        let vote = Vote::new(proposal_id, voter, vote_type);
        
        let mut votes = self.votes.write().await;
        votes.entry(proposal_id)
            .or_insert_with(Vec::new)
            .push(vote);

        self.update_proposal_status(proposal_id).await?;
        Ok(())
    }

    async fn update_proposal_status(&self, proposal_id: u64) -> Result<(), String> {
        let mut proposals = self.proposals.write().await;
        let votes = self.votes.read().await;
        
        let proposal = proposals.get_mut(&proposal_id)
            .ok_or_else(|| "Proposal not found".to_string())?;

        if let Some(proposal_votes) = votes.get(&proposal_id) {
            let total_votes = proposal_votes.len() as u64;
            let yes_votes = proposal_votes.iter()
                .filter(|v| v.vote_type == VoteType::For)
                .count() as u64;

            if total_votes > 0 {
                let quorum_met = total_votes * 100 / self.config.required_quorum_percentage >= 100;
                let majority_met = yes_votes * 100 / total_votes >= self.config.required_majority_percentage;

                if quorum_met && majority_met {
                    proposal.status = ProposalStatus::Passed;
                } else {
                    proposal.status = ProposalStatus::Rejected;
                }
            }
        }

        if proposal.is_expired() {
            proposal.status = ProposalStatus::Expired;
        }

        Ok(())
    }

    pub async fn get_votes(&self, proposal_id: u64) -> Result<Vec<Vote>, String> {
        let votes = self.votes.read().await;
        Ok(votes.get(&proposal_id).cloned().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_proposal_creation() {
        let manager = GovernanceManager::new(GovernanceConfig::default());
        let result = manager.create_proposal(
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            ProposalType::ParameterChange,
        ).await;
        
        assert!(result.is_ok());
        let proposal_id = result.unwrap();
        assert_eq!(proposal_id, 1);
    }

    #[tokio::test]
    async fn test_voting() {
        let manager = GovernanceManager::new(GovernanceConfig::default());
        let proposal_id = manager.create_proposal(
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            ProposalType::ParameterChange,
        ).await.unwrap();

        assert!(manager.add_vote(proposal_id, "voter1".to_string(), VoteType::For).await.is_ok());
        assert!(manager.add_vote(proposal_id, "voter2".to_string(), VoteType::For).await.is_ok());
        assert!(manager.add_vote(proposal_id, "voter3".to_string(), VoteType::Against).await.is_ok());

        let votes = manager.get_votes(proposal_id).await.unwrap();
        assert_eq!(votes.len(), 3);
    }
}

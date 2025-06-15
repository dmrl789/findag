pub mod domain;
pub mod governance;
pub mod governance_test;
pub mod handle;
pub mod multisig;
pub mod reputation;
pub mod vote;

pub use handle::*;
pub use multisig::*;
pub use reputation::*;
pub use vote::*;
pub use governance::*;
pub use domain::*;

use crate::types::{
    governance::{Proposal, ProposalStatus, ProposalType},
    vote::{Vote, VoteType},
};
use std::collections::HashMap;
use std::sync::RwLock;

pub struct Registry {
    proposals: RwLock<HashMap<u64, Proposal>>,
    votes: RwLock<HashMap<u64, Vec<Vote>>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            proposals: RwLock::new(HashMap::new()),
            votes: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_proposal(&self, proposal: Proposal) -> Result<(), String> {
        let mut proposals = self.proposals.write().map_err(|_| "Failed to acquire write lock")?;
        proposals.insert(proposal.id, proposal);
        Ok(())
    }

    pub fn get_proposal(&self, id: u64) -> Result<Option<Proposal>, String> {
        let proposals = self.proposals.read().map_err(|_| "Failed to acquire read lock")?;
        Ok(proposals.get(&id).cloned())
    }

    pub fn update_proposal_status(&self, id: u64, status: ProposalStatus) -> Result<(), String> {
        let mut proposals = self.proposals.write().map_err(|_| "Failed to acquire write lock")?;
        if let Some(proposal) = proposals.get_mut(&id) {
            proposal.status = status;
            Ok(())
        } else {
            Err("Proposal not found".to_string())
        }
    }

    pub fn add_vote(&self, vote: Vote) -> Result<(), String> {
        let mut votes = self.votes.write().map_err(|_| "Failed to acquire write lock")?;
        votes.entry(vote.proposal_id)
            .or_insert_with(Vec::new)
            .push(vote);
        Ok(())
    }

    pub fn get_votes(&self, proposal_id: u64) -> Result<Vec<Vote>, String> {
        let votes = self.votes.read().map_err(|_| "Failed to acquire read lock")?;
        Ok(votes.get(&proposal_id).cloned().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proposal_management() {
        let registry = Registry::new();
        let proposal = Proposal::new(
            1,
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            ProposalType::ParameterChange,
        );

        assert!(registry.add_proposal(proposal.clone()).is_ok());
        assert_eq!(registry.get_proposal(1).unwrap().unwrap().id, 1);
        assert!(registry.update_proposal_status(1, ProposalStatus::Passed).is_ok());
        assert_eq!(registry.get_proposal(1).unwrap().unwrap().status, ProposalStatus::Passed);
    }

    #[test]
    fn test_vote_management() {
        let registry = Registry::new();
        let vote = Vote::new(1, "voter1".to_string(), VoteType::For);

        assert!(registry.add_vote(vote.clone()).is_ok());
        let votes = registry.get_votes(1).unwrap();
        assert_eq!(votes.len(), 1);
        assert_eq!(votes[0].voter, "voter1");
    }
}

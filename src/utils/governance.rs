use crate::types::{
    governance::{Proposal, ProposalStatus, ProposalType},
    vote::{Vote, VoteType},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
// use crate::types::governance::{GovernanceProposalStatus};

pub struct GovernanceManager {
    proposals: Arc<Mutex<HashMap<u64, Proposal>>>,
    votes: Arc<Mutex<HashMap<u64, Vec<Vote>>>>,
}

impl GovernanceManager {
    pub fn new() -> Self {
        Self {
            proposals: Arc::new(Mutex::new(HashMap::new())),
            votes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_proposal(&self, proposal: Proposal) -> Result<u64, String> {
        let mut proposals = self.proposals.lock().map_err(|_| "Failed to lock proposals")?;
        let id = proposals.len() as u64 + 1;
        proposals.insert(id, proposal);
        Ok(id)
    }

    pub fn get_proposal(&self, proposal_id: u64) -> Result<Option<Proposal>, String> {
        let proposals = self.proposals.lock().map_err(|_| "Failed to lock proposals")?;
        Ok(proposals.get(&proposal_id).cloned())
    }

    pub fn get_passed_proposals(&self) -> Result<Vec<Proposal>, String> {
        let proposals = self.proposals.lock().map_err(|_| "Failed to lock proposals")?;
        Ok(proposals.values()
            .filter(|p| matches!(p.status, ProposalStatus::Passed))
            .cloned()
            .collect())
    }

    pub fn get_active_proposals(&self) -> Result<Vec<Proposal>, String> {
        let proposals = self.proposals.lock().map_err(|_| "Failed to lock proposals")?;
        Ok(proposals.values()
            .filter(|p| matches!(p.status, ProposalStatus::Active))
            .cloned()
            .collect())
    }

    pub fn get_finalized_proposals(&self) -> Result<Vec<Proposal>, String> {
        let proposals = self.proposals.lock().map_err(|_| "Failed to lock proposals")?;
        Ok(proposals.values()
            .filter(|p| p.status == ProposalStatus::Passed || p.status == ProposalStatus::Rejected || p.status == ProposalStatus::Expired)
            .cloned()
            .collect())
    }

    pub fn add_vote(&self, proposal_id: u64, vote: Vote) -> Result<(), String> {
        let mut votes = self.votes.lock().map_err(|_| "Failed to lock votes")?;
        votes.entry(proposal_id).or_insert_with(Vec::new).push(vote);
        Ok(())
    }

    pub fn get_votes(&self, proposal_id: u64) -> Result<Vec<Vote>, String> {
        let votes = self.votes.lock().map_err(|_| "Failed to lock votes")?;
        Ok(votes.get(&proposal_id).cloned().unwrap_or_default())
    }

    pub fn get_vote_counts(&self, proposal_id: u64) -> Result<(usize, usize, usize), String> {
        let votes = self.get_votes(proposal_id)?;
        let for_votes = votes.iter().filter(|v| v.vote_type == VoteType::For).count();
        let against_votes = votes.iter().filter(|v| v.vote_type == VoteType::Against).count();
        let abstain_votes = votes.iter().filter(|v| v.vote_type == VoteType::Abstain).count();
        Ok((for_votes, against_votes, abstain_votes))
    }
}

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

pub fn calculate_voting_power(votes: &[Vote]) -> (u64, u64, u64) {
    let mut for_votes = 0;
    let mut against_votes = 0;
    let mut abstain_votes = 0;
    for vote in votes {
        match vote.vote_type {
            VoteType::For => for_votes += 1,
            VoteType::Against => against_votes += 1,
            VoteType::Abstain => abstain_votes += 1,
        }
    }
    (for_votes, against_votes, abstain_votes)
}

pub fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn is_proposal_active(start_time: u64, end_time: u64, current_time: u64) -> bool {
    current_time >= start_time && current_time <= end_time
}

pub fn validate_proposal_parameters(
    title: &str,
    description: &str,
    proposal_type: &ProposalType,
) -> Result<(), String> {
    if title.is_empty() {
        return Err("Proposal title cannot be empty".to_string());
    }

    if description.is_empty() {
        return Err("Proposal description cannot be empty".to_string());
    }

    Ok(())
}

pub fn check_proposal_quorum(
    total_votes: u64,
    required_quorum: u64,
    total_stake: u64,
) -> bool {
    if total_stake == 0 {
        return false;
    }
    (total_votes * 100) / total_stake >= required_quorum
}

pub fn check_proposal_majority(
    for_votes: u64,
    total_votes: u64,
    required_majority: u64,
) -> bool {
    if total_votes == 0 {
        return false;
    }
    (for_votes * 100) / total_votes >= required_majority
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_proposal_parameters() {
        assert!(validate_proposal_parameters(
            "Test Title",
            "Test Description",
            &ProposalType::ParameterChange,
        ).is_ok());

        assert!(validate_proposal_parameters(
            "",
            "Test Description",
            &ProposalType::ParameterChange,
        ).is_err());

        assert!(validate_proposal_parameters(
            "Test Title",
            "",
            &ProposalType::ParameterChange,
        ).is_err());
    }

    #[test]
    fn test_calculate_voting_power() {
        let votes = vec![
            Vote::new(1, "voter1".to_string(), VoteType::For),
            Vote::new(1, "voter2".to_string(), VoteType::For),
            Vote::new(1, "voter3".to_string(), VoteType::Against),
            Vote::new(1, "voter4".to_string(), VoteType::Abstain),
        ];

        let (for_votes, against_votes, abstain_votes) = calculate_voting_power(&votes);
        assert_eq!(for_votes, 2);
        assert_eq!(against_votes, 1);
        assert_eq!(abstain_votes, 1);
    }

    #[test]
    fn test_check_proposal_quorum() {
        assert!(check_proposal_quorum(30, 30, 100));
        assert!(!check_proposal_quorum(20, 30, 100));
        assert!(!check_proposal_quorum(30, 30, 0));
    }

    #[test]
    fn test_check_proposal_majority() {
        assert!(check_proposal_majority(60, 100, 51));
        assert!(!check_proposal_majority(40, 100, 51));
        assert!(!check_proposal_majority(60, 0, 51));
    }
}

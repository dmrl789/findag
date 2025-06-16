use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
    Expired,
}

impl fmt::Display for ProposalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProposalStatus::Active => write!(f, "Active"),
            ProposalStatus::Passed => write!(f, "Passed"),
            ProposalStatus::Rejected => write!(f, "Rejected"),
            ProposalStatus::Executed => write!(f, "Executed"),
            ProposalStatus::Expired => write!(f, "Expired"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalType {
    ParameterChange,
    ProtocolUpgrade,
    Emergency,
}

impl fmt::Display for ProposalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProposalType::ParameterChange => write!(f, "Parameter Change"),
            ProposalType::ProtocolUpgrade => write!(f, "Protocol Upgrade"),
            ProposalType::Emergency => write!(f, "Emergency"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub status: ProposalStatus,
    pub created_at: u64,
    pub end_time: u64,
}

impl Proposal {
    pub fn new(
        id: u64,
        title: String,
        description: String,
        proposal_type: ProposalType,
    ) -> Self {
        Self {
            id,
            title,
            description,
            proposal_type,
            status: ProposalStatus::Active,
            created_at: chrono::Utc::now().timestamp() as u64,
            end_time: (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as u64,
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == ProposalStatus::Active
    }

    pub fn is_expired(&self) -> bool {
        chrono::Utc::now().timestamp() as u64 > self.end_time
    }
}

impl fmt::Display for Proposal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Proposal {{ id: {}, title: {}, type: {}, status: {} }}",
            self.id, self.title, self.proposal_type, self.status
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub voter: String,
    pub choice: VoteChoice,
    pub timestamp: u64,
    pub weight: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceConfig {
    pub proposal_duration: u64,
    pub min_proposal_stake: u64,
    pub required_quorum_percentage: u64,
    pub required_majority_percentage: u64,
    pub voting_power_decay: u64,
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self {
            proposal_duration: 7 * 24 * 60 * 60, // 7 days in seconds
            min_proposal_stake: 1000,
            required_quorum_percentage: 20,
            required_majority_percentage: 51,
            voting_power_decay: 30 * 24 * 60 * 60, // 30 days in seconds
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proposal_creation() {
        let proposal = Proposal::new(
            1,
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            ProposalType::ParameterChange,
        );
        assert_eq!(proposal.id, 1);
        assert_eq!(proposal.status, ProposalStatus::Active);
        assert!(proposal.is_active());
        assert!(!proposal.is_expired());
    }

    #[test]
    fn test_proposal_status_display() {
        assert_eq!(ProposalStatus::Active.to_string(), "Active");
        assert_eq!(ProposalStatus::Passed.to_string(), "Passed");
        assert_eq!(ProposalStatus::Rejected.to_string(), "Rejected");
        assert_eq!(ProposalStatus::Executed.to_string(), "Executed");
        assert_eq!(ProposalStatus::Expired.to_string(), "Expired");
    }

    #[test]
    fn test_proposal_type_display() {
        assert_eq!(ProposalType::ParameterChange.to_string(), "Parameter Change");
        assert_eq!(ProposalType::ProtocolUpgrade.to_string(), "Protocol Upgrade");
        assert_eq!(ProposalType::Emergency.to_string(), "Emergency");
    }
}


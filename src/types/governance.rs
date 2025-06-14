use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GovernanceProposalType {
    AddValidator { node_id: String },
    RemoveValidator { node_id: String },
    UpdateParam { key: String, value: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub id: String,
    pub proposer: String,
    pub proposal_type: GovernanceProposalType,
    pub timestamp: u64,
    pub votes_for: Vec<String>,
    pub votes_against: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VoteType {
    Yes,
    No,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub voter: String,
    pub vote: VoteType,
}

pub type Proposal = GovernanceProposal;
pub use Vote;
pub use VoteType;

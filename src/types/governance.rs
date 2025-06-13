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

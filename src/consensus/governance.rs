use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceState {
    pub proposals: Vec<GovernanceProposal>,
    pub active_votes: Vec<Vote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub id: String,
    pub proposer: String,
    pub proposal_type: String,
    pub content: String,
    pub status: ProposalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub proposal_id: String,
    pub voter: String,
    pub vote: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalStatus {
    Active,
    Passed,
    Failed,
    Expired,
}

impl Default for GovernanceState {
    fn default() -> Self {
        Self {
            proposals: Vec::new(),
            active_votes: Vec::new(),
        }
    }
} 
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Governance proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub timestamp: u64,
    pub active: bool,
}

/// Proposal status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalStatus {
    Active,
    Passed,
    Failed,
    Expired,
}

/// Governance state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceState {
    pub proposals: HashMap<String, Proposal>,
    pub votes: HashMap<String, Vec<Vote>>,
    pub active_proposals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub proposal_id: String,
    pub voter: String,
    pub vote: bool, // true for yes, false for no
    pub timestamp: u64,
}

impl Default for GovernanceState {
    fn default() -> Self {
        Self {
            proposals: HashMap::new(),
            votes: HashMap::new(),
            active_proposals: Vec::new(),
        }
    }
} 
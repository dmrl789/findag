use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteType {
    Approve,
    Reject,
    Abstain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ballot {
    pub voter: String,
    pub vote_type: VoteType,
    pub timestamp: DateTime<Utc>,
}

impl Ballot {
    pub fn new(voter: String, vote_type: VoteType) -> Self {
        Self {
            voter,
            vote_type,
            timestamp: Utc::now(),
        }
    }
} 
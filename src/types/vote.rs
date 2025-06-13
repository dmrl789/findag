use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub description: String,
    pub options: Vec<String>,
    pub deadline: u64,
    pub creator: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ballot {
    pub proposal_id: String,
    pub voter: String,
    pub choice: usize,
    pub signature: String,
}

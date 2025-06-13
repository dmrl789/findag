import os
import ace_tools as tools

# Define the files again after state reset
file_structure = {
    "src/governance/mod.rs": """
use crate::types::governance::{Proposal, Vote};
use crate::types::validator::ValidatorSet;
use crate::storage::DB;

pub fn submit_proposal(db: &DB, validator_id: &str, proposal: Proposal) -> Result<(), String> {
    if !ValidatorSet::is_validator(db, validator_id) {
        return Err("Only validators can submit proposals.".into());
    }
    db.save_proposal(proposal);
    Ok(())
}

pub fn submit_vote(db: &DB, validator_id: &str, vote: Vote) -> Result<(), String> {
    if !ValidatorSet::is_validator(db, validator_id) {
        return Err("Only validators can vote.".into());
    }
    db.save_vote(vote);
    Ok(())
}
""",
    "src/types/governance.rs": """
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub proposer: String,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub proposal_id: String,
    pub validator: String,
    pub support: bool,
    pub timestamp: u64,
}
""",
    "src/types/validator.rs": """
use serde::{Deserialize, Serialize};
use crate::storage::DB;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub id: String,
    pub pubkey: String,
}

pub struct ValidatorSet;

impl ValidatorSet {
    pub fn is_validator(db: &DB, validator_id: &str) -> bool {
        db.get_validator(validator_id).is_some()
    }
}
""",
    "src/storage/mod.rs": """
use crate::types::{governance::{Proposal, Vote}, validator::Validator};
use sled::Db;

pub struct DB {
    db: Db,
}

impl DB {
    pub fn new(path: &str) -> Self {
        let db = sled::open(path).expect("Failed to open DB");
        DB { db }
    }

    pub fn save_proposal(&self, proposal: Proposal) {
        let key = format!("proposal:{}", proposal.id);
        let val = serde_json::to_vec(&proposal).unwrap();
        self.db.insert(key, val).unwrap();
    }

    pub fn save_vote(&self, vote: Vote) {
        let key = format!("vote:{}:{}", vote.proposal_id, vote.validator);
        let val = serde_json::to_vec(&vote).unwrap();
        self.db.insert(key, val).unwrap();
    }

    pub fn get_validator(&self, id: &str) -> Option<Validator> {
        self.db.get(format!("validator:{}", id))
            .ok()
            .flatten()
            .and_then(|val| serde_json::from_slice(&val).ok())
    }
}
""",
    "src/lib.rs": """
pub mod governance;
pub mod storage;
pub mod types;
"""
}

# Create files
project_path = "/mnt/data/findag-core"
for filepath, content in file_structure.items():
    full_path = os.path.join(project_path, filepath)
    os.makedirs(os.path.dirname(full_path), exist_ok=True)
    with open(full_path, "w") as f:
        f.write(content.strip())

tools.display_dataframe_to_user(name="Generated Governance Files", dataframe=[{"Path": k} for k in file_structure.keys()])

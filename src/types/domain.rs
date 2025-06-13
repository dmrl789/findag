use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainRecord {
    pub domain: String,
    pub owner: String,
    pub updated_at: u64,
    pub metadata: Option<String>,
}


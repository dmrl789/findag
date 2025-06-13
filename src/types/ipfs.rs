use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpfsMetadata {
    pub cid: String,
    pub description: Option<String>,
    pub owner: String,
    pub timestamp: u64,
}

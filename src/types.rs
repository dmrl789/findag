use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FinDAGTransaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub currency: String,
    pub signature: String,
    pub hashtimer: [u8; 32],
} 
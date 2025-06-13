use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultisigAccount {
    pub handle: String,
    pub owners: HashSet<String>, // public keys
    pub threshold: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingMultisigTx {
    pub tx_id: String,
    pub initiator: String,
    pub target: String,
    pub action: String, // for example: "transfer:amount:to"
    pub approvals: HashSet<String>, // approved by
    pub required: u8,
}

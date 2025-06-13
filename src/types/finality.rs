use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FinalityVote {
    pub block_hash: String,
    pub signer: String,
    pub signature: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Justification {
    pub block_hash: String,
    pub signers: Vec<String>,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryShare {
    pub index: u8,
    pub share: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryData {
    pub threshold: u8,
    pub total_shares: u8,
    pub shares: Vec<RecoveryShare>,
}

impl RecoveryShare {
    pub fn new(index: u8, share: Vec<u8>, signature: Vec<u8>) -> Self {
        Self {
            index,
            share,
            signature,
        }
    }
}

impl RecoveryData {
    pub fn new(threshold: u8, total_shares: u8) -> Self {
        Self {
            threshold,
            total_shares,
            shares: Vec::new(),
        }
    }

    pub fn add_share(&mut self, share: RecoveryShare) {
        self.shares.push(share);
    }

    pub fn is_complete(&self) -> bool {
        self.shares.len() >= self.threshold as usize
    }
} 
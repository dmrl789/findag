use crate::types::asset::{AssetRecord};
use serde::{Serialize, Deserialize};
use std::error::Error;
use sha2::{Sha256, Digest};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use hex;
use chrono;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TxType {
    LoadAsset(AssetRecord),
    UnloadAsset(String), // ID of asset
    TransferAsset { id: String, from: String, to: String },
    UpdateHandle { owner: String, new_handle: String }, 
    AuthorizeValidator,
    RevokeValidator,
    Transfer,
    CreateAsset,
    UpdateAsset,
    DeleteAsset,
    Vote,
    Proposal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: Vec<u8>,
    pub tx_id: String,
    pub timestamp: i64,
    pub tx_type: TxType,
    pub sender: Vec<u8>,
    pub recipient: Vec<u8>,
    pub amount: u64,
    pub signature: Vec<u8>,
    pub data: Vec<u8>,
}

impl Transaction {
    pub fn new(
        tx_type: TxType,
        sender: Vec<u8>,
        recipient: Vec<u8>,
        amount: u64,
        data: Vec<u8>,
    ) -> Self {
        Self {
            hash: vec![],
            tx_id: String::new(),
            timestamp: chrono::Utc::now().timestamp(),
            tx_type,
            sender,
            recipient,
            amount,
            signature: Vec::new(),
            data,
        }
    }

    pub fn compute_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.timestamp.to_le_bytes());
        hasher.update(&self.data);
        hasher.update(self.amount.to_le_bytes());
        hasher.update(serde_json::to_vec(&self.tx_type).unwrap());
        hasher.finalize().to_vec()
    }

    pub fn sign(&mut self, private_key: &[u8]) -> Result<(), Box<dyn Error>> {
        // TODO: Implement signing
        Ok(())
    }

    pub fn verify(&self) -> Result<bool, Box<dyn Error>> {
        // TODO: Implement verification
        Ok(true)
    }
}

pub use self::{Transaction, TxType};
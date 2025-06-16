use crate::types::Asset;
use serde::{Serialize, Deserialize};
use std::error::Error;
use sha2::{Sha256, Digest};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use hex;
use chrono;
use std::fmt;
use crate::utils::crypto::verify_signature;
use blake3::Hash;
use std::time::SystemTime;
use base64;
use base64::Engine;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TxType {
    Transfer,
    LoadAsset(Asset),
    UnloadAsset(String),
    TransferAsset { id: String, from: String, to: String },
    UpdateHandle { owner: String, new_handle: String },
    RevokeValidator,
    CreateAsset(Asset),
    AuthorizeValidator,
    GovernanceVote,
    FinalityVote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: Vec<u8>,
    pub data: Vec<u8>,
    pub timestamp: SystemTime,
    pub signature: String,
    pub public_key: Vec<u8>,
    pub status: TransactionStatus,
}

impl Transaction {
    pub fn new(data: Vec<u8>, signature: String, public_key: Vec<u8>) -> Self {
        Self {
            hash: vec![0u8; 32],
            data,
            timestamp: SystemTime::now(),
            signature,
            public_key,
            status: TransactionStatus::Pending,
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.data);
        hasher.update(self.signature.as_bytes());
        hasher.update(&self.public_key);
        hasher.finalize().as_bytes().to_vec()
    }

    pub fn signature(&self) -> Result<Signature, Box<dyn Error>> {
        let signature_bytes = base64::engine::general_purpose::STANDARD.decode(&self.signature)?;
        let signature_array: [u8; 64] = signature_bytes.try_into().map_err(|_| "Invalid signature length")?;
        Ok(Signature::from_bytes(&signature_array))
    }

    pub fn set_status(&mut self, status: TransactionStatus) {
        self.status = status;
    }
}

#[derive(Debug, Clone)]
pub struct TransactionList {
    pub transactions: Vec<Transaction>,
}

impl TransactionList {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }

    pub fn add(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }
}

impl fmt::Display for TransactionList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TransactionList with {} transactions", self.transactions.len())
    }
}
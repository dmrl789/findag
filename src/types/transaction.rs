use crate::storage::types::AssetType;
use serde::{Serialize, Deserialize};
use std::error::Error;
use sha2::{Sha256, Digest};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use hex;
use chrono;
use std::fmt;
use crate::utils::crypto::verify_signature;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TxType {
    Transfer,
    LoadAsset(AssetType),
    UnloadAsset(String),
    TransferAsset { id: String, from: String, to: String },
    UpdateHandle { owner: String, new_handle: String },
    RevokeValidator,
    CreateAsset(AssetType),
    AuthorizeValidator,
    GovernanceVote,
    FinalityVote,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub tx_type: TxType,
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub amount: u64,
    pub payload: Vec<u8>,
    pub initiator: String,
    pub hash: Vec<u8>,
    pub signature: Vec<u8>,
    pub timestamp: u64,
    pub data: Vec<u8>,
}

impl Transaction {
    pub fn new(tx_type: TxType, from: Vec<u8>, to: Vec<u8>, amount: u64, payload: Vec<u8>, initiator: String, timestamp: u64, data: Vec<u8>) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(&data);
        let hash = hasher.finalize().to_vec();
        Self {
            tx_type,
            from,
            to,
            amount,
            payload,
            initiator,
            hash,
            signature: Vec::new(),
            timestamp,
            data,
        }
    }

    pub fn hash(&self) -> &[u8] {
        &self.hash
    }

    pub fn verify_signature(&self) -> Result<bool, Box<dyn Error>> {
        // TODO: Implement signature verification
        Ok(true)
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Transaction {{ sender: {}, recipient: {}, amount: {}, timestamp: {} }}",
            self.initiator, hex::encode(&self.to), self.amount, self.timestamp)
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
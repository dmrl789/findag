use std::error::Error;
use serde::{Serialize, Deserialize};
use ed25519_dalek::Signature;
use blake3::Hash;
use hex;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: Vec<u8>,
    pub data: Vec<u8>,
    pub timestamp: SystemTime,
    pub signature: String,
    pub public_key: Vec<u8>,
}

impl Transaction {
    pub fn new(data: Vec<u8>, signature: String, public_key: Vec<u8>) -> Self {
        Self {
            hash: vec![0u8; 32],
            data,
            timestamp: SystemTime::now(),
            signature,
            public_key,
        }
    }

    pub fn hash(&self) -> Hash {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.data);
        hasher.update(self.signature.as_bytes());
        hasher.update(&self.public_key);
        hasher.finalize()
    }

    pub fn signature(&self) -> Result<Signature, Box<dyn Error>> {
        // Convert the stored signature string to bytes and create a Signature
        let signature_bytes = hex::decode(&self.signature)?;
        let signature_array: [u8; 64] = signature_bytes.try_into().map_err(|_| "Invalid signature length")?;
        Ok(Signature::from_bytes(&signature_array))
    }
} 
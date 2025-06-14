use std::error::Error;
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddressType {
    User,
    Validator,
    Contract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinDagKeypair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

impl FinDagKeypair {
    pub fn generate() -> Result<Self, Box<dyn Error>> {
        let keypair = SigningKey::generate(&mut rand::rngs::OsRng);
        let public_key = keypair.verifying_key().to_bytes().to_vec();
        let private_key = keypair.to_bytes().to_vec();

        Ok(Self {
            public_key,
            private_key,
        })
    }

    pub fn from_private_key(private_key: &[u8]) -> Result<Self, Box<dyn Error>> {
        let signing_key = SigningKey::from_bytes(private_key)?;
        let public_key = signing_key.verifying_key().to_bytes().to_vec();

        Ok(Self {
            public_key,
            private_key: private_key.to_vec(),
        })
    }

    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let signing_key = SigningKey::from_bytes(&self.private_key)?;
        let signature = signing_key.sign(message);
        Ok(signature.to_bytes().to_vec())
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, Box<dyn Error>> {
        let verifying_key = VerifyingKey::from_bytes(&self.public_key)?;
        let signature = ed25519_dalek::Signature::from_bytes(signature)?;
        Ok(verifying_key.verify(message, &signature).is_ok())
    }
}

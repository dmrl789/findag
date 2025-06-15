use std::error::Error;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use serde::{Serialize, Deserialize};
use crate::utils::crypto::verify_signature;
use rand::rngs::OsRng;
use rand::Rng;
use rand::RngCore;
use std::fmt;
use hex;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AddressType {
    Validator,
    User,
    Contract,
    System,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address {
    pub public_key: [u8; 32],
    pub name: Option<String>,
    pub created_at: u64,
}

impl Address {
    pub fn new(public_key: [u8; 32], name: Option<String>) -> Self {
        Self {
            public_key,
            name,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        if bytes.len() != 32 {
            return Err("Invalid address length".into());
        }
        let mut public_key = [0u8; 32];
        public_key.copy_from_slice(bytes);
        Ok(Self {
            public_key,
            name: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.public_key.to_vec()
    }

    pub fn generate_validator() -> (Self, SigningKey) {
        let mut rng = OsRng;
        let mut secret = [0u8; 32];
        rng.fill_bytes(&mut secret);
        let signing_key = SigningKey::from_bytes(&secret);
        let verifying_key = VerifyingKey::from(&signing_key);
        let address = Address {
            public_key: verifying_key.to_bytes(),
            name: Some(String::new()),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        (address, signing_key)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}:{}", self.name, hex::encode(&self.public_key))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinDagKeypair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

impl FinDagKeypair {
    pub fn generate() -> Result<Self, Box<dyn Error>> {
        let mut rng = rand::rngs::OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        let public_key = signing_key.verifying_key().to_bytes().to_vec();
        let private_key = signing_key.to_bytes().to_vec();

        Ok(Self {
            public_key,
            private_key,
        })
    }

    pub fn from_private_key(private_key: &[u8]) -> Result<Self, Box<dyn Error>> {
        let private_key_array: [u8; 32] = private_key.try_into()
            .map_err(|_| "Invalid private key length".to_string())?;
        let signing_key = SigningKey::from_bytes(&private_key_array);
        let public_key = signing_key.verifying_key().to_bytes().to_vec();
        Ok(Self {
            public_key,
            private_key: private_key.to_vec(),
        })
    }

    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let private_key_array: [u8; 32] = self.private_key.as_slice().try_into()
            .map_err(|_| "Invalid private key length".to_string())?;
        let signing_key = SigningKey::from_bytes(&private_key_array);
        let signature = signing_key.sign(message);
        Ok(signature.to_bytes().to_vec())
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, Box<dyn Error>> {
        verify_signature(message, signature, &self.public_key)
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn Error>)
    }
}

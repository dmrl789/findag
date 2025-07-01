use rand::rngs::OsRng; 
use libp2p_identity;
use ed25519_dalek::{Keypair, PublicKey};
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(pub String);

impl Address {
    pub fn new(addr: String) -> Self {
        Self(addr)
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
    
    pub fn from_public_key(public_key: &PublicKey) -> Self {
        let addr_bytes = public_key.to_bytes();
        let addr_hex = hex::encode(&addr_bytes[..8]); // Use first 8 bytes for shorter address
        Address(format!("fdg1q{}", addr_hex))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn generate_address() -> (Keypair, Address) {
    let keypair = Keypair::generate(&mut OsRng);
    let address = Address::from_public_key(&keypair.public);
    (keypair, address)
}

pub fn generate_keypair() -> Keypair {
    Keypair::generate(&mut OsRng)
}

pub fn generate_deterministic_keypair(seed: &[u8; 32]) -> Keypair {
    use ed25519_dalek::SecretKey;
    use ed25519_dalek::SECRET_KEY_LENGTH;
    
    // Use the seed directly as the secret key
    let mut secret_key_bytes = [0u8; SECRET_KEY_LENGTH];
    secret_key_bytes.copy_from_slice(&seed[..SECRET_KEY_LENGTH]);
    
    let secret_key = SecretKey::from_bytes(&secret_key_bytes)
        .expect("Failed to create secret key from seed");
    
    Keypair::from_bytes(&secret_key_bytes)
        .expect("Failed to create keypair from seed")
}

pub fn generate_address_from_keypair(keypair: &Keypair) -> Address {
    Address::from_public_key(&keypair.public)
} 
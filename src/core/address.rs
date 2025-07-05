use ed25519_dalek::{Keypair, PublicKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a FinDAG address
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(pub String);

impl Address {
    /// Create a new address from a string
    pub fn new(addr: String) -> Self {
        Self(addr)
    }

    /// Get the address as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Generate a new random address
    pub fn random() -> Self {
        let keypair = Keypair::generate(&mut OsRng);
        let address_bytes = keypair.public.to_bytes();
        let address_hex = hex::encode(&address_bytes[..8]); // Use first 8 bytes for shorter addresses
        Self(format!("FD{}", address_hex))
    }

    /// Generate a new address from a keypair
    pub fn from_keypair(keypair: &Keypair) -> Self {
        let address_bytes = keypair.public.to_bytes();
        let address_hex = hex::encode(&address_bytes[..8]);
        Self(format!("FD{}", address_hex))
    }

    /// Generate address from public key
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

impl From<String> for Address {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Address {
    fn from(s: &str) -> Self {
        Self(s.to_string())
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
    
    let _secret_key = SecretKey::from_bytes(&secret_key_bytes)
        .expect("Failed to create secret key from seed");
    
    Keypair::from_bytes(&secret_key_bytes)
        .expect("Failed to create keypair from seed")
}

pub fn generate_address_from_keypair(keypair: &Keypair) -> Address {
    Address::from_public_key(&keypair.public)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_creation() {
        let addr = Address::new("test_address".to_string());
        assert_eq!(addr.as_str(), "test_address");
    }

    #[test]
    fn test_random_address() {
        let addr1 = Address::random();
        let addr2 = Address::random();
        assert_ne!(addr1, addr2);
        assert!(addr1.as_str().starts_with("FD"));
        assert!(addr2.as_str().starts_with("FD"));
    }

    #[test]
    fn test_address_from_keypair() {
        let keypair = Keypair::generate(&mut OsRng);
        let addr = Address::from_keypair(&keypair);
        assert!(addr.as_str().starts_with("FD"));
    }
} 
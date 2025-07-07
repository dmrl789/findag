use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::{rngs::OsRng, RngCore};
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
        let mut rng = OsRng;
        let mut secret_bytes = [0u8; 32];
        rng.fill_bytes(&mut secret_bytes);
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let verifying_key = signing_key.verifying_key();
        let address_bytes = verifying_key.to_bytes();
        let address_hex = hex::encode(&address_bytes[..8]); // Use first 8 bytes for shorter addresses
        Self(format!("FD{address_hex}"))
    }

    /// Generate a new address from a signing key
    pub fn from_signing_key(signing_key: &SigningKey) -> Self {
        let verifying_key = signing_key.verifying_key();
        let address_bytes = verifying_key.to_bytes();
        let address_hex = hex::encode(&address_bytes[..8]);
        Self(format!("FD{address_hex}"))
    }

    /// Generate address from verifying key
    pub fn from_verifying_key(verifying_key: &VerifyingKey) -> Self {
        let addr_bytes = verifying_key.to_bytes();
        let addr_hex = hex::encode(&addr_bytes[..8]); // Use first 8 bytes for shorter address
        Address(format!("FD{addr_hex}"))
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

pub fn generate_address() -> (SigningKey, Address) {
    let mut rng = OsRng;
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);
    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let address = Address::from_verifying_key(&signing_key.verifying_key());
    (signing_key, address)
}

pub fn generate_signing_key() -> SigningKey {
    let mut rng = OsRng;
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);
    SigningKey::from_bytes(&secret_bytes)
}

pub fn generate_deterministic_signing_key(seed: &[u8; 32]) -> SigningKey {
    SigningKey::from_bytes(seed)
}

pub fn generate_address_from_signing_key(signing_key: &SigningKey) -> Address {
    Address::from_verifying_key(&signing_key.verifying_key())
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
    fn test_address_from_signing_key() {
        let mut rng = OsRng;
        let mut secret_bytes = [0u8; 32];
        rng.fill_bytes(&mut secret_bytes);
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let addr = Address::from_signing_key(&signing_key);
        assert!(addr.as_str().starts_with("FD"));
    }
} 
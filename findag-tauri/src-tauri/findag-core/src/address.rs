use serde::{Serialize, Deserialize};
use bech32::{self, ToBase32, FromBase32};
use std::fmt;

/// FinDAG address type with validation and formatting
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(pub String);

impl Address {
    /// Create a new FinDAG address
    pub fn new(address: String) -> Result<Self, crate::FinDAGError> {
        if Self::is_valid(&address) {
            Ok(Address(address))
        } else {
            Err(crate::FinDAGError::InvalidAddress(format!("Invalid address format: {}", address)))
        }
    }

    /// Create a new address from a public key
    pub fn from_public_key(public_key: &[u8; 32]) -> Result<Self, crate::FinDAGError> {
        let mut hasher = sha2::Sha256::new();
        hasher.update(public_key);
        hasher.update(b"findag");
        let hash = hasher.finalize();
        
        let address_data = hash.to_vec();
        let address = bech32::encode("fdg", address_data.to_base32(), bech32::Variant::Bech32)
            .map_err(|e| crate::FinDAGError::InvalidAddress(format!("Bech32 encoding failed: {}", e)))?;
        
        Ok(Address(address))
    }

    /// Validate if an address string is valid
    pub fn is_valid(address: &str) -> bool {
        if !address.starts_with("fdg1") {
            return false;
        }
        
        match bech32::decode(address) {
            Ok((hrp, data, _)) => {
                if hrp != "fdg" {
                    return false;
                }
                
                let _decoded = Vec::<u8>::from_base32(&data).is_ok();
                true
            }
            Err(_) => false,
        }
    }

    /// Get the address as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get the address bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>, crate::FinDAGError> {
        let (hrp, data, _) = bech32::decode(&self.0)
            .map_err(|e| crate::FinDAGError::InvalidAddress(format!("Bech32 decoding failed: {}", e)))?;
        
        if hrp != "fdg" {
            return Err(crate::FinDAGError::InvalidAddress("Invalid address prefix".to_string()));
        }
        
        Vec::<u8>::from_base32(&data)
            .map_err(|e| crate::FinDAGError::InvalidAddress(format!("Base32 decoding failed: {}", e)))
    }

    /// Generate a random test address
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let bytes: [u8; 32] = rng.gen();
        
        let address_data = bytes.to_vec();
        let address = bech32::encode("fdg", address_data.to_base32(), bech32::Variant::Bech32)
            .expect("Failed to encode random address");
        
        Address(address)
    }

    /// Check if this is a genesis address
    pub fn is_genesis(&self) -> bool {
        self.0 == "fdg1genesis"
    }

    /// Check if this is a system address
    pub fn is_system(&self) -> bool {
        self.0.starts_with("fdg1system")
    }

    /// Get the address type (user, system, genesis)
    pub fn address_type(&self) -> AddressType {
        if self.is_genesis() {
            AddressType::Genesis
        } else if self.is_system() {
            AddressType::System
        } else {
            AddressType::User
        }
    }
}

/// Address types in FinDAG
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressType {
    User,
    System,
    Genesis,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for Address {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Address {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<String> for Address {
    fn from(s: String) -> Self {
        Address(s)
    }
}

impl From<&str> for Address {
    fn from(s: &str) -> Self {
        Address(s.to_string())
    }
}

impl AsRef<str> for Address {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_validation() {
        // Valid addresses
        assert!(Address::is_valid("fdg1qtest123456789abcdef"));
        assert!(Address::is_valid("fdg1genesis"));
        assert!(Address::is_valid("fdg1system"));
        
        // Invalid addresses
        assert!(!Address::is_valid("invalid"));
        assert!(!Address::is_valid("btc1qtest"));
        assert!(!Address::is_valid(""));
    }

    #[test]
    fn test_address_creation() {
        let addr = Address::new("fdg1qtest123456789abcdef".to_string());
        assert!(addr.is_ok());
        
        let invalid_addr = Address::new("invalid".to_string());
        assert!(invalid_addr.is_err());
    }

    #[test]
    fn test_address_from_public_key() {
        let public_key = [1u8; 32];
        let addr = Address::from_public_key(&public_key);
        assert!(addr.is_ok());
        assert!(addr.unwrap().as_str().starts_with("fdg1"));
    }

    #[test]
    fn test_random_address() {
        let addr = Address::random();
        assert!(Address::is_valid(addr.as_str()));
    }

    #[test]
    fn test_address_types() {
        let genesis = Address::from("fdg1genesis");
        assert_eq!(genesis.address_type(), AddressType::Genesis);
        
        let system = Address::from("fdg1system");
        assert_eq!(system.address_type(), AddressType::System);
        
        let user = Address::from("fdg1quser123");
        assert_eq!(user.address_type(), AddressType::User);
    }

    #[test]
    fn test_address_serialization() {
        let addr = Address::from("fdg1qtest123456789abcdef");
        let serialized = serde_json::to_string(&addr).unwrap();
        let deserialized: Address = serde_json::from_str(&serialized).unwrap();
        assert_eq!(addr, deserialized);
    }
} 
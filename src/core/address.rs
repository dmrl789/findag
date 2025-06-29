use ed25519_dalek::{Keypair, PublicKey};
use rand07::rngs::OsRng;
use sha2::{Sha256, Digest};
use bech32::{self, ToBase32, Variant};
use serde::{Serialize, Deserialize};

const HRP: &str = "fdg";
const VERSION: u8 = 0x01;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(pub String);

impl Address {
    /// Create an address from a public key (Bech32 encoded)
    pub fn from_public_key(pk: &PublicKey) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(pk.as_bytes());
        let hash = hasher.finalize();
        let mut data = vec![VERSION];
        data.extend_from_slice(&hash[..32]);
        let bech = bech32::encode(HRP, data.as_slice().to_base32(), Variant::Bech32).unwrap();
        Address(bech)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Generate a new Ed25519 keypair and FinDAG address
pub fn generate_address() -> (Keypair, Address) {
    let mut csprng = OsRng {};
    let keypair = Keypair::generate(&mut csprng);
    let address = Address::from_public_key(&keypair.public);
    (keypair, address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_generation() {
        let (keypair, address) = generate_address();
        println!("Public Key: {}", hex::encode(keypair.public.as_bytes()));
        println!("Address: {}", address.as_str());
        assert!(address.as_str().starts_with(HRP));
    }
} 
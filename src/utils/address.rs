use sha2::{Sha256, Digest};
use ripemd::{Ripemd160};
use base58::ToBase58;
use ed25519_dalek::VerifyingKey;
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressType {
    Standard,  // fd1
    Contract,  // fd2
    Validator, // fd3
    Asset,     // fd4
}

impl AddressType {
    pub fn to_prefix(&self) -> u8 {
        match self {
            AddressType::Standard => 0x00, // fd1
            AddressType::Contract => 0x01, // fd2
            AddressType::Validator => 0x02, // fd3
            AddressType::Asset => 0x03, // fd4
        }
    }
}

pub fn generate_address(public_key: &VerifyingKey, addr_type: AddressType) -> String {
    let pk_bytes = public_key.as_bytes();

    // Step 1: SHA-256
    let sha256 = Sha256::digest(pk_bytes);

    // Step 2: RIPEMD-160
    let ripemd = Ripemd160::digest(&sha256);

    // Step 3: Prefix
    let prefix: u8 = addr_type.to_prefix();

    let mut payload = vec![prefix];
    payload.extend(&ripemd);

    // Step 4: Checksum (first 4 bytes of double SHA256)
    let checksum = Sha256::digest(&Sha256::digest(&payload));
    payload.extend(&checksum[0..4]);

    // Step 5: Base58
    payload.to_base58()
}

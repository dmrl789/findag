use sha2::{Sha256, Digest};
use ripemd::{Ripemd160};
use base58::ToBase58;
use ed25519_dalek::PublicKey;

pub enum AddressType {
    Standard,
    Multisig,
}

pub fn generate_address(public_key: &PublicKey, addr_type: AddressType) -> String {
    let pk_bytes = public_key.as_bytes();

    // Step 1: SHA-256
    let sha256 = Sha256::digest(pk_bytes);

    // Step 2: RIPEMD-160
    let ripemd = Ripemd160::digest(&sha256);

    // Step 3: Prefix
    let prefix: u8 = match addr_type {
        AddressType::Standard => 0x00, // fd1
        AddressType::Multisig => 0x05, // fd3
    };

    let mut payload = vec![prefix];
    payload.extend(&ripemd);

    // Step 4: Checksum (first 4 bytes of double SHA256)
    let checksum = Sha256::digest(&Sha256::digest(&payload));
    payload.extend(&checksum[0..4]);

    // Step 5: Base58
    payload.to_base58()
}

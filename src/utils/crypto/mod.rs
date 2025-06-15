use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use sha2::{Sha256, Digest};
use hex;

pub fn verify_signature(
    message: &[u8],
    signature: &[u8],
    public_key: &[u8],
) -> Result<bool, String> {
    // Convert public key to fixed size array
    let public_key_array: [u8; 32] = public_key.try_into()
        .map_err(|_| "Invalid public key length".to_string())?;
    
    // Convert signature to fixed size array
    let signature_array: [u8; 64] = signature.try_into()
        .map_err(|_| "Invalid signature length".to_string())?;

    let verifying_key = VerifyingKey::from_bytes(&public_key_array)
        .map_err(|e| format!("Invalid public key: {}", e))?;
    
    let signature = Signature::from_bytes(&signature_array);
    
    Ok(verifying_key.verify(message, &signature).is_ok())
}

pub fn hash_data(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn hash_to_hex(hash: &[u8]) -> String {
    hex::encode(hash)
}

pub fn hex_to_bytes(hex_str: &str) -> Result<Vec<u8>, String> {
    hex::decode(hex_str).map_err(|e| format!("Invalid hex string: {}", e))
}

pub mod hash;
pub mod signature;
pub mod encryption;

pub use hash::*;
pub use signature::*;
pub use encryption::*; 
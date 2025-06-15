use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use rand::Rng;
use rand::RngCore;
use serde::{Serialize, Deserialize};
use chrono;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use hex;
use crate::blockchain::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub user_id: String,
    pub token: String,
    pub expires_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCredentials {
    pub user_id: String,
    pub public_key: Vec<u8>,
}

#[derive(Debug)]
pub struct SecurityManager {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl SecurityManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut rng = OsRng;
        let mut secret = [0u8; 32];
        rng.fill_bytes(&mut secret);
        let signing_key = SigningKey::from_bytes(&secret);
        let verifying_key = VerifyingKey::from(&signing_key);
        Ok(Self {
            signing_key,
            verifying_key,
        })
    }

    pub fn verify_token(&self, token: &str) -> bool {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 2 {
            return false;
        }
        let message = parts[0];
        let signature_str = parts[1];
        let signature_vec = match BASE64.decode(signature_str) {
            Ok(arr) => arr,
            Err(_) => return false,
        };
        if signature_vec.len() != 64 {
            return false;
        }
        let signature_array: [u8; 64] = signature_vec.as_slice().try_into().unwrap();
        let signature = Signature::from_bytes(&signature_array);
        self.verifying_key.verify(message.as_bytes(), &signature).is_ok()
    }
}

pub fn verify_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool, Box<dyn Error>> {
    if signature.len() != 64 || public_key.len() != 32 {
        return Ok(false);
    }
    let signature_array: [u8; 64] = signature.try_into().unwrap();
    let public_key_array: [u8; 32] = public_key.try_into().unwrap();
    let verifying_key = VerifyingKey::from_bytes(&public_key_array)?;
    let signature = Signature::from_bytes(&signature_array);
    Ok(verifying_key.verify_strict(message, &signature).is_ok())
}

pub fn verify_message_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool, Box<dyn Error>> {
    if signature.len() != 64 || public_key.len() != 32 {
        return Ok(false);
    }
    let signature_array: [u8; 64] = signature.try_into().unwrap();
    let public_key_array: [u8; 32] = public_key.try_into().unwrap();
    let verifying_key = VerifyingKey::from_bytes(&public_key_array)?;
    let signature = Signature::from_bytes(&signature_array);
    Ok(verifying_key.verify_strict(message, &signature).is_ok())
}

pub fn verify_authentication(message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool, Box<dyn Error>> {
    if signature.len() != 64 || public_key.len() != 32 {
        return Ok(false);
    }
    let signature_array: [u8; 64] = signature.try_into().unwrap();
    let public_key_array: [u8; 32] = public_key.try_into().unwrap();
    let verifying_key = VerifyingKey::from_bytes(&public_key_array)?;
    let signature = Signature::from_bytes(&signature_array);
    Ok(verifying_key.verify_strict(message, &signature).is_ok())
}

pub fn verify_transaction(tx: &Transaction) -> Result<bool, String> {
    let message = &tx.data;
    let signature_array = BASE64.decode(&tx.signature)
        .map_err(|e| format!("Failed to decode signature: {}", e))?;
    let signature_array: [u8; 64] = signature_array[..64].try_into().map_err(|_| "Invalid signature length")?;
    let signature = Signature::from_bytes(&signature_array);
    let verifying_key = match VerifyingKey::from_bytes(&tx.public_key[..32].try_into().map_err(|_| "Invalid public key length")?) {
        Ok(key) => key,
        Err(e) => return Err(format!("Invalid public key: {}", e)),
    };
    Ok(verifying_key.verify(message, &signature).is_ok())
}

impl SecurityManager {
    pub fn generate_token(&self, user_id: &str, duration_hours: i64) -> AuthToken {
        let expires_at = chrono::Utc::now().timestamp() + (duration_hours * 3600);
        let token_data = format!("{}:{}", user_id, expires_at);
        let signature = self.signing_key.sign(token_data.as_bytes());
        let token = BASE64.encode(signature.to_bytes());
        AuthToken {
            user_id: user_id.to_string(),
            token,
            expires_at,
        }
    }
} 
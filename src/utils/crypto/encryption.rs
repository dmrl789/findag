use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use rand::{rngs::OsRng, RngCore};
use std::error::Error;

pub fn generate_key() -> Key<Aes256Gcm> {
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    Key::<Aes256Gcm>::from_slice(&key_bytes).clone()
}

pub fn generate_nonce() -> [u8; 12] {
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    nonce_bytes
}

pub fn encrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(&[0u8; 12]);
    cipher.encrypt(nonce, data).map_err(|e| e.to_string().into())
}

pub fn decrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(&[0u8; 12]);
    cipher.decrypt(nonce, data).map_err(|e| e.to_string().into())
} 
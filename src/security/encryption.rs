use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use rand::{rngs::OsRng, RngCore};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub tag: Vec<u8>,
}

pub struct EncryptionManager {
    cipher: Aes256Gcm,
}

impl EncryptionManager {
    pub fn new(key: &[u8]) -> Self {
        let cipher_key = Key::<Aes256Gcm>::from_slice(key);
        Self {
            cipher: Aes256Gcm::new(cipher_key),
        }
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<EncryptedData, String> {
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        match self.cipher.encrypt(nonce, data) {
            Ok(ciphertext) => {
                let tag = ciphertext[ciphertext.len() - 16..].to_vec();
                let ciphertext = ciphertext[..ciphertext.len() - 16].to_vec();
                Ok(EncryptedData {
                    ciphertext,
                    nonce: nonce_bytes.to_vec(),
                    tag,
                })
            }
            Err(e) => Err(format!("Encryption failed: {}", e)),
        }
    }

    pub fn decrypt(&self, encrypted: &EncryptedData) -> Result<Vec<u8>, String> {
        let nonce = Nonce::from_slice(&encrypted.nonce);
        let mut ciphertext = encrypted.ciphertext.clone();
        ciphertext.extend_from_slice(&encrypted.tag);

        match self.cipher.decrypt(nonce, ciphertext.as_ref()) {
            Ok(plaintext) => Ok(plaintext),
            Err(e) => Err(format!("Decryption failed: {}", e)),
        }
    }
} 
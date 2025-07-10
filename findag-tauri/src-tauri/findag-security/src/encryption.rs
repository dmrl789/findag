//! Encryption module
//! 
//! This module handles data encryption and decryption operations.

use findag_types::{FindDAGResult, FindDAGError};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use chacha20poly1305::{ChaCha20Poly1305, Key as ChaChaKey, Nonce as ChaChaNonce};
use chacha20poly1305::aead::{Aead as ChaChaAead, NewAead as ChaChaNewAead};
use rand::{Rng, RngCore};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use metrics::{counter, histogram};

/// Encryption manager
pub struct EncryptionManager {
    /// Master key
    master_key: Vec<u8>,
    /// Key derivation salt
    salt: Vec<u8>,
    /// Key cache
    key_cache: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    /// Configuration
    config: EncryptionConfig,
}

/// Encryption configuration
#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    /// Algorithm to use
    pub algorithm: EncryptionAlgorithm,
    /// Key size in bytes
    pub key_size: usize,
    /// Nonce size in bytes
    pub nonce_size: usize,
    /// Enable key caching
    pub enable_key_cache: bool,
    /// Cache size limit
    pub cache_size_limit: usize,
}

#[derive(Debug, Clone)]
pub enum EncryptionAlgorithm {
    AES256GCM,
    ChaCha20Poly1305,
    XChaCha20Poly1305,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::AES256GCM,
            key_size: 32,
            nonce_size: 12,
            enable_key_cache: true,
            cache_size_limit: 1000,
        }
    }
}

impl EncryptionManager {
    /// Create a new encryption manager
    pub fn new(master_key: Vec<u8>, salt: Vec<u8>, config: EncryptionConfig) -> Self {
        Self {
            master_key,
            salt,
            key_cache: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Encrypt data
    pub async fn encrypt(&self, data: &[u8], context: &str) -> FindDAGResult<Vec<u8>> {
        let start_time = std::time::Instant::now();
        
        // Derive key for context
        let key = self.derive_key(context).await?;
        
        // Encrypt data
        let encrypted = match self.config.algorithm {
            EncryptionAlgorithm::AES256GCM => {
                self.encrypt_aes256gcm(data, &key).await?
            }
            EncryptionAlgorithm::ChaCha20Poly1305 => {
                self.encrypt_chacha20poly1305(data, &key).await?
            }
            EncryptionAlgorithm::XChaCha20Poly1305 => {
                self.encrypt_xchacha20poly1305(data, &key).await?
            }
        };
        
        // Record metrics
        let latency = start_time.elapsed().as_millis() as f64;
        counter!("findag_encryption_operations", 1);
        histogram!("findag_encryption_latency_ms", latency);
        
        Ok(encrypted)
    }

    /// Decrypt data
    pub async fn decrypt(&self, encrypted_data: &[u8], context: &str) -> FindDAGResult<Vec<u8>> {
        let start_time = std::time::Instant::now();
        
        // Derive key for context
        let key = self.derive_key(context).await?;
        
        // Decrypt data
        let decrypted = match self.config.algorithm {
            EncryptionAlgorithm::AES256GCM => {
                self.decrypt_aes256gcm(encrypted_data, &key).await?
            }
            EncryptionAlgorithm::ChaCha20Poly1305 => {
                self.decrypt_chacha20poly1305(encrypted_data, &key).await?
            }
            EncryptionAlgorithm::XChaCha20Poly1305 => {
                self.decrypt_xchacha20poly1305(encrypted_data, &key).await?
            }
        };
        
        // Record metrics
        let latency = start_time.elapsed().as_millis() as f64;
        counter!("findag_decryption_operations", 1);
        histogram!("findag_decryption_latency_ms", latency);
        
        Ok(decrypted)
    }

    /// Derive key for context
    async fn derive_key(&self, context: &str) -> FindDAGResult<Vec<u8>> {
        // Check cache first
        if self.config.enable_key_cache {
            let cache = self.key_cache.read().await;
            if let Some(key) = cache.get(context) {
                return Ok(key.clone());
            }
        }
        
        // Derive key using PBKDF2
        let mut key = vec![0u8; self.config.key_size];
        pbkdf2::pbkdf2::<hmac::Hmac<sha2::Sha256>>(
            &self.master_key,
            format!("{}:{}", context, hex::encode(&self.salt)).as_bytes(),
            10000, // iterations
            &mut key,
        );
        
        // Cache key if enabled
        if self.config.enable_key_cache {
            let mut cache = self.key_cache.write().await;
            if cache.len() < self.config.cache_size_limit {
                cache.insert(context.to_string(), key.clone());
            }
        }
        
        Ok(key)
    }

    /// Encrypt with AES-256-GCM
    async fn encrypt_aes256gcm(&self, data: &[u8], key: &[u8]) -> FindDAGResult<Vec<u8>> {
        let cipher = Aes256Gcm::new_from_slice(key)?;
        let nonce = self.generate_nonce();
        
        let ciphertext = cipher.encrypt(&nonce, data)?;
        
        // Combine nonce and ciphertext
        let mut result = Vec::new();
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }

    /// Decrypt with AES-256-GCM
    async fn decrypt_aes256gcm(&self, encrypted_data: &[u8], key: &[u8]) -> FindDAGResult<Vec<u8>> {
        if encrypted_data.len() < self.config.nonce_size {
            return Err(FindDAGError::DecryptionFailed("Invalid encrypted data".to_string()));
        }
        
        let cipher = Aes256Gcm::new_from_slice(key)?;
        let nonce = &encrypted_data[..self.config.nonce_size];
        let ciphertext = &encrypted_data[self.config.nonce_size..];
        
        let plaintext = cipher.decrypt(nonce, ciphertext)?;
        Ok(plaintext)
    }

    /// Encrypt with ChaCha20-Poly1305
    async fn encrypt_chacha20poly1305(&self, data: &[u8], key: &[u8]) -> FindDAGResult<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new_from_slice(key)?;
        let nonce = self.generate_nonce();
        
        let ciphertext = cipher.encrypt(&nonce, data)?;
        
        // Combine nonce and ciphertext
        let mut result = Vec::new();
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }

    /// Decrypt with ChaCha20-Poly1305
    async fn decrypt_chacha20poly1305(&self, encrypted_data: &[u8], key: &[u8]) -> FindDAGResult<Vec<u8>> {
        if encrypted_data.len() < self.config.nonce_size {
            return Err(FindDAGError::DecryptionFailed("Invalid encrypted data".to_string()));
        }
        
        let cipher = ChaCha20Poly1305::new_from_slice(key)?;
        let nonce = &encrypted_data[..self.config.nonce_size];
        let ciphertext = &encrypted_data[self.config.nonce_size..];
        
        let plaintext = cipher.decrypt(nonce, ciphertext)?;
        Ok(plaintext)
    }

    /// Encrypt with XChaCha20-Poly1305
    async fn encrypt_xchacha20poly1305(&self, data: &[u8], key: &[u8]) -> FindDAGResult<Vec<u8>> {
        // TODO: Implement XChaCha20-Poly1305 encryption
        Ok(data.to_vec())
    }

    /// Decrypt with XChaCha20-Poly1305
    async fn decrypt_xchacha20poly1305(&self, encrypted_data: &[u8], key: &[u8]) -> FindDAGResult<Vec<u8>> {
        // TODO: Implement XChaCha20-Poly1305 decryption
        Ok(encrypted_data.to_vec())
    }

    /// Generate nonce
    fn generate_nonce(&self) -> Vec<u8> {
        let mut nonce = vec![0u8; self.config.nonce_size];
        rand::thread_rng().fill_bytes(&mut nonce);
        nonce
    }

    /// Clear key cache
    pub async fn clear_cache(&self) {
        let mut cache = self.key_cache.write().await;
        cache.clear();
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> (usize, usize) {
        let cache = self.key_cache.read().await;
        (cache.len(), self.config.cache_size_limit)
    }
} 
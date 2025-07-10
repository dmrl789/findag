//! Key management module
//! 
//! This module handles cryptographic key generation, storage, and rotation.

use findag_types::{FindDAGResult, FindDAGError};
use ed25519_dalek::{Keypair, SecretKey, PublicKey};
use x25519_dalek::{StaticSecret, PublicKey as X25519PublicKey};
use rand::{Rng, RngCore};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};

/// Key manager
pub struct KeyManager {
    /// Key pairs
    key_pairs: Arc<RwLock<HashMap<String, KeyPair>>>,
    /// Key metadata
    key_metadata: Arc<RwLock<HashMap<String, KeyMetadata>>>,
    /// Configuration
    config: KeyConfig,
}

/// Key pair
#[derive(Debug, Clone)]
pub struct KeyPair {
    /// Ed25519 keypair
    pub ed25519: Keypair,
    /// X25519 keypair
    pub x25519: (StaticSecret, X25519PublicKey),
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Expires at
    pub expires_at: Option<DateTime<Utc>>,
}

/// Key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Key ID
    pub key_id: String,
    /// Key name
    pub name: String,
    /// Key purpose
    pub purpose: KeyPurpose,
    /// Key status
    pub status: KeyStatus,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Last used
    pub last_used: Option<DateTime<Utc>>,
    /// Rotation interval in days
    pub rotation_interval_days: u32,
}

/// Key purpose
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyPurpose {
    Signing,
    Encryption,
    Authentication,
    KeyExchange,
}

/// Key status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyStatus {
    Active,
    Expired,
    Revoked,
    Pending,
}

/// Key configuration
#[derive(Debug, Clone)]
pub struct KeyConfig {
    /// Default key size
    pub default_key_size: usize,
    /// Default rotation interval in days
    pub default_rotation_interval: u32,
    /// Enable automatic key rotation
    pub enable_auto_rotation: bool,
    /// Key backup enabled
    pub enable_key_backup: bool,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            default_key_size: 32,
            default_rotation_interval: 90,
            enable_auto_rotation: true,
            enable_key_backup: true,
        }
    }
}

impl KeyManager {
    /// Create a new key manager
    pub fn new(config: KeyConfig) -> Self {
        Self {
            key_pairs: Arc::new(RwLock::new(HashMap::new())),
            key_metadata: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Generate new key pair
    pub async fn generate_key_pair(
        &self,
        name: &str,
        purpose: KeyPurpose,
    ) -> FindDAGResult<String> {
        // Generate Ed25519 keypair
        let ed25519_keypair = Keypair::generate(&mut rand::thread_rng());
        
        // Generate X25519 keypair
        let x25519_secret = StaticSecret::random_from_rng(&mut rand::thread_rng());
        let x25519_public = X25519PublicKey::from(&x25519_secret);
        
        // Create key pair
        let key_pair = KeyPair {
            ed25519: ed25519_keypair,
            x25519: (x25519_secret, x25519_public),
            created_at: Utc::now(),
            expires_at: Some(Utc::now() + chrono::Duration::days(self.config.default_rotation_interval as i64)),
        };
        
        // Generate key ID
        let key_id = uuid::Uuid::new_v4().to_string();
        
        // Store key pair
        {
            let mut key_pairs = self.key_pairs.write().await;
            key_pairs.insert(key_id.clone(), key_pair);
        }
        
        // Create metadata
        let metadata = KeyMetadata {
            key_id: key_id.clone(),
            name: name.to_string(),
            purpose,
            status: KeyStatus::Active,
            created_at: Utc::now(),
            last_used: None,
            rotation_interval_days: self.config.default_rotation_interval,
        };
        
        // Store metadata
        {
            let mut key_metadata = self.key_metadata.write().await;
            key_metadata.insert(key_id.clone(), metadata);
        }
        
        info!("Generated key pair: {}", key_id);
        
        Ok(key_id)
    }

    /// Get key pair
    pub async fn get_key_pair(&self, key_id: &str) -> FindDAGResult<Option<KeyPair>> {
        let key_pairs = self.key_pairs.read().await;
        Ok(key_pairs.get(key_id).cloned())
    }

    /// Get public key
    pub async fn get_public_key(&self, key_id: &str) -> FindDAGResult<Option<PublicKey>> {
        if let Some(key_pair) = self.get_key_pair(key_id).await? {
            Ok(Some(key_pair.ed25519.public))
        } else {
            Ok(None)
        }
    }

    /// Sign data
    pub async fn sign_data(&self, key_id: &str, data: &[u8]) -> FindDAGResult<Vec<u8>> {
        if let Some(key_pair) = self.get_key_pair(key_id).await? {
            let signature = key_pair.ed25519.sign(data);
            
            // Update last used
            self.update_last_used(key_id).await;
            
            Ok(signature.to_bytes().to_vec())
        } else {
            Err(FindDAGError::KeyNotFound(key_id.to_string()))
        }
    }

    /// Verify signature
    pub async fn verify_signature(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> FindDAGResult<bool> {
        if let Some(key_pair) = self.get_key_pair(key_id).await? {
            let signature = ed25519_dalek::Signature::from_bytes(signature)?;
            Ok(key_pair.ed25519.public.verify(data, &signature).is_ok())
        } else {
            Ok(false)
        }
    }

    /// Encrypt data with X25519
    pub async fn encrypt_data(&self, key_id: &str, data: &[u8]) -> FindDAGResult<Vec<u8>> {
        if let Some(key_pair) = self.get_key_pair(key_id).await? {
            // TODO: Implement X25519 encryption
            Ok(data.to_vec())
        } else {
            Err(FindDAGError::KeyNotFound(key_id.to_string()))
        }
    }

    /// Decrypt data with X25519
    pub async fn decrypt_data(&self, key_id: &str, encrypted_data: &[u8]) -> FindDAGResult<Vec<u8>> {
        if let Some(key_pair) = self.get_key_pair(key_id).await? {
            // TODO: Implement X25519 decryption
            Ok(encrypted_data.to_vec())
        } else {
            Err(FindDAGError::KeyNotFound(key_id.to_string()))
        }
    }

    /// Rotate key
    pub async fn rotate_key(&self, key_id: &str) -> FindDAGResult<String> {
        // Get current metadata
        let metadata = {
            let key_metadata = self.key_metadata.read().await;
            key_metadata.get(key_id).cloned()
        };
        
        if let Some(metadata) = metadata {
            // Generate new key pair
            let new_key_id = self.generate_key_pair(&metadata.name, metadata.purpose).await?;
            
            // Mark old key as expired
            {
                let mut key_metadata = self.key_metadata.write().await;
                if let Some(old_metadata) = key_metadata.get_mut(key_id) {
                    old_metadata.status = KeyStatus::Expired;
                }
            }
            
            info!("Rotated key: {} -> {}", key_id, new_key_id);
            
            Ok(new_key_id)
        } else {
            Err(FindDAGError::KeyNotFound(key_id.to_string()))
        }
    }

    /// Revoke key
    pub async fn revoke_key(&self, key_id: &str) -> FindDAGResult<()> {
        {
            let mut key_metadata = self.key_metadata.write().await;
            if let Some(metadata) = key_metadata.get_mut(key_id) {
                metadata.status = KeyStatus::Revoked;
            }
        }
        
        info!("Revoked key: {}", key_id);
        
        Ok(())
    }

    /// List all keys
    pub async fn list_keys(&self) -> FindDAGResult<Vec<KeyMetadata>> {
        let key_metadata = self.key_metadata.read().await;
        Ok(key_metadata.values().cloned().collect())
    }

    /// Update last used timestamp
    async fn update_last_used(&self, key_id: &str) {
        let mut key_metadata = self.key_metadata.write().await;
        if let Some(metadata) = key_metadata.get_mut(key_id) {
            metadata.last_used = Some(Utc::now());
        }
    }
} 
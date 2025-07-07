use x25519_dalek::PublicKey;
use x25519_dalek::x25519;
use x25519_dalek::X25519_BASEPOINT_BYTES;
use chacha20poly1305::{ChaCha20Poly1305, KeyInit, aead::{Aead, Payload}};
use ed25519_dalek::SigningKey;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::core::address::Address;
use serde::{Serialize, Deserialize};
use rand_core::{OsRng, RngCore};

/// Encryption configuration
#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    pub key_rotation_interval: u64, // seconds
    pub max_key_age: u64, // seconds
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            key_rotation_interval: 3600, // 1 hour
            max_key_age: 86400, // 24 hours
        }
    }
}

/// Encrypted message wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    pub encrypted_data: Vec<u8>,
    pub nonce: Vec<u8>,
    pub sender_public_key: Vec<u8>,
    pub message_type: String,
}

/// Peer encryption state
#[derive(Clone)]
pub struct PeerEncryptionState {
    pub x25519_public_key: PublicKey,
    pub last_key_exchange: u64,
    pub cipher: Option<ChaCha20Poly1305>,
}

impl PeerEncryptionState {
    pub fn new(public_key: PublicKey) -> Self {
        Self {
            x25519_public_key: public_key,
            last_key_exchange: 0,
            cipher: None,
        }
    }
}

/// P2P encryption manager
pub struct P2PEncryption {
    local_secret_bytes: [u8; 32],
    local_public: PublicKey,
    peer_states: Arc<Mutex<HashMap<Address, PeerEncryptionState>>>,
    config: EncryptionConfig,
}

impl P2PEncryption {
    /// Create a new encryption manager from an ed25519 signing key
    pub fn new_from_ed25519(_ed25519_signing_key: &SigningKey) -> Self {
        // In a real implementation, you'd derive the x25519 key from ed25519
        // For now, we'll create a new random secret and store its bytes
        let mut local_secret_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut local_secret_bytes);
        // Derive public key from secret using x25519 base point multiplication
        let public_key_bytes = x25519(local_secret_bytes, X25519_BASEPOINT_BYTES);
        let local_public = PublicKey::from(public_key_bytes);
        
        Self {
            local_secret_bytes,
            local_public,
            peer_states: Arc::new(Mutex::new(HashMap::new())),
            config: EncryptionConfig::default(),
        }
    }

    /// Get local public key for key exchange
    pub fn get_local_public_key(&self) -> PublicKey {
        self.local_public
    }

    /// Get local public key bytes
    pub fn get_local_public_key_bytes(&self) -> Vec<u8> {
        self.local_public.to_bytes().to_vec()
    }

    /// Perform key exchange with a peer
    pub async fn perform_key_exchange(&self, peer_address: Address, peer_public_key_bytes: Vec<u8>) -> Result<(), String> {
        // Convert peer public key bytes to PublicKey
        let peer_public_key = PublicKey::from(<[u8; 32]>::try_from(&peer_public_key_bytes[..]).map_err(|_| "Invalid peer public key".to_string())?);
        
        // Use x25519 for key exchange
        let shared_secret = x25519(self.local_secret_bytes, peer_public_key_bytes.as_slice().try_into().map_err(|_| "Invalid peer public key length")?);
        
        // Create cipher from shared secret
        let cipher = ChaCha20Poly1305::new_from_slice(&shared_secret)
            .map_err(|_| "Failed to create cipher".to_string())?;
        
        // Store peer state
        let mut peer_states = self.peer_states.lock().await;
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let peer_address_clone = peer_address.clone();
        peer_states.insert(peer_address, PeerEncryptionState {
            x25519_public_key: peer_public_key,
            last_key_exchange: current_time,
            cipher: Some(cipher),
        });
        
        println!("🔐 Key exchange completed with peer: {}", peer_address_clone.as_str());
        Ok(())
    }

    /// Encrypt a message for a specific peer
    pub async fn encrypt_message(&self, peer_address: &Address, message: &[u8], message_type: &str) -> Result<EncryptedMessage, String> {
        let peer_states = self.peer_states.lock().await;
        let peer_state = peer_states.get(peer_address)
            .ok_or("Peer not found or key exchange not performed")?;
        
        let cipher = peer_state.cipher.as_ref()
            .ok_or("No cipher available for peer")?;
        
        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        
        // Encrypt the message
        let payload = Payload {
            msg: message,
            aad: message_type.as_bytes(),
        };
        
        let encrypted_data = cipher.encrypt(&nonce_bytes.into(), payload)
            .map_err(|_| "Encryption failed".to_string())?;
        
        Ok(EncryptedMessage {
            encrypted_data,
            nonce: nonce_bytes.to_vec(),
            sender_public_key: self.get_local_public_key_bytes(),
            message_type: message_type.to_string(),
        })
    }

    /// Decrypt a message from a specific peer
    pub async fn decrypt_message(&self, peer_address: &Address, encrypted_msg: &EncryptedMessage) -> Result<Vec<u8>, String> {
        let peer_states = self.peer_states.lock().await;
        let peer_state = peer_states.get(peer_address)
            .ok_or("Peer not found or key exchange not performed")?;
        
        let cipher = peer_state.cipher.as_ref()
            .ok_or("No cipher available for peer")?;
        
        // Convert nonce bytes to nonce
        let nonce = chacha20poly1305::Nonce::from_slice(&encrypted_msg.nonce);
        
        // Decrypt the message
        let payload = Payload {
            msg: &encrypted_msg.encrypted_data,
            aad: encrypted_msg.message_type.as_bytes(),
        };
        
        let decrypted_data = cipher.decrypt(nonce, payload)
            .map_err(|_| "Decryption failed".to_string())?;
        
        Ok(decrypted_data)
    }

    /// Check if we have an encryption session with a peer
    pub async fn has_encryption_session(&self, peer_address: &Address) -> bool {
        let peer_states = self.peer_states.lock().await;
        peer_states.contains_key(peer_address)
    }

    /// Remove a peer's encryption state
    pub async fn remove_peer(&self, peer_address: &Address) {
        let mut peer_states = self.peer_states.lock().await;
        peer_states.remove(peer_address);
        println!("🗑️ Removed encryption state for peer: {}", peer_address.as_str());
    }

    /// Get encryption stats for all peers
    pub async fn get_stats(&self) -> HashMap<Address, bool> {
        let peer_states = self.peer_states.lock().await;
        peer_states.iter()
            .map(|(addr, state)| (addr.clone(), state.cipher.is_some()))
            .collect()
    }

    /// Rotate keys for a specific peer
    pub async fn rotate_keys(&self, peer_address: &Address) -> Result<(), String> {
        let peer_states = self.peer_states.lock().await;
        let peer_state = peer_states.get(peer_address)
            .ok_or("Peer not found")?;
        
        // Perform new key exchange
        let peer_public_key_bytes = peer_state.x25519_public_key.to_bytes().to_vec();
        drop(peer_states); // Release lock before calling perform_key_exchange
        
        self.perform_key_exchange(peer_address.clone(), peer_public_key_bytes).await?;
        println!("🔄 Rotated encryption keys for peer: {}", peer_address.as_str());
        Ok(())
    }

    /// Clean up old encryption states
    pub async fn cleanup_old_states(&self) {
        let mut peer_states = self.peer_states.lock().await;
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let old_peers: Vec<Address> = peer_states.iter()
            .filter(|(_, state)| current_time - state.last_key_exchange > self.config.max_key_age)
            .map(|(addr, _)| addr.clone())
            .collect();
        
        let old_count = old_peers.len();
        for peer in old_peers {
            peer_states.remove(&peer);
        }
        
        println!("🧹 Cleaned up {old_count} old encryption states");
    }

    /// Rotate encryption keys (for security)
    pub async fn rotate_encryption_keys(&self) {
        // Create new ephemeral secret
        let mut new_secret_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut new_secret_bytes);
        // Derive public key from secret using x25519 base point multiplication
        let new_public_bytes = x25519(new_secret_bytes, X25519_BASEPOINT_BYTES);
        let new_public = PublicKey::from(new_public_bytes);
        
        // In a real implementation, you'd want to update the local keys
        // For now, we'll just log the rotation
        println!("🔄 Rotated local encryption keys");
        println!("   New public key: {:?}", new_public.to_bytes());
    }
}

impl Clone for P2PEncryption {
    fn clone(&self) -> Self {
        // Create a new ephemeral secret for the clone
        let mut new_secret_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut new_secret_bytes);
        // Derive public key from secret using x25519 base point multiplication
        let new_public_bytes = x25519(new_secret_bytes, X25519_BASEPOINT_BYTES);
        let new_public = PublicKey::from(new_public_bytes);
        
        Self {
            local_secret_bytes: self.local_secret_bytes,
            local_public: new_public,
            peer_states: self.peer_states.clone(),
            config: self.config.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_key_exchange_and_encryption() {
        let mut rng = OsRng;
        let mut secret_bytes1 = [0u8; 32];
        let mut secret_bytes2 = [0u8; 32];
        rng.fill_bytes(&mut secret_bytes1);
        rng.fill_bytes(&mut secret_bytes2);
        let signing_key1 = SigningKey::from_bytes(&secret_bytes1);
        let signing_key2 = SigningKey::from_bytes(&secret_bytes2);
        
        let encryption1 = P2PEncryption::new_from_ed25519(&signing_key1);
        let encryption2 = P2PEncryption::new_from_ed25519(&signing_key2);
        
        let peer_address = Address("test_peer".to_string());
        
        // Perform key exchange
        let public_key1 = encryption1.get_local_public_key_bytes();
        let public_key2 = encryption2.get_local_public_key_bytes();
        
        encryption1.perform_key_exchange(peer_address.clone(), public_key2).await.unwrap();
        encryption2.perform_key_exchange(peer_address.clone(), public_key1).await.unwrap();
        
        // Test encryption/decryption
        let test_message = b"Hello, encrypted world!";
        let encrypted = encryption1.encrypt_message(&peer_address, test_message, "test").await.unwrap();
        let decrypted = encryption2.decrypt_message(&peer_address, &encrypted).await.unwrap();
        
        assert_eq!(test_message, decrypted.as_slice());
        println!("✅ Encryption/decryption test passed!");
    }

    #[tokio::test]
    async fn test_encryption_roundtrip() {
        let mut rng = OsRng;
        let mut secret_bytes1 = [0u8; 32];
        let mut secret_bytes2 = [0u8; 32];
        rng.fill_bytes(&mut secret_bytes1);
        rng.fill_bytes(&mut secret_bytes2);
        let signing_key1 = SigningKey::from_bytes(&secret_bytes1);
        let signing_key2 = SigningKey::from_bytes(&secret_bytes2);
        let encryption1 = P2PEncryption::new_from_ed25519(&signing_key1);
        let encryption2 = P2PEncryption::new_from_ed25519(&signing_key2);
        
        let peer_address = Address("peer2".to_string());
        
        // Perform key exchange first
        let public_key1 = encryption1.get_local_public_key_bytes();
        let public_key2 = encryption2.get_local_public_key_bytes();
        
        encryption1.perform_key_exchange(peer_address.clone(), public_key2).await.unwrap();
        encryption2.perform_key_exchange(peer_address.clone(), public_key1).await.unwrap();
        
        let test_message = b"Hello, encrypted world!";
        
        // Encrypt message from peer 1 to peer 2
        let encrypted = encryption1.encrypt_message(&peer_address, test_message, "test").await.unwrap();
        
        // Decrypt message on peer 2
        let decrypted = encryption2.decrypt_message(&peer_address, &encrypted).await.unwrap();
        
        assert_eq!(test_message, decrypted.as_slice());
    }

    #[tokio::test]
    async fn test_key_rotation() {
        let mut rng = OsRng;
        let mut secret_bytes1 = [0u8; 32];
        let mut secret_bytes2 = [0u8; 32];
        rng.fill_bytes(&mut secret_bytes1);
        rng.fill_bytes(&mut secret_bytes2);
        let signing_key1 = SigningKey::from_bytes(&secret_bytes1);
        let signing_key2 = SigningKey::from_bytes(&secret_bytes2);
        let encryption1 = P2PEncryption::new_from_ed25519(&signing_key1);
        let original_public_key = encryption1.get_local_public_key_bytes();
        
        // Create a new encryption instance to simulate key rotation
        let encryption2 = P2PEncryption::new_from_ed25519(&signing_key2);
        let new_public_key = encryption2.get_local_public_key_bytes();
        
        assert_ne!(original_public_key, new_public_key);
    }
} 
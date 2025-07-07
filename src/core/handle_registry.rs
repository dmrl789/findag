use ed25519_dalek::{SigningKey, Signature, Signer, VerifyingKey, Verifier};
use libp2p_identity::PublicKey;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};
use hex;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandleRecord {
    pub handle: String,
    pub parent: Option<String>,
    pub public_key: VerifyingKey,
    pub key_history: Vec<KeyHistory>,
    pub metadata: Option<serde_json::Value>,
    pub registered_at: DateTime<Utc>,
    pub revoked: bool,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revocation_reason: Option<String>,
    pub children: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyHistory {
    pub public_key: VerifyingKey,
    pub rotated_at: DateTime<Utc>,
    pub rotated_by: String,
}

#[derive(Default)]
pub struct HandleRegistry {
    pub handles: HashMap<String, HandleRecord>,
    pub pubkey_to_handle: Vec<(VerifyingKey, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterSubhandleInstruction {
    pub handle: String,
    pub parent: String,
    pub new_pubkey: String, // base64
    pub metadata: Option<serde_json::Value>,
    pub timestamp: String,
    pub parent_signature: String, // base64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateKeyInstruction {
    pub handle: String,
    pub new_pubkey: String, // base64
    pub timestamp: String,
    pub signature: String, // base64, signed by current key
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeHandleInstruction {
    pub handle: String,
    pub reason: String,
    pub timestamp: String,
    pub parent_signature: String, // base64, signed by parent
}

impl HandleRegistry {
    /// Register a new subhandle (must be signed by parent)
    pub fn register_subhandle(
        &mut self,
        instr: &RegisterSubhandleInstruction,
    ) -> Result<(), String> {
        // 1. Check parent exists
        let parent_record = self.handles.get(&instr.parent)
            .ok_or("Parent handle does not exist")?;
        if parent_record.revoked {
            return Err("Parent handle is revoked".to_string());
        }

        // 2. Check handle doesn't already exist
        if self.handles.contains_key(&instr.handle) {
            return Err("Handle already exists".to_string());
        }

        // 3. Verify parent signature
        let payload = Self::subhandle_payload_to_sign(instr);
        let parent_pubkey = parent_record.public_key;
        let sig_bytes = STANDARD.decode(&instr.parent_signature)
            .map_err(|_| "Invalid base64 signature")?;
        let signature = Signature::from_bytes(&sig_bytes.try_into().map_err(|_| "Invalid signature length")?);
        parent_pubkey.verify(payload.as_bytes(), &signature)
            .map_err(|_| "Parent signature verification failed")?;

        // 4. Parse new pubkey
        let new_pubkey_bytes = STANDARD.decode(&instr.new_pubkey)
            .map_err(|_| "Invalid base64 pubkey")?;
        let new_pubkey = PublicKey::try_decode_protobuf(&new_pubkey_bytes)
            .map_err(|_| "Invalid pubkey bytes")?;
        
        // Convert libp2p_identity::PublicKey to ed25519_dalek::VerifyingKey
        let new_pubkey_ed25519 = VerifyingKey::from_bytes(&new_pubkey_bytes.try_into().map_err(|_| "Invalid pubkey length")?)
            .map_err(|_| "Invalid ed25519 public key format")?;

        // 5. Check pubkey not already in use
        if self.pubkey_to_handle.iter().any(|&(pubkey, _)| pubkey == new_pubkey_ed25519) {
            return Err("Public key already in use by another handle".to_string());
        }

        // 6. Parse timestamp
        let now = DateTime::parse_from_rfc3339(&instr.timestamp)
            .map_err(|_| "Invalid timestamp")?
            .with_timezone(&Utc);

        // 7. Create record
        let record = HandleRecord {
            handle: instr.handle.clone(),
            parent: Some(instr.parent.clone()),
            public_key: new_pubkey_ed25519,
            key_history: vec![KeyHistory {
                public_key: new_pubkey_ed25519,
                rotated_at: now,
                rotated_by: instr.parent.clone(),
            }],
            metadata: instr.metadata.clone(),
            registered_at: now,
            revoked: false,
            revoked_at: None,
            revocation_reason: None,
            children: vec![],
        };

        // 8. Insert record and update indexes
        self.handles.insert(instr.handle.clone(), record);
        self.pubkey_to_handle.push((new_pubkey_ed25519, instr.handle.clone()));
        
        // 9. Update parent's children
        let parent = self.handles.get_mut(&instr.parent).unwrap();
        parent.children.push(instr.handle.clone());
        
        Ok(())
    }

    /// Rotate key for a handle (must be signed by current key)
    pub fn rotate_key(
        &mut self,
        instr: &RotateKeyInstruction,
    ) -> Result<(), String> {
        // 1. Check handle exists and not revoked
        let record = self.handles.get_mut(&instr.handle)
            .ok_or("Handle does not exist")?;
        if record.revoked {
            return Err("Handle is revoked".to_string());
        }

        // 2. Verify current key signature
        let payload = Self::rotate_key_payload_to_sign(instr);
        let current_pubkey = record.public_key;
        let sig_bytes = STANDARD.decode(&instr.signature)
            .map_err(|_| "Invalid base64 signature")?;
        let signature = Signature::from_bytes(&sig_bytes.try_into().map_err(|_| "Invalid signature length")?);
        current_pubkey.verify(payload.as_bytes(), &signature)
            .map_err(|_| "Current key signature verification failed")?;

        // 3. Parse new pubkey
        let new_pubkey_bytes = STANDARD.decode(&instr.new_pubkey)
            .map_err(|_| "Invalid base64 pubkey")?;
        let new_pubkey = PublicKey::try_decode_protobuf(&new_pubkey_bytes)
            .map_err(|_| "Invalid pubkey bytes")?;
        
        // Convert libp2p_identity::PublicKey to ed25519_dalek::VerifyingKey
        let new_pubkey_ed25519 = VerifyingKey::from_bytes(&new_pubkey_bytes.try_into().map_err(|_| "Invalid pubkey length")?)
            .map_err(|_| "Invalid ed25519 public key format")?;
        
        // 5. Check pubkey not already in use
        if self.pubkey_to_handle.iter().any(|&(pubkey, _)| pubkey == new_pubkey_ed25519) {
            return Err("Public key already in use by another handle".to_string());
        }
        
        // 6. Update record
        let now = Utc::now();
        record.key_history.push(KeyHistory {
            public_key: record.public_key,
            rotated_at: now,
            rotated_by: instr.handle.clone(),
        });
        
        // 7. Update indexes
        self.pubkey_to_handle.push((new_pubkey_ed25519, instr.handle.clone()));
        
        // 8. Update record
        record.public_key = new_pubkey_ed25519;

        Ok(())
    }

    /// Revoke a handle (must be signed by parent)
    pub fn revoke_handle(
        &mut self,
        instr: &RevokeHandleInstruction,
    ) -> Result<(), String> {
        // 1. Check handle exists and get parent handle
        let parent_handle = {
            let record = self.handles.get(&instr.handle)
                .ok_or("Handle does not exist")?;
            if record.revoked {
                return Err("Handle already revoked".to_string());
            }
            record.parent.clone().ok_or("Cannot revoke root handle")?
        };

        // 2. Verify parent signature
        let parent_record = self.handles.get(&parent_handle)
            .ok_or("Parent handle does not exist")?;
        if parent_record.revoked {
            return Err("Parent handle is revoked".to_string());
        }

        let payload = Self::revoke_handle_payload_to_sign(instr);
        let parent_pubkey = parent_record.public_key;
        let sig_bytes = STANDARD.decode(&instr.parent_signature)
            .map_err(|_| "Invalid base64 signature")?;
        let signature = Signature::from_bytes(&sig_bytes.try_into().map_err(|_| "Invalid signature length")?);
        parent_pubkey.verify(payload.as_bytes(), &signature)
            .map_err(|_| "Parent signature verification failed")?;

        // 3. Parse timestamp
        let now = DateTime::parse_from_rfc3339(&instr.timestamp)
            .map_err(|_| "Invalid timestamp")?
            .with_timezone(&Utc);

        // 4. Update handle record
        if let Some(record) = self.handles.get_mut(&instr.handle) {
            record.revoked = true;
            record.revoked_at = Some(now);
            record.revocation_reason = Some(instr.reason.clone());
            
            // Remove from pubkey index
            self.pubkey_to_handle.retain(|&(pubkey, _)| pubkey != record.public_key);
        }

        // 5. Remove from parent's children
        if let Some(parent) = self.handles.get_mut(&parent_handle) {
            parent.children.retain(|h| h != &instr.handle);
        }

        Ok(())
    }

    /// Lookup handle info
    pub fn resolve(&self, handle: &str) -> Option<&HandleRecord> {
        self.handles.get(handle)
    }

    /// Lookup by public key
    pub fn resolve_by_pubkey(&self, pubkey: &VerifyingKey) -> Option<&HandleRecord> {
        self.pubkey_to_handle.iter().find_map(|(pk, handle)| {
            if pk == pubkey {
                self.handles.get(handle)
            } else {
                None
            }
        })
    }

    /// Get all children of a handle
    pub fn get_children(&self, handle: &str) -> Vec<&HandleRecord> {
        self.handles.get(handle)
            .map(|record| record.children.iter()
                .filter_map(|child| self.handles.get(child))
                .collect())
            .unwrap_or_default()
    }

    /// Check if handle is valid (exists and not revoked)
    pub fn is_valid(&self, handle: &str) -> bool {
        self.handles.get(handle)
            .map(|record| !record.revoked)
            .unwrap_or(false)
    }

    // Payload generation methods for signing

    pub fn subhandle_payload_to_sign(instr: &RegisterSubhandleInstruction) -> String {
        format!(
            "register_subhandle|{}|{}|{}|{}|{}",
            instr.handle,
            instr.parent,
            instr.new_pubkey,
            instr.timestamp,
            instr.metadata.as_ref().map(|m| m.to_string()).unwrap_or_default()
        )
    }

    pub fn rotate_key_payload_to_sign(instr: &RotateKeyInstruction) -> String {
        format!(
            "rotate_key|{}|{}|{}",
            instr.handle,
            instr.new_pubkey,
            instr.timestamp
        )
    }

    pub fn revoke_handle_payload_to_sign(instr: &RevokeHandleInstruction) -> String {
        format!(
            "revoke_handle|{}|{}|{}",
            instr.handle,
            instr.reason,
            instr.timestamp
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Keypair, Signer};
    use rand::rngs::OsRng;
    use serde_json::json;

    #[test]
    fn test_register_subhandle() {
        let mut registry = HandleRegistry::default();
        
        // Create parent keypair
        let parent_keypair = Keypair::generate(&mut OsRng);
        let parent_handle = "@hsbc.london.fd".to_string();
        
        // Register parent (simplified, no signature for test)
        let parent_record = HandleRecord {
            handle: parent_handle.clone(),
            parent: None,
            public_key: parent_keypair.public,
            key_history: vec![],
            metadata: None,
            registered_at: Utc::now(),
            revoked: false,
            revoked_at: None,
            revocation_reason: None,
            children: vec![],
        };
        registry.handles.insert(parent_handle.clone(), parent_record);
        registry.pubkey_to_handle.push((parent_keypair.public, parent_handle.clone()));

        // Create subhandle instruction
        let sub_keypair = Keypair::generate(&mut OsRng);
        let sub_handle = "@trading.hsbc.london.fd".to_string();
        let timestamp = Utc::now().to_rfc3339();
        
        let mut instr = RegisterSubhandleInstruction {
            handle: sub_handle.clone(),
            parent: parent_handle.clone(),
            new_pubkey: STANDARD.encode(sub_keypair.public.to_bytes()),
            metadata: Some(json!({"role": "trading desk"})),
            timestamp: timestamp.clone(),
            parent_signature: "".to_string(),
        };

        // Sign with parent key
        let payload = HandleRegistry::subhandle_payload_to_sign(&instr);
        let sig = parent_keypair.sign(payload.as_bytes());
        instr.parent_signature = STANDARD.encode(sig.to_bytes());

        // Register subhandle
        let result = registry.register_subhandle(&instr);
        assert!(result.is_ok());

        // Verify registration
        let record = registry.resolve(&sub_handle);
        assert!(record.is_some());
        let record = record.unwrap();
        assert_eq!(record.handle, sub_handle);
        assert_eq!(record.parent, Some(parent_handle));
        assert_eq!(record.public_key, sub_keypair.public);
        assert!(!record.revoked);
    }
} 
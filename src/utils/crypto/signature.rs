use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

pub struct SignatureManager {
    keys: Arc<RwLock<HashMap<String, SigningKey>>>,
}

impl SignatureManager {
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn generate_key_pair(&self, id: String) -> Result<VerifyingKey, String> {
        let mut keys = self.keys.write().await;
        let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
        let verifying_key = signing_key.verifying_key();
        keys.insert(id, signing_key);
        Ok(verifying_key)
    }

    pub async fn sign(&self, id: &str, message: &[u8]) -> Result<Signature, String> {
        let keys = self.keys.read().await;
        let signing_key = keys.get(id)
            .ok_or_else(|| "Key not found".to_string())?;
        Ok(signing_key.sign(message))
    }

    pub async fn verify(&self, verifying_key: &VerifyingKey, message: &[u8], signature: &Signature) -> Result<(), String> {
        verifying_key.verify(message, signature)
            .map_err(|e| e.to_string())
    }

    pub async fn remove_key(&self, id: &str) -> Result<(), String> {
        let mut keys = self.keys.write().await;
        keys.remove(id)
            .ok_or_else(|| "Key not found".to_string())?;
        Ok(())
    }
}

pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> Signature {
    signing_key.sign(message)
}

pub fn verify_signature(verifying_key: &VerifyingKey, message: &[u8], signature: &Signature) -> bool {
    verifying_key.verify(message, signature).is_ok()
} 
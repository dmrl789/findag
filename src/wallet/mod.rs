use std::sync::{Arc, Mutex};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use bip39::{Mnemonic, Language};
use serde::{Serialize, Deserialize};
// use crate::config::WalletConfig;
use std::path::Path;
use std::fs;
use std::io::{self, Read, Write};
use std::collections::HashMap;
use tokio::sync::Mutex as TokioMutex;
use crate::types::{Address, RecoveryShare};
use serde_json;
use crate::types::address::FinDagKeypair;

// pub use crate::config::WalletConfig;

#[derive(Debug, thiserror::Error)]
pub enum WalletError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Deserialization error: {0}")]
    DeserializationError(serde_json::Error),
    #[error("Invalid key length")]
    InvalidKeyLength,
    #[error("Invalid key format")]
    InvalidKeyFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub keypair: FinDagKeypair,
    // pub config: WalletConfig,
    pub recovery_data: Option<Vec<RecoveryShare>>,
    pub addressbook: HashMap<String, String>,
}

impl Wallet {
    pub fn new(/*config: WalletConfig*/) -> Result<Self, WalletError> {
        let keypair = FinDagKeypair::generate().map_err(|e| WalletError::InvalidInput(e.to_string()))?;
        Ok(Self {
            keypair,
            // config,
            recovery_data: None,
            addressbook: HashMap::new(),
        })
    }

    pub fn add_address(&mut self, name: String, address: String) -> Result<(), WalletError> {
        let sanitized_name = name.trim().to_lowercase().replace(|c: char| !c.is_alphanumeric(), "_");
        if sanitized_name.is_empty() {
            return Err(WalletError::InvalidInput("Name cannot be empty".to_string()));
        }
        if !is_valid_address(&address) {
            return Err(WalletError::InvalidInput("Invalid address format".to_string()));
        }
        self.addressbook.insert(sanitized_name, address);
        Ok(())
    }

    pub fn get_address(&self, name: &str) -> Option<&String> {
        let sanitized_name = name.trim().to_lowercase().replace(|c: char| !c.is_alphanumeric(), "_");
        self.addressbook.get(&sanitized_name)
    }

    pub fn remove_address(&mut self, name: &str) {
        let sanitized_name = name.trim().to_lowercase().replace(|c: char| !c.is_alphanumeric(), "_");
        self.addressbook.remove(&sanitized_name);
    }

    pub fn list_addresses(&self) -> Vec<(String, String)> {
        self.addressbook.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    pub async fn save(&self, path: &Path) -> Result<(), WalletError> {
        let data = serde_json::to_string(self).map_err(|e| WalletError::SerializationError(e))?;
        std::fs::write(path, data).map_err(WalletError::IoError)
    }

    pub async fn load(path: &Path) -> Result<Self, WalletError> {
        let data = std::fs::read_to_string(path).map_err(WalletError::IoError)?;
        serde_json::from_str(&data).map_err(|e| WalletError::DeserializationError(e))
    }

    // Synchronous wrappers for CLI compatibility
    pub fn save_to_file(&self, path: &Path) -> Result<(), WalletError> {
        futures::executor::block_on(self.save(path))
    }
    pub fn load_from_file(path: &Path) -> Result<Self, WalletError> {
        futures::executor::block_on(Self::load(path))
    }

    pub fn get_mnemonic(&self) -> Option<String> {
        None // Stub: implement if you add mnemonic support
    }

    pub async fn encrypt_private_key(&self, _password: &str) -> Result<String, WalletError> {
        Ok("encrypted_key_stub".to_string())
    }

    pub async fn decrypt_private_key(&mut self, _password: &str, _encrypted_key: &str) -> Result<(), WalletError> {
        Ok(())
    }
    pub async fn setup_recovery(&mut self, _threshold: u32, _holders: Vec<String>) -> Result<(), WalletError> {
        Ok(())
    }
    pub async fn recover_from_shares(&mut self, _shares: Vec<RecoveryShare>) -> Result<(), WalletError> {
        Ok(())
    }
}

fn is_valid_address(address: &str) -> bool {
    if address.len() != 44 {
        return false;
    }
    true
} 
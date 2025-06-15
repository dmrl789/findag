use std::sync::{Arc, Mutex};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use bip39::{Mnemonic, Language};
use serde::{Serialize, Deserialize};
use crate::config::WalletConfig;
use std::path::Path;
use std::fs;
use std::io::{self, Read, Write};

#[derive(Debug)]
pub enum WalletError {
    InvalidKeyLength,
    InvalidKeyFormat,
    EncryptionError(String),
    DecryptionError(String),
    RecoveryError(String),
    IoError(io::Error),
}

impl std::fmt::Display for WalletError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalletError::InvalidKeyLength => write!(f, "Invalid key length"),
            WalletError::InvalidKeyFormat => write!(f, "Invalid key format"),
            WalletError::EncryptionError(msg) => write!(f, "Encryption error: {}", msg),
            WalletError::DecryptionError(msg) => write!(f, "Decryption error: {}", msg),
            WalletError::RecoveryError(msg) => write!(f, "Recovery error: {}", msg),
            WalletError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for WalletError {}

impl From<io::Error> for WalletError {
    fn from(err: io::Error) -> Self {
        WalletError::IoError(err)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    #[serde(skip)]
    private_key: Arc<Mutex<Option<SigningKey>>>,
    private_key_bytes: Vec<u8>,
    pub config: WalletConfig,
    mnemonic: Option<String>,
    recovery_data: Option<Vec<u8>>,
}

impl Wallet {
    pub fn new(config: WalletConfig) -> Result<Self, WalletError> {
        let mut rng = OsRng;
        let private_key = SigningKey::generate(&mut rng);
        let private_key_bytes = private_key.to_bytes().to_vec();
        
        let mnemonic = if config.mnemonic_enabled {
            Some(Mnemonic::generate_in(Language::English, 24)
                .map_err(|e| WalletError::RecoveryError(e.to_string()))?
                .to_string())
        } else {
            None
        };

        Ok(Self {
            private_key: Arc::new(Mutex::new(Some(private_key))),
            private_key_bytes,
            config,
            mnemonic,
            recovery_data: None,
        })
    }

    pub fn from_bytes(private_key_bytes: Vec<u8>, config: WalletConfig) -> Result<Self, WalletError> {
        let private_key = SigningKey::from_bytes(&private_key_bytes.try_into()
            .map_err(|_| WalletError::InvalidKeyLength)?)
            .map_err(|_| WalletError::InvalidKeyFormat)?;
            
        let mnemonic = if config.mnemonic_enabled {
            Some(Mnemonic::generate_in(Language::English, 24)
                .map_err(|e| WalletError::RecoveryError(e.to_string()))?
                .to_string())
        } else {
            None
        };

        Ok(Self {
            private_key: Arc::new(Mutex::new(Some(private_key))),
            private_key_bytes,
            config,
            mnemonic,
            recovery_data: None,
        })
    }

    pub fn get_public_key(&self) -> VerifyingKey {
        let private_key = self.private_key.lock().unwrap();
        private_key.as_ref().unwrap().verifying_key()
    }

    pub fn get_private_key_bytes(&self) -> Vec<u8> {
        self.private_key_bytes.clone()
    }

    pub fn get_mnemonic(&self) -> Option<String> {
        self.mnemonic.clone()
    }

    pub fn get_config(&self) -> &WalletConfig {
        &self.config
    }

    pub async fn encrypt_private_key(&mut self, password: &str) -> Result<(), WalletError> {
        // Placeholder for encryption logic
        Ok(())
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), WalletError> {
        let data = serde_json::to_string(self)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self, WalletError> {
        let data = fs::read_to_string(path)?;
        let wallet: Wallet = serde_json::from_str(&data)?;
        Ok(wallet)
    }
} 
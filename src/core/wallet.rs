use crate::core::address::{Address, generate_address};
use crate::core::types::Transaction;
use ed25519_dalek::{SigningKey, Signer};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use argon2::{Argon2, PasswordHasher, password_hash::{rand_core::OsRng, SaltString}};
use aes_gcm::{Aes256Gcm, Key, Nonce, KeyInit};
use aes_gcm::aead::Aead;
use rand::Rng;
use hex;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use serde_json;

/// Encrypted wallet data structure
#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedWallet {
    pub version: u32,
    pub created_at: DateTime<Utc>,
    pub salt: String,
    pub encrypted_data: Vec<u8>,
    pub nonce: Vec<u8>,
    pub argon2_params: Argon2Params,
}

/// Argon2 parameters for key derivation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Argon2Params {
    pub memory_cost: u32,
    pub time_cost: u32,
    pub parallelism: u32,
}

impl Default for Argon2Params {
    fn default() -> Self {
        Self {
            memory_cost: 65536, // 64MB
            time_cost: 3,
            parallelism: 4,
        }
    }
}

/// Wallet account information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletAccount {
    pub name: String,
    pub signing_key: SigningKey,
    pub address: Address,
    pub balance: HashMap<String, u64>, // asset -> amount
    pub metadata: Option<serde_json::Value>,
}

impl WalletAccount {
    pub fn new(name: String, signing_key: SigningKey) -> Self {
        let address = Address::from_verifying_key(&signing_key.verifying_key());
        Self {
            name,
            signing_key,
            address,
            balance: HashMap::new(),
            metadata: None,
        }
    }

    pub fn sign_transaction(&self, transaction: &mut Transaction) -> Result<(), String> {
        // Create message to sign
        let mut message = Vec::new();
        message.extend_from_slice(transaction.from.as_str().as_bytes());
        message.extend_from_slice(transaction.to.as_str().as_bytes());
        message.extend_from_slice(&transaction.amount.to_be_bytes());
        message.extend_from_slice(&transaction.findag_time.to_be_bytes());
        message.extend_from_slice(&transaction.hashtimer);
        
        // Sign the message
        let signature = self.signing_key.sign(&message);
        
        // Update transaction
        transaction.signature = signature;
        transaction.public_key = self.signing_key.verifying_key();
        
        Ok(())
    }
}

/// Main wallet structure
#[derive(Serialize, Deserialize)]
pub struct Wallet {
    signing_key: SigningKey,
    address: Address,
    accounts: Vec<WalletAccount>,
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

impl Wallet {
    /// Create a new wallet with a new signing key
    pub fn new() -> Self {
        let (signing_key, address) = generate_address();
        let account = WalletAccount {
            name: "default".to_string(),
            signing_key: signing_key.clone(),
            address: address.clone(),
            balance: HashMap::new(),
            metadata: None,
        };
        
        Self {
            signing_key,
            address,
            accounts: vec![account],
        }
    }

    /// Create wallet from existing signing key
    pub fn from_signing_key(signing_key: SigningKey) -> Self {
        let address = Address::from_verifying_key(&signing_key.verifying_key());
        let account = WalletAccount {
            name: "default".to_string(),
            signing_key: signing_key.clone(),
            address: address.clone(),
            balance: HashMap::new(),
            metadata: None,
        };
        
        Self {
            signing_key,
            address,
            accounts: vec![account],
        }
    }

    /// Get the wallet address
    pub fn address(&self) -> &Address {
        &self.address
    }

    /// Get the public key as hex string
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.signing_key.verifying_key().to_bytes())
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.signing_key.sign(message).to_bytes().to_vec()
    }

    /// Get all accounts
    pub fn accounts(&self) -> &[WalletAccount] {
        &self.accounts
    }

    /// Add a new account
    pub fn add_account(&mut self, name: &str) -> Result<(), String> {
        let (new_signing_key, new_address) = generate_address();
        let account = WalletAccount {
            name: name.to_string(),
            signing_key: new_signing_key,
            address: new_address,
            balance: HashMap::new(),
            metadata: None,
        };
        
        self.accounts.push(account);
        Ok(())
    }

    /// Export private key as hex (for backup purposes)
    pub fn export_private_key(&self) -> String {
        hex::encode(self.signing_key.to_bytes())
    }

    /// Import private key from hex
    pub fn from_private_key_hex(private_key_hex: &str) -> Result<Self, String> {
        let secret_bytes = hex::decode(private_key_hex)
            .map_err(|_| "Invalid hex string")?;
        
        if secret_bytes.len() != 32 {
            return Err("Invalid private key length".to_string());
        }
        
        let signing_key = SigningKey::from_bytes(&secret_bytes.try_into().map_err(|_| "Invalid private key length")?);
        
        Ok(Self::from_signing_key(signing_key))
    }
}

/// Wallet manager for encrypted storage
pub struct WalletManager {
    wallet_path: String,
}

impl WalletManager {
    /// Create a new wallet manager
    pub fn new(wallet_path: &str) -> Self {
        Self {
            wallet_path: wallet_path.to_string(),
        }
    }

    /// Create a new encrypted wallet
    pub fn create_wallet(&self, password: &str) -> Result<Wallet, String> {
        let wallet = Wallet::new();
        self.save_wallet(&wallet, password)?;
        Ok(wallet)
    }

    /// Load wallet from encrypted file
    pub fn load_wallet(&self, password: &str) -> Result<Wallet, String> {
        if !Path::new(&self.wallet_path).exists() {
            return Err("Wallet file does not exist".to_string());
        }

        let encrypted_data = fs::read(&self.wallet_path)
            .map_err(|_| "Failed to read wallet file")?;
        
        let encrypted_wallet: EncryptedWallet = bincode::deserialize(&encrypted_data)
            .map_err(|_| "Failed to deserialize wallet data")?;
        
        self.decrypt_wallet(&encrypted_wallet, password)
    }

    /// Save wallet to encrypted file
    pub fn save_wallet(&self, wallet: &Wallet, password: &str) -> Result<(), String> {
        let encrypted_wallet = self.encrypt_wallet(wallet, password)?;
        let serialized = bincode::serialize(&encrypted_wallet)
            .map_err(|_| "Failed to serialize wallet")?;
        
        fs::write(&self.wallet_path, serialized)
            .map_err(|_| "Failed to write wallet file")?;
        
        Ok(())
    }

    /// Check if wallet exists
    pub fn wallet_exists(&self) -> bool {
        Path::new(&self.wallet_path).exists()
    }

    /// Encrypt wallet data
    fn encrypt_wallet(&self, wallet: &Wallet, password: &str) -> Result<EncryptedWallet, String> {
        // Serialize wallet data
        let wallet_data = bincode::serialize(wallet)
            .map_err(|_| "Failed to serialize wallet")?;
        
        // Generate salt for Argon2
        let salt = SaltString::generate(&mut OsRng);
        
        // Derive key from password using Argon2
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|_| "Failed to hash password")?;
        
        let hash = password_hash.hash.ok_or("Failed to get hash")?;
        let key_bytes = hash.as_bytes();
        if key_bytes.len() != 32 {
            return Err("Derived key is not 32 bytes for AES-256-GCM".to_string());
        }
        let key = Key::<Aes256Gcm>::from_slice(key_bytes);
        
        // Generate random nonce for AES-GCM
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Encrypt wallet data
        let cipher = Aes256Gcm::new(key);
        let encrypted_data = cipher.encrypt(nonce, wallet_data.as_ref())
            .map_err(|_| "Failed to encrypt wallet data")?;
        
        Ok(EncryptedWallet {
            version: 1,
            created_at: Utc::now(),
            salt: salt.to_string(),
            encrypted_data,
            nonce: nonce_bytes.to_vec(),
            argon2_params: Argon2Params::default(),
        })
    }

    /// Decrypt wallet data
    fn decrypt_wallet(&self, encrypted_wallet: &EncryptedWallet, password: &str) -> Result<Wallet, String> {
        // Parse salt
        let salt = SaltString::from_b64(&encrypted_wallet.salt)
            .map_err(|_| "Invalid salt")?;
        
        // Derive key from password
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|_| "Failed to hash password")?;
        
        let hash = password_hash.hash.ok_or("Failed to get hash")?;
        let key_bytes = hash.as_bytes();
        if key_bytes.len() != 32 {
            return Err("Derived key is not 32 bytes for AES-256-GCM".to_string());
        }
        let key = Key::<Aes256Gcm>::from_slice(key_bytes);
        
        // Decrypt wallet data
        let nonce = Nonce::from_slice(&encrypted_wallet.nonce);
        let cipher = Aes256Gcm::new(key);
        let decrypted_data = cipher.decrypt(nonce, encrypted_wallet.encrypted_data.as_ref())
            .map_err(|_| "Failed to decrypt wallet data - wrong password?")?;
        
        // Deserialize wallet
        let wallet: Wallet = bincode::deserialize(&decrypted_data)
            .map_err(|_| "Failed to deserialize wallet data")?;
        
        Ok(wallet)
    }
}

/// Utility function to prompt for password securely
pub fn prompt_password(prompt: &str) -> Result<String, String> {
    print!("{prompt}: ");
    io::stdout().flush().map_err(|_| "Failed to flush stdout")?;
    
    rpassword::read_password()
        .map_err(|_| "Failed to read password".to_string())
}

/// Utility function to prompt for password confirmation
pub fn prompt_password_confirm() -> Result<String, String> {
    let password = prompt_password("Enter password")?;
    let confirm = prompt_password("Confirm password")?;
    
    if password != confirm {
        return Err("Passwords do not match".to_string());
    }
    
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long".to_string());
    }
    
    Ok(password)
} 
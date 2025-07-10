//! Password management module
//! 
//! This module handles password hashing, validation, and policy enforcement.

use findag_types::{FindDAGResult, FindDAGError};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{rand_core::OsRng, SaltString}};
use bcrypt::{hash, verify, DEFAULT_COST};
use scrypt::{scrypt, Params};
use pbkdf2::{pbkdf2, pbkdf2_verify};
use hmac::Hmac;
use sha2::Sha256;
use rand::{Rng, RngCore};
use serde::{Serialize, Deserialize};
use tracing::{info, warn, error, debug};
use metrics::{counter, histogram};

/// Password manager
pub struct PasswordManager {
    /// Password policy
    policy: PasswordPolicy,
    /// Hash algorithm
    algorithm: HashAlgorithm,
    /// Configuration
    config: PasswordConfig,
}

/// Password policy
#[derive(Debug, Clone)]
pub struct PasswordPolicy {
    /// Minimum length
    pub min_length: usize,
    /// Maximum length
    pub max_length: usize,
    /// Require uppercase
    pub require_uppercase: bool,
    /// Require lowercase
    pub require_lowercase: bool,
    /// Require numbers
    pub require_numbers: bool,
    /// Require special characters
    pub require_special: bool,
    /// Maximum age in days
    pub max_age_days: u32,
    /// Prevent common passwords
    pub prevent_common: bool,
}

/// Hash algorithm
#[derive(Debug, Clone)]
pub enum HashAlgorithm {
    Argon2,
    Bcrypt,
    Scrypt,
    PBKDF2,
}

/// Password configuration
#[derive(Debug, Clone)]
pub struct PasswordConfig {
    /// Hash algorithm
    pub algorithm: HashAlgorithm,
    /// Argon2 memory cost
    pub argon2_memory_cost: u32,
    /// Argon2 time cost
    pub argon2_time_cost: u32,
    /// Argon2 parallelism
    pub argon2_parallelism: u32,
    /// Bcrypt cost
    pub bcrypt_cost: u32,
    /// Scrypt parameters
    pub scrypt_params: ScryptParams,
    /// PBKDF2 iterations
    pub pbkdf2_iterations: u32,
}

/// Scrypt parameters
#[derive(Debug, Clone)]
pub struct ScryptParams {
    /// Log N parameter
    pub log_n: u8,
    /// R parameter
    pub r: u32,
    /// P parameter
    pub p: u32,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 8,
            max_length: 128,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special: true,
            max_age_days: 90,
            prevent_common: true,
        }
    }
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            algorithm: HashAlgorithm::Argon2,
            argon2_memory_cost: 65536,
            argon2_time_cost: 4,
            argon2_parallelism: 1,
            bcrypt_cost: DEFAULT_COST,
            scrypt_params: ScryptParams {
                log_n: 14,
                r: 8,
                p: 1,
            },
            pbkdf2_iterations: 100000,
        }
    }
}

impl PasswordManager {
    /// Create a new password manager
    pub fn new(policy: PasswordPolicy, config: PasswordConfig) -> Self {
        Self {
            policy,
            algorithm: config.algorithm.clone(),
            config,
        }
    }

    /// Hash password
    pub async fn hash_password(&self, password: &str) -> FindDAGResult<String> {
        let start_time = std::time::Instant::now();
        
        // Validate password
        self.validate_password(password)?;
        
        // Hash password
        let hash = match self.algorithm {
            HashAlgorithm::Argon2 => {
                self.hash_argon2(password).await?
            }
            HashAlgorithm::Bcrypt => {
                self.hash_bcrypt(password).await?
            }
            HashAlgorithm::Scrypt => {
                self.hash_scrypt(password).await?
            }
            HashAlgorithm::PBKDF2 => {
                self.hash_pbkdf2(password).await?
            }
        };
        
        // Record metrics
        let latency = start_time.elapsed().as_millis() as f64;
        counter!("findag_password_hash_operations", 1);
        histogram!("findag_password_hash_latency_ms", latency);
        
        Ok(hash)
    }

    /// Verify password
    pub async fn verify_password(&self, password: &str, hash: &str) -> FindDAGResult<bool> {
        let start_time = std::time::Instant::now();
        
        // Verify password
        let valid = match self.algorithm {
            HashAlgorithm::Argon2 => {
                self.verify_argon2(password, hash).await?
            }
            HashAlgorithm::Bcrypt => {
                self.verify_bcrypt(password, hash).await?
            }
            HashAlgorithm::Scrypt => {
                self.verify_scrypt(password, hash).await?
            }
            HashAlgorithm::PBKDF2 => {
                self.verify_pbkdf2(password, hash).await?
            }
        };
        
        // Record metrics
        let latency = start_time.elapsed().as_millis() as f64;
        counter!("findag_password_verify_operations", 1);
        histogram!("findag_password_verify_latency_ms", latency);
        
        Ok(valid)
    }

    /// Validate password against policy
    pub fn validate_password(&self, password: &str) -> FindDAGResult<()> {
        if password.len() < self.policy.min_length {
            return Err(FindDAGError::ValidationError(
                format!("Password must be at least {} characters long", self.policy.min_length)
            ));
        }
        
        if password.len() > self.policy.max_length {
            return Err(FindDAGError::ValidationError(
                format!("Password must be at most {} characters long", self.policy.max_length)
            ));
        }
        
        if self.policy.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            return Err(FindDAGError::ValidationError(
                "Password must contain at least one uppercase letter".to_string()
            ));
        }
        
        if self.policy.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
            return Err(FindDAGError::ValidationError(
                "Password must contain at least one lowercase letter".to_string()
            ));
        }
        
        if self.policy.require_numbers && !password.chars().any(|c| c.is_numeric()) {
            return Err(FindDAGError::ValidationError(
                "Password must contain at least one number".to_string()
            ));
        }
        
        if self.policy.require_special && !password.chars().any(|c| !c.is_alphanumeric()) {
            return Err(FindDAGError::ValidationError(
                "Password must contain at least one special character".to_string()
            ));
        }
        
        if self.policy.prevent_common && self.is_common_password(password) {
            return Err(FindDAGError::ValidationError(
                "Password is too common".to_string()
            ));
        }
        
        Ok(())
    }

    /// Hash with Argon2
    async fn hash_argon2(&self, password: &str) -> FindDAGResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::new(
                self.config.argon2_memory_cost,
                self.config.argon2_time_cost,
                self.config.argon2_parallelism,
                None,
            )?,
        );
        
        let hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(hash.to_string())
    }

    /// Verify with Argon2
    async fn verify_argon2(&self, password: &str, hash: &str) -> FindDAGResult<bool> {
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }

    /// Hash with Bcrypt
    async fn hash_bcrypt(&self, password: &str) -> FindDAGResult<String> {
        let hash = hash(password, self.config.bcrypt_cost)?;
        Ok(hash)
    }

    /// Verify with Bcrypt
    async fn verify_bcrypt(&self, password: &str, hash: &str) -> FindDAGResult<bool> {
        Ok(verify(password, hash)?)
    }

    /// Hash with Scrypt
    async fn hash_scrypt(&self, password: &str) -> FindDAGResult<String> {
        let salt = self.generate_salt();
        let params = Params::new(
            self.config.scrypt_params.log_n,
            self.config.scrypt_params.r,
            self.config.scrypt_params.p,
        )?;
        
        let mut hash = vec![0u8; 32];
        scrypt(password.as_bytes(), &salt, &params, &mut hash)?;
        
        let result = format!("$scrypt$n={},r={},p={}${}${}",
            self.config.scrypt_params.log_n,
            self.config.scrypt_params.r,
            self.config.scrypt_params.p,
            base64::encode(&salt),
            base64::encode(&hash)
        );
        
        Ok(result)
    }

    /// Verify with Scrypt
    async fn verify_scrypt(&self, password: &str, hash: &str) -> FindDAGResult<bool> {
        // TODO: Implement scrypt verification
        Ok(true)
    }

    /// Hash with PBKDF2
    async fn hash_pbkdf2(&self, password: &str) -> FindDAGResult<String> {
        let salt = self.generate_salt();
        let mut hash = vec![0u8; 32];
        
        pbkdf2::<Hmac<Sha256>>(
            password.as_bytes(),
            &salt,
            self.config.pbkdf2_iterations,
            &mut hash,
        );
        
        let result = format!("$pbkdf2-sha256${}${}${}",
            self.config.pbkdf2_iterations,
            base64::encode(&salt),
            base64::encode(&hash)
        );
        
        Ok(result)
    }

    /// Verify with PBKDF2
    async fn verify_pbkdf2(&self, password: &str, hash: &str) -> FindDAGResult<bool> {
        // TODO: Implement PBKDF2 verification
        Ok(true)
    }

    /// Generate salt
    fn generate_salt(&self) -> Vec<u8> {
        let mut salt = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut salt);
        salt
    }

    /// Check if password is common
    fn is_common_password(&self, password: &str) -> bool {
        let common_passwords = vec![
            "password", "123456", "123456789", "qwerty", "abc123",
            "password123", "admin", "letmein", "welcome", "monkey",
        ];
        
        common_passwords.contains(&password.to_lowercase().as_str())
    }

    /// Generate random password
    pub fn generate_password(&self) -> String {
        let mut rng = rand::thread_rng();
        let length = rng.gen_range(self.policy.min_length..=self.policy.max_length);
        
        let mut password = String::new();
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:,.<>?".chars().collect();
        
        // Ensure at least one character from each required category
        if self.policy.require_lowercase {
            password.push(chars[rng.gen_range(0..26)]);
        }
        if self.policy.require_uppercase {
            password.push(chars[rng.gen_range(26..52)]);
        }
        if self.policy.require_numbers {
            password.push(chars[rng.gen_range(52..62)]);
        }
        if self.policy.require_special {
            password.push(chars[rng.gen_range(62..chars.len())]);
        }
        
        // Fill the rest randomly
        while password.len() < length {
            password.push(chars[rng.gen_range(0..chars.len())]);
        }
        
        // Shuffle the password
        let mut password_chars: Vec<char> = password.chars().collect();
        for i in (1..password_chars.len()).rev() {
            let j = rng.gen_range(0..=i);
            password_chars.swap(i, j);
        }
        
        password_chars.into_iter().collect()
    }
} 
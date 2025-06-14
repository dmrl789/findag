use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    pub mnemonic_enabled: bool,
    pub encryption_enabled: bool,
    pub backup_enabled: bool,
    pub auto_lock_timeout: u64, // in seconds
    pub max_failed_attempts: u32,
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self {
            mnemonic_enabled: true,
            encryption_enabled: true,
            backup_enabled: true,
            auto_lock_timeout: 300, // 5 minutes
            max_failed_attempts: 3,
        }
    }
}

//! Wallet type definitions
//! 
//! This module contains types related to wallet management, key management,
//! and wallet operations.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use findag_core::{Address, Hash, HashTimer, FinDAGTime};

/// Wallet type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WalletType {
    /// Standard wallet
    Standard,
    /// Hardware wallet
    Hardware,
    /// Multi-signature wallet
    MultiSig,
    /// Watch-only wallet
    WatchOnly,
}

/// Wallet status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WalletStatus {
    /// Wallet is active
    Active,
    /// Wallet is locked
    Locked,
    /// Wallet is disabled
    Disabled,
    /// Wallet is corrupted
    Corrupted,
}

/// Wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    /// Wallet ID
    pub wallet_id: String,
    /// Wallet name
    pub name: String,
    /// Wallet type
    pub wallet_type: WalletType,
    /// Wallet status
    pub status: WalletStatus,
    /// Wallet address
    pub address: Address,
    /// Public key
    pub public_key: Vec<u8>,
    /// Encrypted private key
    pub encrypted_private_key: Vec<u8>,
    /// Wallet metadata
    pub metadata: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Wallet balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalance {
    /// Asset
    pub asset: String,
    /// Balance amount
    pub amount: u64,
    /// Available balance
    pub available: u64,
    /// Locked balance
    pub locked: u64,
    /// Last update timestamp
    pub last_update: DateTime<Utc>,
}

/// Wallet transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransaction {
    /// Transaction hash
    pub transaction_hash: Hash,
    /// Transaction type
    pub transaction_type: WalletTransactionType,
    /// From address
    pub from: Address,
    /// To address
    pub to: Address,
    /// Asset
    pub asset: String,
    /// Amount
    pub amount: u64,
    /// Fee
    pub fee: u64,
    /// Transaction status
    pub status: WalletTransactionStatus,
    /// Block number
    pub block_number: Option<u64>,
    /// Transaction timestamp
    pub timestamp: FinDAGTime,
    /// Transaction metadata
    pub metadata: Option<String>,
}

/// Wallet transaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WalletTransactionType {
    /// Incoming transaction
    Incoming,
    /// Outgoing transaction
    Outgoing,
    /// Internal transaction
    Internal,
}

/// Wallet transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WalletTransactionStatus {
    /// Pending
    Pending,
    /// Confirmed
    Confirmed,
    /// Failed
    Failed,
    /// Cancelled
    Cancelled,
}

/// Key pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    /// Public key
    pub public_key: Vec<u8>,
    /// Encrypted private key
    pub encrypted_private_key: Vec<u8>,
    /// Key derivation path
    pub derivation_path: Option<String>,
    /// Key metadata
    pub metadata: Option<String>,
}

/// Wallet backup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBackup {
    /// Backup ID
    pub backup_id: String,
    /// Wallet ID
    pub wallet_id: String,
    /// Backup data (encrypted)
    pub backup_data: Vec<u8>,
    /// Backup checksum
    pub checksum: Hash,
    /// Backup timestamp
    pub timestamp: DateTime<Utc>,
    /// Backup metadata
    pub metadata: Option<String>,
}

/// Wallet configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    /// Default fee
    pub default_fee: u64,
    /// Fee asset
    pub fee_asset: String,
    /// Auto-lock timeout in seconds
    pub auto_lock_timeout_seconds: u64,
    /// Enable auto-backup
    pub enable_auto_backup: bool,
    /// Backup interval in hours
    pub backup_interval_hours: u64,
    /// Maximum transaction history
    pub max_transaction_history: usize,
    /// Enable transaction notifications
    pub enable_transaction_notifications: bool,
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self {
            default_fee: 1000,
            fee_asset: "USD".to_string(),
            auto_lock_timeout_seconds: 300, // 5 minutes
            enable_auto_backup: true,
            backup_interval_hours: 24,
            max_transaction_history: 1000,
            enable_transaction_notifications: true,
        }
    }
}

/// Wallet metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletMetrics {
    /// Total transactions
    pub total_transactions: u64,
    /// Total sent
    pub total_sent: u64,
    /// Total received
    pub total_received: u64,
    /// Total fees paid
    pub total_fees_paid: u64,
    /// Wallet uptime in seconds
    pub uptime_seconds: u64,
}

/// Wallet event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletEvent {
    /// Wallet created
    WalletCreated {
        wallet_id: String,
        address: Address,
        timestamp: FinDAGTime,
    },
    /// Wallet unlocked
    WalletUnlocked {
        wallet_id: String,
        timestamp: FinDAGTime,
    },
    /// Wallet locked
    WalletLocked {
        wallet_id: String,
        timestamp: FinDAGTime,
    },
    /// Transaction received
    TransactionReceived {
        wallet_id: String,
        transaction_hash: Hash,
        amount: u64,
        asset: String,
        timestamp: FinDAGTime,
    },
    /// Transaction sent
    TransactionSent {
        wallet_id: String,
        transaction_hash: Hash,
        amount: u64,
        asset: String,
        timestamp: FinDAGTime,
    },
    /// Balance updated
    BalanceUpdated {
        wallet_id: String,
        asset: String,
        new_balance: u64,
        timestamp: FinDAGTime,
    },
}

/// Wallet command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletCommand {
    /// Create wallet
    CreateWallet {
        name: String,
        password: String,
        wallet_type: WalletType,
    },
    /// Import wallet
    ImportWallet {
        name: String,
        backup_data: Vec<u8>,
        password: String,
    },
    /// Export wallet
    ExportWallet {
        wallet_id: String,
        password: String,
    },
    /// Unlock wallet
    UnlockWallet {
        wallet_id: String,
        password: String,
    },
    /// Lock wallet
    LockWallet {
        wallet_id: String,
    },
    /// Send transaction
    SendTransaction {
        wallet_id: String,
        to: Address,
        amount: u64,
        asset: String,
        fee: Option<u64>,
    },
    /// Get balance
    GetBalance {
        wallet_id: String,
        asset: Option<String>,
    },
    /// Get transaction history
    GetTransactionHistory {
        wallet_id: String,
        limit: Option<usize>,
        offset: Option<usize>,
    },
    /// Backup wallet
    BackupWallet {
        wallet_id: String,
        password: String,
    },
    /// Restore wallet
    RestoreWallet {
        backup_data: Vec<u8>,
        password: String,
    },
} 
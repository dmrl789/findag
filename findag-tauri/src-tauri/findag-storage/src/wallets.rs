//! Wallet storage operations
//! 
//! This module handles storage operations for wallet management.

use sled::Tree;
use findag_core::{Address};
use findag_types::{Wallet, FindDAGResult};

/// Wallet storage manager
pub struct WalletStorage {
    /// Wallets tree
    wallets_tree: Tree,
}

impl WalletStorage {
    /// Create a new wallet storage manager
    pub fn new(wallets_tree: Tree) -> Self {
        Self { wallets_tree }
    }

    /// Store a wallet
    pub fn store_wallet(&self, wallet: &Wallet) -> FindDAGResult<()> {
        let key = format!("wallet:{}", wallet.wallet_id);
        let value = bincode::serialize(wallet)?;
        
        self.wallets_tree.insert(key.as_bytes(), value)?;
        Ok(())
    }

    /// Get a wallet by ID
    pub fn get_wallet(&self, wallet_id: &str) -> FindDAGResult<Option<Wallet>> {
        let key = format!("wallet:{}", wallet_id);
        let result = self.wallets_tree.get(key.as_bytes())?;
        
        if let Some(value) = result {
            let wallet: Wallet = bincode::deserialize(&value)?;
            Ok(Some(wallet))
        } else {
            Ok(None)
        }
    }

    /// Get wallet by address
    pub fn get_wallet_by_address(&self, address: &Address) -> FindDAGResult<Option<Wallet>> {
        for result in self.wallets_tree.iter() {
            let (_, value) = result?;
            if let Ok(wallet) = bincode::deserialize::<Wallet>(&value) {
                if wallet.address == *address {
                    return Ok(Some(wallet));
                }
            }
        }
        Ok(None)
    }
} 
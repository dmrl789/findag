//! Asset storage operations
//! 
//! This module handles storage operations for blockchain assets.

use sled::Tree;
use findag_core::{Address};
use findag_types::{Asset, Balance, FindDAGResult};

/// Asset storage manager
pub struct AssetStorage {
    /// Assets tree
    assets_tree: Tree,
    /// Balances tree
    balances_tree: Tree,
}

impl AssetStorage {
    /// Create a new asset storage manager
    pub fn new(assets_tree: Tree, balances_tree: Tree) -> Self {
        Self { assets_tree, balances_tree }
    }

    /// Store an asset
    pub fn store_asset(&self, asset: &Asset) -> FindDAGResult<()> {
        let key = format!("asset:{}", asset.code);
        let value = bincode::serialize(asset)?;
        
        self.assets_tree.insert(key.as_bytes(), value)?;
        Ok(())
    }

    /// Get an asset by code
    pub fn get_asset(&self, code: &str) -> FindDAGResult<Option<Asset>> {
        let key = format!("asset:{}", code);
        let result = self.assets_tree.get(key.as_bytes())?;
        
        if let Some(value) = result {
            let asset: Asset = bincode::deserialize(&value)?;
            Ok(Some(asset))
        } else {
            Ok(None)
        }
    }

    /// Store a balance
    pub fn store_balance(&self, balance: &Balance) -> FindDAGResult<()> {
        let key = format!("balance:{}:{}", balance.owner, balance.asset);
        let value = bincode::serialize(balance)?;
        
        self.balances_tree.insert(key.as_bytes(), value)?;
        Ok(())
    }

    /// Get a balance
    pub fn get_balance(&self, owner: &Address, asset: &str) -> FindDAGResult<Option<Balance>> {
        let key = format!("balance:{}:{}", owner, asset);
        let result = self.balances_tree.get(key.as_bytes())?;
        
        if let Some(value) = result {
            let balance: Balance = bincode::deserialize(&value)?;
            Ok(Some(balance))
        } else {
            Ok(None)
        }
    }

    /// Get all balances for an owner
    pub fn get_balances_by_owner(&self, owner: &Address) -> FindDAGResult<Vec<Balance>> {
        let mut balances = Vec::new();
        let prefix = format!("balance:{}:", owner);
        
        for result in self.balances_tree.scan_prefix(prefix.as_bytes()) {
            let (_, value) = result?;
            if let Ok(balance) = bincode::deserialize::<Balance>(&value) {
                balances.push(balance);
            }
        }
        
        Ok(balances)
    }
} 
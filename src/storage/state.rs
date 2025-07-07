use sled::Db;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub balance: u64,
    pub nonce: u64,
}

pub struct StateManager {
    accounts: HashMap<String, Account>,
    shard_states: HashMap<u16, HashMap<String, u64>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            shard_states: HashMap::new(),
        }
    }

    pub fn get_balance(&self, shard_id: u16, address: &str, _asset: &str) -> u64 {
        if let Some(shard_state) = self.shard_states.get(&shard_id) {
            shard_state.get(address).copied().unwrap_or(0)
        } else {
            0
        }
    }

    pub fn set_balance(&mut self, shard_id: u16, address: &str, balance: u64) {
        let shard_state = self.shard_states.entry(shard_id).or_default();
        shard_state.insert(address.to_string(), balance);
    }

    pub fn get_account(&self, address: &str) -> Option<&Account> {
        self.accounts.get(address)
    }

    pub fn create_account(&mut self, address: String, initial_balance: u64) {
        let account = Account {
            address: address.clone(),
            balance: initial_balance,
            nonce: 0,
        };
        self.accounts.insert(address, account);
    }

    pub fn update_balance(&mut self, address: &str, new_balance: u64) {
        if let Some(account) = self.accounts.get_mut(address) {
            account.balance = new_balance;
        }
    }

    pub fn increment_nonce(&mut self, address: &str) {
        if let Some(account) = self.accounts.get_mut(address) {
            account.nonce += 1;
        }
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// State database for managing account balances and cross-shard state
pub struct StateDB {
    db: Db,
}

impl StateDB {
    pub fn new(path: &str) -> Self {
        let db = sled::open(path).expect("Failed to open sled state DB");
        Self { db }
    }

    /// Get balance for an account on a specific shard and asset
    pub fn get_balance(&self, shard_id: u16, address: &str, asset: &str) -> u64 {
        let key = format!("state:{shard_id}:{address}:{asset}");
        if let Ok(Some(value)) = self.db.get(key) {
            if let Ok(balance) = String::from_utf8(value.to_vec()) {
                balance.parse::<u64>().unwrap_or(0)
            } else {
                0
            }
        } else {
            0
        }
    }

    /// Set balance for an account on a specific shard
    pub fn set_balance(&self, shard_id: u16, address: &str, asset: &str, balance: u64) -> Result<(), String> {
        let key = format!("state:{shard_id}:{address}:{asset}");
        let value = balance.to_string();
        self.db.insert(key, value.as_bytes())
            .map_err(|e| format!("Failed to set balance: {e}"))?;
        Ok(())
    }

    /// Transfer funds between accounts (same shard)
    pub fn transfer(&self, shard_id: u16, from: &str, to: &str, amount: u64, asset: &str) -> Result<(), String> {
        let from_balance = self.get_balance(shard_id, from, asset);
        if from_balance < amount {
            return Err("Insufficient funds".to_string());
        }

        let to_balance = self.get_balance(shard_id, to, asset);
        
        self.set_balance(shard_id, from, asset, from_balance - amount)?;
        self.set_balance(shard_id, to, asset, to_balance + amount)?;
        
        Ok(())
    }

    /// Cross-shard transfer (two-phase commit)
    pub fn cross_shard_transfer(&self, source: u16, dest: u16, from: &str, to: &str, amount: u64, asset: &str) -> Result<(), String> {
        println!("[StateDB] Cross-shard transfer: {from} -> {to} ({source} -> {dest})");
        
        // Phase 1: Lock funds on source shard
        let source_balance = self.get_balance(source, from, asset);
        if source_balance < amount {
            return Err("Insufficient funds on source shard".to_string());
        }
        
        // For now, simple transfer. In production, implement proper two-phase commit
        self.set_balance(source, from, asset, source_balance - amount)?;
        
        // Phase 2: Credit funds on destination shard
        let dest_balance = self.get_balance(dest, to, asset);
        self.set_balance(dest, to, asset, dest_balance + amount)?;
        
        Ok(())
    }

    /// Get all accounts on a shard
    pub fn get_accounts(&self, shard_id: u16) -> Vec<String> {
        let mut accounts = Vec::new();
        let prefix = format!("state:{shard_id}:");
        
        for result in self.db.scan_prefix(prefix.as_bytes()) {
            if let Ok((key, _)) = result {
                if let Ok(key_str) = String::from_utf8(key.to_vec()) {
                    if let Some(account) = key_str.strip_prefix(&prefix) {
                        accounts.push(account.to_string());
                    }
                }
            }
        }
        
        accounts
    }
}

impl Default for StateDB {
    fn default() -> Self {
        Self::new("default_state_db")
    }
}

// Usage:
// let state_db = StateDB::new("findag_state_db");
// state_db.set_balance(address, "USD", 1000);
// let bal = state_db.get_balance(address, "USD");
// state_db.transfer(from, to, "USD", 100); 
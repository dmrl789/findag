use sled::Db;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use bincode;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AccountState {
    pub balances: HashMap<String, u64>, // asset -> amount
}

pub struct StateDB {
    db: Db,
}

impl StateDB {
    pub fn new(path: &str) -> Self {
        let db = sled::open(path).expect("Failed to open sled state DB");
        Self { db }
    }

    pub fn get_account(&self, shard_id: u16, address: &str) -> AccountState {
        let key = format!("state:{}:{}", shard_id, address);
        self.db.get(key).unwrap()
            .map(|ivec| bincode::deserialize(&ivec).unwrap())
            .unwrap_or_default()
    }

    pub fn set_account(&self, shard_id: u16, address: &str, state: &AccountState) {
        let key = format!("state:{}:{}", shard_id, address);
        let value = bincode::serialize(state).unwrap();
        self.db.insert(key, value).unwrap();
    }

    pub fn get_balance(&self, shard_id: u16, address: &str, asset: &str) -> u64 {
        self.get_account(shard_id, address).balances.get(asset).cloned().unwrap_or(0)
    }

    pub fn set_balance(&self, shard_id: u16, address: &str, asset: &str, amount: u64) {
        let mut state = self.get_account(shard_id, address);
        state.balances.insert(asset.to_string(), amount);
        self.set_account(shard_id, address, &state);
    }

    pub fn transfer(&self, shard_id: u16, from: &str, to: &str, asset: &str, amount: u64, cross_shard: Option<(u16, u16)>) -> bool {
        // Cross-shard transaction protocol (scaffold)
        if let Some((source, dest)) = cross_shard {
            // TODO: Implement two-phase commit for cross-shard txs
            // Phase 1: Lock/prepare on source shard
            // Phase 2: Commit/acknowledge on destination shard
            // Finalize and update state on both shards
            println!("[StateDB] Cross-shard transfer: {} -> {} ({} -> {})", from, to, source, dest);
            // For now, reject or queue cross-shard txs
            return false;
        }
        let mut from_state = self.get_account(shard_id, from);
        let mut to_state = self.get_account(shard_id, to);
        let from_balance = from_state.balances.get(asset).cloned().unwrap_or(0);
        if from_balance < amount {
            return false; // insufficient funds
        }
        from_state.balances.insert(asset.to_string(), from_balance - amount);
        let to_balance = to_state.balances.get(asset).cloned().unwrap_or(0);
        to_state.balances.insert(asset.to_string(), to_balance + amount);
        self.set_account(shard_id, from, &from_state);
        self.set_account(shard_id, to, &to_state);
        true
    }
}

// Usage:
// let state_db = StateDB::new("findag_state_db");
// state_db.set_balance(address, "USD", 1000);
// let bal = state_db.get_balance(address, "USD");
// state_db.transfer(from, to, "USD", 100); 
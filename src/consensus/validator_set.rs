use ed25519_dalek::PublicKey;
use crate::core::address::Address;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidatorStatus {
    Active,
    Inactive,
    Slashed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    pub address: Address,
    pub public_key: PublicKey,
    pub status: ValidatorStatus,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSet {
    pub validators: HashMap<String, ValidatorInfo>, // address string -> info
    pub shard_assignments: HashMap<String, Vec<u16>>, // address string -> shard IDs
}

impl ValidatorSet {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
            shard_assignments: HashMap::new(),
        }
    }
    pub fn add_validator(&mut self, info: ValidatorInfo) {
        self.validators.insert(info.address.as_str().to_string(), info);
    }
    pub fn remove_validator(&mut self, address: &str) {
        self.validators.remove(address);
        self.shard_assignments.remove(address);
    }
    pub fn set_status(&mut self, address: &str, status: ValidatorStatus) {
        if let Some(v) = self.validators.get_mut(address) {
            v.status = status;
        }
    }
    pub fn get_validator(&self, address: &str) -> Option<&ValidatorInfo> {
        self.validators.get(address)
    }
    pub fn active_validators(&self) -> Vec<&ValidatorInfo> {
        self.validators.values().filter(|v| v.status == ValidatorStatus::Active).collect()
    }
    /// Assign validators to shards in round-robin fashion
    pub fn assign_validators_to_shards(&mut self, shard_count: u16) {
        self.shard_assignments.clear();
        let mut shard = 0;
        let mut active = self.active_validators();
        active.sort_by_key(|v| v.address.as_str().to_string()); // deterministic order
        for v in active {
            self.shard_assignments.entry(v.address.as_str().to_string()).or_default().push(shard);
            shard = (shard + 1) % shard_count;
        }
    }
    /// Get shards for a validator
    pub fn shards_for_validator(&self, address: &str) -> Option<&Vec<u16>> {
        self.shard_assignments.get(address)
    }
    /// Get validators for a shard
    pub fn validators_for_shard(&self, shard_id: u16) -> Vec<&ValidatorInfo> {
        self.shard_assignments.iter()
            .filter(|(_, shards)| shards.contains(&shard_id))
            .filter_map(|(addr, _)| self.validators.get(addr))
            .collect()
    }
} 
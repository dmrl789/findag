use crate::blockchain::error::BlockchainError;
use crate::types::Asset;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct State {
    balances: Arc<RwLock<HashMap<Vec<u8>, u64>>>,
    handles: Arc<RwLock<HashMap<String, String>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            balances: Arc::new(RwLock::new(HashMap::new())),
            handles: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_balance(&self, address: &[u8]) -> u64 {
        let balances = self.balances.read().await;
        *balances.get(address).unwrap_or(&0)
    }

    pub async fn set_balance(&self, address: Vec<u8>, amount: u64) {
        let mut balances = self.balances.write().await;
        balances.insert(address, amount);
    }

    pub async fn get_handle_owner(&self, handle: &str) -> Option<String> {
        let handles = self.handles.read().await;
        handles.get(handle).cloned()
    }

    pub async fn set_handle_owner(&self, handle: String, owner: String) {
        let mut handles = self.handles.write().await;
        handles.insert(handle, owner);
    }
}

#[derive(Clone)]
pub struct BlockchainState {
    balances: Arc<RwLock<HashMap<Vec<u8>, u64>>>,
    handles: Arc<RwLock<HashMap<String, String>>>,
}

impl BlockchainState {
    pub fn new() -> Self {
        Self {
            balances: Arc::new(RwLock::new(HashMap::new())),
            handles: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_balance(&self, address: &[u8]) -> u64 {
        let balances = self.balances.read().await;
        *balances.get(address).unwrap_or(&0)
    }

    pub async fn set_balance(&self, address: Vec<u8>, amount: u64) {
        let mut balances = self.balances.write().await;
        balances.insert(address, amount);
    }

    pub async fn get_handle_owner(&self, handle: &str) -> Option<String> {
        let handles = self.handles.read().await;
        handles.get(handle).cloned()
    }

    pub async fn set_handle_owner(&self, handle: String, owner: String) {
        let mut handles = self.handles.write().await;
        handles.insert(handle, owner);
    }
} 
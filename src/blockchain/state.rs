use std::error::Error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct State {
    pub balances: Arc<Mutex<HashMap<Vec<u8>, u64>>>,
    pub nonces: Arc<Mutex<HashMap<Vec<u8>, u64>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            balances: Arc::new(Mutex::new(HashMap::new())),
            nonces: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_balance(&self, address: &[u8]) -> u64 {
        let balances = self.balances.lock().unwrap();
        *balances.get(address).unwrap_or(&0)
    }

    pub fn get_nonce(&self, address: &[u8]) -> u64 {
        let nonces = self.nonces.lock().unwrap();
        *nonces.get(address).unwrap_or(&0)
    }
} 
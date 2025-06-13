use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::types::round::RoundInfo;

#[derive(Clone, Default)]
pub struct RoundIndex {
    index: Arc<RwLock<HashMap<u64, RoundInfo>>>,
}

impl RoundIndex {
    pub fn new() -> Self {
        Self {
            index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn insert(&self, round_number: u64, info: RoundInfo) {
        self.index.write().unwrap().insert(round_number, info);
    }

    pub fn get(&self, round_number: u64) -> Option<RoundInfo> {
        self.index.read().unwrap().get(&round_number).cloned()
    }

    pub fn latest(&self) -> Option<RoundInfo> {
        self.index
            .read()
            .unwrap()
            .values()
            .max_by_key(|info| info.round_number)
            .cloned()
    }

    pub fn all(&self) -> Vec<RoundInfo> {
        self.index.read().unwrap().values().cloned().collect()
    }
}

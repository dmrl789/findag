use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::types::asset::{AssetRecord, AssetType};

#[derive(Clone, Default)]
pub struct AssetIndex {
    index: Arc<RwLock<HashMap<String, AssetRecord>>>,
}

impl AssetIndex {
    pub fn new() -> Self {
        Self {
            index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn insert(&self, id: String, record: AssetRecord) {
        self.index.write().unwrap().insert(id, record);
    }

    pub fn get(&self, id: &str) -> Option<AssetRecord> {
        self.index.read().unwrap().get(id).cloned()
    }

    pub fn all(&self) -> Vec<AssetRecord> {
        self.index.read().unwrap().values().cloned().collect()
    }

    pub fn by_type(&self, asset_type: AssetType) -> Vec<AssetRecord> {
        self.index
            .read()
            .unwrap()
            .values()
            .filter(|r| r.asset_type == asset_type)
            .cloned()
            .collect()
    }
}

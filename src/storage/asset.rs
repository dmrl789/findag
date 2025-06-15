use crate::types::asset::AssetRecord;
use sled::Db;
use std::error::Error;
use crate::storage::types::{AssetId, AssetType};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

const ASSET_PREFIX: &str = "asset:";

pub fn define_asset(db: &Db, asset: &AssetRecord) -> Result<(), Box<dyn Error>> {
    let store = AssetStore::new(db.clone());
    store.store_asset(asset)
}

pub fn get_asset(db: &Db, asset_id: &str) -> Result<Option<AssetRecord>, Box<dyn Error>> {
    let store = AssetStore::new(db.clone());
    store.get_asset(asset_id)
}

pub struct AssetStore {
    db: Db,
}

impl AssetStore {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub fn store_asset(&self, asset: &AssetRecord) -> Result<(), Box<dyn Error>> {
        let key = format!("{}{}", ASSET_PREFIX, asset.asset_id());
        let value = bincode::serialize(asset)?;
        self.db.insert(key.as_bytes(), value)?;
        Ok(())
    }

    pub fn get_asset(&self, asset_id: &str) -> Result<Option<AssetRecord>, Box<dyn Error>> {
        let key = format!("{}{}", ASSET_PREFIX, asset_id);
        if let Some(data) = self.db.get(key.as_bytes())? {
            let asset = bincode::deserialize(&data)?;
            Ok(Some(asset))
        } else {
            Ok(None)
        }
    }
}

pub struct AssetManager {
    assets: Arc<RwLock<HashMap<AssetId, AssetType>>>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            assets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn store_asset(&self, asset: AssetType) -> Result<(), String> {
        let mut assets = self.assets.write().await;
        let asset_id = AssetId::new(format!("asset_{}", assets.len()));
        assets.insert(asset_id, asset);
        Ok(())
    }

    pub async fn get_asset(&self, asset_id: AssetId) -> Result<AssetType, String> {
        let assets = self.assets.read().await;
        assets.get(&asset_id)
            .cloned()
            .ok_or_else(|| "Asset not found".to_string())
    }

    pub async fn list_assets(&self) -> Vec<(AssetId, AssetType)> {
        let assets = self.assets.read().await;
        assets.iter()
            .map(|(id, asset)| (id.clone(), asset.clone()))
            .collect()
    }

    pub async fn remove_asset(&self, asset_id: AssetId) -> Result<(), String> {
        let mut assets = self.assets.write().await;
        assets.remove(&asset_id)
            .ok_or_else(|| "Asset not found".to_string())?;
        Ok(())
    }
}

use crate::types::asset::Asset;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AssetIndex {
    assets: Arc<RwLock<HashMap<String, Asset>>>,
}

impl AssetIndex {
    pub fn new() -> Self {
        Self {
            assets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn index_asset(&self, asset: Asset) -> Result<(), String> {
        let mut assets = self.assets.write().await;
        assets.insert(asset.id.clone(), asset);
        Ok(())
    }

    pub async fn get_asset(&self, asset_id: &str) -> Option<Asset> {
        let assets = self.assets.read().await;
        assets.get(asset_id).cloned()
    }

    pub async fn search_assets(&self, query: &str) -> Vec<Asset> {
        let assets = self.assets.read().await;
        assets.values()
            .filter(|asset| {
                asset.name.to_lowercase().contains(&query.to_lowercase()) ||
                asset.description.to_lowercase().contains(&query.to_lowercase())
            })
            .cloned()
            .collect()
    }
}

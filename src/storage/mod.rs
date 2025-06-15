pub mod asset;
pub mod asset_index;
pub mod db;
pub mod finality_store;
pub mod ipfs;
pub mod round_index;
pub mod sled;
pub mod snapshot;
pub mod types;

use std::sync::Arc;
use crate::types::block::Block;
use std::error::Error;
use std::path::Path;
use ::sled::open;
use bincode::{self, Options};
use hex;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use crate::storage::types::{AssetId, AssetType, PeerId};

pub use db::{KVStore, bytes_to_ivec};

pub struct Storage {
    db: sled::Db,
    pub db_path: String,
    pub kv_store: KVStore,
    assets: Arc<RwLock<HashMap<AssetId, AssetType>>>,
}

impl Storage {
    pub fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        let db = open(path)?;
        let kv_store = KVStore::new(path.to_str().ok_or("Invalid path")?);
        Ok(Self {
            db,
            kv_store,
            db_path: path.to_str().ok_or("Invalid path")?.to_string(),
            assets: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub fn get_block(&self, hash: &[u8]) -> Result<Option<Block>, Box<dyn std::error::Error>> {
        let key = format!("block:{}", hex::encode(hash));
        if let Some(data) = self.db.get(key)? {
            let block = bincode::deserialize(&data)?;
            Ok(Some(block))
        } else {
            Ok(None)
        }
    }

    pub fn store_block(&self, block: &Block) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("block:{}", hex::encode(block.timestamp.to_le_bytes()));
        let data = bincode::serialize(block)?;
        self.db.insert(key, data)?;
        Ok(())
    }

    pub fn get_all_blocks_sorted(&self) -> Vec<Block> {
        // placeholder logic for fetching blocks in order
        vec![]
    }

    pub fn get_all_block_hashes_sorted(&self) -> Vec<String> {
        // placeholder logic for fetching ordered block hashes
        vec![]
    }

    pub fn load_blocks_by_hashes(&self, _hashes: &[String]) -> Result<(), String> {
        // placeholder: load block data from hash list
        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        match self.db.get(key)? {
            Some(data) => Ok(Some(data.to_vec())),
            None => Ok(None),
        }
    }

    pub fn set(&self, key: &[u8], value: &[u8]) -> Result<(), Box<dyn Error>> {
        self.db.insert(key, value)?;
        Ok(())
    }

    pub fn delete(&self, key: &[u8]) -> Result<(), Box<dyn Error>> {
        self.db.remove(key)?;
        Ok(())
    }

    pub fn scan_prefix(&self, prefix: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>, Box<dyn Error>> {
        let mut result = Vec::new();
        for item in self.db.scan_prefix(prefix) {
            let (key, value) = item?;
            let key_vec = key.to_vec();
            let value_vec = value.to_vec();
            result.push((key_vec, value_vec));
        }
        Ok(result)
    }

    pub fn get_blocks(&self) -> Result<Vec<Block>, Box<dyn Error>> {
        let mut blocks = Vec::new();
        for item in self.db.scan_prefix(b"block:") {
            let (_, value) = item?;
            let value_vec = value.to_vec();
            let block: Block = bincode::deserialize(&value_vec)?;
            blocks.push(block);
        }
        Ok(blocks)
    }

    pub fn save_block(&self, block: &Block) -> Result<(), Box<dyn Error>> {
        let key = format!("block:{}", hex::encode(&block.hash));
        let value = bincode::serialize(block)?;
        self.db.insert(key.as_bytes(), value)?;
        Ok(())
    }

    pub async fn store_asset(&self, asset: AssetType) -> Result<AssetId, Box<dyn Error>> {
        let mut assets = self.assets.write().await;
        let asset_id = AssetId::new(format!("asset_{}", assets.len()));
        assets.insert(asset_id.clone(), asset);
        Ok(asset_id)
    }

    pub async fn get_asset(&self, asset_id: &AssetId) -> Result<Option<AssetType>, Box<dyn Error>> {
        let assets = self.assets.read().await;
        Ok(assets.get(asset_id).cloned())
    }

    pub async fn list_assets(&self) -> Vec<(AssetId, AssetType)> {
        let assets = self.assets.read().await;
        assets.iter()
            .map(|(id, asset)| (id.clone(), asset.clone()))
            .collect()
    }

    pub async fn remove_asset(&self, asset_id: &AssetId) -> Result<(), Box<dyn Error>> {
        let mut assets = self.assets.write().await;
        assets.remove(asset_id)
            .ok_or_else(|| "Asset not found".to_string())?;
        Ok(())
    }
}

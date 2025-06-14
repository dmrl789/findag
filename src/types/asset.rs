use std::error::Error;
use serde::{Deserialize, Serialize};
use chrono;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Token,
    NFT,
    Document,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRecord {
    pub asset_id: String,
    pub asset_type: AssetType,
    pub owner: Vec<u8>,
    pub metadata: Vec<u8>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl AssetRecord {
    pub fn new(
        asset_type: AssetType,
        owner: Vec<u8>,
        metadata: Vec<u8>,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            asset_id: String::new(), // TODO: Generate unique ID
            asset_type,
            owner,
            metadata,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_metadata(&mut self, metadata: Vec<u8>) {
        self.metadata = metadata;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn transfer(&mut self, new_owner: Vec<u8>) {
        self.owner = new_owner;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

pub use self::{AssetRecord, AssetType};

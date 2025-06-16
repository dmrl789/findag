use std::error::Error;
use serde::{Deserialize, Serialize};
use chrono;
use serde_json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssetType {
    Token,
    NFT,
    Document,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner: String,
    pub issuer: String,
    pub metadata: HashMap<String, String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRecord {
    pub asset: String,
    pub version: u32,
    pub signature: Vec<u8>,
    pub timestamp: u64,
    pub asset_type: String,
}

impl Asset {
    pub fn new(
        id: String,
        name: String,
        description: String,
        owner: String,
        issuer: String,
        metadata: HashMap<String, String>,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id,
            name,
            description,
            owner,
            issuer,
            metadata,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, name: Option<String>, description: Option<String>, metadata: Option<HashMap<String, String>>) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(description) = description {
            self.description = description;
        }
        if let Some(metadata) = metadata {
            self.metadata = metadata;
        }
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

impl AssetRecord {
    pub fn new(asset: String, asset_type: String) -> Self {
        Self {
            asset,
            version: 1,
            signature: Vec::new(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            asset_type,
        }
    }

    pub fn asset_id(&self) -> String {
        format!("{}:{}", self.asset, self.version)
    }

    pub fn verify_signature(&self, _public_key: &[u8]) -> Result<bool, Box<dyn Error>> {
        // TODO: Implement signature verification
        Ok(true)
    }
}

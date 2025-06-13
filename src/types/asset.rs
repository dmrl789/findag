use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Fiat,
    Crypto,
    TokenizedStock,
    Derivative,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRecord {
    pub asset_id: String,
    pub name: String,
    pub symbol: String,
    pub asset_type: AssetType,
    pub decimals: u8,
    pub issuer: String, // .fd address
    pub metadata: Option<String>,
}

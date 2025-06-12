use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AssetType {
    Bond,
    Equity,
    Currency,
    Derivative,
    Custom(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetRecord {
    pub id: String,               // Unique ID (e.g., ISIN or UUID)
    pub asset_type: AssetType,    // Asset category
    pub issuer: String,           // Address of issuing node
    pub metadata: String,         // JSON or URI for off-chain info
    pub loaded: bool,             // True = available for trade
}

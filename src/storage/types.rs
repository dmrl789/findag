use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AssetType {
    Token,
    NFT,
    Document,
    Custom(String),
} 
use crate::types::asset::{AssetRecord};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TxType {
    LoadAsset(AssetRecord),
    UnloadAsset(String), // ID of asset
    TransferAsset { id: String, from: String, to: String },
    UpdateHandle { owner: String, new_handle: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub tx_id: String,
    pub timestamp: u64,
    pub tx_type: TxType,
    pub initiator: String,
}

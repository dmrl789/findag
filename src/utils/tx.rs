use crate::types::transaction::{Transaction, TxType};
use crate::types::asset::{AssetRecord, AssetType};
use crate::utils::time::get_findag_time_micro;
use uuid::Uuid;

pub fn create_load_tx(initiator: &str, asset_id: &str, asset_type: AssetType, metadata: &str) -> Transaction {
    let record = AssetRecord {
        id: asset_id.to_string(),
        asset_type,
        issuer: initiator.to_string(),
        metadata: metadata.to_string(),
        loaded: true,
    };

    Transaction {
        tx_id: Uuid::new_v4().to_string(),
        timestamp: get_findag_time_micro(),
        tx_type: TxType::LoadAsset(record),
        initiator: initiator.to_string(),
    }
}

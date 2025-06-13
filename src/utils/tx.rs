use crate::types::transaction::{Transaction, TxType};
use crate::types::asset::{AssetRecord, AssetType};
use crate::utils::time::get_findag_time_micro;
use uuid::Uuid;
use crate::consensus::validators::ValidatorSet;

pub fn process_transaction(tx: &Transaction, validators: &mut ValidatorSet) -> Result<(), String> {
    match tx.tx_type {
        // ...
        TxType::AuthorizeValidator => {
            let key = tx.payload.get("key").ok_or("Missing validator key")?;
            validators.authorize(key);
            println!("✅ Authorized validator: {}", key);
        }
        TxType::RevokeValidator => {
            let key = tx.payload.get("key").ok_or("Missing validator key")?;
            validators.revoke(key);
            println!("❌ Revoked validator: {}", key);
        }
        _ => {}
    }
    Ok(())
}


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

use crate::validation::transaction::{Transaction, TxType};
use crate::storage::types::AssetType;
use crate::utils::time::get_findag_time_micro;
use uuid::Uuid;
use crate::consensus::validators::ValidatorSet;
use serde_json;
use std::error::Error;

pub fn process_transaction(tx: &Transaction, validators: &mut ValidatorSet) -> Result<(), String> {
    match tx.tx_type {
        // ...
        TxType::AuthorizeValidator => {
            let key = String::from_utf8(tx.payload.clone())
                .map_err(|_| "Invalid validator key encoding")?;
            validators.authorize(&key);
            println!("✅ Authorized validator: {}", key);
        }
        TxType::RevokeValidator => {
            let key = String::from_utf8(tx.payload.clone())
                .map_err(|_| "Invalid validator key encoding")?;
            validators.revoke(&key);
            println!("❌ Revoked validator: {}", key);
        }
        _ => {}
    }
    Ok(())
}

pub fn create_validator_tx(validator_key: &str) -> Result<Transaction, String> {
    let timestamp = get_findag_time_micro();
    let tx = Transaction::new(
        TxType::AuthorizeValidator,
        vec![], // sender
        vec![], // recipient
        0,      // amount
        validator_key.as_bytes().to_vec(), // payload
        validator_key.to_string(), // initiator
        timestamp,
        vec![], // data
    );
    Ok(tx)
}

pub fn get_asset_type_string(asset_type: &AssetType) -> String {
    match asset_type {
        AssetType::Token => "token".to_string(),
        AssetType::NFT => "nft".to_string(),
        AssetType::Document => "document".to_string(),
        AssetType::Custom(custom) => custom.clone(),
    }
}

pub fn create_asset_type(name: &str, _description: &str, _owner: &str) -> AssetType {
    match name.to_lowercase().as_str() {
        "token" => AssetType::Token,
        "nft" => AssetType::NFT,
        "document" => AssetType::Document,
        custom => AssetType::Custom(custom.to_string()),
    }
}

pub fn create_asset_tx(
    asset_id: &str,
    asset_type: AssetType,
    metadata: &str,
    initiator: &str,
) -> Result<Transaction, String> {
    let asset = Asset {
        id: asset_id.to_string(),
        name: asset_id.to_string(), // or another name field if available
        description: String::new(), // or pass as parameter
        owner: initiator.to_string(),
        metadata: serde_json::from_str(metadata).unwrap_or(serde_json::json!({})),
        created_at: get_findag_time_micro() as i64,
        updated_at: get_findag_time_micro() as i64,
    };
    let asset_record = AssetRecord {
        asset: asset_id.to_string(),
        version: 1,
        signature: vec![], // You may want to sign this properly
        timestamp: get_findag_time_micro() as u64,
        asset_type: match asset_type {
            AssetType::Token => "token".to_string(),
            AssetType::NFT => "nft".to_string(),
            AssetType::Document => "document".to_string(),
            AssetType::Custom(ref custom) => custom.clone(),
        },
    };

    let timestamp = get_findag_time_micro();
    let tx = Transaction::new(
        TxType::LoadAsset(asset),
        initiator.as_bytes().to_vec(),
        vec![],
        0,
        vec![],
        initiator.to_string(),
        timestamp,
        vec![], // data
    );
    Ok(tx)
}

pub fn create_transfer_tx(
    from: &str,
    to: &str,
    amount: u64,
    data: &str,
) -> Result<Transaction, String> {
    let timestamp = get_findag_time_micro();
    let tx = Transaction::new(
        TxType::Transfer,
        from.as_bytes().to_vec(),
        to.as_bytes().to_vec(),
        amount,
        data.as_bytes().to_vec(),
        from.to_string(),
        timestamp,
        vec![], // data
    );
    Ok(tx)
}

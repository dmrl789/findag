use crate::types::{Transaction, TxType, Asset, AssetType};
use crate::utils::time::get_findag_time_micro;
use uuid::Uuid;
use crate::consensus::validators::ValidatorSet;
use serde_json;
use std::error::Error;
use std::collections::HashMap;
use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

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

pub fn create_transfer_tx(
    from: &[u8],
    to: &[u8],
    amount: u64,
    signing_key: &SigningKey,
) -> Result<Transaction, Box<dyn Error>> {
    let data = serde_json::to_vec(&TransactionData {
        tx_type: TransactionType::Transfer,
        from: from.to_vec(),
        to: to.to_vec(),
        amount,
        handle: String::new(),
    })?;

    let signature = BASE64.encode(signing_key.sign(&data).to_bytes());
    let public_key = signing_key.verifying_key().to_bytes().to_vec();

    Ok(Transaction::new(data, signature, public_key))
}

pub fn create_handle_tx(
    owner: &[u8],
    handle: String,
    signing_key: &SigningKey,
) -> Result<Transaction, Box<dyn Error>> {
    let data = serde_json::to_vec(&TransactionData {
        tx_type: TransactionType::RegisterHandle,
        from: owner.to_vec(),
        to: vec![],
        amount: 0,
        handle,
    })?;

    let signature = BASE64.encode(signing_key.sign(&data).to_bytes());
    let public_key = signing_key.verifying_key().to_bytes().to_vec();

    Ok(Transaction::new(data, signature, public_key))
}

pub fn create_transaction(
    tx_type: TransactionType,
    from: &[u8],
    to: &[u8],
    amount: u64,
    handle: String,
    signing_key: &SigningKey,
) -> Result<Transaction, Box<dyn Error>> {
    let data = serde_json::to_vec(&TransactionData {
        tx_type,
        from: from.to_vec(),
        to: to.to_vec(),
        amount,
        handle,
    })?;

    let signature = BASE64.encode(signing_key.sign(&data).to_bytes());
    let public_key = signing_key.verifying_key().to_bytes().to_vec();

    Ok(Transaction::new(data, signature, public_key))
}

pub fn sign_transaction(
    transaction: &mut Transaction,
    signing_key: &SigningKey,
) -> Result<(), Box<dyn Error>> {
    let signature = BASE64.encode(signing_key.sign(&transaction.data).to_bytes());
    transaction.signature = signature;
    transaction.public_key = signing_key.verifying_key().to_bytes().to_vec();
    Ok(())
}

#[derive(serde::Serialize)]
struct TransactionData {
    tx_type: TransactionType,
    from: Vec<u8>,
    to: Vec<u8>,
    amount: u64,
    handle: String,
}

#[derive(serde::Serialize)]
enum TransactionType {
    Transfer,
    RegisterHandle,
}

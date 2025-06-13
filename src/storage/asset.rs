use crate::types::asset::AssetRecord;
use sled::{Db, IVec};
use serde_json;
use std::str;

const ASSET_PREFIX: &str = "asset_";

pub fn define_asset(db: &Db, asset: &AssetRecord) -> Result<(), String> {
    let key = format!("{}{}", ASSET_PREFIX, asset.asset_id);
    let value = serde_json::to_vec(asset).map_err(|e| e.to_string())?;
    db.insert(key, value).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_asset(db: &Db, asset_id: &str) -> Option<AssetRecord> {
    let key = format!("{}{}", ASSET_PREFIX, asset_id);
    match db.get(key).ok().flatten() {
        Some(data) => serde_json::from_slice(&data).ok(),
        None => None,
    }
}

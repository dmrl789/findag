use crate::types::asset::AssetRecord;
use crate::storage::asset::{define_asset, get_asset};
use sled::Db;
use warp::http::StatusCode;
use warp::{reply::json, Rejection, Reply};

pub async fn define_asset_handler(asset: AssetRecord, db: Db) -> Result<impl Reply, Rejection> {
    match define_asset(&db, &asset) {
        Ok(_) => Ok(warp::reply::with_status("Asset defined", StatusCode::CREATED)),
        Err(e) => Ok(warp::reply::with_status(e, StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

pub async fn get_asset_handler(asset_id: String, db: Db) -> Result<impl Reply, Rejection> {
    match get_asset(&db, &asset_id) {
        Some(asset) => Ok(json(&asset)),
        None => Ok(warp::reply::with_status("Asset not found", StatusCode::NOT_FOUND)),
    }
}

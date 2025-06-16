// use crate::types::asset::AssetType;
use crate::storage::asset::{define_asset, get_asset};
use crate::types::asset::AssetRecord;
use sled::Db;
use warp::http::StatusCode;
use warp::{reply::json, Rejection, Reply};
use serde::Serialize;

#[derive(Serialize)]
struct StatusMessage<'a> {
    message: &'a str,
}

#[derive(Serialize)]
struct JsonResponse<T> {
    data: Option<T>,
    status: u16,
}

impl<T: Serialize + std::marker::Send> Reply for JsonResponse<T> {
    fn into_response(self) -> warp::reply::Response {
        json(&self).into_response()
    }
}

pub async fn define_asset_handler(asset: AssetRecord, db: Db) -> Result<impl Reply, Rejection> {
    match define_asset(&db, &asset) {
        Ok(_) => Ok(JsonResponse { data: Some(asset), status: StatusCode::CREATED.as_u16() }),
        Err(_) => Ok(JsonResponse { data: None, status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() }),
    }
}

pub async fn get_asset_handler(asset_id: String, db: Db) -> Result<impl Reply, Rejection> {
    match get_asset(&db, &asset_id) {
        Ok(Some(asset)) => Ok(JsonResponse { data: Some(asset), status: StatusCode::OK.as_u16() }),
        Ok(None) => Ok(JsonResponse { data: None, status: StatusCode::NOT_FOUND.as_u16() }),
        Err(_) => Ok(JsonResponse { data: None, status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() }),
    }
}

use warp::http::StatusCode;
use serde::{Serialize, Deserialize};
use warp::reply::json;
use warp::Reply;
use std::sync::Arc;
use crate::registry::HandleRegistry;
use crate::registry::handle::ResolveResponse;
use crate::registry::handle::HandleRecord;
use chrono::Utc;

#[derive(Serialize)]
struct StatusMessage<'a> {
    message: &'a str,
}

#[derive(Serialize)]
struct JsonResponse<T> {
    data: T,
    status: u16,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub handle: String,
    pub address: String,
    pub role: Option<String>,
    pub location: Option<String>,
}

impl<T: Serialize + std::marker::Send> Reply for JsonResponse<T> {
    fn into_response(self) -> warp::reply::Response {
        json(&self).into_response()
    }
}

pub async fn register_handle(req: RegisterRequest) -> Result<impl Reply, warp::Rejection> {
    let record = HandleRecord {
        id: req.handle.clone(),
        data: req.address.clone(),
        owner: req.address.clone(),
        created_at: Utc::now().timestamp_millis() as u64,
        role: req.role,
        location: req.location,
    };

    // TODO: Store record in database
    let response = ResolveResponse {
        handle: record.id,
        address: record.data,
    };
    
    Ok(JsonResponse {
        data: response,
        status: StatusCode::OK.as_u16(),
    })
}

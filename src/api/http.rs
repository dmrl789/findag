use warp::{Filter, Rejection, Reply};
use std::sync::Arc;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Mutex;
use crate::registry::handle::{HandleRegistry, HandleRecord};
use warp::http::StatusCode;
use serde_json::json;

pub fn handle_routes(
    registry: Arc<Mutex<HandleRegistry>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let register = warp::path!("register" / String / String)
        .and(warp::post())
        .and(with_registry(registry.clone()))
        .and_then(register_handle);

    let lookup = warp::path!("lookup" / String)
        .and(warp::get())
        .and(with_registry(registry.clone()))
        .and_then(lookup_address);

    register.or(lookup)
}

fn with_registry(
    registry: Arc<Mutex<HandleRegistry>>,
) -> impl Filter<Extract = (Arc<Mutex<HandleRegistry>>,), Error = Infallible> + Clone {
    warp::any().map(move || registry.clone())
}

async fn register_handle(
    handle: String,
    address: String,
    reg: Arc<Mutex<HandleRegistry>>,
) -> Result<impl Reply, Rejection> {
    let record = HandleRecord {
        id: handle.clone(),
        data: address,
        created_at: chrono::Utc::now().timestamp_millis() as u64,
        owner: "system".to_string(),
    };
    
    match reg.lock().unwrap().register_handle(&handle, &record) {
        Ok(_) => Ok(warp::reply::json(&json!({
            "status": "success",
            "message": "Handle registered successfully"
        }))),
        Err(e) => Ok(warp::reply::json(&json!({
            "status": "error",
            "message": e.to_string()
        }))),
    }
}

async fn lookup_address(
    handle: String,
    reg: Arc<Mutex<HandleRegistry>>,
) -> Result<impl Reply, Rejection> {
    match reg.lock().unwrap().lookup_handle(&handle) {
        Some(record) => Ok(warp::reply::json(&record)),
        None => Ok(warp::reply::json(&json!({
            "status": "error",
            "message": "Handle not found"
        }))),
    }
}

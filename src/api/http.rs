use warp::Filter;
use std::sync::Arc;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Mutex;
use crate::registry::handle::{HandleRegistry, HandleRecord};
use warp::http::StatusCode;

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
        .and_then(lookup_handle);

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
    registry: Arc<Mutex<HandleRegistry>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut reg = registry.lock().unwrap();
    match reg.register_handle(&handle, &address) {
        Ok(_) => Ok(warp::reply::with_status("Handle registered", StatusCode::OK)),
        Err(_) => Ok(warp::reply::with_status("Registration failed", StatusCode::BAD_REQUEST)),
    }
}

async fn lookup_handle(
    handle: String,
    registry: Arc<Mutex<HandleRegistry>>,
) -> Result<impl warp::Reply, Infallible> {
    let reg = registry.lock().unwrap();
    match reg.lookup_address(&handle) {
        Some(addr) => Ok(warp::reply::json(&addr)),
        None => Ok(warp::reply::with_status("Handle not found", StatusCode::NOT_FOUND)),
    }
}

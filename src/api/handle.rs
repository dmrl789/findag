use warp::{http::StatusCode, Filter, Rejection, Reply};
use crate::registry::handle::{register_handle, resolve_handle};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub handle: String,
    pub address: String,
}

#[derive(Serialize)]
pub struct ResolveResponse {
    pub address: String,
}

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("handle")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(register)
        .or(warp::path!("handle" / String)
            .and(warp::get())
            .and_then(resolve))
}

async fn register(req: RegisterRequest) -> Result<impl Reply, Rejection> {
    match register_handle(&req.handle, &req.address) {
        Ok(_) => Ok(warp::reply::with_status("Handle registered", StatusCode::OK)),
        Err(e) => Ok(warp::reply::with_status(e, StatusCode::BAD_REQUEST)),
    }
}

async fn resolve(handle: String) -> Result<impl Reply, Rejection> {
    match resolve_handle(&handle) {
        Some(address) => Ok(warp::reply::json(&ResolveResponse { address })),
        None => Ok(warp::reply::with_status("Not found", StatusCode::NOT_FOUND)),
    }
}

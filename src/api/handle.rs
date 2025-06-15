use warp::{http::StatusCode, Filter, Rejection, Reply, reply::json};
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

#[derive(Serialize)]
struct StatusMessage<'a> {
    message: &'a str,
}

#[derive(Serialize)]
struct JsonResponse<T> {
    data: T,
    status: u16,
}

impl<T: Serialize + std::marker::Send> Reply for JsonResponse<T> {
    fn into_response(self) -> warp::reply::Response {
        json(&self).into_response()
    }
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
        Ok(_) => Ok(JsonResponse { data: Some(ResolveResponse { address: req.address }), status: StatusCode::OK.as_u16() }),
        Err(_) => Ok(JsonResponse { data: None::<ResolveResponse>, status: StatusCode::BAD_REQUEST.as_u16() }),
    }
}

async fn resolve(handle: String) -> Result<impl Reply, Rejection> {
    match resolve_handle(&handle) {
        Some(address) => Ok(JsonResponse { data: Some(ResolveResponse { address }), status: StatusCode::OK.as_u16() }),
        None => Ok(JsonResponse { data: None::<ResolveResponse>, status: StatusCode::NOT_FOUND.as_u16() }),
    }
}

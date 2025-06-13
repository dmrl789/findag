async fn register_handle(
    registry: Arc<HandleRegistry>,
    req: RegisterRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let record = HandleRecord {
        owner: req.owner.clone(),
        created_at: Utc::now().timestamp_millis() as u64,
    };
    match registry.register_handle(&req.handle, &record) {
        Ok(_) => Ok(warp::reply::with_status("Handle registered", StatusCode::OK)),
        Err(msg) => Ok(warp::reply::with_status(msg, StatusCode::BAD_REQUEST)),
    }
}

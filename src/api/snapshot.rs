use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use crate::sync::replay::ReplayState;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::storage::Storage;
use crate::storage::snapshot::Snapshot;

pub fn create_snapshot_routes(
    storage: Arc<Storage>,
    state: Arc<std::sync::Mutex<ReplayState>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let st = state.clone();
    let s = storage.clone();
    let st2 = state.clone();
    let s2 = storage.clone();
    
    let create_route = warp::path("create")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |filename: String| {
            let st_lock = st.lock().unwrap();
            let snap = Snapshot::create(&s, &*st_lock).unwrap();
            snap.save_to_file(&filename).unwrap();
            warp::reply::json(&"Snapshot created")
        });

    let restore_route = warp::path("restore")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |filename: String| {
            let mut st_lock = st2.lock().unwrap();
            let snap = Snapshot::load_from_file(&filename).unwrap();
            snap.restore(&s2, &mut *st_lock).unwrap();
            warp::reply::json(&"Snapshot restored")
        });

    create_route.or(restore_route)
}

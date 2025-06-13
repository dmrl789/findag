use warp::Filter;
use crate::storage::snapshot::Snapshot;
use crate::sync::replay::ReplayState;
use crate::storage::Storage;
use std::sync::{Arc, Mutex};

pub fn routes(storage: Arc<Storage>, state: Arc<Mutex<ReplayState>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let s = storage.clone(); let st = state.clone();

    let create = warp::path!("snapshot" / "create" / String)
        .and(warp::get())
        .map(move |filename: String| {
            let mut st_lock = st.lock().unwrap();
            let snap = Snapshot::create(&s, &*st_lock).unwrap();
            snap.save_to_file(&filename).unwrap();
            warp::reply::json(&format!("Snapshot saved: {}", filename))
        });

    let load = warp::path!("snapshot" / "load" / String)
        .and(warp::get())
        .map(move |filename: String| {
            let mut st_lock = st.lock().unwrap();
            let snap = Snapshot::load_from_file(&filename).unwrap();
            snap.restore(&s, &mut *st_lock).unwrap();
            warp::reply::json(&format!("Snapshot loaded: {}", filename))
        });

    create.or(load)
}

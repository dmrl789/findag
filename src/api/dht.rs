use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DhtRecord {
    pub hash_timer: String,
    pub metadata: String,
    pub node_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DhtAnnounceRequest {
    pub hash_timer: String,
    pub metadata: String,
    pub node_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DhtAnnounceResponse {
    pub status: String,
    pub announced_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DhtLookupResponse {
    pub record: Option<DhtRecord>,
    pub found: bool,
}

type DhtStore = Arc<Mutex<HashMap<String, DhtRecord>>>;

pub fn dht_routes(store: DhtStore) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let announce = warp::path!("dht" / "announce")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_store(store.clone()))
        .and_then(announce_handler);

    let lookup = warp::path!("dht" / "lookup")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(with_store(store.clone()))
        .and_then(lookup_handler);

    announce.or(lookup)
}

fn with_store(store: DhtStore) -> impl Filter<Extract = (DhtStore,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || store.clone())
}

async fn announce_handler(body: DhtAnnounceRequest, store: DhtStore) -> Result<impl warp::Reply, warp::Rejection> {
    let mut map = store.lock().unwrap();
    let now = chrono::Utc::now().to_rfc3339();

    map.insert(
        body.hash_timer.clone(),
        DhtRecord {
            hash_timer: body.hash_timer.clone(),
            metadata: body.metadata,
            node_id: body.node_id,
        },
    );

    Ok(warp::reply::json(&DhtAnnounceResponse {
        status: "ok".into(),
        announced_at: now,
    }))
}

async fn lookup_handler(query: HashMap<String, String>, store: DhtStore) -> Result<impl warp::Reply, warp::Rejection> {
    let map = store.lock().unwrap();
    if let Some(hash_timer) = query.get("hash_timer") {
        let result = map.get(hash_timer).cloned();
        Ok(warp::reply::json(&DhtLookupResponse {
            record: result.clone(),
            found: result.is_some(),
        }))
    } else {
        Ok(warp::reply::json(&DhtLookupResponse {
            record: None,
            found: false,
        }))
    }
}

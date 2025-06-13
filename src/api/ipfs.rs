use warp::Filter;
use crate::types::ipfs::IpfsMetadata;
use crate::storage::ipfs::IpfsRegistry;
use std::sync::Arc;

pub fn ipfs_routes(ipfs_registry: Arc<IpfsRegistry>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let reg_filter = warp::any().map(move || ipfs_registry.clone());

    let register = warp::path("ipfs")
        .and(warp::post())
        .and(warp::body::json())
        .and(reg_filter.clone())
        .map(|meta: IpfsMetadata, reg: Arc<IpfsRegistry>| {
            reg.register(meta.clone());
            warp::reply::json(&meta)
        });

    let get = warp::path!("ipfs" / String)
        .and(warp::get())
        .and(reg_filter.clone())
        .map(|cid: String, reg: Arc<IpfsRegistry>| {
            warp::reply::json(&reg.get(&cid))
        });

    register.or(get)
}

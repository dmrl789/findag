use warp::Filter;
use crate::registry::reputation::ReputationEngine;

pub fn routes(rep_engine: ReputationEngine) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("reputation" / "all")
        .and(warp::get())
        .map(move || {
            let data = rep_engine.get_all();
            warp::reply::json(&data)
        })
}

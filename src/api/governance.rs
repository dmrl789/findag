use warp::Filter;
use crate::utils::governance::{Governance, VoteType, Vote};
use crate::types::governance::Proposal;
use std::sync::{Arc, Mutex};

pub fn governance_routes(gov: Arc<Mutex<Governance>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let submit = warp::path("propose")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_gov(gov.clone()))
        .map(|proposal: Proposal, gov| {
            let mut g = gov.lock().unwrap();
            if g.submit_proposal(proposal) {
                warp::reply::json(&"Proposal submitted")
            } else {
                warp::reply::json(&"Proposal already exists")
            }
        });

    let vote = warp::path("vote")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(with_gov(gov.clone()))
        .map(|vote: Vote, params: std::collections::HashMap<String, String>, gov| {
            let id = params.get("id").unwrap_or(&"".to_string()).clone();
            let mut g = gov.lock().unwrap();
            if g.vote(&id, vote) {
                warp::reply::json(&"Vote cast")
            } else {
                warp::reply::json(&"Invalid vote or already voted")
            }
        });

    submit.or(vote)
}

fn with_gov(gov: Arc<Mutex<Governance>>) -> impl Filter<Extract = (Arc<Mutex<Governance>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || gov.clone())
}

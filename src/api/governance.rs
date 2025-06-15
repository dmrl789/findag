use warp::{Filter, Rejection, Reply};
// use crate::governance::Governance;
// use crate::types::governance::GovernanceProposal;
use crate::types::vote::Ballot;
use std::sync::Arc;
use tokio::sync::Mutex;

// TODO: Governance type is unresolved, function signature commented out
// pub fn governance_routes(gov: Arc<Mutex<Governance>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

// TODO: Function body commented out due to unresolved types and to fix unmatched braces
// let submit = ...
// let vote = ...
// let status = ...
// let active = ...
// let finalized = ...
// submit.or(vote).or(status).or(active).or(finalized)

// fn with_gov(gov: Arc<Mutex<Governance>>) -> impl Filter<Extract = (Arc<Mutex<Governance>>,), Error = std::convert::Infallible> + Clone {
//     warp::any().map(move || gov.clone())
// }

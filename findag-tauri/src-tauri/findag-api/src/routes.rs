//! API route definitions
//! 
//! This module contains route definitions for the FinDAG API.

use findag_types::{FindDAGResult, FindDAGError};
use crate::{AppState, handlers::*};

use axum::Router;
use std::sync::Arc;

/// Create all API routes
pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(create_blockchain_routes())
        .merge(create_trading_routes())
        .merge(create_wallet_routes())
        .merge(create_network_routes())
        .merge(create_admin_routes())
        .merge(create_metrics_routes())
        .with_state(state)
}

/// Create blockchain routes
fn create_blockchain_routes() -> Router {
    Router::new()
        .route("/blocks", axum::routing::get(get_blocks))
        .route("/blocks/:hash", axum::routing::get(get_block))
        .route("/rounds", axum::routing::get(get_rounds))
        .route("/rounds/:number", axum::routing::get(get_round))
        .route("/transactions", axum::routing::get(get_transactions))
        .route("/transactions/:hash", axum::routing::get(get_transaction))
        .route("/transactions", axum::routing::post(submit_transaction))
        .route("/dag", axum::routing::get(get_dag_status))
        .route("/dag/blocks", axum::routing::get(get_dag_blocks))
        .route("/dag/transactions", axum::routing::get(get_dag_transactions))
}

/// Create trading routes
fn create_trading_routes() -> Router {
    Router::new()
        .route("/trading/pairs", axum::routing::get(get_trading_pairs))
        .route("/trading/market-data", axum::routing::get(get_market_data))
        .route("/trading/orders", axum::routing::get(get_orders))
        .route("/trading/orders", axum::routing::post(place_order))
        .route("/trading/orders/:id", axum::routing::delete(cancel_order))
        .route("/trading/portfolio", axum::routing::get(get_portfolio))
        .route("/trading/history", axum::routing::get(get_trading_history))
}

/// Create wallet routes
fn create_wallet_routes() -> Router {
    Router::new()
        .route("/wallet/balance", axum::routing::get(get_wallet_balance))
        .route("/wallet/transactions", axum::routing::get(get_wallet_transactions))
        .route("/wallet/send", axum::routing::post(send_transaction))
        .route("/wallet/addresses", axum::routing::get(get_wallet_addresses))
        .route("/wallet/create", axum::routing::post(create_wallet))
        .route("/wallet/import", axum::routing::post(import_wallet))
        .route("/wallet/export", axum::routing::post(export_wallet))
}

/// Create network routes
fn create_network_routes() -> Router {
    Router::new()
        .route("/network/status", axum::routing::get(get_network_status))
        .route("/network/peers", axum::routing::get(get_peers))
        .route("/network/peers", axum::routing::post(add_peer))
        .route("/network/peers/:id", axum::routing::delete(remove_peer))
        .route("/network/topology", axum::routing::get(get_network_topology))
        .route("/network/bandwidth", axum::routing::get(get_network_bandwidth))
}

/// Create admin routes
fn create_admin_routes() -> Router {
    Router::new()
        .route("/admin/validators", axum::routing::get(get_validators))
        .route("/admin/validators", axum::routing::post(add_validator))
        .route("/admin/validators/:id", axum::routing::delete(remove_validator))
        .route("/admin/governance", axum::routing::get(get_governance))
        .route("/admin/governance/proposals", axum::routing::post(submit_proposal))
        .route("/admin/system/status", axum::routing::get(get_system_status))
        .route("/admin/system/config", axum::routing::get(get_system_config))
        .route("/admin/system/config", axum::routing::put(update_system_config))
}

/// Create metrics routes
fn create_metrics_routes() -> Router {
    Router::new()
        .route("/metrics", axum::routing::get(get_metrics))
        .route("/health", axum::routing::get(health_check))
        .route("/status", axum::routing::get(get_status))
} 
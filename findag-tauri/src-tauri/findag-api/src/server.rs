//! HTTP server implementation
//! 
//! This module contains the HTTP server implementation for the FinDAG API.

use findag_types::{FindDAGResult, FindDAGError};
use crate::{APIServer, APIConfig, AppState};

use axum::Router;
use tower::ServiceBuilder;
use tracing::{info, warn, error, debug};

impl APIServer {
    /// Create HTTP server
    pub async fn create_http_server(
        config: APIConfig,
        state: Arc<AppState>,
    ) -> FindDAGResult<Router> {
        info!("Creating HTTP server with configuration: {:?}", config);
        
        // Create router with routes
        let router = Router::new()
            .merge(Self::create_health_routes())
            .merge(Self::create_api_routes())
            .with_state(state);
        
        Ok(router)
    }
    
    /// Create health check routes
    fn create_health_routes() -> Router {
        Router::new()
            .route("/health", axum::routing::get(Self::health_check))
            .route("/ready", axum::routing::get(Self::ready_check))
    }
    
    /// Create API routes
    fn create_api_routes() -> Router {
        Router::new()
            .merge(Self::create_v1_routes())
    }
    
    /// Create v1 API routes
    fn create_v1_routes() -> Router {
        Router::new()
            .route("/v1/blocks", axum::routing::get(crate::handlers::get_blocks))
            .route("/v1/blocks/:hash", axum::routing::get(crate::handlers::get_block))
            .route("/v1/transactions", axum::routing::get(crate::handlers::get_transactions))
            .route("/v1/transactions", axum::routing::post(crate::handlers::submit_transaction))
            .route("/v1/transactions/:hash", axum::routing::get(crate::handlers::get_transaction))
    }
    
    /// Health check handler
    async fn health_check() -> &'static str {
        "OK"
    }
    
    /// Ready check handler
    async fn ready_check() -> &'static str {
        "READY"
    }
} 
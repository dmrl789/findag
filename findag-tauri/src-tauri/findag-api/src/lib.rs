//! FinDAG HTTP API and WebSocket Server
//! 
//! This crate implements the HTTP API and WebSocket server for FinDAG,
//! providing RESTful endpoints and real-time communication.

pub mod server;
pub mod routes;
pub mod websocket;
pub mod middleware;
pub mod handlers;
pub mod models;
pub mod errors;
pub mod metrics;

pub use server::*;
pub use routes::*;
pub use websocket::*;
pub use middleware::*;
pub use handlers::*;
pub use models::*;
pub use errors::*;
pub use metrics::*;

use findag_core::{Address, Hash, FinDAGTime};
use findag_types::{
    Block, Round, Transaction, Asset, Balance, Wallet, Validator, GovernanceProposal,
    FindDAGResult, FindDAGError,
};
use findag_consensus::ConsensusManager;
use findag_network::NetworkManager;
use findag_storage::StorageManager;
use findag_security::SecurityManager;

use axum::{
    Router, Server, extract::State, http::{StatusCode, HeaderMap},
    response::{Json, Response}, body::Body,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
    compression::CompressionLayer,
};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use metrics::{counter, gauge, histogram};

/// API server
pub struct APIServer {
    /// HTTP server
    http_server: Option<Server<hyper::server::conn::AddrIncoming, Router>>,
    /// WebSocket server
    websocket_server: Option<WebSocketServer>,
    /// Configuration
    config: APIConfig,
    /// Application state
    state: Arc<AppState>,
    /// Metrics
    metrics: APIMetrics,
}

/// API configuration
#[derive(Debug, Clone)]
pub struct APIConfig {
    /// HTTP server address
    pub http_address: String,
    /// WebSocket server address
    pub websocket_address: String,
    /// CORS origins
    pub cors_origins: Vec<String>,
    /// Rate limiting
    pub rate_limit: RateLimitConfig,
    /// Authentication
    pub auth_config: AuthConfig,
    /// Logging
    pub logging_config: LoggingConfig,
}

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Burst size
    pub burst_size: u32,
    /// Enable rate limiting
    pub enabled: bool,
}

/// Authentication configuration
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// JWT secret
    pub jwt_secret: String,
    /// Token expiration in seconds
    pub token_expiration: u64,
    /// Require authentication
    pub require_auth: bool,
}

/// Logging configuration
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// Log level
    pub log_level: String,
    /// Enable request logging
    pub enable_request_logging: bool,
    /// Enable response logging
    pub enable_response_logging: bool,
}

impl Default for APIConfig {
    fn default() -> Self {
        Self {
            http_address: "127.0.0.1:8080".to_string(),
            websocket_address: "127.0.0.1:8081".to_string(),
            cors_origins: vec!["*".to_string()],
            rate_limit: RateLimitConfig::default(),
            auth_config: AuthConfig::default(),
            logging_config: LoggingConfig::default(),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 1000,
            burst_size: 100,
            enabled: true,
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "findag_jwt_secret_change_in_production".to_string(),
            token_expiration: 3600,
            require_auth: false,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            enable_request_logging: true,
            enable_response_logging: true,
        }
    }
}

/// Application state
#[derive(Debug, Clone)]
pub struct AppState {
    /// Consensus manager
    pub consensus: Arc<ConsensusManager>,
    /// Network manager
    pub network: Arc<NetworkManager>,
    /// Storage manager
    pub storage: Arc<StorageManager>,
    /// Security manager
    pub security: Arc<SecurityManager>,
    /// Active connections
    pub connections: Arc<RwLock<HashMap<String, Connection>>>,
    /// API metrics
    pub metrics: Arc<RwLock<APIMetrics>>,
}

/// Connection information
#[derive(Debug, Clone)]
pub struct Connection {
    /// Connection ID
    pub id: String,
    /// IP address
    pub ip_address: String,
    /// User agent
    pub user_agent: String,
    /// Connected at
    pub connected_at: chrono::DateTime<chrono::Utc>,
    /// Last activity
    pub last_activity: chrono::DateTime<chrono::Utc>,
    /// User ID
    pub user_id: Option<String>,
}

/// API metrics
#[derive(Debug, Clone)]
pub struct APIMetrics {
    /// Total requests
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Active connections
    pub active_connections: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Requests per second
    pub requests_per_second: f64,
    /// Error rate
    pub error_rate: f64,
}

impl APIServer {
    /// Create a new API server
    pub fn new(
        config: APIConfig,
        consensus: Arc<ConsensusManager>,
        network: Arc<NetworkManager>,
        storage: Arc<StorageManager>,
        security: Arc<SecurityManager>,
    ) -> Self {
        let state = Arc::new(AppState {
            consensus,
            network,
            storage,
            security,
            connections: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(APIMetrics::default())),
        });
        
        let metrics = APIMetrics::default();
        
        Self {
            http_server: None,
            websocket_server: None,
            config,
            state,
            metrics,
        }
    }

    /// Start the API server
    pub async fn start(&mut self) -> FindDAGResult<()> {
        info!("Starting FinDAG API server");
        
        // Create HTTP router
        let router = self.create_router().await?;
        
        // Create HTTP server
        let http_server = Server::bind(&self.config.http_address.parse()?)
            .serve(router.into_make_service());
        
        // Create WebSocket server
        let websocket_server = WebSocketServer::new(
            self.config.websocket_address.clone(),
            self.state.clone(),
        );
        
        self.http_server = Some(http_server);
        self.websocket_server = Some(websocket_server);
        
        info!("API server started on {}", self.config.http_address);
        info!("WebSocket server started on {}", self.config.websocket_address);
        
        Ok(())
    }

    /// Stop the API server
    pub async fn stop(&mut self) -> FindDAGResult<()> {
        info!("Stopping FinDAG API server");
        
        // Stop WebSocket server
        if let Some(ws_server) = &mut self.websocket_server {
            ws_server.stop().await?;
        }
        
        // Stop HTTP server
        if let Some(http_server) = &mut self.http_server {
            // TODO: Implement graceful shutdown
        }
        
        info!("API server stopped");
        
        Ok(())
    }

    /// Create HTTP router
    async fn create_router(&self) -> FindDAGResult<Router> {
        // Create CORS layer
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);
        
        // Create rate limiting layer
        let rate_limit = if self.config.rate_limit.enabled {
            tower::limit::RateLimitLayer::new(
                self.config.rate_limit.requests_per_minute,
                std::time::Duration::from_secs(60),
            )
        } else {
            tower::limit::RateLimitLayer::new(
                u32::MAX,
                std::time::Duration::from_secs(1),
            )
        };
        
        // Create timeout layer
        let timeout = tower::timeout::TimeoutLayer::new(
            std::time::Duration::from_secs(30),
        );
        
        // Create compression layer
        let compression = CompressionLayer::new();
        
        // Create trace layer
        let trace = TraceLayer::new_for_http();
        
        // Create middleware stack
        let middleware = ServiceBuilder::new()
            .layer(cors)
            .layer(rate_limit)
            .layer(timeout)
            .layer(compression)
            .layer(trace);
        
        // Create router with routes
        let router = Router::new()
            .merge(self.create_blockchain_routes().await?)
            .merge(self.create_trading_routes().await?)
            .merge(self.create_wallet_routes().await?)
            .merge(self.create_network_routes().await?)
            .merge(self.create_admin_routes().await?)
            .merge(self.create_metrics_routes().await?)
            .layer(middleware)
            .with_state(self.state.clone());
        
        Ok(router)
    }

    /// Create blockchain routes
    async fn create_blockchain_routes(&self) -> FindDAGResult<Router> {
        let router = Router::new()
            .route("/blocks", axum::routing::get(handlers::get_blocks))
            .route("/blocks/:hash", axum::routing::get(handlers::get_block))
            .route("/rounds", axum::routing::get(handlers::get_rounds))
            .route("/rounds/:number", axum::routing::get(handlers::get_round))
            .route("/transactions", axum::routing::get(handlers::get_transactions))
            .route("/transactions/:hash", axum::routing::get(handlers::get_transaction))
            .route("/transactions", axum::routing::post(handlers::submit_transaction))
            .route("/dag", axum::routing::get(handlers::get_dag_status))
            .route("/dag/blocks", axum::routing::get(handlers::get_dag_blocks))
            .route("/dag/transactions", axum::routing::get(handlers::get_dag_transactions));
        
        Ok(router)
    }

    /// Create trading routes
    async fn create_trading_routes(&self) -> FindDAGResult<Router> {
        let router = Router::new()
            .route("/trading/pairs", axum::routing::get(handlers::get_trading_pairs))
            .route("/trading/market-data", axum::routing::get(handlers::get_market_data))
            .route("/trading/orders", axum::routing::get(handlers::get_orders))
            .route("/trading/orders", axum::routing::post(handlers::place_order))
            .route("/trading/orders/:id", axum::routing::delete(handlers::cancel_order))
            .route("/trading/portfolio", axum::routing::get(handlers::get_portfolio))
            .route("/trading/history", axum::routing::get(handlers::get_trading_history));
        
        Ok(router)
    }

    /// Create wallet routes
    async fn create_wallet_routes(&self) -> FindDAGResult<Router> {
        let router = Router::new()
            .route("/wallet/balance", axum::routing::get(handlers::get_wallet_balance))
            .route("/wallet/transactions", axum::routing::get(handlers::get_wallet_transactions))
            .route("/wallet/send", axum::routing::post(handlers::send_transaction))
            .route("/wallet/addresses", axum::routing::get(handlers::get_wallet_addresses))
            .route("/wallet/create", axum::routing::post(handlers::create_wallet))
            .route("/wallet/import", axum::routing::post(handlers::import_wallet))
            .route("/wallet/export", axum::routing::post(handlers::export_wallet));
        
        Ok(router)
    }

    /// Create network routes
    async fn create_network_routes(&self) -> FindDAGResult<Router> {
        let router = Router::new()
            .route("/network/status", axum::routing::get(handlers::get_network_status))
            .route("/network/peers", axum::routing::get(handlers::get_peers))
            .route("/network/peers", axum::routing::post(handlers::add_peer))
            .route("/network/peers/:id", axum::routing::delete(handlers::remove_peer))
            .route("/network/topology", axum::routing::get(handlers::get_network_topology))
            .route("/network/bandwidth", axum::routing::get(handlers::get_network_bandwidth));
        
        Ok(router)
    }

    /// Create admin routes
    async fn create_admin_routes(&self) -> FindDAGResult<Router> {
        let router = Router::new()
            .route("/admin/validators", axum::routing::get(handlers::get_validators))
            .route("/admin/validators", axum::routing::post(handlers::add_validator))
            .route("/admin/validators/:id", axum::routing::delete(handlers::remove_validator))
            .route("/admin/governance", axum::routing::get(handlers::get_governance))
            .route("/admin/governance/proposals", axum::routing::post(handlers::submit_proposal))
            .route("/admin/system/status", axum::routing::get(handlers::get_system_status))
            .route("/admin/system/config", axum::routing::get(handlers::get_system_config))
            .route("/admin/system/config", axum::routing::put(handlers::update_system_config));
        
        Ok(router)
    }

    /// Create metrics routes
    async fn create_metrics_routes(&self) -> FindDAGResult<Router> {
        let router = Router::new()
            .route("/metrics", axum::routing::get(handlers::get_metrics))
            .route("/health", axum::routing::get(handlers::health_check))
            .route("/status", axum::routing::get(handlers::get_status));
        
        Ok(router)
    }

    /// Get API metrics
    pub async fn get_metrics(&self) -> APIMetrics {
        self.metrics.clone()
    }

    /// Update metrics
    pub async fn update_metrics(&self, request_time_ms: f64, success: bool) {
        let mut metrics = self.metrics.clone();
        
        metrics.total_requests += 1;
        if success {
            metrics.successful_requests += 1;
        } else {
            metrics.failed_requests += 1;
        }
        
        // Update average response time
        metrics.avg_response_time_ms = 
            (metrics.avg_response_time_ms + request_time_ms) / 2.0;
        
        // Update error rate
        metrics.error_rate = metrics.failed_requests as f64 / metrics.total_requests as f64;
        
        // Update Prometheus metrics
        counter!("findag_api_total_requests", 1);
        if success {
            counter!("findag_api_successful_requests", 1);
        } else {
            counter!("findag_api_failed_requests", 1);
        }
        histogram!("findag_api_response_time_ms", request_time_ms);
        gauge!("findag_api_error_rate", metrics.error_rate);
        
        self.metrics = metrics;
    }
}

impl Default for APIMetrics {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            active_connections: 0,
            avg_response_time_ms: 0.0,
            requests_per_second: 0.0,
            error_rate: 0.0,
        }
    }
} 
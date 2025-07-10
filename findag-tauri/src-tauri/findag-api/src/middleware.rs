//! HTTP API middleware
//! 
//! This module contains middleware for the FinDAG HTTP API.

use findag_types::{FindDAGResult, FindDAGError};
use crate::{AppState, models::*};

use axum::{
    extract::State,
    http::{Request, StatusCode, HeaderMap},
    response::Response,
    body::Body,
};
use tower::{Service, Layer};
use std::task::{Context, Poll};
use std::pin::Pin;
use std::future::Future;
use tracing::{info, warn, error, debug};
use metrics::{counter, histogram};
use std::time::Instant;
use serde::{Serialize, Deserialize};

/// Authentication middleware
pub struct AuthMiddleware<S> {
    inner: S,
    state: std::sync::Arc<AppState>,
}

impl<S> AuthMiddleware<S> {
    /// Create new authentication middleware
    pub fn new(inner: S, state: std::sync::Arc<AppState>) -> Self {
        Self { inner, state }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for AuthMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let mut inner = std::mem::replace(&mut self.inner, unsafe { std::mem::zeroed() });
        let state = self.state.clone();
        
        Box::pin(async move {
            // Extract authorization header
            let auth_header = req.headers().get("Authorization");
            
            if let Some(auth_value) = auth_header {
                if let Ok(auth_str) = auth_value.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];
                        
                        // TODO: Implement JWT token validation
                        // For now, accept any token
                        debug!("Authentication token provided: {}", token);
                    } else {
                        warn!("Invalid authorization header format");
                        return Ok(Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .body(Body::from("Invalid authorization header"))
                            .unwrap());
                    }
                } else {
                    warn!("Invalid authorization header encoding");
                    return Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::from("Invalid authorization header encoding"))
                        .unwrap());
                }
            } else {
                // TODO: Make authentication optional based on configuration
                debug!("No authorization header provided");
            }
            
            // Call inner service
            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}

/// Logging middleware
pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S> LoggingMiddleware<S> {
    /// Create new logging middleware
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for LoggingMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let mut inner = std::mem::replace(&mut self.inner, unsafe { std::mem::zeroed() });
        
        Box::pin(async move {
            let start_time = Instant::now();
            let method = req.method().clone();
            let uri = req.uri().clone();
            let user_agent = req.headers()
                .get("User-Agent")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("Unknown");
            
            info!("Request: {} {} (User-Agent: {})", method, uri, user_agent);
            
            let response = inner.call(req).await?;
            
            let duration = start_time.elapsed();
            let status = response.status();
            
            info!("Response: {} {} - {} ({:?})", method, uri, status, duration);
            
            // Record metrics
            counter!("findag_api_requests_total", 1);
            histogram!("findag_api_request_duration_ms", duration.as_millis() as f64);
            
            if status.is_success() {
                counter!("findag_api_successful_requests", 1);
            } else {
                counter!("findag_api_failed_requests", 1);
            }
            
            Ok(response)
        })
    }
}

/// Rate limiting middleware
pub struct RateLimitMiddleware<S> {
    inner: S,
    requests_per_minute: u32,
    burst_size: u32,
}

impl<S> RateLimitMiddleware<S> {
    /// Create new rate limiting middleware
    pub fn new(inner: S, requests_per_minute: u32, burst_size: u32) -> Self {
        Self {
            inner,
            requests_per_minute,
            burst_size,
        }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for RateLimitMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let mut inner = std::mem::replace(&mut self.inner, unsafe { std::mem::zeroed() });
        let requests_per_minute = self.requests_per_minute;
        let burst_size = self.burst_size;
        
        Box::pin(async move {
            // TODO: Implement actual rate limiting logic
            // For now, just pass through
            debug!("Rate limiting check passed");
            
            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}

/// CORS middleware
pub struct CorsMiddleware<S> {
    inner: S,
    allowed_origins: Vec<String>,
}

impl<S> CorsMiddleware<S> {
    /// Create new CORS middleware
    pub fn new(inner: S, allowed_origins: Vec<String>) -> Self {
        Self {
            inner,
            allowed_origins,
        }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for CorsMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let mut inner = std::mem::replace(&mut self.inner, unsafe { std::mem::zeroed() });
        let allowed_origins = self.allowed_origins.clone();
        
        Box::pin(async move {
            let response = inner.call(req).await?;
            
            // Add CORS headers
            let mut response_builder = Response::builder();
            
            // Add Access-Control-Allow-Origin header
            if allowed_origins.contains(&"*".to_string()) {
                response_builder = response_builder.header("Access-Control-Allow-Origin", "*");
            } else {
                // TODO: Check actual origin and add appropriate header
                response_builder = response_builder.header("Access-Control-Allow-Origin", "*");
            }
            
            // Add other CORS headers
            response_builder = response_builder
                .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
                .header("Access-Control-Allow-Headers", "Content-Type, Authorization")
                .header("Access-Control-Max-Age", "86400");
            
            // TODO: Build response with CORS headers
            Ok(response)
        })
    }
}

/// Metrics middleware
pub struct MetricsMiddleware<S> {
    inner: S,
}

impl<S> MetricsMiddleware<S> {
    /// Create new metrics middleware
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for MetricsMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let mut inner = std::mem::replace(&mut self.inner, unsafe { std::mem::zeroed() });
        
        Box::pin(async move {
            let start_time = Instant::now();
            let method = req.method().as_str();
            let path = req.uri().path();
            
            let response = inner.call(req).await?;
            
            let duration = start_time.elapsed();
            let status = response.status();
            
            // Record metrics
            counter!("findag_api_requests_total", 1, "method" => method.to_string(), "path" => path.to_string());
            histogram!("findag_api_request_duration_ms", duration.as_millis() as f64, "method" => method.to_string(), "path" => path.to_string());
            
            if status.is_success() {
                counter!("findag_api_successful_requests", 1, "method" => method.to_string(), "path" => path.to_string());
            } else {
                counter!("findag_api_failed_requests", 1, "method" => method.to_string(), "path" => path.to_string());
            }
            
            Ok(response)
        })
    }
}

/// Error handling middleware
pub struct ErrorHandlingMiddleware<S> {
    inner: S,
}

impl<S> ErrorHandlingMiddleware<S> {
    /// Create new error handling middleware
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for ErrorHandlingMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let mut inner = std::mem::replace(&mut self.inner, unsafe { std::mem::zeroed() });
        
        Box::pin(async move {
            match inner.call(req).await {
                Ok(response) => Ok(response),
                Err(e) => {
                    error!("Request failed: {:?}", e);
                    
                    // Return error response
                    let error_response = ErrorResponse {
                        error: "Internal Server Error".to_string(),
                        message: "An unexpected error occurred".to_string(),
                        code: "INTERNAL_ERROR".to_string(),
                    };
                    
                    let response_body = serde_json::to_string(&error_response)
                        .unwrap_or_else(|_| r#"{"error":"Internal Server Error","message":"Failed to serialize error response","code":"SERIALIZATION_ERROR"}"#.to_string());
                    
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .header("Content-Type", "application/json")
                        .body(Body::from(response_body))
                        .unwrap())
                }
            }
        })
    }
}

/// Middleware layer for authentication
pub struct AuthLayer {
    state: std::sync::Arc<AppState>,
}

impl AuthLayer {
    /// Create new authentication layer
    pub fn new(state: std::sync::Arc<AppState>) -> Self {
        Self { state }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        AuthMiddleware::new(service, self.state.clone())
    }
}

/// Middleware layer for logging
pub struct LoggingLayer;

impl LoggingLayer {
    /// Create new logging layer
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        LoggingMiddleware::new(service)
    }
}

/// Middleware layer for rate limiting
pub struct RateLimitLayer {
    requests_per_minute: u32,
    burst_size: u32,
}

impl RateLimitLayer {
    /// Create new rate limiting layer
    pub fn new(requests_per_minute: u32, burst_size: u32) -> Self {
        Self {
            requests_per_minute,
            burst_size,
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        RateLimitMiddleware::new(service, self.requests_per_minute, self.burst_size)
    }
}

/// Middleware layer for CORS
pub struct CorsLayer {
    allowed_origins: Vec<String>,
}

impl CorsLayer {
    /// Create new CORS layer
    pub fn new(allowed_origins: Vec<String>) -> Self {
        Self { allowed_origins }
    }
}

impl<S> Layer<S> for CorsLayer {
    type Service = CorsMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        CorsMiddleware::new(service, self.allowed_origins.clone())
    }
}

/// Middleware layer for metrics
pub struct MetricsLayer;

impl MetricsLayer {
    /// Create new metrics layer
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for MetricsLayer {
    type Service = MetricsMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        MetricsMiddleware::new(service)
    }
}

/// Middleware layer for error handling
pub struct ErrorHandlingLayer;

impl ErrorHandlingLayer {
    /// Create new error handling layer
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for ErrorHandlingLayer {
    type Service = ErrorHandlingMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        ErrorHandlingMiddleware::new(service)
    }
} 
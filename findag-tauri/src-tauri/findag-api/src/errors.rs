//! API error handling
//! 
//! This module contains error types and handling for the FinDAG API.

use findag_types::{FindDAGResult, FindDAGError};
use thiserror::Error;
use serde::{Serialize, Deserialize};
use axum::http::StatusCode;

/// API error
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum APIError {
    /// Authentication error
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    /// Authorization error
    #[error("Authorization failed: {0}")]
    Authorization(String),
    
    /// Validation error
    #[error("Validation failed: {0}")]
    Validation(String),
    
    /// Not found error
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    /// Rate limit error
    #[error("Rate limit exceeded")]
    RateLimit,
    
    /// Internal server error
    #[error("Internal server error: {0}")]
    Internal(String),
    
    /// Bad request error
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    /// Service unavailable error
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

impl APIError {
    /// Get HTTP status code for error
    pub fn status_code(&self) -> StatusCode {
        match self {
            APIError::Authentication(_) => StatusCode::UNAUTHORIZED,
            APIError::Authorization(_) => StatusCode::FORBIDDEN,
            APIError::Validation(_) => StatusCode::BAD_REQUEST,
            APIError::NotFound(_) => StatusCode::NOT_FOUND,
            APIError::RateLimit => StatusCode::TOO_MANY_REQUESTS,
            APIError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::BadRequest(_) => StatusCode::BAD_REQUEST,
            APIError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
    
    /// Convert to FindDAGError
    pub fn into_findag_error(self) -> FindDAGError {
        FindDAGError::API(self.to_string())
    }
}

impl From<FindDAGError> for APIError {
    fn from(error: FindDAGError) -> Self {
        match error {
            FindDAGError::Validation(msg) => APIError::Validation(msg),
            FindDAGError::NotFound(msg) => APIError::NotFound(msg),
            FindDAGError::Authentication(msg) => APIError::Authentication(msg),
            FindDAGError::Authorization(msg) => APIError::Authorization(msg),
            FindDAGError::Internal(msg) => APIError::Internal(msg),
            _ => APIError::Internal(error.to_string()),
        }
    }
}

impl From<std::io::Error> for APIError {
    fn from(error: std::io::Error) -> Self {
        APIError::Internal(error.to_string())
    }
}

impl From<serde_json::Error> for APIError {
    fn from(error: serde_json::Error) -> Self {
        APIError::BadRequest(error.to_string())
    }
}

impl From<axum::extract::rejection::JsonRejection> for APIError {
    fn from(error: axum::extract::rejection::JsonRejection) -> Self {
        APIError::BadRequest(error.to_string())
    }
} 
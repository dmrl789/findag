//! Time and HashTimer error handling
//! 
//! This module contains error types and handling for the FinDAG Time and HashTimer systems.

use findag_types::{FindDAGResult, FindDAGError};
use thiserror::Error;
use serde::{Serialize, Deserialize};

/// Time manager error
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum TimeManagerError {
    /// Time synchronization error
    #[error("Time synchronization failed: {0}")]
    SyncError(String),
    
    /// HashTimer generation error
    #[error("HashTimer generation failed: {0}")]
    HashTimerGenerationError(String),
    
    /// HashTimer verification error
    #[error("HashTimer verification failed: {0}")]
    HashTimerVerificationError(String),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    /// Time drift error
    #[error("Time drift too large: {0}ns")]
    TimeDriftError(i64),
    
    /// Invalid time error
    #[error("Invalid time: {0}")]
    InvalidTimeError(String),
}

impl From<TimeManagerError> for FindDAGError {
    fn from(error: TimeManagerError) -> Self {
        FindDAGError::Internal(error.to_string())
    }
}

impl From<std::io::Error> for TimeManagerError {
    fn from(error: std::io::Error) -> Self {
        TimeManagerError::SyncError(error.to_string())
    }
}

impl From<serde_json::Error> for TimeManagerError {
    fn from(error: serde_json::Error) -> Self {
        TimeManagerError::ConfigurationError(error.to_string())
    }
} 
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlockchainError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Timestamp error: {0}")]
    TimestampError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Finality error: {0}")]
    FinalityError(String),
} 
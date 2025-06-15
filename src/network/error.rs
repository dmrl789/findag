use thiserror::Error;
use std::io;
use std::error::Error as StdError;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Protocol error: {0}")]
    Protocol(String),
    #[error("Discovery error: {0}")]
    Discovery(String),
    #[error("Backend error: {0}")]
    Backend(String),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Peer not found: {0}")]
    PeerNotFound(String),
    #[error("Timeout")]
    Timeout,
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<Box<dyn StdError>> for NetworkError {
    fn from(err: Box<dyn StdError>) -> Self {
        NetworkError::Unknown(err.to_string())
    }
}

pub type NetworkResult<T> = Result<T, NetworkError>; 
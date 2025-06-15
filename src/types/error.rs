use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Proposal not found")]
    ProposalNotFound,

    #[error("Proposal already exists")]
    ProposalExists,

    #[error("Proposal has expired")]
    ProposalExpired,

    #[error("Proposal is not active")]
    ProposalNotActive,

    #[error("Insufficient voting power")]
    InsufficientVotingPower,

    #[error("Invalid vote")]
    InvalidVote,

    #[error("Lock error")]
    LockError,

    #[error("Invalid configuration")]
    InvalidConfiguration,

    #[error("Internal error: {0}")]
    InternalError(String),
} 
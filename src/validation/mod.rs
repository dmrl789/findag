pub mod transaction;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid initiator address")]
    InvalidInitiator,

    #[error("Asset ID is missing")]
    MissingAssetId,

    #[error("Handle format is invalid")]
    InvalidHandleFormat,

    #[error("Unauthorized action")]
    UnauthorizedNode,

    #[error("Transaction type not supported")]
    UnsupportedTxType,

    #[error("Signature check failed")]
    InvalidSignature,
}

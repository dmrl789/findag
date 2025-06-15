pub mod api;
pub mod blockchain;
pub mod cli;
pub mod consensus;
pub mod domain;
pub mod governance;
pub mod network;
pub mod registry;
pub mod security;
pub mod storage;
pub mod sync;
pub mod types;
pub mod utils;
pub mod vote;
pub mod wallet;

// Re-export commonly used types
pub use types::{
    address,
    asset,
    transaction,
    round,
    finality,
};

pub use utils::{
    time,
    crypto,
};

pub use blockchain::{
    block,
    state,
};

pub use consensus::{
    validators,
};

pub use storage::{
    db,
};

pub use security::{
    audit,
    monitoring,
    response,
};

pub use registry::{
    handle,
    reputation,
    multisig,
};

// ✅ Correct

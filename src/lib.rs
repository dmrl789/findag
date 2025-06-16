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
pub mod validation;
pub mod vote;
pub mod wallet;
pub mod config;

// Re-export commonly used types
pub use types::{
    Address,
    Asset,
    Block,
    FinalityVote,
    Message,
    RecoveryShare,
    Transaction,
    TxType,
    Wallet,
    Proposal,
    ProposalStatus,
    ProposalType,
    GovernanceConfig,
    VoteChoice,
    Vote,
    VoteType,
    Ballot,
    // Add any missing types that are actually defined in the types module
    AddressType,
    AssetType,
    MessageType,
    TransactionStatus,
    Justification,
    FinalityProof
};

// ✅ Correct

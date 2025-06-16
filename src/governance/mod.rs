pub mod proposals;
pub mod types;
pub mod validation;

// Explicitly re-export only necessary types
pub use types::{Proposal, ProposalStatus, ProposalType, GovernanceConfig, VoteChoice};
pub use validation::{validate_proposal, validate_proposal_status}; 
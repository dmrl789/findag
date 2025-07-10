//! Blockchain type definitions
//! 
//! This module contains core blockchain data structures including blocks, transactions,
//! rounds, and related types.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use findag_core::{Address, Hash, HashTimer, FinDAGTime};

/// Transaction type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionType {
    /// Transfer assets between addresses
    Transfer,
    /// Create a new asset
    CreateAsset,
    /// Update asset metadata
    UpdateAsset,
    /// Register a new handle
    RegisterHandle,
    /// Rotate handle key
    RotateHandle,
    /// Revoke handle
    RevokeHandle,
    /// Governance proposal
    Governance,
    /// Bridge operation
    Bridge,
}

/// Transaction instruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInstruction {
    /// Transaction type
    pub transaction_type: TransactionType,
    /// Sender address
    pub from: Address,
    /// Recipient address (optional)
    pub to: Option<Address>,
    /// Asset being transferred
    pub asset: String,
    /// Amount (for transfers)
    pub amount: Option<u64>,
    /// Metadata (JSON string)
    pub metadata: Option<String>,
    /// Timestamp
    pub timestamp: FinDAGTime,
}

/// Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction hash
    pub hash: Hash,
    /// Transaction instructions
    pub instructions: Vec<TransactionInstruction>,
    /// Sender signature
    pub signature: Vec<u8>,
    /// Transaction timestamp
    pub timestamp: FinDAGTime,
    /// Transaction status
    pub status: TransactionStatus,
}

/// Transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    /// Pending in mempool
    Pending,
    /// Included in a block
    Included,
    /// Finalized in a round
    Finalized,
    /// Failed
    Failed,
    /// Expired
    Expired,
}

/// Block header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Block hash
    pub hash: Hash,
    /// Parent block hashes
    pub parents: Vec<Hash>,
    /// Block producer address
    pub producer: Address,
    /// Block timestamp
    pub timestamp: FinDAGTime,
    /// Block number
    pub number: u64,
    /// Merkle root of transactions
    pub merkle_root: Hash,
    /// Block signature
    pub signature: Vec<u8>,
}

/// Block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Block header
    pub header: BlockHeader,
    /// Block transactions
    pub transactions: Vec<Transaction>,
    /// Block size in bytes
    pub size: usize,
}

/// Round header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundHeader {
    /// Round number
    pub number: u64,
    /// Previous round hash
    pub previous_round: Hash,
    /// Finalized blocks
    pub finalized_blocks: Vec<Hash>,
    /// Round timestamp
    pub timestamp: FinDAGTime,
    /// Validator committee
    pub committee: Vec<Address>,
    /// Round signature
    pub signature: Vec<u8>,
}

/// Round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round {
    /// Round header
    pub header: RoundHeader,
    /// Round blocks
    pub blocks: Vec<Block>,
    /// Round size in bytes
    pub size: usize,
}

/// Asset information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    /// Asset code
    pub code: String,
    /// Asset name
    pub name: String,
    /// Asset description
    pub description: Option<String>,
    /// Asset owner
    pub owner: Address,
    /// Total supply
    pub total_supply: u64,
    /// Circulating supply
    pub circulating_supply: u64,
    /// Asset metadata
    pub metadata: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// Asset code
    pub asset: String,
    /// Owner address
    pub owner: Address,
    /// Balance amount
    pub amount: u64,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Handle information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handle {
    /// Handle name
    pub name: String,
    /// Handle owner
    pub owner: Address,
    /// Parent handle (if any)
    pub parent: Option<String>,
    /// Public key
    pub public_key: Vec<u8>,
    /// Handle metadata
    pub metadata: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Handle status
    pub status: HandleStatus,
}

/// Handle status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandleStatus {
    /// Active
    Active,
    /// Suspended
    Suspended,
    /// Revoked
    Revoked,
}

/// Validator information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    /// Validator address
    pub address: Address,
    /// Validator public key
    pub public_key: Vec<u8>,
    /// Validator metadata
    pub metadata: Option<String>,
    /// Validator status
    pub status: ValidatorStatus,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    /// Last active timestamp
    pub last_active: DateTime<Utc>,
}

/// Validator status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidatorStatus {
    /// Active
    Active,
    /// Inactive
    Inactive,
    /// Slashed
    Slashed,
}

/// Governance proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    /// Proposal ID
    pub id: String,
    /// Proposal type
    pub proposal_type: GovernanceProposalType,
    /// Proposer address
    pub proposer: Address,
    /// Proposal data
    pub data: String,
    /// Proposal status
    pub status: GovernanceProposalStatus,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Voting deadline
    pub voting_deadline: DateTime<Utc>,
    /// Execution timestamp
    pub executed_at: Option<DateTime<Utc>>,
}

/// Governance proposal type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GovernanceProposalType {
    /// Add validator
    AddValidator,
    /// Remove validator
    RemoveValidator,
    /// Slash validator
    SlashValidator,
    /// Add asset
    AddAsset,
    /// Remove asset
    RemoveAsset,
    /// Update parameters
    UpdateParameters,
}

/// Governance proposal status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GovernanceProposalStatus {
    /// Active
    Active,
    /// Approved
    Approved,
    /// Rejected
    Rejected,
    /// Executed
    Executed,
    /// Expired
    Expired,
}

/// Chain state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainState {
    /// Latest round number
    pub latest_round: u64,
    /// Latest block number
    pub latest_block: u64,
    /// Total transactions
    pub total_transactions: u64,
    /// Total assets
    pub total_assets: u64,
    /// Total handles
    pub total_handles: u64,
    /// Active validators
    pub active_validators: u64,
    /// Chain timestamp
    pub timestamp: DateTime<Utc>,
} 
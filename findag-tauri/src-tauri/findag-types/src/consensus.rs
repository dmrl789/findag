//! Consensus type definitions
//! 
//! This module contains types related to the RoundChain consensus engine,
//! including round management, block finalization, and validator coordination.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use findag_core::{Address, Hash, HashTimer, FinDAGTime};

/// Consensus state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusState {
    /// Current round number
    pub current_round: u64,
    /// Latest finalized round
    pub latest_finalized_round: u64,
    /// Current block number
    pub current_block: u64,
    /// Active validator set
    pub active_validators: Vec<Address>,
    /// Consensus status
    pub status: ConsensusStatus,
    /// Last update timestamp
    pub last_update: DateTime<Utc>,
}

/// Consensus status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusStatus {
    /// Consensus is running normally
    Running,
    /// Consensus is syncing
    Syncing,
    /// Consensus is stalled
    Stalled,
    /// Consensus has failed
    Failed,
}

/// Round state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundState {
    /// Round number
    pub round_number: u64,
    /// Round start timestamp
    pub start_timestamp: FinDAGTime,
    /// Round end timestamp
    pub end_timestamp: Option<FinDAGTime>,
    /// Blocks in this round
    pub blocks: Vec<Hash>,
    /// Round status
    pub status: RoundStatus,
    /// Validator votes
    pub votes: Vec<ValidatorVote>,
    /// Round signature
    pub signature: Option<Vec<u8>>,
}

/// Round status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoundStatus {
    /// Round is active
    Active,
    /// Round is being finalized
    Finalizing,
    /// Round is finalized
    Finalized,
    /// Round failed
    Failed,
}

/// Validator vote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorVote {
    /// Validator address
    pub validator: Address,
    /// Vote timestamp
    pub timestamp: FinDAGTime,
    /// Vote signature
    pub signature: Vec<u8>,
    /// Vote data
    pub vote_data: VoteData,
}

/// Vote data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteData {
    /// Round number
    pub round_number: u64,
    /// Block hashes being voted on
    pub block_hashes: Vec<Hash>,
    /// Vote type
    pub vote_type: VoteType,
}

/// Vote type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteType {
    /// Pre-commit vote
    PreCommit,
    /// Commit vote
    Commit,
    /// Finalize vote
    Finalize,
}

/// Block finalization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockFinalization {
    /// Block hash
    pub block_hash: Hash,
    /// Round number
    pub round_number: u64,
    /// Finalization timestamp
    pub finalization_timestamp: FinDAGTime,
    /// Finalization proof
    pub proof: FinalizationProof,
}

/// Finalization proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalizationProof {
    /// Validator signatures
    pub signatures: Vec<Vec<u8>>,
    /// Validator addresses
    pub validators: Vec<Address>,
    /// Proof timestamp
    pub timestamp: FinDAGTime,
}

/// Consensus metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusMetrics {
    /// Rounds per second
    pub rounds_per_sec: f64,
    /// Blocks per second
    pub blocks_per_sec: f64,
    /// Average round latency in milliseconds
    pub avg_round_latency_ms: f64,
    /// Average block finalization time in milliseconds
    pub avg_finalization_time_ms: f64,
    /// Active validators count
    pub active_validators_count: usize,
    /// Total votes in current round
    pub total_votes: usize,
    /// Consensus uptime in seconds
    pub uptime_seconds: u64,
}

/// Validator assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorAssignment {
    /// Validator address
    pub validator: Address,
    /// Assigned block number
    pub block_number: u64,
    /// Assignment timestamp
    pub assignment_timestamp: FinDAGTime,
    /// Assignment status
    pub status: AssignmentStatus,
}

/// Assignment status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssignmentStatus {
    /// Assignment is pending
    Pending,
    /// Assignment is active
    Active,
    /// Assignment is completed
    Completed,
    /// Assignment failed
    Failed,
}

/// Consensus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// Block interval in milliseconds
    pub block_interval_ms: u64,
    /// Round interval in milliseconds
    pub round_interval_ms: u64,
    /// Maximum block size in bytes
    pub max_block_size: usize,
    /// Maximum transactions per block
    pub max_transactions_per_block: usize,
    /// Minimum validators for consensus
    pub min_validators: usize,
    /// Consensus timeout in milliseconds
    pub consensus_timeout_ms: u64,
    /// Enable skip-when-empty optimization
    pub skip_when_empty: bool,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            block_interval_ms: 50,
            round_interval_ms: 250,
            max_block_size: 32 * 1024, // 32KB
            max_transactions_per_block: 1000,
            min_validators: 3,
            consensus_timeout_ms: 1000,
            skip_when_empty: true,
        }
    }
}

/// Consensus event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusEvent {
    /// New round started
    RoundStarted {
        round_number: u64,
        timestamp: FinDAGTime,
    },
    /// Round finalized
    RoundFinalized {
        round_number: u64,
        timestamp: FinDAGTime,
        blocks: Vec<Hash>,
    },
    /// New block produced
    BlockProduced {
        block_hash: Hash,
        block_number: u64,
        producer: Address,
        timestamp: FinDAGTime,
    },
    /// Block finalized
    BlockFinalized {
        block_hash: Hash,
        round_number: u64,
        timestamp: FinDAGTime,
    },
    /// Validator vote received
    VoteReceived {
        validator: Address,
        round_number: u64,
        vote_type: VoteType,
        timestamp: FinDAGTime,
    },
    /// Consensus error
    ConsensusError {
        error: String,
        timestamp: FinDAGTime,
    },
}

/// Consensus command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusCommand {
    /// Start consensus
    Start,
    /// Stop consensus
    Stop,
    /// Update configuration
    UpdateConfig(ConsensusConfig),
    /// Add validator
    AddValidator(Address),
    /// Remove validator
    RemoveValidator(Address),
    /// Force finalize round
    ForceFinalizeRound(u64),
} 
use ed25519_dalek::{Signature, PublicKey};
use crate::core::address::Address;

/// Represents a FinDAG transaction
#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub payload: Vec<u8>,
    pub findag_time: u64,      // FinDAG Time
    pub hashtimer: [u8; 32],  // HashTimer
    pub signature: Signature,  // Ed25519 signature
    pub public_key: PublicKey, // For signature verification
}

/// Represents a block in the FinDAG DAG
#[derive(Debug, Clone)]
pub struct Block {
    pub block_id: [u8; 32],           // Block hash/id
    pub parent_blocks: Vec<[u8; 32]>, // Parent block ids (DAG links)
    pub transactions: Vec<Transaction>,
    pub findag_time: u64,             // FinDAG Time
    pub hashtimer: [u8; 32],          // HashTimer
    pub proposer: Address,            // Block proposer address
    pub signature: Signature,         // Ed25519 signature
    pub public_key: PublicKey,        // For signature verification
}

/// Represents a round checkpoint in the FinDAG DAG
#[derive(Debug, Clone)]
pub struct Round {
    pub round_id: u64,                // Monotonically increasing round number
    pub parent_rounds: Vec<u64>,      // Parent round ids (DAG links)
    pub block_ids: Vec<[u8; 32]>,     // Blocks included in this round
    pub findag_time: u64,             // FinDAG Time
    pub hashtimer: [u8; 32],          // HashTimer
    pub proposer: Address,            // Round proposer address
    pub signature: Signature,         // Ed25519 signature
    pub public_key: PublicKey,        // For signature verification
} 
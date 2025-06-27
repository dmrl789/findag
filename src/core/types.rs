use ed25519_dalek::{Signature, PublicKey};
use crate::core::address::Address;

/// Unique identifier for a shard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShardId(pub u16); // Up to 65536 shards

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
    pub shard_id: ShardId,     // Shard assignment (default: ShardId(0))
    // Cross-shard transaction support
    pub source_shard: Option<u16>, // If present, indicates cross-shard tx
    pub dest_shard: Option<u16>,   // If present, indicates cross-shard tx
    // Cross-chain transaction support
    pub target_chain: Option<String>, // Target chain ID for cross-chain txs
    pub bridge_protocol: Option<String>, // Bridge protocol (e.g., IBC, custom)
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
    pub shard_id: ShardId,            // Shard assignment (default: ShardId(0))
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

/// Supported asset codes for FinDAG, including Euroclear-relevant assets.
pub const SUPPORTED_ASSETS: &[&str] = &[
    // Major fiat
    "EUR", "USD", "GBP", "JPY", "CHF", "SGD", "AED", "CNY",
    // Bonds (Eurobonds, sovereign, corporate)
    "BUND", "OAT", "BTP", "GILT", "UST", "JGB",
    // Money market instruments
    "T-BILL", "CP", "CD",
    // Precious metals
    "XAU", "XAG", "XPT", "XPD",
    // Example ISINs (Eurobonds, equities, funds)
    "XS1234567890", "FR0000120271", "BE0003796134", "DE0001135275",
    // Funds/ETFs (add as needed)
    "ETF1", "UCITS1",
    // Digital assets (optional)
    "BTC", "ETH", "USDT", "USDC",
]; 
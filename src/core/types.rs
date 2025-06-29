use ed25519_dalek::{Signature, PublicKey};
use crate::core::address::Address;
extern crate hex;
use serde::{Serialize, Deserialize};

/// Unique identifier for a shard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShardId(pub u16); // Up to 65536 shards

/// Represents a FinDAG transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Serializable version of Transaction for network transmission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableTransaction {
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub payload: Vec<u8>,
    pub findag_time: u64,
    pub hashtimer: [u8; 32],
    pub signature_bytes: Vec<u8>,
    pub public_key_bytes: Vec<u8>,
    pub shard_id: ShardId,
    pub source_shard: Option<u16>,
    pub dest_shard: Option<u16>,
    pub target_chain: Option<String>,
    pub bridge_protocol: Option<String>,
}

impl From<Transaction> for SerializableTransaction {
    fn from(tx: Transaction) -> Self {
        Self {
            from: tx.from,
            to: tx.to,
            amount: tx.amount,
            payload: tx.payload,
            findag_time: tx.findag_time,
            hashtimer: tx.hashtimer,
            signature_bytes: tx.signature.to_bytes().to_vec(),
            public_key_bytes: tx.public_key.to_bytes().to_vec(),
            shard_id: tx.shard_id,
            source_shard: tx.source_shard,
            dest_shard: tx.dest_shard,
            target_chain: tx.target_chain,
            bridge_protocol: tx.bridge_protocol,
        }
    }
}

impl TryFrom<SerializableTransaction> for Transaction {
    type Error = Box<dyn std::error::Error>;
    
    fn try_from(stx: SerializableTransaction) -> Result<Self, Self::Error> {
        let signature = Signature::from_bytes(&stx.signature_bytes)?;
        let public_key = PublicKey::from_bytes(&stx.public_key_bytes)?;
        
        Ok(Self {
            from: stx.from,
            to: stx.to,
            amount: stx.amount,
            payload: stx.payload,
            findag_time: stx.findag_time,
            hashtimer: stx.hashtimer,
            signature,
            public_key,
            shard_id: stx.shard_id,
            source_shard: stx.source_shard,
            dest_shard: stx.dest_shard,
            target_chain: stx.target_chain,
            bridge_protocol: stx.bridge_protocol,
        })
    }
}

/// Represents a block in the FinDAG DAG
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub merkle_root: Option<[u8; 32]>, // Merkle root of transactions (None for legacy blocks)
}

/// Serializable version of Block for network transmission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableBlock {
    pub block_id: [u8; 32],
    pub parent_blocks: Vec<[u8; 32]>,
    pub transactions: Vec<SerializableTransaction>,
    pub findag_time: u64,
    pub hashtimer: [u8; 32],
    pub proposer: Address,
    pub signature_bytes: Vec<u8>,
    pub public_key_bytes: Vec<u8>,
    pub shard_id: ShardId,
    pub merkle_root: Option<[u8; 32]>,
}

impl From<Block> for SerializableBlock {
    fn from(block: Block) -> Self {
        Self {
            block_id: block.block_id,
            parent_blocks: block.parent_blocks,
            transactions: block.transactions.into_iter().map(|tx| tx.into()).collect(),
            findag_time: block.findag_time,
            hashtimer: block.hashtimer,
            proposer: block.proposer,
            signature_bytes: block.signature.to_bytes().to_vec(),
            public_key_bytes: block.public_key.to_bytes().to_vec(),
            shard_id: block.shard_id,
            merkle_root: block.merkle_root,
        }
    }
}

impl TryFrom<SerializableBlock> for Block {
    type Error = Box<dyn std::error::Error>;
    
    fn try_from(sblock: SerializableBlock) -> Result<Self, Self::Error> {
        let signature = Signature::from_bytes(&sblock.signature_bytes)?;
        let public_key = PublicKey::from_bytes(&sblock.public_key_bytes)?;
        let transactions: Result<Vec<Transaction>, _> = sblock.transactions
            .into_iter()
            .map(|stx| stx.try_into())
            .collect();
        
        Ok(Self {
            block_id: sblock.block_id,
            parent_blocks: sblock.parent_blocks,
            transactions: transactions?,
            findag_time: sblock.findag_time,
            hashtimer: sblock.hashtimer,
            proposer: sblock.proposer,
            signature,
            public_key,
            shard_id: sblock.shard_id,
            merkle_root: sblock.merkle_root,
        })
    }
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

/// Serializable version of Round for network transmission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableRound {
    pub round_id: u64,
    pub parent_rounds: Vec<u64>,
    pub block_ids: Vec<[u8; 32]>,
    pub findag_time: u64,
    pub hashtimer: [u8; 32],
    pub proposer: Address,
    pub signature_bytes: Vec<u8>,
    pub public_key_bytes: Vec<u8>,
}

impl From<Round> for SerializableRound {
    fn from(round: Round) -> Self {
        Self {
            round_id: round.round_id,
            parent_rounds: round.parent_rounds,
            block_ids: round.block_ids,
            findag_time: round.findag_time,
            hashtimer: round.hashtimer,
            proposer: round.proposer,
            signature_bytes: round.signature.to_bytes().to_vec(),
            public_key_bytes: round.public_key.to_bytes().to_vec(),
        }
    }
}

impl TryFrom<SerializableRound> for Round {
    type Error = Box<dyn std::error::Error>;
    
    fn try_from(sround: SerializableRound) -> Result<Self, Self::Error> {
        let signature = Signature::from_bytes(&sround.signature_bytes)?;
        let public_key = PublicKey::from_bytes(&sround.public_key_bytes)?;
        
        Ok(Self {
            round_id: sround.round_id,
            parent_rounds: sround.parent_rounds,
            block_ids: sround.block_ids,
            findag_time: sround.findag_time,
            hashtimer: sround.hashtimer,
            proposer: sround.proposer,
            signature,
            public_key,
        })
    }
}

/// Supported assets for transactions and balance queries
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

impl Block {
    /// Validates that the Merkle root matches the transactions in this block
    pub fn validate_merkle_root(&self) -> bool {
        use crate::core::bridge::merkle_root;
        if let Some(expected_root) = self.merkle_root {
            let tx_hashes: Vec<String> = self.transactions.iter().map(|tx| {
                hex::encode(&tx.hashtimer)
            }).collect();
            let computed_hex = merkle_root(&tx_hashes);
            let mut computed = [0u8; 32];
            if hex::decode_to_slice(&computed_hex, &mut computed as &mut [u8]).is_ok() {
                computed == expected_root
            } else {
                false
            }
        } else {
            // No Merkle root to validate (legacy block)
            true
        }
    }
}

impl Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Transaction {{ from: {}, to: {}, amount: {}, hashtimer: {} }}", 
            self.from.as_str(), 
            self.to.as_str(), 
            self.amount,
            hex::encode(self.hashtimer)
        )
    }
}

/// Asset record for persistent storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRecord {
    pub asset_id: String,
    pub owner: String,
    pub status: String,        // "active", "pending", "unloaded"
    pub amount: String,
    pub history: Vec<AssetHistory>,
}

/// Asset history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetHistory {
    pub timestamp: String,
    pub from: Option<String>,
    pub to: Option<String>,
    pub action: String,        // "load", "transfer", "unload", "update"
} 
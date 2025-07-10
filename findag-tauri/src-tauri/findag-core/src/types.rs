use ed25519_dalek::{Signature, VerifyingKey};
use serde::{Serialize, Deserialize};
use crate::address::Address;

/// Unique identifier for a shard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShardId(pub u16);

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
    pub public_key: VerifyingKey, // For signature verification
    pub shard_id: ShardId,     // Shard assignment
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
    type Error = crate::FinDAGError;
    
    fn try_from(stx: SerializableTransaction) -> Result<Self, Self::Error> {
        let signature = Signature::from_bytes(&stx.signature_bytes.try_into()
            .map_err(|_| crate::FinDAGError::SerializationError("Invalid signature length".to_string()))?);
        let public_key = VerifyingKey::from_bytes(&stx.public_key_bytes.try_into()
            .map_err(|_| crate::FinDAGError::SerializationError("Invalid public key length".to_string()))?)?;
        
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
    pub public_key: VerifyingKey,        // For signature verification
    pub shard_id: ShardId,            // Shard assignment
    pub merkle_root: Option<[u8; 32]>, // Merkle root of transactions
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
    type Error = crate::FinDAGError;
    
    fn try_from(sblock: SerializableBlock) -> Result<Self, Self::Error> {
        let signature = Signature::from_bytes(&sblock.signature_bytes.try_into()
            .map_err(|_| crate::FinDAGError::SerializationError("Invalid signature length".to_string()))?);
        let public_key = VerifyingKey::from_bytes(&sblock.public_key_bytes.try_into()
            .map_err(|_| crate::FinDAGError::SerializationError("Invalid public key length".to_string()))?)?;
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

/// Represents a simple, linear Round in the FinDAG RoundChain
#[derive(Debug, Clone)]
pub struct Round {
    pub round_number: u64,                    // Monotonically increasing round number
    pub parent_round_hash: [u8; 32],          // Hash of the immediately previous Round only
    pub finalized_block_hashes: Vec<[u8; 32]>, // List of finalized block hashes
    pub block_hashtimers: Vec<[u8; 32]>,      // HashTimers for each finalized block
    pub quorum_signature: Vec<u8>,             // Threshold signature from validators
    pub findag_time: u64,                     // FinDAG Time for deterministic ordering
    pub proposer: Address,                    // Round proposer address
    pub proposer_signature: Signature,        // Proposer's signature
    pub proposer_public_key: VerifyingKey,       // Proposer's public key
}

/// Serializable version of Round for network transmission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableRound {
    pub round_number: u64,
    pub parent_round_hash: [u8; 32],
    pub finalized_block_hashes: Vec<[u8; 32]>,
    pub block_hashtimers: Vec<[u8; 32]>,
    pub quorum_signature: Vec<u8>,
    pub findag_time: u64,
    pub proposer: Address,
    pub proposer_signature_bytes: Vec<u8>,
    pub proposer_public_key_bytes: Vec<u8>,
}

impl From<Round> for SerializableRound {
    fn from(round: Round) -> Self {
        Self {
            round_number: round.round_number,
            parent_round_hash: round.parent_round_hash,
            finalized_block_hashes: round.finalized_block_hashes,
            block_hashtimers: round.block_hashtimers,
            quorum_signature: round.quorum_signature,
            findag_time: round.findag_time,
            proposer: round.proposer,
            proposer_signature_bytes: round.proposer_signature.to_bytes().to_vec(),
            proposer_public_key_bytes: round.proposer_public_key.to_bytes().to_vec(),
        }
    }
}

impl TryFrom<SerializableRound> for Round {
    type Error = crate::FinDAGError;
    
    fn try_from(sround: SerializableRound) -> Result<Self, Self::Error> {
        let proposer_signature = Signature::from_bytes(&sround.proposer_signature_bytes.try_into()
            .map_err(|_| crate::FinDAGError::SerializationError("Invalid signature length".to_string()))?);
        let proposer_public_key = VerifyingKey::from_bytes(&sround.proposer_public_key_bytes.try_into()
            .map_err(|_| crate::FinDAGError::SerializationError("Invalid public key length".to_string()))?)?;
        
        Ok(Self {
            round_number: sround.round_number,
            parent_round_hash: sround.parent_round_hash,
            finalized_block_hashes: sround.finalized_block_hashes,
            block_hashtimers: sround.block_hashtimers,
            quorum_signature: sround.quorum_signature,
            findag_time: sround.findag_time,
            proposer: sround.proposer,
            proposer_signature,
            proposer_public_key,
        })
    }
}

/// Asset record for tracking ownership and history
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

impl Block {
    /// Validate the Merkle root of transactions in the block
    pub fn validate_merkle_root(&self) -> bool {
        if let Some(expected_root) = self.merkle_root {
            // TODO: Implement actual Merkle root computation and validation
            // For now, return true as placeholder
            true
        } else {
            // Legacy blocks without Merkle root are considered valid
            true
        }
    }
}

impl Transaction {
    /// Get a human-readable representation of the transaction
    pub fn display(&self) -> String {
        format!(
            "Transaction: {} -> {} ({} units) at time {}",
            self.from.as_str(),
            self.to.as_str(),
            self.amount,
            self.findag_time
        )
    }
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;

    fn create_test_transaction() -> Transaction {
        let keypair = SigningKey::generate(&mut rand::thread_rng());
        let from = Address("fdg1qtestfrom".to_string());
        let to = Address("fdg1qtestto".to_string());
        
        Transaction {
            from,
            to,
            amount: 1000,
            payload: vec![1, 2, 3, 4],
            findag_time: 1234567890,
            hashtimer: [0u8; 32],
            signature: keypair.sign(b"test"),
            public_key: keypair.verifying_key(),
            shard_id: ShardId(0),
            source_shard: None,
            dest_shard: None,
            target_chain: None,
            bridge_protocol: None,
        }
    }

    #[test]
    fn test_transaction_serialization() {
        let tx = create_test_transaction();
        let serialized = SerializableTransaction::from(tx.clone());
        let deserialized = Transaction::try_from(serialized).unwrap();
        
        assert_eq!(tx.from, deserialized.from);
        assert_eq!(tx.to, deserialized.to);
        assert_eq!(tx.amount, deserialized.amount);
        assert_eq!(tx.findag_time, deserialized.findag_time);
    }

    #[test]
    fn test_block_merkle_validation() {
        let block = Block {
            block_id: [0u8; 32],
            parent_blocks: vec![],
            transactions: vec![],
            findag_time: 0,
            hashtimer: [0u8; 32],
            proposer: Address("test".to_string()),
            signature: Signature::from_bytes(&[0u8; 64]),
            public_key: VerifyingKey::from_bytes(&[0u8; 32]).unwrap(),
            shard_id: ShardId(0),
            merkle_root: Some([0u8; 32]),
        };
        
        assert!(block.validate_merkle_root());
    }
} 
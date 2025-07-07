// roundchain.rs
// FinDAG Simple Linear RoundChain Implementation
// 
// This Round implementation uses a simple linear chain.
// No Round DAG logic — finality is strict, ordered, and single-parent.

use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::core::address::Address;
use crate::core::types::Block;
use crate::consensus::validator_set::{ValidatorSet, Committee};
use sha2::{Sha256, Digest};

/// Represents a simple, linear Round in the FinDAG RoundChain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round {
    pub round_number: u64,                    // Monotonically increasing round number
    pub parent_round_hash: [u8; 32],          // Hash of the immediately previous Round only
    pub finalized_block_hashes: Vec<[u8; 32]>, // List of finalized block hashes
    pub block_hashtimers: Vec<[u8; 32]>,      // HashTimers for each finalized block
    pub quorum_signature: Vec<u8>,             // Threshold signature from validators
    pub findag_time: u64,                     // FinDAG Time for deterministic ordering
    pub proposer: Address,                    // Round proposer address
    pub proposer_signature: Signature,        // Proposer's signature
    pub proposer_public_key: VerifyingKey,    // Proposer's public key
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
    type Error = Box<dyn std::error::Error>;
    
    fn try_from(sround: SerializableRound) -> Result<Self, Self::Error> {
        let proposer_signature = Signature::from_bytes(&sround.proposer_signature_bytes.try_into().unwrap());
        let proposer_public_key = VerifyingKey::from_bytes(&sround.proposer_public_key_bytes.try_into().unwrap())?;
        
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

/// RoundChain manages the simple linear chain of Rounds
pub struct RoundChain {
    pub rounds: HashMap<u64, Round>,          // round_number -> Round
    pub latest_round_number: u64,             // Latest finalized round
    pub genesis_round_hash: [u8; 32],         // Hash of genesis round
    pub validator_set: ValidatorSet,          // Validator set for quorum signatures
}

impl RoundChain {
    /// Create a new RoundChain
    pub fn new(validator_set: ValidatorSet) -> Self {
        let genesis_round_hash = [0u8; 32]; // Genesis round hash
        
        Self {
            rounds: HashMap::new(),
            latest_round_number: 0,
            genesis_round_hash,
            validator_set,
        }
    }

    /// Create a new Round with the specified finalized blocks
    pub fn create_round(
        &mut self,
        round_number: u64,
        finalized_blocks: Vec<Block>,
        findag_time: u64,
        proposer_keypair: &SigningKey,
        proposer_address: Address,
    ) -> Result<Round, String> {
        // Validate round number is sequential
        if round_number != self.latest_round_number + 1 {
            return Err(format!("Invalid round number: expected {}, got {}", 
                             self.latest_round_number + 1, round_number));
        }

        // Get parent round hash
        let parent_round_hash = if round_number == 1 {
            self.genesis_round_hash
        } else {
            let parent_round = self.rounds.get(&(round_number - 1))
                .ok_or("Parent round not found")?;
            self.compute_round_hash(parent_round)
        };

        // Extract block hashes and hashtimers
        let finalized_block_hashes: Vec<[u8; 32]> = finalized_blocks.iter()
            .map(|block| block.block_id)
            .collect();
        let block_hashtimers: Vec<[u8; 32]> = finalized_blocks.iter()
            .map(|block| block.hashtimer)
            .collect();

        // Create round content for signing
        let round_content = self.create_round_content(
            round_number,
            &parent_round_hash,
            &finalized_block_hashes,
            &block_hashtimers,
            findag_time,
        );

        // Sign the round content
        let proposer_signature = proposer_keypair.sign(&round_content);

        // Create the round
        let round = Round {
            round_number,
            parent_round_hash,
            finalized_block_hashes,
            block_hashtimers,
            quorum_signature: Vec::new(), // Will be filled by quorum signing
            findag_time,
            proposer: proposer_address,
            proposer_signature,
            proposer_public_key: proposer_keypair.verifying_key(),
        };

        Ok(round)
    }

    /// Add a Round to the chain (before quorum signing)
    pub fn add_round(&mut self, round: Round) -> Result<(), String> {
        // Validate round number is sequential
        if round.round_number != self.latest_round_number + 1 {
            return Err(format!("Invalid round number: expected {}, got {}", 
                             self.latest_round_number + 1, round.round_number));
        }

        // Validate parent round hash
        if round.round_number == 1 {
            if round.parent_round_hash != self.genesis_round_hash {
                return Err("Invalid genesis round parent hash".to_string());
            }
        } else {
            let parent_round = self.rounds.get(&(round.round_number - 1))
                .ok_or("Parent round not found")?;
            let expected_parent_hash = self.compute_round_hash(parent_round);
            if round.parent_round_hash != expected_parent_hash {
                return Err("Invalid parent round hash".to_string());
            }
        }

        // Store the round
        let round_number = round.round_number;
        self.rounds.insert(round_number, round);
        self.latest_round_number = round_number;

        Ok(())
    }

    /// Sign a Round with quorum signature (threshold signature)
    pub fn sign_round_with_quorum(
        &mut self,
        round_number: u64,
        committee: &Committee,
        signatures: &[(Address, Signature)],
    ) -> Result<(), String> {
        // Get round data first to avoid borrow checker issues
        let round_data = {
            let round = self.rounds.get(&round_number)
                .ok_or("Round not found")?;
            (
                round.round_number,
                round.parent_round_hash,
                round.finalized_block_hashes.clone(),
                round.block_hashtimers.clone(),
                round.findag_time,
            )
        };

        // Validate we have enough signatures for quorum
        let quorum_threshold = self.validator_set.quorum_manager.config.min_quorum_size;
        if signatures.len() < quorum_threshold {
            return Err(format!("Insufficient signatures: {} < {}", 
                             signatures.len(), quorum_threshold));
        }

        // Verify all signatures are from committee members
        for (validator_addr, signature) in signatures {
            if !committee.validators.contains(validator_addr) {
                return Err(format!("Signature from non-committee validator: {}", 
                                 validator_addr.as_str()));
            }

            // Verify signature
            if let Some(validator) = self.validator_set.get_validator(validator_addr) {
                let round_content = self.create_round_content(
                    round_data.0,
                    &round_data.1,
                    &round_data.2,
                    &round_data.3,
                    round_data.4,
                );

                if validator.public_key.verify(&round_content, signature).is_err() {
                    return Err(format!("Invalid signature from validator: {}", 
                                     validator_addr.as_str()));
                }
            }
        }

        // Create threshold signature (simplified - in production use proper threshold signing)
        let mut signature_data = Vec::new();
        for (_, signature) in signatures {
            signature_data.extend_from_slice(&signature.to_bytes());
        }
        let signature_hash = Sha256::digest(&signature_data);
        let quorum_signature = signature_hash.to_vec();

        // Now set the quorum_signature field
        let round = self.rounds.get_mut(&round_number).ok_or("Round not found")?;
        round.quorum_signature = quorum_signature;

        Ok(())
    }

    /// Verify a Round's quorum signature
    pub fn verify_round_quorum(&self, round: &Round) -> bool {
        // In a real implementation, this would verify the threshold signature
        // For now, we'll just check that it's not empty
        !round.quorum_signature.is_empty()
    }

    /// Get the latest finalized round
    pub fn get_latest_round(&self) -> Option<&Round> {
        self.rounds.get(&self.latest_round_number)
    }

    /// Get a specific round by number
    pub fn get_round(&self, round_number: u64) -> Option<&Round> {
        self.rounds.get(&round_number)
    }

    /// Get all finalized block hashes up to a specific round
    pub fn get_finalized_blocks_up_to(&self, round_number: u64) -> Vec<[u8; 32]> {
        let mut all_blocks = Vec::new();
        for i in 1..=round_number {
            if let Some(round) = self.rounds.get(&i) {
                all_blocks.extend_from_slice(&round.finalized_block_hashes);
            }
        }
        all_blocks
    }

    /// Check if a block is finalized in any round
    pub fn is_block_finalized(&self, block_hash: &[u8; 32]) -> bool {
        for round in self.rounds.values() {
            if round.finalized_block_hashes.contains(block_hash) {
                return true;
            }
        }
        false
    }

    /// Get the round number where a block was finalized
    pub fn get_block_finalization_round(&self, block_hash: &[u8; 32]) -> Option<u64> {
        for round in self.rounds.values() {
            if round.finalized_block_hashes.contains(block_hash) {
                return Some(round.round_number);
            }
        }
        None
    }

    /// Compute the hash of a Round
    fn compute_round_hash(&self, round: &Round) -> [u8; 32] {
        let content = self.create_round_content(
            round.round_number,
            &round.parent_round_hash,
            &round.finalized_block_hashes,
            &round.block_hashtimers,
            round.findag_time,
        );
        let hash = Sha256::digest(&content);
        let mut result = [0u8; 32];
        result.copy_from_slice(&hash);
        result
    }

    /// Create the content to be signed for a Round
    pub fn create_round_content(
        &self,
        round_number: u64,
        parent_round_hash: &[u8; 32],
        finalized_block_hashes: &[[u8; 32]],
        block_hashtimers: &[[u8; 32]],
        findag_time: u64,
    ) -> Vec<u8> {
        let mut content = Vec::new();
        
        // Round number
        content.extend_from_slice(&round_number.to_be_bytes());
        
        // Parent round hash
        content.extend_from_slice(parent_round_hash);
        
        // Number of blocks
        content.extend_from_slice(&(finalized_block_hashes.len() as u32).to_be_bytes());
        
        // Block hashes
        for hash in finalized_block_hashes {
            content.extend_from_slice(hash);
        }
        
        // Block hashtimers
        for hashtimer in block_hashtimers {
            content.extend_from_slice(hashtimer);
        }
        
        // FinDAG Time
        content.extend_from_slice(&findag_time.to_be_bytes());
        
        content
    }

    /// Get the total number of finalized blocks
    pub fn get_total_finalized_blocks(&self) -> usize {
        self.rounds.values()
            .map(|round| round.finalized_block_hashes.len())
            .sum()
    }

    /// Get statistics about the RoundChain
    pub fn get_statistics(&self) -> RoundChainStats {
        let total_rounds = self.rounds.len();
        let total_blocks = self.get_total_finalized_blocks();
        let average_blocks_per_round = if total_rounds > 0 {
            total_blocks as f64 / total_rounds as f64
        } else {
            0.0
        };

        RoundChainStats {
            total_rounds,
            total_finalized_blocks: total_blocks,
            average_blocks_per_round,
            latest_round_number: self.latest_round_number,
        }
    }
}

/// Statistics about the RoundChain
#[derive(Debug, Clone)]
pub struct RoundChainStats {
    pub total_rounds: usize,
    pub total_finalized_blocks: usize,
    pub average_blocks_per_round: f64,
    pub latest_round_number: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::address::generate_address;
    use crate::consensus::validator_set::ValidatorSet;

    fn create_test_block(block_id: [u8; 32], hashtimer: [u8; 32]) -> Block {
        let (keypair, address) = generate_address();
        Block {
            block_id,
            parent_blocks: vec![],
            transactions: vec![],
            findag_time: 1000,
            hashtimer,
            proposer: address,
            signature: keypair.sign(&block_id),
            public_key: VerifyingKey::from(&keypair),
            shard_id: crate::core::types::ShardId(0),
            merkle_root: None,
        }
    }

    #[test]
    fn test_round_creation() {
        let validator_set = ValidatorSet::new();
        let mut roundchain = RoundChain::new(validator_set);
        let (keypair, address) = generate_address();

        let block1 = create_test_block([1u8; 32], [10u8; 32]);
        let block2 = create_test_block([2u8; 32], [20u8; 32]);
        let finalized_blocks = vec![block1, block2];

        let round = roundchain.create_round(1, finalized_blocks, 1000, &keypair, address.clone())
            .expect("Failed to create round");

        assert_eq!(round.round_number, 1);
        assert_eq!(round.parent_round_hash, [0u8; 32]); // Genesis
        assert_eq!(round.finalized_block_hashes.len(), 2);
        assert_eq!(round.findag_time, 1000);
        assert_eq!(round.proposer, address);
    }

    #[test]
    fn test_sequential_round_numbers() {
        let validator_set = ValidatorSet::new();
        let mut roundchain = RoundChain::new(validator_set);
        let (keypair, address) = generate_address();

        // Create first round
        let block1 = create_test_block([1u8; 32], [10u8; 32]);
        let round1 = roundchain.create_round(1, vec![block1], 1000, &keypair, address.clone())
            .expect("Failed to create round 1");
        roundchain.add_round(round1).expect("Failed to add round 1");

        // Try to create round 3 (should fail)
        let block3 = create_test_block([3u8; 32], [30u8; 32]);
        let result = roundchain.create_round(3, vec![block3], 1000, &keypair, address.clone());
        assert!(result.is_err());

        // Create round 2 (should succeed)
        let block2 = create_test_block([2u8; 32], [20u8; 32]);
        let round2 = roundchain.create_round(2, vec![block2], 1000, &keypair, address.clone())
            .expect("Failed to create round 2");
        roundchain.add_round(round2).expect("Failed to add round 2");

        assert_eq!(roundchain.latest_round_number, 2);
    }

    #[test]
    fn test_block_finalization_tracking() {
        let validator_set = ValidatorSet::new();
        let mut roundchain = RoundChain::new(validator_set);
        let (keypair, address) = generate_address();

        let block1 = create_test_block([1u8; 32], [10u8; 32]);
        let block2 = create_test_block([2u8; 32], [20u8; 32]);
        let finalized_blocks = vec![block1.clone(), block2.clone()];

        let round = roundchain.create_round(1, finalized_blocks, 1000, &keypair, address.clone())
            .expect("Failed to create round");
        roundchain.add_round(round).expect("Failed to add round");

        assert!(roundchain.is_block_finalized(&block1.block_id));
        assert!(roundchain.is_block_finalized(&block2.block_id));
        assert!(!roundchain.is_block_finalized(&[99u8; 32]));

        assert_eq!(roundchain.get_block_finalization_round(&block1.block_id), Some(1));
        assert_eq!(roundchain.get_block_finalization_round(&block2.block_id), Some(1));
        assert_eq!(roundchain.get_block_finalization_round(&[99u8; 32]), None);
    }
} 
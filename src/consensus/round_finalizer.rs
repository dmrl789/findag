// round_finalizer.rs
// FinDAG deterministic validator assignment and round finalization

use ed25519_dalek::{Keypair, PublicKey, Signature, Signer};
use std::collections::HashMap;
use crate::consensus::validator_set::{ValidatorSet, ValidatorInfo, ValidatorStatus};

/// Represents a validator in the network
#[derive(Clone, Debug)]
pub struct Validator {
    pub id: String,
    pub public_key: PublicKey,
}

/// Manages validator assignments and signing
pub struct RoundFinalizer<'a> {
    pub validator_set: &'a ValidatorSet,
    pub local_keypair: Keypair,
}

#[derive(Debug, Clone)]
pub struct RoundCommitment {
    pub round_number: u64,
    pub finalized_hash: [u8; 32],
    pub signer_id: String,
    pub signature: Signature,
}

impl<'a> RoundFinalizer<'a> {
    pub fn new(validator_set: &'a ValidatorSet, local_keypair: Keypair) -> Self {
        Self {
            validator_set,
            local_keypair,
        }
    }

    /// Determine if local node is the finalizer for a given round
    pub fn is_finalizer_for_round(&self, round_number: u64) -> bool {
        let active_validators = self.validator_set.active_validators();
        if active_validators.is_empty() { return false; }
        let assigned_index = (round_number as usize) % active_validators.len();
        let local_id = self.local_id();
        active_validators[assigned_index].address.as_str() == local_id
    }

    /// Produce a signed round commitment if this node is the assigned finalizer
    pub fn finalize_round(&self, round_number: u64, hash: [u8; 32]) -> Option<RoundCommitment> {
        if !self.is_finalizer_for_round(round_number) {
            return None;
        }
        let message = Self::build_message(round_number, &hash);
        let signature = self.local_keypair.sign(&message);
        Some(RoundCommitment {
            round_number,
            finalized_hash: hash,
            signer_id: self.local_id(),
            signature,
        })
    }

    fn build_message(round_number: u64, hash: &[u8; 32]) -> Vec<u8> {
        let mut msg = round_number.to_le_bytes().to_vec();
        msg.extend_from_slice(hash);
        msg
    }

    pub fn local_id(&self) -> String {
        // Use address string as ID
        let address = crate::core::address::Address::from_public_key(&self.local_keypair.public);
        address.as_str().to_string()
    }

    /// Verify a signed round commitment
    pub fn verify_commitment(&self, commitment: &RoundCommitment) -> bool {
        let message = Self::build_message(commitment.round_number, &commitment.finalized_hash);
        if let Some(validator) = self.validator_set.get_validator(&commitment.signer_id) {
            if validator.status == ValidatorStatus::Active {
                return validator.public_key.verify_strict(&message, &commitment.signature).is_ok();
            }
        }
        false
    }

    /// Cross-shard consensus protocol (scaffold)
    /// TODO: Implement cross-shard transaction finality and receipt handling
    /// - Coordinate with other shards for atomic commit
    /// - Track and verify cross-shard receipts
    /// - Ensure both shards reach consensus on the transaction outcome
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::Signer;
    use rand::rngs::OsRng;

    fn create_validator(id: &str) -> (Validator, Keypair) {
        let keypair = Keypair::generate(&mut OsRng);
        let validator = Validator {
            id: id.to_string(),
            public_key: keypair.public,
        };
        (validator, keypair)
    }

    #[test]
    fn test_round_assignment_and_signature() {
        let (v1, k1) = create_validator("v1");
        let (v2, _) = create_validator("v2");

        let rf = RoundFinalizer::new(vec![v1.clone(), v2], k1);

        let rnum = 0;
        let hash = [0u8; 32];

        assert!(rf.is_finalizer_for_round(rnum));
        let commitment = rf.finalize_round(rnum, hash).unwrap();

        assert_eq!(commitment.round_number, rnum);
        assert!(rf.verify_commitment(&commitment));
    }
} 
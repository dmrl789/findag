// round_finalizer.rs
// FinDAG deterministic validator assignment and round finalization

use ed25519_dalek::{Keypair, PublicKey, Signature, Signer};
use std::collections::HashMap;

/// Represents a validator in the network
#[derive(Clone, Debug)]
pub struct Validator {
    pub id: String,
    pub public_key: PublicKey,
}

/// Manages validator assignments and signing
pub struct RoundFinalizer {
    pub validators: Vec<Validator>,
    pub validator_index: HashMap<String, usize>,
    pub local_keypair: Keypair,
}

#[derive(Debug, Clone)]
pub struct RoundCommitment {
    pub round_number: u64,
    pub finalized_hash: [u8; 32],
    pub signer_id: String,
    pub signature: Signature,
}

impl RoundFinalizer {
    pub fn new(validators: Vec<Validator>, local_keypair: Keypair) -> Self {
        let validator_index = validators
            .iter()
            .enumerate()
            .map(|(i, v)| (v.id.clone(), i))
            .collect();

        Self {
            validators,
            validator_index,
            local_keypair,
        }
    }

    /// Determine if local node is the finalizer for a given round
    pub fn is_finalizer_for_round(&self, round_number: u64) -> bool {
        let assigned_index = (round_number as usize) % self.validators.len();
        let local_id = self.local_id();
        self.validators[assigned_index].id == local_id
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
        // Can be BLAKE2b or base64-encoded public key
        base64::encode(self.local_keypair.public.as_bytes())
    }

    /// Verify a signed round commitment
    pub fn verify_commitment(&self, commitment: &RoundCommitment) -> bool {
        let message = Self::build_message(commitment.round_number, &commitment.finalized_hash);

        self.validator_index
            .get(&commitment.signer_id)
            .and_then(|&i| {
                self.validators
                    .get(i)
                    .map(|v| v.public_key.verify_strict(&message, &commitment.signature).is_ok())
            })
            .unwrap_or(false)
    }
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
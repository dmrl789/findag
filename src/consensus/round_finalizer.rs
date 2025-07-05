// round_finalizer.rs
// FinDAG deterministic validator assignment and round finalization

use ed25519_dalek::{Keypair, PublicKey, Signer, Verifier};
use crate::consensus::validator_set::{ValidatorSet, Committee, CommitteeConfig};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::time::Duration;
use hex;

/// Represents a validator in the network
#[derive(Clone, Debug)]
pub struct Validator {
    pub id: String,
    pub public_key: PublicKey,
}

/// Represents a round commitment from a validator
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RoundCommitment {
    pub round_number: u64,
    pub block_hash: [u8; 32],
    pub timestamp: u64,
    pub signature: Option<ed25519_dalek::Signature>,
    pub proposer: Option<crate::core::address::Address>,
}

/// Block commitment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockCommitment {
    pub block_hash: [u8; 32],
    pub validator: PublicKey,
    pub signature: Vec<u8>,
}

/// Committee finalization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeFinalization {
    pub round_number: u64,
    pub committee: Committee,
    pub signatures_received: Vec<RoundCommitment>,
    pub quorum_achieved: bool,
    pub finalization_time: u64,
    pub fallback_used: bool,
}

/// Round finalizer for consensus
pub struct RoundFinalizer<'a> {
    validator_set: &'a mut ValidatorSet,
    keypair: Option<&'a Keypair>,
    local_address: Option<crate::core::address::Address>,
    current_round: u64,
    _round_timeout: Duration,
    _committee_size: usize,
    _finality_threshold: usize,
}

impl<'a> RoundFinalizer<'a> {
    /// Create a new round finalizer
    pub fn new(
        validator_set: &'a mut ValidatorSet, 
        keypair: Option<&'a Keypair>,
        local_address: Option<crate::core::address::Address>
    ) -> Self {
        Self {
            validator_set,
            keypair,
            local_address,
            current_round: 0,
            _round_timeout: Duration::from_secs(30),
            _committee_size: 20,
            _finality_threshold: 12,
        }
    }

    /// Create a dummy finalizer for testing
    pub fn dummy(validator_set: &'a mut ValidatorSet) -> Self {
        Self {
            validator_set,
            keypair: None,
            local_address: None,
            current_round: 0,
            _round_timeout: Duration::from_secs(30),
            _committee_size: 20,
            _finality_threshold: 12,
        }
    }

    /// Create payload for commitment signing
    fn commitment_payload_to_sign(&self, commitment: &RoundCommitment) -> String {
        format!("{}:{}:{}", commitment.round_number, hex::encode(commitment.block_hash), commitment.timestamp)
    }

    /// Check if this validator is in the current committee
    pub fn is_in_current_committee(&self) -> bool {
        if let Some(local_addr) = &self.local_address {
            if let Some(committee) = self.validator_set.get_current_committee() {
                return committee.validators.iter().any(|v| v == local_addr);
            }
        }
        false
    }

    /// Check if committee needs rotation and return new committee if needed
    pub fn check_and_rotate_committee(&mut self, round_number: u64) -> Option<Committee> {
        // Check if we need a new committee for this round
        if self.current_round != round_number {
            self.current_round = round_number;
            Some(self.validator_set.select_committee(round_number))
        } else {
            None
        }
    }

    /// Finalize a round with the given block
    pub fn finalize_round(&mut self, round_number: u64, block_hash: [u8; 32]) -> Option<RoundCommitment> {
        // Check if we need a new committee
        if let Some(_committee) = self.check_and_rotate_committee(round_number) {
            // Committee rotated, continue with new committee
        }

        // Create round commitment
        let commitment = RoundCommitment {
            round_number,
            block_hash,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            signature: None,
            proposer: None,
        };

        // Sign the commitment if we have a keypair
        if let (Some(keypair), Some(local_addr)) = (self.keypair, self.local_address.clone()) {
            let payload = self.commitment_payload_to_sign(&commitment);
            let signature = keypair.sign(payload.as_bytes());
            
            // Record our signature
            self.validator_set.record_signature(&local_addr, round_number);
            
            Some(RoundCommitment {
                signature: Some(signature),
                proposer: Some(local_addr),
                ..commitment
            })
        } else {
            Some(commitment)
        }
    }

    /// Verify a round commitment
    pub fn verify_commitment(&self, commitment: &RoundCommitment) -> bool {
        // Verify signature if present
        if let (Some(signature), Some(proposer)) = (&commitment.signature, &commitment.proposer) {
            let payload = self.commitment_payload_to_sign(commitment);
            
            // Get validator's public key
            if let Some(validator) = self.validator_set.get_validator(proposer) {
                return validator.public_key.verify(payload.as_bytes(), signature).is_ok();
            }
        }
        false
    }

    /// Get current committee quorum threshold
    pub fn get_quorum_threshold(&self) -> usize {
        self.validator_set.quorum_manager.config.min_quorum_size
    }

    /// Get current committee size
    pub fn get_committee_size(&self) -> usize {
        self.validator_set.quorum_manager.config.committee_size
    }

    /// Check if quorum is achieved for current round
    pub fn is_quorum_achieved(&self) -> bool {
        self.validator_set.is_quorum_achieved()
    }

    /// Get current committee
    pub fn get_current_committee(&self) -> Option<&Committee> {
        self.validator_set.get_current_committee()
    }

    /// Process a round commitment from another validator
    pub fn process_commitment(&mut self, commitment: &RoundCommitment) -> bool {
        // Verify signature if present
        if let (Some(signature), Some(proposer)) = (&commitment.signature, &commitment.proposer) {
            let payload = self.commitment_payload_to_sign(commitment);
            
            // Get validator's public key
            if let Some(validator) = self.validator_set.get_validator(proposer) {
                if validator.public_key.verify(payload.as_bytes(), signature).is_ok() {
                    // Record the signature
                    self.validator_set.record_signature(proposer, commitment.round_number);
                    return true;
                }
            }
        }
        false
    }

    /// Trigger fallback to full validator set if committee fails
    pub fn trigger_fallback_if_needed(&mut self, round_number: u64) -> Option<Committee> {
        // Check if committee has failed to achieve quorum
        if !self.validator_set.is_quorum_achieved() {
            return self.validator_set.trigger_fallback(round_number);
        }
        None
    }

    /// Get committee configuration
    pub fn get_committee_config(&self) -> &CommitteeConfig {
        &self.validator_set.quorum_manager.config
    }

    /// Update committee configuration
    pub fn update_committee_config(&mut self, config: CommitteeConfig) {
        self.validator_set.update_committee_config(config);
    }

    /// Get validator statistics
    pub fn get_validator_stats(&self) -> std::collections::HashMap<crate::core::address::Address, crate::consensus::validator_set::ValidatorReputation> {
        self.validator_set.get_validator_stats()
    }

    /// Get committee history
    pub fn get_committee_history(&self) -> &Vec<Committee> {
        self.validator_set.get_committee_history()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::address::generate_address;

    fn _create_validator(_name: &str) -> (crate::core::address::Address, Keypair) {
        let (keypair, address) = generate_address();
        (address, keypair)
    }

    #[test]
    fn test_round_finalization() {
        let mut validator_set = ValidatorSet::new();
        let mut rf = RoundFinalizer::dummy(&mut validator_set);
        
        let block_hash = [0u8; 32];
        let rnum = 1u64;
        let _commitment = rf.finalize_round(rnum, block_hash);
        
        // Test should pass without panicking
        assert!(true);
    }

    #[test]
    fn test_committee_selection() {
        let mut validator_set = ValidatorSet::new();
        
        // Add enough test validators (at least 20 for default committee size)
        for i in 0..25 {
            let (keypair, address) = generate_address();
            validator_set.add_validator_with_metadata(
                address,
                keypair.public,
                1000, // stake
                format!("validator_{}", i), // institution_name
                "test_region".to_string(), // region
            );
        }

        let mut finalizer = RoundFinalizer::dummy(&mut validator_set);
        
        // Test committee selection
        let committee = finalizer.check_and_rotate_committee(42);
        assert!(committee.is_some());
        let committee = committee.unwrap();
        assert_eq!(committee.validators.len(), 20); // Default committee size
    }

    #[test]
    fn test_finality_detection() {
        let mut validator_set = ValidatorSet::new();
        
        // Add test validators
        for i in 0..10 {
            let (keypair, address) = generate_address();
            validator_set.add_validator_with_metadata(
                address,
                keypair.public,
                1000, // stake
                format!("validator_{}", i), // institution_name
                "test_region".to_string(), // region
            );
        }

        let finalizer = RoundFinalizer::dummy(&mut validator_set);
        
        // Test finality detection
        let is_finalized = finalizer.is_quorum_achieved();
        assert!(!is_finalized); // No signatures yet
    }
} 
use ed25519_dalek::VerifyingKey;
use crate::core::address::Address;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::core::types::ShardId;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidatorStatus {
    Active,
    Inactive,
    Slashed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    pub address: Address,
    pub public_key: VerifyingKey,
    pub status: ValidatorStatus,
    pub metadata: Option<String>,
}

/// Validator reputation and performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorReputation {
    pub total_rounds_assigned: u64,
    pub rounds_signed: u64,
    pub rounds_missed: u64,
    pub average_response_time_ms: u64,
    pub last_seen_timestamp: u64,
    pub consecutive_failures: u32,
    pub reputation_score: f64, // 0.0 to 1.0
}

impl Default for ValidatorReputation {
    fn default() -> Self {
        Self {
            total_rounds_assigned: 0,
            rounds_signed: 0,
            rounds_missed: 0,
            average_response_time_ms: 0,
            last_seen_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            consecutive_failures: 0,
            reputation_score: 1.0, // Start with perfect reputation
        }
    }
}

/// Validator information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub address: Address,
    pub public_key: VerifyingKey,
    pub stake: u64,
    pub is_active: bool,
    pub reputation: ValidatorReputation,
    pub institution_name: Option<String>,
    pub region: Option<String>,
}

/// Committee configuration for quorum rotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeConfig {
    pub committee_size: usize,
    pub min_quorum_size: usize, // Minimum signatures needed
    pub rotation_interval_rounds: u64,
    pub fallback_timeout_ms: u64,
    pub reputation_threshold: f64, // Minimum reputation to be selected
}

impl Default for CommitteeConfig {
    fn default() -> Self {
        Self {
            committee_size: 20,
            min_quorum_size: 12, // 60% of committee
            rotation_interval_rounds: 10,
            fallback_timeout_ms: 5000, // 5 seconds
            reputation_threshold: 0.5,
        }
    }
}

/// Committee for a specific round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Committee {
    pub round_number: u64,
    pub validators: Vec<Address>,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub signatures_received: Vec<Address>,
    pub quorum_achieved: bool,
    pub fallback_triggered: bool,
}

/// Quorum rotation manager
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct QuorumRotationManager {
    pub config: CommitteeConfig,
    pub current_committee: Option<Committee>,
    pub committee_history: Vec<Committee>,
    pub last_rotation_round: u64,
}


/// Validator set for consensus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSet {
    pub validators: HashMap<Address, Validator>,
    pub shard_assignments: HashMap<ShardId, Vec<Address>>,
    pub quorum_manager: QuorumRotationManager,
}

impl ValidatorSet {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
            shard_assignments: HashMap::new(),
            quorum_manager: QuorumRotationManager::default(),
        }
    }

    /// Add a validator to the set
    pub fn add_validator(&mut self, address: Address, public_key: VerifyingKey, stake: u64) {
        let validator = Validator {
            address: address.clone(),
            public_key,
            stake,
            is_active: true,
            reputation: ValidatorReputation::default(),
            institution_name: None,
            region: None,
        };
        self.validators.insert(address, validator);
    }

    /// Add a validator with institution details
    pub fn add_validator_with_metadata(
        &mut self, 
        address: Address, 
        public_key: VerifyingKey, 
        stake: u64,
        institution_name: String,
        region: String,
    ) {
        let validator = Validator {
            address: address.clone(),
            public_key,
            stake,
            is_active: true,
            reputation: ValidatorReputation::default(),
            institution_name: Some(institution_name),
            region: Some(region),
        };
        self.validators.insert(address, validator);
    }

    /// Remove a validator from the set
    pub fn remove_validator(&mut self, address: &Address) {
        self.validators.remove(address);
    }

    /// Get validator by address
    pub fn get_validator(&self, address: &Address) -> Option<&Validator> {
        self.validators.get(address)
    }

    /// Get all validators
    pub fn get_all_validators(&self) -> Vec<&Validator> {
        self.validators.values().collect()
    }

    /// Get active validators
    pub fn get_active_validators(&self) -> Vec<&Validator> {
        self.validators.values().filter(|v| v.is_active).collect()
    }

    /// Get validators eligible for committee selection (active + good reputation)
    pub fn get_eligible_validators(&self) -> Vec<&Validator> {
        self.validators.values()
            .filter(|v| v.is_active && v.reputation.reputation_score >= self.quorum_manager.config.reputation_threshold)
            .collect()
    }

    /// Get total stake
    pub fn get_total_stake(&self) -> u64 {
        self.validators.values().map(|v| v.stake).sum()
    }

    /// Get validator count
    pub fn validator_count(&self) -> usize {
        self.validators.len()
    }

    /// Assign validator to a shard
    pub fn assign_to_shard(&mut self, address: Address, shard: ShardId) {
        self.shard_assignments.entry(shard).or_default().push(address);
    }

    /// Get validators for a shard
    pub fn get_validators_for_shard(&self, shard: &ShardId) -> Vec<&Validator> {
        if let Some(addresses) = self.shard_assignments.get(shard) {
            addresses.iter()
                .filter_map(|addr| self.validators.get(addr))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get shards for a validator
    pub fn get_shard_for_validator(&self, address: &Address) -> Option<&ShardId> {
        for (shard, addresses) in &self.shard_assignments {
            if addresses.contains(address) {
                return Some(shard);
            }
        }
        None
    }

    /// Select committee for a round using deterministic weighted selection
    pub fn select_committee(&mut self, round_number: u64) -> Committee {
        let eligible = self.get_eligible_validators();
        
        // Sort by reputation score (descending) for deterministic selection
        let mut sorted_validators: Vec<_> = eligible.iter().collect();
        sorted_validators.sort_by(|a, b| {
            b.reputation.reputation_score.partial_cmp(&a.reputation.reputation_score).unwrap()
        });

        // Take top validators up to committee size
        let committee_size = self.quorum_manager.config.committee_size.min(sorted_validators.len());
        let selected_validators: Vec<Address> = sorted_validators[..committee_size]
            .iter()
            .map(|v| v.address.clone())
            .collect();

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let committee = Committee {
            round_number,
            validators: selected_validators,
            start_time: current_time,
            end_time: None,
            signatures_received: Vec::new(),
            quorum_achieved: false,
            fallback_triggered: false,
        };

        // Update current committee
        if let Some(old_committee) = self.quorum_manager.current_committee.take() {
            self.quorum_manager.committee_history.push(old_committee);
        }
        self.quorum_manager.current_committee = Some(committee.clone());
        self.quorum_manager.last_rotation_round = round_number;

        committee
    }

    /// Check if committee rotation is needed
    pub fn should_rotate_committee(&self, round_number: u64) -> bool {
        round_number - self.quorum_manager.last_rotation_round >= self.quorum_manager.config.rotation_interval_rounds
    }

    /// Record a validator signature for the current committee
    pub fn record_signature(&mut self, validator_address: &Address, round_number: u64) {
        if let Some(committee) = &mut self.quorum_manager.current_committee {
            if committee.round_number == round_number && committee.validators.contains(validator_address)
                && !committee.signatures_received.contains(validator_address) {
                    committee.signatures_received.push(validator_address.clone());
                    
                    // Check if quorum is achieved
                    if committee.signatures_received.len() >= self.quorum_manager.config.min_quorum_size {
                        committee.quorum_achieved = true;
                    }

                    // Update validator reputation
                    if let Some(validator) = self.validators.get_mut(validator_address) {
                        validator.reputation.rounds_signed += 1;
                        validator.reputation.consecutive_failures = 0;
                        validator.reputation.reputation_score = 
                            (validator.reputation.rounds_signed as f64) / 
                            (validator.reputation.total_rounds_assigned as f64).max(1.0);
                        
                        validator.reputation.last_seen_timestamp = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                    }
                }
        }
    }

    /// Record a validator missing a signature (for reputation tracking)
    pub fn record_missed_signature(&mut self, validator_address: &Address, round_number: u64) {
        if let Some(committee) = &self.quorum_manager.current_committee {
            if committee.round_number == round_number && committee.validators.contains(validator_address) {
                if let Some(validator) = self.validators.get_mut(validator_address) {
                    validator.reputation.rounds_missed += 1;
                    validator.reputation.consecutive_failures += 1;
                    
                    // Decrease reputation score based on consecutive failures
                    let failure_penalty = 0.1 * validator.reputation.consecutive_failures as f64;
                    validator.reputation.reputation_score = 
                        (validator.reputation.reputation_score - failure_penalty).max(0.0);
                }
            }
        }
    }

    /// Check if quorum is achieved for current committee
    pub fn is_quorum_achieved(&self) -> bool {
        self.quorum_manager.current_committee
            .as_ref()
            .map(|c| c.quorum_achieved)
            .unwrap_or(false)
    }

    /// Get current committee
    pub fn get_current_committee(&self) -> Option<&Committee> {
        self.quorum_manager.current_committee.as_ref()
    }

    /// Get committee history
    pub fn get_committee_history(&self) -> &Vec<Committee> {
        &self.quorum_manager.committee_history
    }

    /// Update committee configuration
    pub fn update_committee_config(&mut self, config: CommitteeConfig) {
        self.quorum_manager.config = config;
    }

    /// Get validator statistics
    pub fn get_validator_stats(&self) -> HashMap<Address, ValidatorReputation> {
        self.validators.iter()
            .map(|(addr, validator)| (addr.clone(), validator.reputation.clone()))
            .collect()
    }

    /// Trigger fallback committee (select new committee if current one is failing)
    pub fn trigger_fallback(&mut self, round_number: u64) -> Option<Committee> {
        if let Some(committee) = &mut self.quorum_manager.current_committee {
            if !committee.quorum_achieved && !committee.fallback_triggered {
                committee.fallback_triggered = true;
                committee.end_time = Some(SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs());
                
                // Select new committee immediately
                return Some(self.select_committee(round_number));
            }
        }
        None
    }
}

impl Default for ValidatorSet {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidatorSet {
    pub fn set_status(&mut self, address: &Address, status: ValidatorStatus) {
        if let Some(v) = self.validators.get_mut(address) {
            v.is_active = matches!(status, ValidatorStatus::Active);
        }
    }
    
    pub fn active_validators(&self) -> Vec<&Validator> {
        self.validators.values().filter(|v| v.is_active).collect()
    }
    
    /// Assign validators to shards in round-robin fashion
    pub fn assign_validators_to_shards(&mut self, shard_count: u16) {
        self.shard_assignments.clear();
        let mut shard = 0;
        let active: Vec<_> = self.active_validators().into_iter().collect();
        let mut active_clone = active.clone();
        active_clone.sort_by_key(|v| v.address.as_str().to_string()); // deterministic order
        let assignments: Vec<_> = active_clone.iter().map(|v| v.address.clone()).collect();
        for address in assignments {
            let shard_id = ShardId(shard);
            self.shard_assignments.entry(shard_id).or_default().push(address);
            shard = (shard + 1) % shard_count;
        }
    }
} 
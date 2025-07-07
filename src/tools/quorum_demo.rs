// quorum_demo.rs
// Demonstration tool for FinDAG's quorum rotation system

use crate::consensus::validator_set::ValidatorSet;
use crate::core::address::generate_address;
use ed25519_dalek::SigningKey;

pub struct QuorumDemo {
    validator_set: ValidatorSet,
    _demo_validators: Vec<(SigningKey, crate::core::address::Address)>,
}

impl Default for QuorumDemo {
    fn default() -> Self {
        Self::new()
    }
}

impl QuorumDemo {
    pub fn new() -> Self {
        let mut validator_set = ValidatorSet::new();
        
        // Create demo validators
        let mut demo_validators = Vec::new();
        let institutions = vec![
            "JPMorgan Chase", "Bank of America", "Wells Fargo", "Citigroup",
            "Goldman Sachs", "Morgan Stanley", "U.S. Bancorp", "PNC Financial",
            "Capital One", "TD Bank", "Charles Schwab", "American Express",
            "Citizens Financial", "Fifth Third Bank", "KeyCorp", "Regions Financial",
            "BB&T", "SunTrust", "State Street", "Northern Trust"
        ];

        for (i, institution) in institutions.iter().enumerate() {
            let (signing_key, address) = generate_address();
            validator_set.add_validator_with_metadata(
                address.clone(),
                signing_key.verifying_key(),
                1000 + (i as u64 * 100), // Varying stakes
                institution.to_string(),
                "US".to_string(),
            );
            demo_validators.push((signing_key, address));
        }

        Self {
            validator_set,
            _demo_validators: demo_validators,
        }
    }

    pub fn run_demo(&mut self) {
        println!("🚀 FinDAG Quorum Rotation Demo");
        println!("================================\n");

        // Show initial configuration
        self.show_configuration();
        println!();

        // Demo 1: Basic committee selection
        self.demo_committee_selection();
        println!();

        // Demo 2: Reputation system
        self.demo_reputation_system();
        println!();

        // Demo 3: Committee rotation
        self.demo_committee_rotation();
        println!();

        // Demo 4: Fallback mechanism
        self.demo_fallback_mechanism();
        println!();

        // Demo 5: Performance comparison
        self.demo_performance_comparison();
    }

    fn show_configuration(&self) {
        println!("📋 Current Configuration:");
        let config = &self.validator_set.quorum_manager.config;
        println!("  • Committee Size: {}", config.committee_size);
        println!("  • Quorum Threshold: {} ({}%)", 
            config.min_quorum_size, 
            (config.min_quorum_size as f64 / config.committee_size as f64 * 100.0) as u32
        );
        println!("  • Rotation Interval: {} rounds", config.rotation_interval_rounds);
        println!("  • Fallback Timeout: {}ms", config.fallback_timeout_ms);
        println!("  • Reputation Threshold: {:.1}%", config.reputation_threshold * 100.0);
        println!("  • Total Validators: {}", self.validator_set.validator_count());
    }

    fn demo_committee_selection(&mut self) {
        println!("🎯 Demo 1: Committee Selection");
        println!("-------------------------------");

        // Select committee for round 1
        let committee = self.validator_set.select_committee(1);
        
        println!("Selected committee for Round 1:");
        println!("  • Committee Size: {}", committee.validators.len());
        println!("  • Required Signatures: {}", self.validator_set.quorum_manager.config.min_quorum_size);
        
        println!("\nCommittee Members:");
        for (i, validator_addr) in committee.validators.iter().enumerate() {
            if let Some(validator) = self.validator_set.get_validator(validator_addr) {
                println!("  {}. {} ({}) - Reputation: {:.2}", 
                    i + 1,
                    validator.institution_name.as_ref().unwrap_or(&"Unknown".to_string()),
                    &validator_addr.as_str()[..8],
                    validator.reputation.reputation_score
                );
            }
        }
    }

    fn demo_reputation_system(&mut self) {
        println!("⭐ Demo 2: Reputation System");
        println!("----------------------------");

        // Get committee members first
        let committee_validators: Vec<_> = {
            let committee = self.validator_set.get_current_committee().unwrap();
            committee.validators.clone()
        };
        
        println!("Simulating validator performance...");
        
        for (i, validator_addr) in committee_validators.iter().enumerate() {
            if i < 12 { // First 12 validators sign (achieving quorum)
                self.validator_set.record_signature(validator_addr, 1);
                println!("  ✅ {} signed", &validator_addr.as_str()[..8]);
            } else { // Last 8 validators miss the signature
                self.validator_set.record_missed_signature(validator_addr, 1);
                println!("  ❌ {} missed", &validator_addr.as_str()[..8]);
            }
        }

        println!("\nReputation Impact:");
        for validator_addr in &committee_validators {
            if let Some(validator) = self.validator_set.get_validator(validator_addr) {
                let status = if validator.reputation.consecutive_failures > 0 {
                    "📉"
                } else {
                    "📈"
                };
                println!("  {} {}: {:.2} (failures: {})", 
                    status,
                    validator.institution_name.as_ref().unwrap_or(&"Unknown".to_string()),
                    validator.reputation.reputation_score,
                    validator.reputation.consecutive_failures
                );
            }
        }

        println!("\nQuorum Status: {}", 
            if self.validator_set.is_quorum_achieved() { "✅ ACHIEVED" } else { "❌ NOT ACHIEVED" }
        );
    }

    fn demo_committee_rotation(&mut self) {
        println!("🔄 Demo 3: Committee Rotation");
        println!("-----------------------------");

        // Get current committee info first
        let current_committee_info = {
            let committee = self.validator_set.get_current_committee().unwrap();
            (committee.round_number, committee.validators.clone(), committee.quorum_achieved)
        };
        
        println!("Current Committee (Round {}):", current_committee_info.0);
        println!("  • Members: {}", current_committee_info.1.len());
        println!("  • Quorum Achieved: {}", current_committee_info.2);

        // Simulate rotation after 10 rounds
        println!("\nSimulating rotation after {} rounds...", 
            self.validator_set.quorum_manager.config.rotation_interval_rounds
        );

        let new_round = current_committee_info.0 + self.validator_set.quorum_manager.config.rotation_interval_rounds;
        let new_committee = self.validator_set.select_committee(new_round);

        println!("New Committee (Round {}):", new_committee.round_number);
        println!("  • Members: {}", new_committee.validators.len());
        println!("  • Rotation Reason: Scheduled rotation");

        // Show some overlap
        let current_members: std::collections::HashSet<_> = current_committee_info.1.iter().collect();
        let new_members: std::collections::HashSet<_> = new_committee.validators.iter().collect();
        let overlap = current_members.intersection(&new_members).count();
        
        println!("  • Overlap with previous committee: {}/{}", overlap, new_committee.validators.len());
    }

    fn demo_fallback_mechanism(&mut self) {
        println!("🆘 Demo 4: Fallback Mechanism");
        println!("-----------------------------");

        // Get committee info first
        let committee_info = {
            let committee = self.validator_set.get_current_committee().unwrap();
            (committee.validators.clone(), committee.round_number)
        };
        
        println!("Simulating committee failure scenario...");
        println!("Committee has {} members, needs {} signatures", 
            committee_info.0.len(), 
            self.validator_set.quorum_manager.config.min_quorum_size
        );

        // Only 8 out of 20 sign (not enough for quorum)
        for (i, validator_addr) in committee_info.0.iter().enumerate() {
            if i < 8 {
                self.validator_set.record_signature(validator_addr, committee_info.1);
                println!("  ✅ {} signed", &validator_addr.as_str()[..8]);
            } else {
                self.validator_set.record_missed_signature(validator_addr, committee_info.1);
                println!("  ❌ {} missed", &validator_addr.as_str()[..8]);
            }
        }

        println!("\nQuorum Status: {}", 
            if self.validator_set.is_quorum_achieved() { "✅ ACHIEVED" } else { "❌ NOT ACHIEVED" }
        );

        // Trigger fallback
        if let Some(fallback_committee) = self.validator_set.trigger_fallback(committee_info.1 + 1) {
            println!("\n🔄 Fallback Committee Triggered!");
            println!("New Committee (Round {}):", fallback_committee.round_number);
            println!("  • Members: {}", fallback_committee.validators.len());
            println!("  • Reason: Previous committee failed to achieve quorum");
        }
    }

    fn demo_performance_comparison(&self) {
        println!("📊 Demo 5: Performance Comparison");
        println!("--------------------------------");

        let total_validators = self.validator_set.validator_count();
        let committee_size = self.validator_set.quorum_manager.config.committee_size;
        let quorum_size = self.validator_set.quorum_manager.config.min_quorum_size;

        println!("Traditional 2/3+1 vs FinDAG Quorum Rotation:");
        println!();
        
        println!("Traditional BFT (2/3+1):");
        println!("  • Total Validators: {total_validators}");
        println!("  • Required Signatures: {}", (total_validators * 2) / 3 + 1);
        println!("  • Network Messages: O({}²) = ~{}", total_validators, total_validators * total_validators);
        println!("  • Estimated Finality: 10-30 seconds");
        println!();

        println!("FinDAG Quorum Rotation:");
        println!("  • Total Validators: {total_validators}");
        println!("  • Committee Size: {committee_size}");
        println!("  • Required Signatures: {} ({}%)", quorum_size, 
            (quorum_size as f64 / committee_size as f64 * 100.0) as u32
        );
        println!("  • Network Messages: O({}²) = ~{}", committee_size, committee_size * committee_size);
        println!("  • Estimated Finality: 2-5 seconds");
        println!();

        let efficiency_gain = (total_validators * total_validators) as f64 / (committee_size * committee_size) as f64;
        println!("🚀 Efficiency Improvement:");
        println!("  • Network Messages: {}x reduction", efficiency_gain as u32);
        println!("  • Finality Speed: 5-15x faster");
        println!("  • Scalability: Handles {total_validators} validators efficiently");
    }
}

pub fn run_quorum_demo() {
    let mut demo = QuorumDemo::new();
    demo.run_demo();
} 
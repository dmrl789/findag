use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use std::time::{SystemTime, UNIX_EPOCH};

/// Governance proposal types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    AddValidator { address: String, public_key: String },
    RemoveValidator { address: String },
    SlashValidator { address: String, reason: String },
    ParameterChange { parameter: String, new_value: String },
    UpgradeProtocol { version: String, description: String },
    EmergencyPause { reason: String },
    EmergencyResume { reason: String },
}

/// Governance proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub proposal_type: ProposalType,
    pub timestamp: u64,
    pub voting_start: u64,
    pub voting_end: u64,
    pub status: ProposalStatus,
    pub required_quorum: u64,
    pub required_approval_percentage: f64,
}

/// Proposal status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalStatus {
    Pending,    // Created but not yet active
    Active,     // Currently being voted on
    Passed,     // Approved by voters
    Failed,     // Rejected or didn't meet quorum
    Expired,    // Voting period ended without quorum
    Executed,   // Proposal has been enacted
    Cancelled,  // Cancelled by proposer or admin
}

/// Vote with additional metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub proposal_id: String,
    pub voter: String,
    pub vote: bool, // true for yes, false for no
    pub timestamp: u64,
    pub stake_weight: u64, // Voting power based on stake
    pub reason: Option<String>, // Optional reason for vote
}

/// Voting results for a proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingResults {
    pub total_votes: u64,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub total_stake_voted: u64,
    pub yes_stake: u64,
    pub no_stake: u64,
    pub quorum_achieved: bool,
    pub approval_percentage: f64,
    pub passed: bool,
}

/// Governance configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceConfig {
    pub min_proposal_duration: u64, // Minimum voting period in seconds
    pub max_proposal_duration: u64, // Maximum voting period in seconds
    pub min_quorum_percentage: f64, // Minimum percentage of total stake required
    pub min_approval_percentage: f64, // Minimum percentage of yes votes required
    pub proposal_fee: u64, // Fee required to submit a proposal
    pub emergency_threshold: u64, // Stake required for emergency proposals
    pub max_active_proposals: usize, // Maximum number of active proposals
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self {
            min_proposal_duration: 86400, // 24 hours
            max_proposal_duration: 604800, // 7 days
            min_quorum_percentage: 0.4, // 40% of total stake
            min_approval_percentage: 0.6, // 60% of yes votes
            proposal_fee: 1000, // 1000 base units
            emergency_threshold: 100000, // 100k base units
            max_active_proposals: 10,
        }
    }
}

/// Governance analytics and monitoring data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GovernanceAnalytics {
    pub total_proposals_created: u64,
    pub total_proposals_passed: u64,
    pub total_proposals_failed: u64,
    pub total_votes_cast: u64,
    pub average_voting_participation: f64,
    pub most_active_voters: Vec<VoterActivity>,
    pub proposal_type_distribution: HashMap<String, u64>,
    pub voting_trends: Vec<VotingTrend>,
    pub recent_activity: Vec<GovernanceEvent>,
}

/// Voter activity tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterActivity {
    pub voter: String,
    pub total_votes: u64,
    pub total_stake_voted: u64,
    pub participation_rate: f64,
    pub last_vote_timestamp: u64,
}

/// Voting trend data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingTrend {
    pub period: String, // "daily", "weekly", "monthly"
    pub total_votes: u64,
    pub participation_rate: f64,
    pub approval_rate: f64,
    pub timestamp: u64,
}

/// Governance event for activity tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceEvent {
    pub event_type: String, // "proposal_created", "vote_cast", "proposal_passed", "proposal_failed"
    pub timestamp: u64,
    pub actor: String,
    pub details: String,
    pub proposal_id: Option<String>,
}

/// Governance state
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct GovernanceState {
    pub proposals: HashMap<String, Proposal>,
    pub votes: HashMap<String, Vec<Vote>>,
    pub active_proposals: Vec<String>,
    pub executed_proposals: Vec<String>,
    pub config: GovernanceConfig,
    pub total_stake: u64,
    pub proposal_counter: u64,
    pub analytics: GovernanceAnalytics,
    pub events: Vec<GovernanceEvent>,
    pub voter_activity: HashMap<String, VoterActivity>,
}

impl GovernanceState {
    /// Create a new proposal with enhanced tracking
    pub fn create_proposal(
        &mut self,
        proposer: String,
        title: String,
        description: String,
        proposal_type: ProposalType,
        duration: Option<u64>,
    ) -> Result<String, String> {
        // Check if proposer can create proposals
        if !self.can_create_proposal(&proposer) {
            return Err("Insufficient stake to create proposal".to_string());
        }

        // Check active proposal limit
        if self.active_proposals.len() >= self.config.max_active_proposals {
            return Err("Maximum number of active proposals reached".to_string());
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let voting_duration = duration.unwrap_or(self.config.min_proposal_duration);
        if voting_duration < self.config.min_proposal_duration || voting_duration > self.config.max_proposal_duration {
            return Err("Invalid voting duration".to_string());
        }

        self.proposal_counter += 1;
        let proposal_id = format!("proposal_{}", self.proposal_counter);

        let proposal = Proposal {
            id: proposal_id.clone(),
            title,
            description,
            proposer: proposer.clone(),
            proposal_type: proposal_type.clone(),
            timestamp: now,
            voting_start: now,
            voting_end: now + voting_duration,
            status: ProposalStatus::Active,
            required_quorum: (self.total_stake as f64 * self.config.min_quorum_percentage) as u64,
            required_approval_percentage: self.config.min_approval_percentage,
        };

        self.proposals.insert(proposal_id.clone(), proposal);
        self.active_proposals.push(proposal_id.clone());
        self.votes.insert(proposal_id.clone(), Vec::new());

        // Update analytics
        self.analytics.total_proposals_created += 1;
        
        // Update proposal type distribution
        let type_key = match &proposal_type {
            ProposalType::AddValidator { .. } => "add_validator",
            ProposalType::RemoveValidator { .. } => "remove_validator",
            ProposalType::SlashValidator { .. } => "slash_validator",
            ProposalType::ParameterChange { .. } => "parameter_change",
            ProposalType::UpgradeProtocol { .. } => "upgrade_protocol",
            ProposalType::EmergencyPause { .. } => "emergency_pause",
            ProposalType::EmergencyResume { .. } => "emergency_resume",
        };
        *self.analytics.proposal_type_distribution.entry(type_key.to_string()).or_insert(0) += 1;

        // Record event
        self.record_event(
            "proposal_created".to_string(),
            proposer,
            format!("Created proposal: {}", proposal_id),
            Some(proposal_id.clone()),
        );

        Ok(proposal_id)
    }

    /// Submit a vote on a proposal with enhanced tracking
    pub fn submit_vote(
        &mut self,
        proposal_id: &str,
        voter: String,
        vote: bool,
        stake_weight: u64,
        reason: Option<String>,
    ) -> Result<(), String> {
        // Check if proposal exists and is active
        let proposal = self.proposals.get_mut(proposal_id)
            .ok_or("Proposal not found")?;

        if proposal.status != ProposalStatus::Active {
            return Err("Proposal is not active for voting".to_string());
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if now > proposal.voting_end {
            return Err("Voting period has ended".to_string());
        }

        // Check if voter has already voted
        let votes = self.votes.get_mut(proposal_id).unwrap();
        if votes.iter().any(|v| v.voter == voter) {
            return Err("Voter has already voted on this proposal".to_string());
        }

        // Create and record vote
        let vote_record = Vote {
            proposal_id: proposal_id.to_string(),
            voter: voter.clone(),
            vote,
            timestamp: now,
            stake_weight,
            reason,
        };

        votes.push(vote_record);

        // Update analytics
        self.analytics.total_votes_cast += 1;
        self.update_voter_activity(&voter, stake_weight);

        // Record event
        self.record_event(
            "vote_cast".to_string(),
            voter,
            format!("Voted {} on proposal {}", if vote { "YES" } else { "NO" }, proposal_id),
            Some(proposal_id.to_string()),
        );

        // Check if proposal should be finalized
        self.check_proposal_finalization(proposal_id);

        Ok(())
    }

    /// Check if a proposal should be finalized based on voting results
    pub fn check_proposal_finalization(&mut self, proposal_id: &str) {
        let results = self.calculate_voting_results(proposal_id);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if let Some(proposal) = self.proposals.get_mut(proposal_id) {
            // Check if voting period has ended
            if now > proposal.voting_end {
                if results.quorum_achieved && results.passed {
                    proposal.status = ProposalStatus::Passed;
                    self.analytics.total_proposals_passed += 1;
                    self.record_event(
                        "proposal_passed".to_string(),
                        "system".to_string(),
                        format!("Proposal {} passed with {}% approval", proposal_id, (results.approval_percentage * 100.0) as u64),
                        Some(proposal_id.to_string()),
                    );
                } else if results.quorum_achieved && !results.passed {
                    proposal.status = ProposalStatus::Failed;
                    self.analytics.total_proposals_failed += 1;
                    self.record_event(
                        "proposal_failed".to_string(),
                        "system".to_string(),
                        format!("Proposal {} failed with {}% approval", proposal_id, (results.approval_percentage * 100.0) as u64),
                        Some(proposal_id.to_string()),
                    );
                } else {
                    proposal.status = ProposalStatus::Expired;
                    self.record_event(
                        "proposal_expired".to_string(),
                        "system".to_string(),
                        format!("Proposal {} expired without quorum", proposal_id),
                        Some(proposal_id.to_string()),
                    );
                }

                // Remove from active proposals
                if let Some(pos) = self.active_proposals.iter().position(|id| id == proposal_id) {
                    self.active_proposals.remove(pos);
                }
            } else if results.quorum_achieved && results.passed {
                // Early finalization if quorum and approval are met
                proposal.status = ProposalStatus::Passed;
                self.analytics.total_proposals_passed += 1;
                self.record_event(
                    "proposal_passed_early".to_string(),
                    "system".to_string(),
                    format!("Proposal {} passed early with {}% approval", proposal_id, (results.approval_percentage * 100.0) as u64),
                    Some(proposal_id.to_string()),
                );
                if let Some(pos) = self.active_proposals.iter().position(|id| id == proposal_id) {
                    self.active_proposals.remove(pos);
                }
            }
        }
    }

    /// Calculate voting results for a proposal
    pub fn calculate_voting_results(&self, proposal_id: &str) -> VotingResults {
        let empty_votes = Vec::new();
        let votes = self.votes.get(proposal_id).unwrap_or(&empty_votes);
        
        let total_votes = votes.len() as u64;
        let yes_votes = votes.iter().filter(|v| v.vote).count() as u64;
        let no_votes = votes.iter().filter(|v| !v.vote).count() as u64;
        
        let total_stake_voted: u64 = votes.iter().map(|v| v.stake_weight).sum();
        let yes_stake: u64 = votes.iter().filter(|v| v.vote).map(|v| v.stake_weight).sum();
        let no_stake: u64 = votes.iter().filter(|v| !v.vote).map(|v| v.stake_weight).sum();

        let quorum_achieved = total_stake_voted as f64 >= self.config.min_quorum_percentage * self.total_stake as f64;
        let approval_percentage = if total_stake_voted > 0 {
            yes_stake as f64 / total_stake_voted as f64
        } else {
            0.0
        };
        let passed = quorum_achieved && approval_percentage >= self.config.min_approval_percentage;

        VotingResults {
            total_votes,
            yes_votes,
            no_votes,
            total_stake_voted,
            yes_stake,
            no_stake,
            quorum_achieved,
            approval_percentage,
            passed,
        }
    }

    /// Execute a passed proposal
    pub fn execute_proposal(&mut self, proposal_id: &str) -> Result<(), String> {
        let proposal = self.proposals.get(proposal_id)
            .ok_or("Proposal not found")?;

        if proposal.status != ProposalStatus::Passed {
            return Err("Proposal has not been passed".to_string());
        }

        // Mark as executed
        if let Some(proposal) = self.proposals.get_mut(proposal_id) {
            proposal.status = ProposalStatus::Executed;
        }

        self.executed_proposals.push(proposal_id.to_string());

        Ok(())
    }

    /// Cancel a proposal (only proposer or emergency threshold can cancel)
    pub fn cancel_proposal(&mut self, proposal_id: &str, canceller: &str, canceller_stake: u64) -> Result<(), String> {
        let proposal = self.proposals.get(proposal_id)
            .ok_or("Proposal not found")?;

        if proposal.status != ProposalStatus::Active && proposal.status != ProposalStatus::Pending {
            return Err("Proposal cannot be cancelled".to_string());
        }

        // Check if canceller is proposer or has emergency threshold
        if canceller != proposal.proposer && canceller_stake < self.config.emergency_threshold {
            return Err("Insufficient authority to cancel proposal".to_string());
        }

        // Mark as cancelled
        if let Some(proposal) = self.proposals.get_mut(proposal_id) {
            proposal.status = ProposalStatus::Cancelled;
        }

        // Remove from active proposals
        if let Some(pos) = self.active_proposals.iter().position(|id| id == proposal_id) {
            self.active_proposals.remove(pos);
        }

        Ok(())
    }

    /// Update governance configuration (requires governance proposal)
    pub fn update_config(&mut self, new_config: GovernanceConfig) {
        self.config = new_config;
    }

    /// Update total stake (called when validator stakes change)
    pub fn update_total_stake(&mut self, new_total_stake: u64) {
        self.total_stake = new_total_stake;
    }

    /// Check if an address can create proposals
    pub fn can_create_proposal(&self, _proposer: &str) -> bool {
        // This would typically check the proposer's stake
        // For now, allow any validator to create proposals
        true
    }

    /// Get all active proposals
    pub fn get_active_proposals(&self) -> Vec<&Proposal> {
        self.active_proposals.iter()
            .filter_map(|id| self.proposals.get(id))
            .collect()
    }

    /// List all proposals (active and inactive)
    pub fn list_proposals(&self) -> Vec<&Proposal> {
        self.proposals.values().collect()
    }

    /// Get proposal by ID
    pub fn get_proposal(&self, proposal_id: &str) -> Option<&Proposal> {
        self.proposals.get(proposal_id)
    }

    /// Get votes for a proposal
    pub fn get_proposal_votes(&self, proposal_id: &str) -> Option<&Vec<Vote>> {
        self.votes.get(proposal_id)
    }

    /// Get governance statistics
    pub fn get_statistics(&self) -> HashMap<String, u64> {
        let mut stats = HashMap::new();
        stats.insert("total_proposals".to_string(), self.proposals.len() as u64);
        stats.insert("active_proposals".to_string(), self.active_proposals.len() as u64);
        stats.insert("executed_proposals".to_string(), self.executed_proposals.len() as u64);
        stats.insert("total_stake".to_string(), self.total_stake);
        stats
    }

    /// Record a governance event
    fn record_event(&mut self, event_type: String, actor: String, details: String, proposal_id: Option<String>) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let event = GovernanceEvent {
            event_type,
            timestamp: now,
            actor,
            details,
            proposal_id,
        };

        self.events.push(event);
        
        // Keep only last 1000 events to prevent memory bloat
        if self.events.len() > 1000 {
            self.events.remove(0);
        }
    }

    /// Update voter activity tracking
    fn update_voter_activity(&mut self, voter: &str, stake_weight: u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let activity = self.voter_activity.entry(voter.to_string()).or_insert(VoterActivity {
            voter: voter.to_string(),
            total_votes: 0,
            total_stake_voted: 0,
            participation_rate: 0.0,
            last_vote_timestamp: 0,
        });

        activity.total_votes += 1;
        activity.total_stake_voted += stake_weight;
        activity.last_vote_timestamp = now;
        
        // Calculate participation rate (simplified - would need total possible votes)
        activity.participation_rate = (activity.total_votes as f64 / 100.0).min(1.0);
    }

    /// Get governance analytics
    pub fn get_analytics(&self) -> &GovernanceAnalytics {
        &self.analytics
    }

    /// Get recent governance events
    pub fn get_recent_events(&self, limit: usize) -> Vec<&GovernanceEvent> {
        let start = if self.events.len() > limit {
            self.events.len() - limit
        } else {
            0
        };
        self.events[start..].iter().collect()
    }

    /// Get top active voters
    pub fn get_top_voters(&self, limit: usize) -> Vec<&VoterActivity> {
        let mut voters: Vec<&VoterActivity> = self.voter_activity.values().collect();
        voters.sort_by(|a, b| b.total_votes.cmp(&a.total_votes));
        voters.truncate(limit);
        voters
    }

    /// Calculate voting participation rate
    pub fn calculate_participation_rate(&self) -> f64 {
        if self.total_stake == 0 {
            return 0.0;
        }
        
        let total_voted_stake: u64 = self.votes.values()
            .flatten()
            .map(|vote| vote.stake_weight)
            .sum();
        
        total_voted_stake as f64 / self.total_stake as f64
    }

    /// Get proposal success rate
    pub fn get_success_rate(&self) -> f64 {
        let total_completed = self.analytics.total_proposals_passed + self.analytics.total_proposals_failed;
        if total_completed == 0 {
            return 0.0;
        }
        self.analytics.total_proposals_passed as f64 / total_completed as f64
    }
} 
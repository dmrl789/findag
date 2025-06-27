use serde::{Serialize, Deserialize};
use crate::core::address::Address;
use crate::consensus::validator_set::ValidatorStatus;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProposalType {
    AddValidator { address: Address, public_key: Vec<u8>, metadata: Option<String> },
    RemoveValidator { address: Address },
    SlashValidator { address: Address },
    AddAsset { code: String, description: String, metadata: Option<String> },
    RemoveAsset { code: String },
    // Extendable for other protocol changes
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub proposer: Address,
    pub proposal_type: ProposalType,
    pub status: ProposalStatus,
    pub votes_for: HashSet<Address>,
    pub votes_against: HashSet<Address>,
    pub created_at: u64, // FinDAG Time or unix timestamp
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceState {
    pub proposals: HashMap<u64, Proposal>,
    pub next_proposal_id: u64,
}

impl GovernanceState {
    pub fn new() -> Self {
        Self { proposals: HashMap::new(), next_proposal_id: 1 }
    }
    pub fn submit_proposal(&mut self, proposer: Address, proposal_type: ProposalType, created_at: u64) -> u64 {
        let id = self.next_proposal_id;
        self.next_proposal_id += 1;
        let proposal = Proposal {
            id,
            proposer,
            proposal_type,
            status: ProposalStatus::Pending,
            votes_for: HashSet::new(),
            votes_against: HashSet::new(),
            created_at,
        };
        self.proposals.insert(id, proposal);
        id
    }
    pub fn vote(&mut self, proposal_id: u64, voter: Address, approve: bool) -> bool {
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            if approve {
                proposal.votes_for.insert(voter);
            } else {
                proposal.votes_against.insert(voter);
            }
            true
        } else {
            false
        }
    }
    pub fn get_proposal(&self, proposal_id: u64) -> Option<&Proposal> {
        self.proposals.get(&proposal_id)
    }
    pub fn proposals_pending(&self) -> Vec<&Proposal> {
        self.proposals.values().filter(|p| p.status == ProposalStatus::Pending).collect()
    }
    pub fn is_approved(&self, proposal_id: u64, active_validator_count: usize) -> bool {
        if let Some(proposal) = self.proposals.get(&proposal_id) {
            // Majority approval required
            proposal.votes_for.len() > active_validator_count / 2
        } else {
            false
        }
    }
    pub fn enact_proposal(
        &mut self,
        proposal_id: u64,
        validator_set: &mut crate::consensus::validator_set::ValidatorSet,
        asset_whitelist: &mut Vec<String>,
        storage: &crate::storage::persistent::PersistentStorage,
    ) -> bool {
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            if proposal.status == ProposalStatus::Approved || self.is_approved(proposal_id, validator_set.active_validators().len()) {
                match &proposal.proposal_type {
                    ProposalType::AddValidator { address, public_key, metadata } => {
                        use ed25519_dalek::PublicKey;
                        if let Ok(pk) = PublicKey::from_bytes(public_key) {
                            let info = crate::consensus::validator_set::ValidatorInfo {
                                address: address.clone(),
                                public_key: pk,
                                status: crate::consensus::validator_set::ValidatorStatus::Active,
                                metadata: metadata.clone(),
                            };
                            validator_set.add_validator(info);
                            storage.save_validator_set(validator_set);
                        }
                    }
                    ProposalType::RemoveValidator { address } => {
                        validator_set.remove_validator(address.as_str());
                        storage.save_validator_set(validator_set);
                    }
                    ProposalType::SlashValidator { address } => {
                        validator_set.set_status(address.as_str(), crate::consensus::validator_set::ValidatorStatus::Slashed);
                        storage.save_validator_set(validator_set);
                    }
                    ProposalType::AddAsset { code, .. } => {
                        if !asset_whitelist.contains(code) {
                            asset_whitelist.push(code.clone());
                            storage.save_asset_whitelist(asset_whitelist);
                        }
                    }
                    ProposalType::RemoveAsset { code } => {
                        asset_whitelist.retain(|c| c != code);
                        storage.save_asset_whitelist(asset_whitelist);
                    }
                }
                proposal.status = ProposalStatus::Executed;
                return true;
            }
        }
        false
    }
} 
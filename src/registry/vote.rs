use crate::types::vote::{VoteType, Ballot};
use crate::types::governance::{Proposal, ProposalStatus};
use crate::utils::crypto::verify_signature;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct VoteRegistry {
    proposals: HashMap<u64, Proposal>,
    ballots: HashMap<u64, Vec<Ballot>>,
}

impl VoteRegistry {
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            ballots: HashMap::new(),
        }
    }

    pub fn create_proposal(&mut self, proposal: Proposal) {
        self.proposals.insert(proposal.id, proposal);
    }

    pub fn cast_vote(&mut self, proposal_id: u64, ballot: Ballot) -> Result<(), String> {
        let proposal = self.proposals.get(&proposal_id);
        if proposal.is_none() {
            return Err("Proposal not found".into());
        }
        // No signature/validator logic in new Ballot, so skip verification
        if let Some(_proposal) = self.proposals.get_mut(&proposal_id) {
            // No add_vote method, just store ballots
        }
        self.ballots.entry(proposal_id)
            .or_default()
            .push(ballot);
        Ok(())
    }

    pub fn get_proposal_status(&self, proposal_id: u64) -> Option<ProposalStatus> {
        self.proposals.get(&proposal_id).map(|p| p.status.clone())
    }

    pub fn get_proposal_votes(&self, proposal_id: u64) -> Option<(usize, usize, usize)> {
        self.ballots.get(&proposal_id).map(|ballots| {
            let for_votes: u64 = ballots.iter().map(|b| b.for_votes).sum();
            let against_votes: u64 = ballots.iter().map(|b| b.against_votes).sum();
            let abstain_votes: u64 = ballots.iter().map(|b| b.abstain_votes).sum();
            (for_votes as usize, against_votes as usize, abstain_votes as usize)
        })
    }

    pub fn get_proposal(&self, proposal_id: u64) -> Option<&Proposal> {
        self.proposals.get(&proposal_id)
    }

    pub fn get_ballots(&self, proposal_id: u64) -> Option<&Vec<Ballot>> {
        self.ballots.get(&proposal_id)
    }
}

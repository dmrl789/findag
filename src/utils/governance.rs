use crate::types::governance::{Proposal, Vote, VoteType};
use std::collections::HashMap;

pub struct Governance {
    pub proposals: HashMap<String, Proposal>, // id → proposal
}

impl Governance {
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
        }
    }

    pub fn submit_proposal(&mut self, proposal: Proposal) -> bool {
        if self.proposals.contains_key(&proposal.id) {
            return false;
        }
        self.proposals.insert(proposal.id.clone(), proposal);
        true
    }

    pub fn vote(&mut self, id: &str, vote: Vote) -> bool {
        if let Some(p) = self.proposals.get_mut(id) {
            if p.votes.iter().any(|v| v.voter == vote.voter) {
                return false;
            }
            p.votes.push(vote);
            return true;
        }
        false
    }

    pub fn is_approved(&self, id: &str, total_validators: usize) -> bool {
        if let Some(p) = self.proposals.get(id) {
            let yes_votes = p.votes.iter().filter(|v| v.vote == VoteType::Yes).count();
            yes_votes as f32 >= (total_validators as f32 * 0.66)
        } else {
            false
        }
    }
}

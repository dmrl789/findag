use crate::types::vote::{Proposal, Ballot};
use crate::utils::crypto::verify_signature;
use std::collections::HashMap;

pub struct VoteRegistry {
    proposals: HashMap<String, Proposal>,
    ballots: HashMap<String, Vec<Ballot>>,
}

impl VoteRegistry {
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            ballots: HashMap::new(),
        }
    }

    pub fn create_proposal(&mut self, proposal: Proposal) {
        self.proposals.insert(proposal.id.clone(), proposal);
    }

    pub fn cast_vote(&mut self, ballot: Ballot) -> Result<(), String> {
        let proposal = self.proposals.get(&ballot.proposal_id);
        if proposal.is_none() {
            return Err("Proposal not found".into());
        }

        let verified = verify_signature(
            &ballot.voter,
            &format!("{}:{}", ballot.proposal_id, ballot.choice),
            &ballot.signature,
        );

        if !verified {
            return Err("Invalid signature".into());
        }

        self.ballots.entry(ballot.proposal_id.clone())
            .or_default()
            .push(ballot);

        Ok(())
    }

    pub fn tally_votes(&self, proposal_id: &str) -> Option<Vec<usize>> {
        let proposal = self.proposals.get(proposal_id)?;
        let ballots = self.ballots.get(proposal_id)?;

        let mut counts = vec![0; proposal.options.len()];
        for ballot in ballots {
            if ballot.choice < counts.len() {
                counts[ballot.choice] += 1;
            }
        }

        Some(counts)
    }
}

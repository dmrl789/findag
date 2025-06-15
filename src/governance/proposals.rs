use crate::types::vote::{VoteType, Vote};
use crate::types::governance::ProposalStatus;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub votes: Vec<Vote>,
    pub status: ProposalStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Proposal {
    pub fn new(id: &str, title: &str, description: &str, proposer: &str) -> Self {
        let now = chrono::Utc::now().timestamp();
        Proposal {
            id: id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            proposer: proposer.to_string(),
            votes: Vec::new(),
            status: ProposalStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_vote(&mut self, vote: Vote) {
        self.votes.push(vote);
        self.update_status();
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn update_status(&mut self) {
        let (for_count, against_count) = self.votes.iter().fold((0, 0), |(for_c, against_c), vote| {
            match vote.vote_type {
                VoteType::For => (for_c + 1, against_c),
                VoteType::Against => (for_c, against_c + 1),
                _ => (for_c, against_c),
            }
        });

        self.status = if for_count > against_count {
            ProposalStatus::Passed
        } else if against_count > for_count {
            ProposalStatus::Rejected
        } else {
            ProposalStatus::Active
        };
    }

    pub fn is_accepted(&self) -> bool {
        self.status == ProposalStatus::Passed
    }

    pub fn get_vote_counts(&self) -> (usize, usize, usize) {
        self.votes.iter().fold((0, 0, 0), |(for_c, against_c, abstain_c), vote| {
            match vote.vote_type {
                VoteType::For => (for_c + 1, against_c, abstain_c),
                VoteType::Against => (for_c, against_c + 1, abstain_c),
                VoteType::Abstain => (for_c, against_c, abstain_c + 1),
            }
        })
    }
}


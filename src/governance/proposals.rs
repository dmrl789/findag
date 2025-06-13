use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub votes_for: u32,
    pub votes_against: u32,
    pub active: bool,
}

impl Proposal {
    pub fn new(id: &str, title: &str, description: &str, proposer: &str) -> Self {
        Proposal {
            id: id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            proposer: proposer.to_string(),
            votes_for: 0,
            votes_against: 0,
            active: true,
        }
    }

    pub fn vote(&mut self, support: bool) {
        if support {
            self.votes_for += 1;
        } else {
            self.votes_against += 1;
        }
    }

    pub fn is_accepted(&self) -> bool {
        self.votes_for > self.votes_against
    }

    pub fn close(&mut self) {
        self.active = false;
    }
}

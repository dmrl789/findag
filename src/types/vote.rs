use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub proposal_id: u64,
    pub voter: String,
    pub vote_type: VoteType,
    pub timestamp: u64,
}

impl Vote {
    pub fn new(proposal_id: u64, voter: String, vote_type: VoteType) -> Self {
        Self {
            proposal_id,
            voter,
            vote_type,
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }
}

impl fmt::Display for Vote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vote {{ proposal_id: {}, voter: {}, vote_type: {}, timestamp: {} }}",
            self.proposal_id, self.voter, self.vote_type, self.timestamp
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteType {
    For,
    Against,
    Abstain,
}

impl fmt::Display for VoteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VoteType::For => write!(f, "For"),
            VoteType::Against => write!(f, "Against"),
            VoteType::Abstain => write!(f, "Abstain"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ballot {
    pub proposal_id: u64,
    pub votes: Vec<Vote>,
    pub total_votes: u64,
    pub for_votes: u64,
    pub against_votes: u64,
    pub abstain_votes: u64,
}

impl Ballot {
    pub fn new(proposal_id: u64) -> Self {
        Self {
            proposal_id,
            votes: Vec::new(),
            total_votes: 0,
            for_votes: 0,
            against_votes: 0,
            abstain_votes: 0,
        }
    }

    pub fn add_vote(&mut self, vote: Vote) {
        let vote_type = vote.vote_type;
        self.votes.push(vote);
        self.total_votes += 1;
        match vote_type {
            VoteType::For => self.for_votes += 1,
            VoteType::Against => self.against_votes += 1,
            VoteType::Abstain => self.abstain_votes += 1,
        }
    }

    pub fn get_majority(&self) -> Option<VoteType> {
        if self.total_votes == 0 {
            return None;
        }

        if self.for_votes > self.against_votes && self.for_votes > self.abstain_votes {
            Some(VoteType::For)
        } else if self.against_votes > self.for_votes && self.against_votes > self.abstain_votes {
            Some(VoteType::Against)
        } else if self.abstain_votes > self.for_votes && self.abstain_votes > self.against_votes {
            Some(VoteType::Abstain)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vote_creation() {
        let vote = Vote::new(1, "voter1".to_string(), VoteType::For);
        assert_eq!(vote.proposal_id, 1);
        assert_eq!(vote.voter, "voter1");
        assert_eq!(vote.vote_type, VoteType::For);
        assert!(vote.timestamp > 0);
    }

    #[test]
    fn test_vote_type_display() {
        assert_eq!(VoteType::For.to_string(), "For");
        assert_eq!(VoteType::Against.to_string(), "Against");
        assert_eq!(VoteType::Abstain.to_string(), "Abstain");
    }

    #[test]
    fn test_ballot_creation() {
        let mut ballot = Ballot::new(1);
        assert_eq!(ballot.proposal_id, 1);
        assert_eq!(ballot.total_votes, 0);

        let vote = Vote::new(1, "voter1".to_string(), VoteType::For);
        ballot.add_vote(vote);
        assert_eq!(ballot.total_votes, 1);
        assert_eq!(ballot.for_votes, 1);
    }

    #[test]
    fn test_ballot_majority() {
        let mut ballot = Ballot::new(1);
        
        // Add votes
        ballot.add_vote(Vote::new(1, "voter1".to_string(), VoteType::For));
        ballot.add_vote(Vote::new(1, "voter2".to_string(), VoteType::For));
        ballot.add_vote(Vote::new(1, "voter3".to_string(), VoteType::Against));
        
        assert_eq!(ballot.get_majority(), Some(VoteType::For));
    }
} 
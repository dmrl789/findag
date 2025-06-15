use std::error::Error;
use serde::{Serialize, Deserialize};
use chrono;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundInfo {
    pub round_id: u64,
    pub proposer: String,
    pub block_hash: String,
    pub timestamp: i64,
    pub state: RoundState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundState {
    pub round_id: u64,
    pub phase: RoundPhase,
    pub votes: Vec<String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundMessage {
    pub round_id: u64,
    pub sender: String,
    pub message_type: MessageType,
    pub content: Vec<u8>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoundPhase {
    Propose,
    Prevote,
    Precommit,
    Commit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Proposal,
    Vote,
    Timeout,
}

impl RoundInfo {
    pub fn new(round_id: u64, proposer: &str, block_hash: &str) -> Self {
        Self {
            round_id,
            proposer: proposer.to_string(),
            block_hash: block_hash.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            state: RoundState {
                round_id,
                phase: RoundPhase::Propose,
                votes: Vec::new(),
                timestamp: chrono::Utc::now().timestamp(),
            },
        }
    }
}

impl RoundState {
    pub fn new(round_id: u64) -> Self {
        Self {
            round_id,
            phase: RoundPhase::Propose,
            votes: Vec::new(),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}

impl RoundMessage {
    pub fn new(round_id: u64, sender: &str, message_type: MessageType, content: Vec<u8>) -> Self {
        Self {
            round_id,
            sender: sender.to_string(),
            message_type,
            content,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
} 
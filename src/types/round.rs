use std::error::Error;
use serde::{Serialize, Deserialize};
use chrono;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundInfo {
    pub round_id: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub block_count: u32,
    pub validator_count: u32,
    pub status: RoundStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoundStatus {
    Active,
    Completed,
    Failed,
}

impl RoundInfo {
    pub fn new(round_id: u64) -> Self {
        Self {
            round_id,
            start_time: chrono::Utc::now().timestamp(),
            end_time: 0,
            block_count: 0,
            validator_count: 0,
            status: RoundStatus::Active,
        }
    }

    pub fn complete(&mut self) {
        self.end_time = chrono::Utc::now().timestamp();
        self.status = RoundStatus::Completed;
    }

    pub fn fail(&mut self) {
        self.end_time = chrono::Utc::now().timestamp();
        self.status = RoundStatus::Failed;
    }

    pub fn add_block(&mut self) {
        self.block_count += 1;
    }

    pub fn add_validator(&mut self) {
        self.validator_count += 1;
    }
} 
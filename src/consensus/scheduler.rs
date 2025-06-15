use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};
use sled::Db;

fn bytes_to_ivec(bytes: [u8; 8]) -> sled::IVec {
    sled::IVec::from(bytes.to_vec())
}

#[derive(Clone, Debug)]
pub struct ValidatorInfo {
    pub address: String,
    pub uptime: f64,
    pub latency: u64,
    pub last_active: u64,
}

#[derive(Clone)]
pub struct Scheduler {
    pub queue: VecDeque<String>,
    pub scores: HashMap<String, f64>,
    db: Db,
}

impl Scheduler {
    pub fn new(validators: &[ValidatorInfo], db: Db) -> Self {
        let mut scores = HashMap::new();
        for v in validators {
            let score = Scheduler::score_validator(v);
            scores.insert(v.address.clone(), score);
            let _ = db.insert(v.address.as_bytes(), bytes_to_ivec(score.to_le_bytes()));
        }

        let mut sorted: Vec<_> = scores.iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        let queue = sorted.into_iter().map(|(k, _)| k.clone()).collect();

        Scheduler { queue, scores, db }
    }

    pub fn next_validator(&mut self) -> Option<String> {
        if let Some(validator) = self.queue.pop_front() {
            self.queue.push_back(validator.clone());
            Some(validator)
        } else {
            None
        }
    }

    pub fn score_validator(info: &ValidatorInfo) -> f64 {
        let latency_penalty = (info.latency as f64) / 100.0;
        let freshness = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - info.last_active;
        let freshness_penalty = freshness as f64 / 100.0;

        info.uptime - latency_penalty - freshness_penalty
    }

    pub fn get_score(&self, address: &str) -> Option<f64> {
        if let Ok(Some(val)) = self.db.get(address.as_bytes()) {
            let bytes: [u8; 8] = val.as_ref().try_into().ok()?;
            Some(f64::from_le_bytes(bytes))
        } else {
            None
        }
    }
}

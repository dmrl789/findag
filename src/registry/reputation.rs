use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use sled::Db;
use tokio::time::{interval, Duration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidatorScore {
    pub peer_id: String,
    pub uptime: u64,
    pub last_seen: u64,
    pub reputation: f64,
}

#[derive(Clone)]
pub struct ReputationEngine {
    db: Arc<Db>,
}

impl ReputationEngine {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub fn get_all(&self) -> Vec<ValidatorScore> {
        self.db
            .iter()
            .filter_map(|res| res.ok())
            .filter_map(|(_, v)| bincode::deserialize(&v).ok())
            .collect()
    }

    pub fn save_score(&self, score: &ValidatorScore) {
        let _ = self.db.insert(score.peer_id.clone(), bincode::serialize(score).unwrap());
    }

    pub fn update_score(&self, peer_id: &str, new_uptime: u64, last_seen: u64) {
        let score = self.db.get(peer_id).ok().flatten().and_then(|v| bincode::deserialize(&v).ok());

        let mut updated = match score {
            Some(mut s) => {
                s.uptime += new_uptime;
                s.last_seen = last_seen;
                s.reputation = (s.uptime as f64 / (last_seen.max(1) as f64)).min(1.0);
                s
            }
            None => ValidatorScore {
                peer_id: peer_id.to_string(),
                uptime: new_uptime,
                last_seen,
                reputation: 1.0,
            },
        };

        self.save_score(&updated);
    }

    pub fn start_background_updater(self) {
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                let now = chrono::Utc::now().timestamp() as u64;
                for score in self.get_all() {
                    self.update_score(&score.peer_id, 1, now);
                }
            }
        });
    }
}

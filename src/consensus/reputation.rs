use sled::Db;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ValidatorStats {
    pub address: String,
    pub uptime_score: u64,
    pub validations: u64,
    pub total_rounds: u64,
}

impl ValidatorStats {
    pub fn reputation_score(&self) -> f64 {
        if self.total_rounds == 0 {
            return 0.0;
        }
        let uptime_ratio = self.uptime_score as f64 / self.total_rounds as f64;
        let validation_ratio = self.validations as f64 / self.total_rounds as f64;
        0.6 * uptime_ratio + 0.4 * validation_ratio
    }
}

pub fn load_all_validators(db: &Db) -> HashMap<String, ValidatorStats> {
    let tree = db.open_tree("validator_stats").unwrap();
    let mut map = HashMap::new();
    for result in tree.iter() {
        if let Ok((key, val)) = result {
            if let Ok(stat) = bincode::deserialize::<ValidatorStats>(&val) {
                let key_str = String::from_utf8(key.to_vec()).unwrap();
                map.insert(key_str, stat);
            }
        }
    }
    map
}

pub fn store_validator_stats(db: &Db, stats: &ValidatorStats) {
    let tree = db.open_tree("validator_stats").unwrap();
    let key = stats.address.as_bytes();
    let val = bincode::serialize(stats).unwrap();
    tree.insert(key, val).unwrap();
}

pub fn top_validators(db: &Db, count: usize) -> Vec<ValidatorStats> {
    let mut validators: Vec<_> = load_all_validators(db).into_values().collect();
    validators.sort_by(|a, b| b.reputation_score().partial_cmp(&a.reputation_score()).unwrap());
    validators.into_iter().take(count).collect()
}

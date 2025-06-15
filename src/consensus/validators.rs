use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct ValidatorSet {
    validators: Vec<String>,
    total_stake: u64,
}

impl ValidatorSet {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
            total_stake: 0,
        }
    }

    pub fn authorize(&mut self, key: &str) -> bool {
        self.validators.push(key.to_string());
        true
    }

    pub fn revoke(&mut self, key: &str) -> bool {
        self.validators.retain(|k| k != key);
        true
    }

    pub fn is_authorized(&self, key: &str) -> bool {
        self.validators.iter().any(|k| k == key)
    }
}

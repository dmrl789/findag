use std::collections::HashSet;

pub struct ValidatorSet {
    pub validators: Vec<String>,
}

impl ValidatorSet {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
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
        self.validators.contains(key)
    }
}

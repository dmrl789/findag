use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct ValidatorSet {
    pub authorized: HashSet<String>,
}

impl ValidatorSet {
    pub fn new() -> Self {
        Self {
            authorized: HashSet::new(),
        }
    }

    pub fn authorize(&mut self, key: &str) -> bool {
        self.authorized.insert(key.to_string())
    }

    pub fn revoke(&mut self, key: &str) -> bool {
        self.authorized.remove(key)
    }

    pub fn is_authorized(&self, key: &str) -> bool {
        self.authorized.contains(key)
    }
}

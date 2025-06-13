use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct NodeRegistry {
    pub authorized: HashSet<String>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        let mut set = HashSet::new();
        // Static authorized addresses
        set.insert("1abc...".to_string());
        set.insert("1bank001...".to_string());
        set.insert("1finhub...".to_string());

        NodeRegistry { authorized: set }
    }

    pub fn is_authorized(&self, address: &str) -> bool {
        self.authorized.contains(address)
    }

    pub fn add(&mut self, address: String) {
        self.authorized.insert(address);
    }

    pub fn remove(&mut self, address: &str) {
        self.authorized.remove(address);
    }

    pub fn print(&self) {
        for node in &self.authorized {
            println!("✔ Authorized: {}", node);
        }
    }
}

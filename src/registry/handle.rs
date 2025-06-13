use chrono::Utc;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::collections::HashSet;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HandleRecord {
    pub owner: String,
    pub created_at: u64,
}

pub struct HandleRegistry {
    db: Db,
    authorized_nodes: Mutex<HashSet<String>>,
}

impl HandleRegistry {
    pub fn new(db_path: &str) -> Self {
        let db = sled::open(db_path).expect("Failed to open sled DB");
        let mut authorized = HashSet::new();
        authorized.insert("authorized_node_1".to_string()); // Example ID
        authorized.insert("authorized_node_2".to_string());

        Self {
            db,
            authorized_nodes: Mutex::new(authorized),
        }
    }

    pub fn is_valid_fd_handle(handle: &str) -> bool {
        let allowed = handle.ends_with(".fd")
            && handle.len() >= 5
            && handle.chars().all(|c| c.is_ascii_lowercase() || c.is_numeric() || c == '_' || c == '.');
        allowed
    }

    pub fn is_authorized(&self, node_id: &str) -> bool {
        self.authorized_nodes.lock().unwrap().contains(node_id)
    }

    pub fn register_handle(
        &self,
        handle: &str,
        record: &HandleRecord,
    ) -> Result<(), String> {
        if !Self::is_valid_fd_handle(handle) {
            return Err("Invalid `.fd` handle format.".into());
        }

        if !self.is_authorized(&record.owner) {
            return Err("Node is not authorized to register `.fd` domains.".into());
        }

        match self.db.get(handle) {
            Ok(Some(_)) => Err("Handle already exists.".into()),
            Ok(None) => {
                let value = serde_json::to_vec(record).unwrap();
                self.db.insert(handle, value).unwrap();
                Ok(())
            }
            Err(e) => Err(format!("Storage error: {}", e)),
        }
    }

    pub fn resolve_handle(&self, handle: &str) -> Option<HandleRecord> {
        match self.db.get(handle) {
            Ok(Some(value)) => serde_json::from_slice(&value).ok(),
            _ => None,
        }
    }
}

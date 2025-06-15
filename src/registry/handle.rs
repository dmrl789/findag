use chrono::Utc;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref REGISTRY: HandleRegistry = HandleRegistry::new("handles.db");
}

pub fn register_handle(handle: &str, address: &str) -> Result<(), String> {
    let record = HandleRecord {
        id: handle.to_string(),
        data: address.to_string(),
        owner: "system".to_string(),
        created_at: Utc::now().timestamp() as u64,
    };
    REGISTRY.register_handle(handle, &record)
}

pub fn resolve_handle(handle: &str) -> Option<String> {
    REGISTRY.resolve_handle(handle)
        .ok()
        .flatten()
        .map(|record| record.data)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandleRecord {
    pub id: String,
    pub data: String,
    pub owner: String,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub handle: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveResponse {
    pub handle: String,
    pub address: String,
}

pub struct HandleRegistry {
    pub handles: std::collections::HashMap<String, HandleRecord>,
    pub db: sled::Db,
    pub authorized_nodes: Mutex<HashSet<String>>,
}

impl HandleRegistry {
    pub fn new(db_path: &str) -> Self {
        let db = sled::open(db_path).expect("Failed to open sled DB");
        let mut authorized = HashSet::new();
        authorized.insert("authorized_node_1".to_string()); // Example ID
        authorized.insert("authorized_node_2".to_string());

        Self {
            handles: std::collections::HashMap::new(),
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

        if !self.is_authorized(&record.id) {
            return Err("Node is not authorized to register `.fd` domains.".into());
        }

        match self.db.get(handle) {
            Ok(Some(_)) => Err("Handle already exists.".into()),
            Ok(None) => {
                let value = serde_json::to_vec(record)
                    .map_err(|e| format!("Failed to serialize record: {}", e))?;
                self.db.insert(handle, value)
                    .map_err(|e| format!("Failed to store record: {}", e))?;
                Ok(())
            }
            Err(e) => Err(format!("Storage error: {}", e)),
        }
    }

    pub fn resolve_handle(&self, handle: &str) -> Result<Option<HandleRecord>, String> {
        match self.db.get(handle)
            .map_err(|e| format!("Failed to get handle: {}", e))? {
            Some(value) => serde_json::from_slice(&value)
                .map_err(|e| format!("Failed to deserialize record: {}", e))
                .map(Some),
            None => Ok(None),
        }
    }

    pub fn lookup_handle(&self, handle: &str) -> Option<HandleRecord> {
        self.resolve_handle(handle).ok().flatten()
    }
}

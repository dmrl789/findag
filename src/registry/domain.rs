use crate::types::domain::DomainRecord;
use crate::storage::db;
use crate::utils::time::get_findag_time_micro;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct DomainRegistry {
    store: Arc<RwLock<HashMap<String, DomainRecord>>>,
}

impl DomainRegistry {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register_or_update_domain(
        &self,
        domain: String,
        owner: String,
        metadata: Option<String>,
    ) -> Result<(), String> {
        let mut store = self.store.write().unwrap();
        store.insert(
            domain.clone(),
            DomainRecord {
                domain,
                owner,
                updated_at: get_findag_time_micro(),
                metadata,
            },
        );
        Ok(())
    }

    pub fn get_domain(&self, domain: &str) -> Option<DomainRecord> {
        self.store.read().unwrap().get(domain).cloned()
    }

    pub fn delete_domain(&self, domain: &str, caller: &str) -> Result<(), String> {
        let mut store = self.store.write().unwrap();
        if let Some(record) = store.get(domain) {
            if record.owner == caller {
                store.remove(domain);
                Ok(())
            } else {
                Err("Only the owner can delete this domain".into())
            }
        } else {
            Err("Domain not found".into())
        }
    }

    pub fn list_domains(&self) -> Vec<DomainRecord> {
        self.store.read().unwrap().values().cloned().collect()
    }
}

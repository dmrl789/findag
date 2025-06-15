use serde::{Serialize, Deserialize};
use chrono;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handle {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub metadata: Option<Vec<u8>>,
}

impl Handle {
    pub fn new(owner: &str, name: &str) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            owner: owner.to_string(),
            name: name.to_string(),
            created_at: now,
            updated_at: now,
            metadata: None,
        }
    }

    pub fn update_metadata(&mut self, metadata: Vec<u8>) {
        self.metadata = Some(metadata);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn transfer_ownership(&mut self, new_owner: &str) {
        self.owner = new_owner.to_string();
        self.updated_at = chrono::Utc::now().timestamp();
    }
} 
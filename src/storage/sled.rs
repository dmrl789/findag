use std::path::Path;
use sled::Db;

pub struct Storage {
    db: Db,
}

impl Storage {
    pub fn init(path: &str) -> Self {
        let db = sled::open(path).expect("Failed to open database");
        Self { db }
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.db.get(key).ok().flatten().map(|v| v.to_vec())
    }

    pub fn set(&self, key: &[u8], value: &[u8]) -> Result<(), sled::Error> {
        self.db.insert(key, value)?;
        Ok(())
    }

    pub fn remove(&self, key: &[u8]) -> Result<(), sled::Error> {
        self.db.remove(key)?;
        Ok(())
    }
}

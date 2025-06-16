use sled::{Db, IVec};
use std::str;

pub fn bytes_to_ivec(bytes: &[u8]) -> IVec {
    IVec::from(bytes)
}

#[derive(Clone)]
pub struct KVStore {
    db: Db,
}

impl KVStore {
    pub fn new(path: &str) -> Self {
        let db = sled::open(path).expect("Failed to open sled DB");
        KVStore { db }
    }

    pub fn set(&self, key: &str, value: &str) {
        self.db.insert(key, value.as_bytes()).expect("Insert failed");
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.db.get(key).ok().flatten().map(|v: IVec| {
            String::from_utf8(v.to_vec()).unwrap_or_else(|_| "<invalid UTF-8>".into())
        })
    }

    pub fn delete(&self, key: &str) {
        self.db.remove(key).expect("Delete failed");
    }

    pub fn flush(&self) {
        self.db.flush().expect("Flush failed");
    }
}

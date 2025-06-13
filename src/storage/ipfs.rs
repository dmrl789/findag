use crate::types::ipfs::IpfsMetadata;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct IpfsRegistry {
    records: Arc<Mutex<HashMap<String, IpfsMetadata>>>,
}

impl IpfsRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, meta: IpfsMetadata) {
        self.records.lock().unwrap().insert(meta.cid.clone(), meta);
    }

    pub fn get(&self, cid: &str) -> Option<IpfsMetadata> {
        self.records.lock().unwrap().get(cid).cloned()
    }

    pub fn all(&self) -> Vec<IpfsMetadata> {
        self.records.lock().unwrap().values().cloned().collect()
    }
}

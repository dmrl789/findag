use std::error::Error;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub hash: Vec<u8>,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

impl Transaction {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            hash: vec![], // TODO: Implement proper hashing
            data,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
} 
use crate::types::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub hash_timer: u64,
    pub parent_hashes: Vec<String>,
    pub transactions: Vec<Transaction>,
    pub creator: String,
}

impl Block {
    pub fn new(parent_hashes: Vec<String>, transactions: Vec<Transaction>, creator: String) -> Self {
        Block {
            hash_timer: get_findag_time_micro(),
            parent_hashes,
            transactions,
            creator,
        }
    }
}

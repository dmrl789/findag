use serde::{Serialize, Deserialize};
use crate::types::finality::Justification;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub hash_timer: u64,
    pub content: String,
    pub justification: Option<Justification>,
}

impl Block {
    pub fn new(content: String) -> Self {
        Self {
            hash_timer: crate::utils::time::get_findag_time_micro(),
            content,
        }
    }
}

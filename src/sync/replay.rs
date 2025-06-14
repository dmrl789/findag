use crate::blockchain::block::{Block, BlockHeader};
use crate::storage::Storage;
use crate::utils::time::validate_hashtimer;
use crate::types::address::Address;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ReplayState {
    pub state: String,
}

pub fn replay_chain(storage: &Storage) -> Result<(), String> {
    println!("🔁 Replaying chain from genesis...");

    let mut state: HashMap<Address, u64> = HashMap::new();
    let blocks = storage.get_all_blocks_sorted();

    for block in blocks {
        validate_block(&block, &mut state)?;
    }

    println!("✅ Replay completed. {} blocks processed.", blocks.len());
    Ok(())
}

fn validate_block(block: &Block, state: &mut HashMap<Address, u64>) -> Result<(), String> {
    validate_hashtimer(&block.hash_timer)?;

    for tx in &block.content {
        let from_balance = state.entry(Address::from_bytes(&tx.sender)).or_insert(0);
        if *from_balance < tx.amount {
            return Err(format!("Insufficient balance for tx from: {:?}", tx.sender));
        }

        *from_balance -= tx.amount;
        *state.entry(Address::from_bytes(&tx.recipient)).or_insert(0) += tx.amount;
    }

    Ok(())
}

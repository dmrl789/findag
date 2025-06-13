use crate::blockchain::block::{Block, BlockHeader};
use crate::storage::Storage;
use crate::utils::time::validate_hashtimer;
use crate::types::address::Address;
use std::collections::HashMap;

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
    validate_hashtimer(&block.header.hashtimer)?;

    for tx in &block.transactions {
        let from_balance = state.entry(tx.from.clone()).or_insert(0);
        if *from_balance < tx.amount {
            return Err(format!("Insufficient balance for tx from: {:?}", tx.from));
        }

        *from_balance -= tx.amount;
        *state.entry(tx.to.clone()).or_insert(0) += tx.amount;
    }

    Ok(())
}

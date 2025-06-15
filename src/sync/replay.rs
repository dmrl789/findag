use crate::blockchain::block::Block;
use crate::storage::types::AssetType;
use crate::types::transaction::Transaction;
use crate::types::address::Address;
use crate::storage::Storage;
use crate::utils::time::validate_hashtimer;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::sync::Mutex;

#[derive(Clone, Serialize, Deserialize)]
pub struct ReplayState {
    pub balances: HashMap<Address, u64>,
}

impl ReplayState {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }
}

pub struct ReplayManager {
    storage: Storage,
    state: Mutex<ReplayState>,
}

impl ReplayManager {
    pub fn new(storage: Storage) -> Self {
        Self { storage, state: Mutex::new(ReplayState::new()) }
    }

    pub async fn replay_block(&self, block: &Block) -> Result<(), Box<dyn Error>> {
        // Process block content
        for tx in &block.data {
            self.process_transaction(tx)?;
        }
        Ok(())
    }

    fn process_transaction(&self, tx: &Transaction) -> Result<(), Box<dyn Error>> {
        let mut state = self.state.lock().unwrap();
        
        // Handle transfer transaction
        if tx.amount > 0 {
            let from_address = Address::from_bytes(&tx.from)?;
            let to_address = Address::from_bytes(&tx.to)?;
            
            let from_balance = state.balances.entry(from_address).or_insert(0);
            if *from_balance < tx.amount {
                return Err("Insufficient balance".into());
            }
            *from_balance -= tx.amount;
            
            *state.balances.entry(to_address).or_insert(0) += tx.amount;
        }
        
        Ok(())
    }

    pub async fn replay_chain(&self, blocks: Vec<Block>) -> Result<(), Box<dyn Error>> {
        for block in blocks {
            self.replay_block(&block).await?;
        }
        Ok(())
    }
}

pub fn replay_chain(storage: &Storage, state: &mut ReplayState) -> Result<(), String> {
    println!("🔁 Replaying chain from genesis...");

    for block in storage.get_blocks().map_err(|e| e.to_string())? {
        validate_hashtimer(block.timestamp)?;
        
        for tx in &block.data {
            let from_address = Address::from_bytes(&tx.from).map_err(|e| e.to_string())?;
            let to_address = Address::from_bytes(&tx.to).map_err(|e| e.to_string())?;
            
            let from_balance = state.balances.entry(from_address).or_insert(0);
            if *from_balance < tx.amount {
                return Err(format!("Insufficient balance for tx from: {:?}", tx.from));
            }
            *from_balance -= tx.amount;
            *state.balances.entry(to_address).or_insert(0) += tx.amount;
        }
    }

    println!("✅ Replay completed. {} blocks processed.", storage.get_blocks().map_err(|e| e.to_string())?.len());
    Ok(())
}

fn validate_block(block: &Block, state: &mut ReplayState) -> Result<(), String> {
    validate_hashtimer(block.timestamp)?;

    for tx in &block.data {
        let from_address = Address::from_bytes(&tx.from).map_err(|e| e.to_string())?;
        let to_address = Address::from_bytes(&tx.to).map_err(|e| e.to_string())?;
        
        let from_balance = state.balances.entry(from_address).or_insert(0);
        if *from_balance < tx.amount {
            return Err(format!("Insufficient balance for tx from: {:?}", tx.from));
        }

        *from_balance -= tx.amount;
        *state.balances.entry(to_address).or_insert(0) += tx.amount;
    }

    Ok(())
}

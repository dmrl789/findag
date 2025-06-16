use crate::blockchain::block::Block;
use crate::types::{AssetType, Transaction, Address};
use crate::storage::Storage;
use crate::utils::time::validate_hashtimer;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::sync::Mutex;
use crate::blockchain::state::State;

#[derive(Clone, Serialize, Deserialize)]
pub struct ReplayState {
    pub balances: HashMap<Address, u64>,
    pub handles: HashMap<String, Address>,
}

impl ReplayState {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
            handles: HashMap::new(),
        }
    }

    pub fn apply_transaction(&mut self, tx: &Transaction) -> Result<(), String> {
        // Extract transaction data
        let data = serde_json::from_slice::<TransactionData>(&tx.data)
            .map_err(|e| format!("Failed to parse transaction data: {}", e))?;

        match data.tx_type {
            TransactionType::Transfer => {
                let from_address = Address::from_bytes(&data.from)
                    .map_err(|e| format!("Invalid from address: {}", e))?;
                let to_address = Address::from_bytes(&data.to)
                    .map_err(|e| format!("Invalid to address: {}", e))?;
                let amount = data.amount;

                let from_balance = self.balances.entry(from_address).or_insert(0);
                if *from_balance < amount {
                    return Err(format!("Insufficient balance for tx from: {:?}", data.from));
                }

                *from_balance -= amount;
                *self.balances.entry(to_address).or_insert(0) += amount;
            }
            TransactionType::RegisterHandle => {
                let owner = Address::from_bytes(&data.owner)
                    .map_err(|e| format!("Invalid owner address: {}", e))?;
                let handle = data.handle;

                if self.handles.contains_key(&handle) {
                    return Err(format!("Handle already registered: {}", handle));
                }

                self.handles.insert(handle, owner);
            }
        }

        Ok(())
    }
}

#[derive(serde::Deserialize)]
struct TransactionData {
    tx_type: TransactionType,
    from: Vec<u8>,
    to: Vec<u8>,
    amount: u64,
    owner: Vec<u8>,
    handle: String,
}

#[derive(serde::Deserialize)]
enum TransactionType {
    Transfer,
    RegisterHandle,
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
        let data = serde_json::from_slice::<TransactionData>(&tx.data)?;
        match data.tx_type {
            TransactionType::Transfer => {
                let from_address = Address::from_bytes(&data.from)?;
                let to_address = Address::from_bytes(&data.to)?;
                let amount = data.amount;
                let from_balance = state.balances.entry(from_address).or_insert(0);
                if *from_balance < amount {
                    return Err("Insufficient balance".into());
                }
                *from_balance -= amount;
                *state.balances.entry(to_address).or_insert(0) += amount;
            }
            TransactionType::RegisterHandle => {
                let owner = Address::from_bytes(&data.owner)?;
                let handle = data.handle;
                if state.handles.contains_key(&handle) {
                    return Err("Handle already registered".into());
                }
                state.handles.insert(handle, owner);
            }
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
            let data = serde_json::from_slice::<TransactionData>(&tx.data).map_err(|e| e.to_string())?;
            match data.tx_type {
                TransactionType::Transfer => {
                    let from_address = Address::from_bytes(&data.from).map_err(|e| e.to_string())?;
                    let to_address = Address::from_bytes(&data.to).map_err(|e| e.to_string())?;
                    let amount = data.amount;
                    let from_balance = state.balances.entry(from_address).or_insert(0);
                    if *from_balance < amount {
                        return Err(format!("Insufficient balance for tx from: {:?}", data.from));
                    }
                    *from_balance -= amount;
                    *state.balances.entry(to_address).or_insert(0) += amount;
                }
                TransactionType::RegisterHandle => {
                    let owner = Address::from_bytes(&data.owner).map_err(|e| e.to_string())?;
                    let handle = data.handle;
                    if state.handles.contains_key(&handle) {
                        return Err(format!("Handle already registered: {}", handle));
                    }
                    state.handles.insert(handle, owner);
                }
            }
        }
    }
    println!("✅ Replay completed. {} blocks processed.", storage.get_blocks().map_err(|e| e.to_string())?.len());
    Ok(())
}

fn validate_block(block: &Block, state: &mut ReplayState) -> Result<(), String> {
    validate_hashtimer(block.timestamp)?;
    for tx in &block.data {
        let data = serde_json::from_slice::<TransactionData>(&tx.data).map_err(|e| e.to_string())?;
        match data.tx_type {
            TransactionType::Transfer => {
                let from_address = Address::from_bytes(&data.from).map_err(|e| e.to_string())?;
                let to_address = Address::from_bytes(&data.to).map_err(|e| e.to_string())?;
                let amount = data.amount;
                let from_balance = state.balances.entry(from_address).or_insert(0);
                if *from_balance < amount {
                    return Err(format!("Insufficient balance for tx from: {:?}", data.from));
                }
                *from_balance -= amount;
                *state.balances.entry(to_address).or_insert(0) += amount;
            }
            TransactionType::RegisterHandle => {
                let owner = Address::from_bytes(&data.owner).map_err(|e| e.to_string())?;
                let handle = data.handle;
                if state.handles.contains_key(&handle) {
                    return Err(format!("Handle already registered: {}", handle));
                }
                state.handles.insert(handle, owner);
            }
        }
    }
    Ok(())
}

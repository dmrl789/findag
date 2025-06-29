// Usage:
// let storage = Arc::new(PersistentStorage::new("findag_db"));
// let (tx, rx) = mpsc::unbounded_channel();
// storage.clone().spawn_background_writer(rx);
// tx.send(PersistMsg::Block(block)).unwrap();
// tx.send(PersistMsg::Round(round)).unwrap(); 

use sled;
use crate::core::types::{Block, Round, SerializableBlock, SerializableRound, AssetRecord};
use bincode;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::consensus::validator_set::ValidatorSet;
use crate::core::handle_registry::HandleRecord;

pub struct PersistentStorage {
    db: sled::Db,
}

impl PersistentStorage {
    pub fn new(path: &str) -> Result<Self, sled::Error> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn save_block(&self, block: &Block) {
        let key = [b"block:".as_ref(), &block.block_id].concat();
        let serializable = SerializableBlock::from(block.clone());
        let value = bincode::serialize(&serializable).unwrap();
        self.db.insert(key, value).unwrap();
    }

    pub fn save_round(&self, round: &Round) {
        let key = [b"round:".as_ref(), &round.round_id.to_be_bytes()].concat();
        let serializable = SerializableRound::from(round.clone());
        let value = bincode::serialize(&serializable).unwrap();
        self.db.insert(key, value).unwrap();
    }

    pub fn load_block(&self, block_id: &[u8; 32]) -> Option<Block> {
        let key = [b"block:".as_ref(), block_id].concat();
        self.db.get(key).unwrap().map(|ivec| {
            let serializable: SerializableBlock = bincode::deserialize(&ivec).unwrap();
            Block::try_from(serializable).unwrap()
        })
    }

    pub fn load_round(&self, round_id: u64) -> Option<Round> {
        let key = [b"round:".as_ref(), &round_id.to_be_bytes()].concat();
        self.db.get(key).unwrap().map(|ivec| {
            let serializable: SerializableRound = bincode::deserialize(&ivec).unwrap();
            Round::try_from(serializable).unwrap()
        })
    }

    pub fn store_validator_set(&self, set: &ValidatorSet) -> Result<(), Box<dyn std::error::Error>> {
        let key = b"validator_set";
        let value = bincode::serialize(set)?;
        self.db.insert(key, value)?;
        Ok(())
    }

    pub fn load_validator_set(&self) -> Result<Option<ValidatorSet>, Box<dyn std::error::Error>> {
        let key = b"validator_set";
        match self.db.get(key)? {
            Some(ivec) => {
                let set: ValidatorSet = bincode::deserialize(&ivec)?;
                Ok(Some(set))
            }
            None => Ok(None),
        }
    }

    pub fn store_governance_state(&self, state: &crate::consensus::governance::GovernanceState) -> Result<(), Box<dyn std::error::Error>> {
        let key = b"governance_state";
        let value = bincode::serialize(state)?;
        self.db.insert(key, value)?;
        Ok(())
    }

    pub fn load_governance_state(&self) -> Result<Option<crate::consensus::governance::GovernanceState>, Box<dyn std::error::Error>> {
        let key = b"governance_state";
        match self.db.get(key)? {
            Some(ivec) => {
                let state: crate::consensus::governance::GovernanceState = bincode::deserialize(&ivec)?;
                Ok(Some(state))
            }
            None => Ok(None),
        }
    }

    pub fn store_parameter(&self, key: &str, value: &str) -> Result<(), sled::Error> {
        let key_bytes = key.as_bytes();
        let value_bytes = value.as_bytes();
        self.db.insert(key_bytes, value_bytes)?;
        Ok(())
    }

    pub fn load_parameter(&self, key: &str) -> Result<Option<String>, sled::Error> {
        let key_bytes = key.as_bytes();
        match self.db.get(key_bytes)? {
            Some(ivec) => {
                let value = String::from_utf8(ivec.to_vec())
                    .map_err(|_| sled::Error::Corruption { at: None, bt: () })?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    pub fn flush(&self) -> Result<(), sled::Error> {
        self.db.flush()?;
        Ok(())
    }

    /// Async, batched write example: send blocks/rounds to this channel for background persistence
    pub fn spawn_background_writer(self: Arc<Self>, mut rx: mpsc::UnboundedReceiver<PersistMsg>) {
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match msg {
                    PersistMsg::Block(block) => self.save_block(&block),
                    PersistMsg::Round(round) => self.save_round(&round),
                }
            }
        });
    }

    // Store an asset record
    pub fn store_asset(&self, asset: &AssetRecord) -> Result<(), sled::Error> {
        let key = format!("asset:{}", asset.asset_id);
        let value = bincode::serialize(asset).unwrap();
        self.db.insert(key.as_bytes(), value)?;
        Ok(())
    }

    // Load an asset record
    pub fn load_asset(&self, asset_id: &str) -> Option<AssetRecord> {
        let key = format!("asset:{}", asset_id);
        self.db.get(key.as_bytes()).unwrap().map(|ivec| {
            bincode::deserialize(&ivec).unwrap()
        })
    }

    // Store a handle record
    pub fn store_handle(&self, handle: &HandleRecord) -> Result<(), sled::Error> {
        let key = format!("handle:{}", handle.handle);
        let value = bincode::serialize(handle).unwrap();
        self.db.insert(key.as_bytes(), value)?;
        Ok(())
    }

    // Load a handle record
    pub fn load_handle(&self, handle: &str) -> Option<HandleRecord> {
        let key = format!("handle:{}", handle);
        self.db.get(key.as_bytes()).unwrap().map(|ivec| {
            bincode::deserialize(&ivec).unwrap()
        })
    }
}

#[derive(Debug)]
pub enum PersistMsg {
    Block(Block),
    Round(Round),
} 
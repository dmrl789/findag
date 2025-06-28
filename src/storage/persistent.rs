use sled::{Db};
use crate::core::types::{Block, Round, SerializableBlock, SerializableRound};
use bincode;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::consensus::validator_set::ValidatorSet;
use crate::consensus::governance::GovernanceState;

pub struct PersistentStorage {
    pub db: Arc<Db>,
}

impl PersistentStorage {
    pub fn new(path: &str) -> Self {
        let db = sled::open(path).expect("Failed to open sled DB");
        Self { db: Arc::new(db) }
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

    pub fn save_validator_set(&self, set: &ValidatorSet) {
        let key = b"validators";
        let value = bincode::serialize(set).unwrap();
        self.db.insert(key, value).unwrap();
    }

    pub fn load_validator_set(&self) -> Option<ValidatorSet> {
        let key = b"validators";
        self.db.get(key).unwrap().map(|ivec| bincode::deserialize(&ivec).unwrap())
    }

    pub fn save_governance_state(&self, state: &GovernanceState) {
        let key = b"governance";
        let value = bincode::serialize(state).unwrap();
        self.db.insert(key, value).unwrap();
    }

    pub fn load_governance_state(&self) -> Option<GovernanceState> {
        let key = b"governance";
        self.db.get(key).unwrap().map(|ivec| bincode::deserialize(&ivec).unwrap())
    }

    pub fn save_asset_whitelist(&self, whitelist: &Vec<String>) {
        let key = b"asset_whitelist";
        let value = bincode::serialize(whitelist).unwrap();
        self.db.insert(key, value).unwrap();
    }

    pub fn load_asset_whitelist(&self) -> Option<Vec<String>> {
        let key = b"asset_whitelist";
        self.db.get(key).unwrap().map(|ivec| bincode::deserialize(&ivec).unwrap())
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
}

#[derive(Debug)]
pub enum PersistMsg {
    Block(Block),
    Round(Round),
}

// Usage:
// let storage = Arc::new(PersistentStorage::new("findag_db"));
// let (tx, rx) = mpsc::unbounded_channel();
// storage.clone().spawn_background_writer(rx);
// tx.send(PersistMsg::Block(block)).unwrap();
// tx.send(PersistMsg::Round(round)).unwrap(); 
use crate::blockchain::block::Block;
use crate::blockchain::error::BlockchainError;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;
use crate::types::block::{RoundHash};
use crate::types::transaction::Transaction;

pub struct Dag {
    blocks: Arc<TokioMutex<HashMap<Vec<u8>, Block>>>,
    transactions: TokioMutex<Vec<Transaction>>,
}

impl Dag {
    pub fn new() -> Self {
        Self {
            blocks: Arc::new(TokioMutex::new(HashMap::new())),
            transactions: TokioMutex::new(Vec::new()),
        }
    }

    pub async fn add_block(&self, block: Block) {
        let mut blocks = self.blocks.lock().await;
        blocks.insert(block.hash.clone(), block);
    }

    pub async fn add_transaction(&self, transaction: Transaction) {
        let mut transactions = self.transactions.lock().await;
        transactions.push(transaction);
    }

    pub async fn get_block(&self, hash: &[u8]) -> Option<Block> {
        let blocks = self.blocks.lock().await;
        blocks.get(hash).cloned()
    }

    pub async fn get_blocks(&self) -> Vec<Block> {
        let blocks = self.blocks.lock().await;
        blocks.values().cloned().collect()
    }

    pub async fn get_transactions(&self) -> Vec<Transaction> {
        let transactions = self.transactions.lock().await;
        transactions.clone()
    }

    pub async fn get_children(&self, parent_hash: &[u8]) -> Vec<Block> {
        let blocks = self.blocks.lock().await;
        blocks.values()
            .filter(|block| block.parents.contains(&parent_hash.to_vec()))
            .cloned()
            .collect()
    }
}

#[derive(Debug, Default, Clone)]
pub struct DagState {
    pub blocks: HashMap<Vec<u8>, Block>,
    pub edges: HashMap<Vec<u8>, HashSet<Vec<u8>>>, // parent -> children
    pub round_hashes: Vec<RoundHash>,
}

impl DagState {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            edges: HashMap::new(),
            round_hashes: Vec::new(),
        }
    }

    pub fn insert_block(&mut self, block: Block) {
        let block_hash = block.hash.clone();
        if self.blocks.contains_key(&block_hash) {
            return;
        }

        for parent in &block.parents {
            self.edges.entry(parent.clone()).or_default().insert(block_hash.clone());
        }

        self.blocks.insert(block_hash, block);
    }

    pub fn insert_round(&mut self, round: RoundHash) {
        self.round_hashes.push(round);
    }

    pub fn get_latest_blocks(&self) -> Vec<Block> {
        self.blocks.values().cloned().collect()
    }

    pub fn get_rounds(&self) -> Vec<RoundHash> {
        self.round_hashes.clone()
    }
}

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use crate::types::block::{Block, RoundHash};
use crate::utils::hashtimer::HashTimer;

#[derive(Debug, Default, Clone)]
pub struct DagState {
    pub blocks: HashMap<String, Block>,
    pub edges: HashMap<String, HashSet<String>>, // parent -> children
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

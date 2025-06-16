use crate::blockchain::block::Block;
use crate::types::transaction::Transaction;
use crate::blockchain::dag::Dag;
use crate::security::authentication::verify_signature;
use crate::validation::transaction::TransactionValidator;
use crate::utils::time::get_findag_time_micro;
use sha2::{Sha256, Digest};
use std::error::Error;
use std::collections::{HashSet, BinaryHeap};
use std::cmp::Ordering;
use crate::blockchain::error::BlockchainError;
use futures;
use crate::blockchain::state::State;
use tokio::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use blake3;

const MAX_BLOCK_SIZE: usize = 2 * 1024 * 1024; // 2MB
const MAX_TRANSACTIONS_PER_BLOCK: usize = 1000;
const MAX_PARENTS: usize = 3;
const MIN_PARENTS: usize = 1;

pub struct Assembler {
    state: State,
    validator: Mutex<TransactionValidator>,
}

impl Assembler {
    pub fn new(state: State) -> Self {
        Self {
            state: state.clone(),
            validator: Mutex::new(TransactionValidator::new(state)),
        }
    }

    pub async fn assemble_block(&self, transactions: Vec<Transaction>) -> Result<Block, Box<dyn Error>> {
        // Filter and validate transactions
        let mut valid_txs = Vec::new();
        let mut validator = self.validator.lock().await;
        for tx in transactions {
            if validator.validate(&tx).await? {
                valid_txs.push(tx);
            }
        }
        // Sort transactions by hash for deterministic ordering
        valid_txs.sort_by(|a, b| a.hash.cmp(&b.hash));
        // Create block
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let mut block = Block {
            hash: vec![],
            parents: vec![],
            timestamp,
            data: valid_txs,
            signature: vec![],
            justification: None,
        };
        // Calculate block hash
        let mut hasher = blake3::Hasher::new();
        for tx in &block.data {
            hasher.update(&tx.hash);
        }
        hasher.update(&timestamp.to_le_bytes());
        block.hash = hasher.finalize().as_bytes().to_vec();
        // Sign block
        let signature = self.sign_block(&block)?;
        block.signature = signature;
        Ok(block)
    }

    fn sign_block(&self, _block: &Block) -> Result<Vec<u8>, Box<dyn Error>> {
        // In a real implementation, this would use the node's private key
        // For now, we'll just return a dummy signature
        Ok(vec![0; 64])
    }
}

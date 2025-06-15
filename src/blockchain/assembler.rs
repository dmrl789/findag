use crate::blockchain::block::Block;
use crate::types::transaction::{Transaction, TxType};
use crate::blockchain::dag::DAG;
use crate::security::authentication::sign_message;
use crate::validation::transaction::TransactionValidator;
use crate::utils::time::get_findag_time_micro;
use sha2::{Sha256, Digest};
use std::error::Error;
use std::collections::{HashSet, BinaryHeap};
use std::cmp::Ordering;

const MAX_BLOCK_SIZE: usize = 2 * 1024 * 1024; // 2MB
const MAX_TRANSACTIONS_PER_BLOCK: usize = 1000;
const MAX_PARENTS: usize = 3;
const MIN_PARENTS: usize = 1;

#[derive(Clone)]
struct PrioritizedTransaction {
    transaction: Transaction,
    priority: u64,
}

impl Eq for PrioritizedTransaction {}

impl PartialEq for PrioritizedTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.transaction.hash == other.transaction.hash
    }
}

impl Ord for PrioritizedTransaction {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for PrioritizedTransaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct BlockAssembler {
    transactions: BinaryHeap<PrioritizedTransaction>,
    dag: DAG,
    private_key: Vec<u8>,
    validator: TransactionValidator,
}

impl BlockAssembler {
    pub fn new(dag: DAG, private_key: Vec<u8>, validator: TransactionValidator) -> Self {
        Self {
            transactions: BinaryHeap::new(),
            dag,
            private_key,
            validator,
        }
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), Box<dyn Error>> {
        // Validate transaction before adding
        self.validator.validate_transaction(&tx)?;

        // Calculate priority based on transaction type and urgency
        let current_time = get_findag_time_micro();
        let age = current_time.saturating_sub(tx.timestamp);
        
        let priority = match tx.tx_type {
            // High priority for critical financial operations
            TxType::TransferAsset { .. } => {
                // Urgent transfers get higher priority
                if age < 60_000_000 { // Within last minute (in microseconds)
                    1000
                } else if age < 300_000_000 { // Within last 5 minutes (in microseconds)
                    500
                } else {
                    100
                }
            }
            // Medium priority for standard operations
            TxType::Transfer => {
                if age < 300_000_000 { // Within last 5 minutes (in microseconds)
                    300
                } else {
                    200
                }
            }
            // Lower priority for administrative operations
            TxType::LoadAsset(_) | TxType::UnloadAsset(_) => 50,
            // Lowest priority for non-critical operations
            TxType::UpdateHandle { .. } | TxType::GovernanceVote => 10,
            // Default priority
            _ => 1,
        };

        self.transactions.push(PrioritizedTransaction {
            transaction: tx,
            priority,
        });

        Ok(())
    }

    pub fn select_parents(&self) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
        let mut parents = Vec::new();
        let mut seen = HashSet::new();
        
        // Get tips from DAG
        let tips = self.dag.get_tips();
        
        // Sort tips by timestamp (newer first)
        let mut sorted_tips: Vec<_> = tips.into_iter().collect();
        sorted_tips.sort_by(|a, b| {
            let block_a = self.dag.get_block(a).unwrap();
            let block_b = self.dag.get_block(b).unwrap();
            block_b.timestamp.cmp(&block_a.timestamp)
        });
        
        // Select parents from tips
        for tip in sorted_tips {
            if !seen.contains(&tip) {
                // Verify block is valid
                if let Some(block) = self.dag.get_block(&tip) {
                    if self.verify_block(block)? {
                        parents.push(tip);
                        seen.insert(tip);
                    }
                }
            }
            if parents.len() >= MAX_PARENTS {
                break;
            }
        }
        
        // Ensure we have minimum number of parents
        if parents.len() < MIN_PARENTS {
            return Err("Insufficient valid parents".into());
        }
        
        Ok(parents)
    }

    fn verify_block(&self, block: &Block) -> Result<bool, Box<dyn Error>> {
        // Verify block signature
        let message = block.hash();
        if !self.verify_signature(&message, &block.signature)? {
            return Ok(false);
        }

        // Verify all transactions in block
        for tx in &block.data {
            if self.validator.validate_transaction(tx).is_err() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn assemble_block(&self) -> Result<Block, Box<dyn Error>> {
        let mut block_size = 0;
        let mut selected_txs = Vec::new();
        
        // Select transactions based on priority and size limits
        let mut temp_heap = self.transactions.clone();
        while let Some(prioritized_tx) = temp_heap.pop() {
            let tx_size = prioritized_tx.transaction.data.len();
            if block_size + tx_size > MAX_BLOCK_SIZE || selected_txs.len() >= MAX_TRANSACTIONS_PER_BLOCK {
                break;
            }
            selected_txs.push(prioritized_tx.transaction);
            block_size += tx_size;
        }
        
        // Sort transactions by hash for deterministic ordering
        selected_txs.sort_by(|a, b| a.hash().cmp(&b.hash()));
        
        // Calculate block hash
        let mut hasher = Sha256::new();
        for tx in &selected_txs {
            hasher.update(tx.hash());
            hasher.update(&tx.signature);
        }
        let hash = hasher.finalize().to_vec();
        
        // Select parents
        let parents = self.select_parents()?;
        
        // Create block
        let mut block = Block {
            hash,
            parents,
            timestamp: get_findag_time_micro(),
            data: selected_txs,
            signature: Vec::new(),
            justification: None,
        };
        
        // Sign block
        let message = block.hash();
        block.signature = sign_message(&message, &self.private_key)?;
        
        Ok(block)
    }

    fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<bool, Box<dyn Error>> {
        // TODO: Implement proper signature verification
        Ok(true)
    }
}

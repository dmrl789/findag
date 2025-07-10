//! Transaction mempool management
//! 
//! This module handles transaction mempool operations and management.

use findag_core::{Address, Hash, HashTimer, FinDAGTime};
use findag_types::{Transaction, TransactionStatus, FindDAGResult};

/// Transaction mempool
pub struct Mempool {
    /// Pending transactions
    transactions: std::collections::HashMap<Hash, Transaction>,
}

impl Mempool {
    /// Create a new mempool
    pub fn new() -> Self {
        Self {
            transactions: std::collections::HashMap::new(),
        }
    }

    /// Add a transaction to the mempool
    pub fn add_transaction(&mut self, transaction: Transaction) -> FindDAGResult<()> {
        let hash = transaction.hash.clone();
        self.transactions.insert(hash, transaction);
        Ok(())
    }

    /// Remove a transaction from the mempool
    pub fn remove_transaction(&mut self, hash: &Hash) -> FindDAGResult<()> {
        if self.transactions.remove(hash).is_none() {
            return Err(findag_types::FindDAGError::ValidationError(
                "Transaction not found in mempool".to_string()
            ));
        }
        Ok(())
    }

    /// Get transaction by hash
    pub fn get_transaction(&self, hash: &Hash) -> Option<&Transaction> {
        self.transactions.get(hash)
    }

    /// Get all transactions
    pub fn get_all_transactions(&self) -> Vec<&Transaction> {
        self.transactions.values().collect()
    }

    /// Get mempool size
    pub fn size(&self) -> usize {
        self.transactions.len()
    }
} 
//! Transaction storage operations
//! 
//! This module handles storage operations for blockchain transactions.

use sled::Tree;
use findag_core::{Hash, Address, FinDAGTime};
use findag_types::{Transaction, FindDAGResult};

/// Transaction storage manager
pub struct TransactionStorage {
    /// Transactions tree
    transactions_tree: Tree,
}

impl TransactionStorage {
    /// Create a new transaction storage manager
    pub fn new(transactions_tree: Tree) -> Self {
        Self { transactions_tree }
    }

    /// Store a transaction
    pub fn store_transaction(&self, transaction: &Transaction) -> FindDAGResult<()> {
        let key = transaction.hash.to_string();
        let value = bincode::serialize(transaction)?;
        
        self.transactions_tree.insert(key.as_bytes(), value)?;
        Ok(())
    }

    /// Get a transaction by hash
    pub fn get_transaction(&self, hash: &Hash) -> FindDAGResult<Option<Transaction>> {
        let key = hash.to_string();
        let result = self.transactions_tree.get(key.as_bytes())?;
        
        if let Some(value) = result {
            let transaction: Transaction = bincode::deserialize(&value)?;
            Ok(Some(transaction))
        } else {
            Ok(None)
        }
    }

    /// Get transactions by sender
    pub fn get_transactions_by_sender(&self, sender: &Address) -> FindDAGResult<Vec<Transaction>> {
        let mut transactions = Vec::new();
        
        for result in self.transactions_tree.iter() {
            let (_, value) = result?;
            if let Ok(transaction) = bincode::deserialize::<Transaction>(&value) {
                for instruction in &transaction.instructions {
                    if instruction.from == *sender {
                        transactions.push(transaction.clone());
                        break;
                    }
                }
            }
        }
        
        Ok(transactions)
    }

    /// Get transactions by recipient
    pub fn get_transactions_by_recipient(&self, recipient: &Address) -> FindDAGResult<Vec<Transaction>> {
        let mut transactions = Vec::new();
        
        for result in self.transactions_tree.iter() {
            let (_, value) = result?;
            if let Ok(transaction) = bincode::deserialize::<Transaction>(&value) {
                for instruction in &transaction.instructions {
                    if let Some(to) = &instruction.to {
                        if *to == *recipient {
                            transactions.push(transaction.clone());
                            break;
                        }
                    }
                }
            }
        }
        
        Ok(transactions)
    }

    /// Get transactions by asset
    pub fn get_transactions_by_asset(&self, asset: &str) -> FindDAGResult<Vec<Transaction>> {
        let mut transactions = Vec::new();
        
        for result in self.transactions_tree.iter() {
            let (_, value) = result?;
            if let Ok(transaction) = bincode::deserialize::<Transaction>(&value) {
                for instruction in &transaction.instructions {
                    if instruction.asset == asset {
                        transactions.push(transaction.clone());
                        break;
                    }
                }
            }
        }
        
        Ok(transactions)
    }

    /// Get transactions by timestamp range
    pub fn get_transactions_by_timestamp_range(&self, start: FinDAGTime, end: FinDAGTime) -> FindDAGResult<Vec<Transaction>> {
        let mut transactions = Vec::new();
        
        for result in self.transactions_tree.iter() {
            let (_, value) = result?;
            if let Ok(transaction) = bincode::deserialize::<Transaction>(&value) {
                if transaction.timestamp >= start && transaction.timestamp <= end {
                    transactions.push(transaction);
                }
            }
        }
        
        Ok(transactions)
    }

    /// Get transaction count
    pub fn get_transaction_count(&self) -> FindDAGResult<u64> {
        let count = self.transactions_tree.len();
        Ok(count)
    }

    /// Delete transaction
    pub fn delete_transaction(&self, hash: &Hash) -> FindDAGResult<()> {
        let key = hash.to_string();
        self.transactions_tree.remove(key.as_bytes())?;
        Ok(())
    }

    /// Check if transaction exists
    pub fn transaction_exists(&self, hash: &Hash) -> FindDAGResult<bool> {
        let key = hash.to_string();
        let result = self.transactions_tree.get(key.as_bytes())?;
        Ok(result.is_some())
    }
} 
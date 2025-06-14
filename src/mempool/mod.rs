use std::error::Error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A shared mempool that stores pending transactions per asset type.
pub struct Mempool {
    transactions: Arc<Mutex<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl Mempool {
    /// Creates a new mempool instance.
    pub fn new() -> Self {
        Mempool {
            transactions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Adds a transaction to the mempool if it hasn't been seen yet.
    pub fn add_transaction(&self, tx_hash: Vec<u8>, tx_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        println!("Adding transaction to mempool");
        let mut txs = self.transactions.lock().unwrap();
        txs.insert(tx_hash, tx_data);
        Ok(())
    }

    /// Pops the next transaction (FIFO) from the mempool.
    pub fn get_transaction(&self, tx_hash: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        println!("Getting transaction from mempool");
        let txs = self.transactions.lock().unwrap();
        Ok(txs.get(tx_hash).cloned())
    }

    /// Peeks all transactions currently in the pool.
    pub fn remove_transaction(&self, tx_hash: &[u8]) -> Result<(), Box<dyn Error>> {
        println!("Removing transaction from mempool");
        let mut txs = self.transactions.lock().unwrap();
        txs.remove(tx_hash);
        Ok(())
    }

    /// Returns the number of transactions currently in the pool.
    pub fn size(&self) -> usize {
        let txs = self.transactions.lock().unwrap();
        txs.len()
    }

    /// Clears the mempool.
    pub fn clear(&self) {
        let mut txs = self.transactions.lock().unwrap();
        txs.clear();
    }
}

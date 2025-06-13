use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use crate::types::transaction::{Transaction, TxType, TxHash};

/// A shared mempool that stores pending transactions per asset type.
#[derive(Clone, Debug)]
pub struct Mempool {
    queue: Arc<Mutex<VecDeque<Transaction>>>,
    seen_hashes: Arc<Mutex<HashSet<TxHash>>>,
}

impl Mempool {
    /// Creates a new mempool instance.
    pub fn new() -> Self {
        Mempool {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            seen_hashes: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Adds a transaction to the mempool if it hasn't been seen yet.
    pub fn add_transaction(&self, tx: Transaction) -> bool {
        let hash = tx.hash();
        let mut seen = self.seen_hashes.lock().unwrap();
        if seen.contains(&hash) {
            return false; // duplicate
        }

        seen.insert(hash);
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(tx);
        true
    }

    /// Pops the next transaction (FIFO) from the mempool.
    pub fn pop_transaction(&self) -> Option<Transaction> {
        let mut queue = self.queue.lock().unwrap();
        if let Some(tx) = queue.pop_front() {
            let mut seen = self.seen_hashes.lock().unwrap();
            seen.remove(&tx.hash());
            Some(tx)
        } else {
            None
        }
    }

    /// Peeks all transactions currently in the pool.
    pub fn all_transactions(&self) -> Vec<Transaction> {
        let queue = self.queue.lock().unwrap();
        queue.iter().cloned().collect()
    }

    /// Returns the number of transactions currently in the pool.
    pub fn size(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.len()
    }

    /// Clears the mempool.
    pub fn clear(&self) {
        let mut queue = self.queue.lock().unwrap();
        queue.clear();
        let mut seen = self.seen_hashes.lock().unwrap();
        seen.clear();
    }
}

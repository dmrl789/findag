use std::collections::VecDeque;
use tokio::sync::Mutex;
use crate::core::types::Transaction;

/// Minimal Mempool for instructions/transactions
/// Works alongside the existing TxPool for block production
#[derive(Debug)]
pub struct Mempool {
    queue: Mutex<VecDeque<Transaction>>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    /// Add transaction to mempool
    pub async fn add(&self, tx: Transaction) {
        let mut q = self.queue.lock().await;
        q.push_back(tx);
    }

    /// Get next transaction (FIFO)
    pub async fn next(&self) -> Option<Transaction> {
        let mut q = self.queue.lock().await;
        q.pop_front()
    }

    /// Get a batch of transactions (FIFO)
    pub async fn get_batch(&self, max_count: usize) -> Vec<Transaction> {
        let mut q = self.queue.lock().await;
        let mut batch = Vec::new();
        for _ in 0..max_count {
            if let Some(tx) = q.pop_front() {
                batch.push(tx);
            } else {
                break;
            }
        }
        batch
    }

    /// Peek length
    pub async fn len(&self) -> usize {
        let q = self.queue.lock().await;
        q.len()
    }
} 
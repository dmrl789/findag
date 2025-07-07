use std::collections::VecDeque;
use tokio::sync::Mutex;
use crate::core::types::Transaction;

/// Minimal Mempool for instructions/transactions
/// Works alongside the existing TxPool for block production
#[derive(Debug)]
pub struct Mempool {
    queue: Mutex<VecDeque<Transaction>>,
}

impl Default for Mempool {
    fn default() -> Self {
        Self::new()
    }
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    /// Add transaction to mempool
    pub async fn add(&self, tx: Transaction) {
        let from = tx.from.0.clone();
        let to = tx.to.0.clone();
        let amount = tx.amount;
        let mut q = self.queue.lock().await;
        q.push_back(tx);
        println!("[Mempool] Added transaction: from={}, to={}, amount={}, queue_size={}", 
                 from, to, amount, q.len());
    }

    /// Get next transaction (FIFO)
    pub async fn next(&self) -> Option<Transaction> {
        let mut q = self.queue.lock().await;
        let result = q.pop_front();
        if result.is_some() {
            println!("[Mempool] Retrieved 1 transaction, queue_size={}", q.len());
        }
        result
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
        println!("[Mempool] Retrieved {} transactions, queue_size={}", batch.len(), q.len());
        batch
    }

    /// Get a batch of transactions (FIFO) - synchronous version for when called from async context
    /// This version tries to acquire the lock without blocking indefinitely
    pub fn get_batch_sync(&self, max_count: usize) -> Vec<Transaction> {
        // Use try_lock to avoid deadlock - if we can't get the lock immediately, return empty
        if let Ok(mut q) = self.queue.try_lock() {
            let mut batch = Vec::new();
            for _ in 0..max_count {
                if let Some(tx) = q.pop_front() {
                    batch.push(tx);
                } else {
                    break;
                }
            }
            println!("[Mempool] Retrieved {} transactions, queue_size={}", batch.len(), q.len());
            batch
        } else {
            println!("[Mempool] Could not acquire queue lock, returning empty batch");
            Vec::new()
        }
    }

    /// Peek length
    pub async fn len(&self) -> usize {
        let q = self.queue.lock().await;
        q.len()
    }

    /// Get all transactions without removing them (for viewing)
    pub async fn get_all(&self) -> Vec<Transaction> {
        let q = self.queue.lock().await;
        q.iter().cloned().collect()
    }
} 
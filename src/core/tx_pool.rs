use crate::core::types::Transaction;
use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, Mutex};

const SHARD_COUNT: usize = 16;

/// Transaction Pool (Mempool) for FinDAG
/// - Deduplicates by transaction hash
/// - Prioritizes by FinDAG Time (oldest first)
/// - Enforces a maximum pool size per shard
pub struct TxPool {
    // Transaction hash -> Transaction
    pub transactions: HashMap<[u8; 32], Transaction>,
    // FinDAG Time -> set of transaction hashes (for prioritization)
    pub time_index: BTreeMap<u64, Vec<[u8; 32]>>,
    pub max_size: usize,
}

impl TxPool {
    pub fn new(max_size: usize) -> Self {
        Self {
            transactions: HashMap::new(),
            time_index: BTreeMap::new(),
            max_size,
        }
    }

    /// Add a new transaction to the pool. Returns true if added.
    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        let tx_hash = tx.hashtimer; // Or use a real tx hash if available
        if self.transactions.contains_key(&tx_hash) {
            return false; // Duplicate
        }
        if self.transactions.len() >= self.max_size {
            // Evict oldest transaction(s) to make room
            if let Some((&oldest_time, hashes)) = self.time_index.iter_mut().next() {
                if let Some(evict_hash) = hashes.pop() {
                    self.transactions.remove(&evict_hash);
                }
                if hashes.is_empty() {
                    self.time_index.remove(&oldest_time);
                }
            }
        }
        self.time_index.entry(tx.findag_time).or_default().push(tx_hash);
        self.transactions.insert(tx_hash, tx);
        true
    }

    /// Remove a transaction (e.g., after block inclusion)
    pub fn remove_transaction(&mut self, tx_hash: &[u8; 32], findag_time: u64) {
        self.transactions.remove(tx_hash);
        if let Some(hashes) = self.time_index.get_mut(&findag_time) {
            hashes.retain(|h| h != tx_hash);
            if hashes.is_empty() {
                self.time_index.remove(&findag_time);
            }
        }
    }

    /// Get transactions for block production (oldest first, up to a limit)
    pub fn get_transactions(&self, limit: usize) -> Vec<&Transaction> {
        let mut result = Vec::with_capacity(limit);
        for (_time, hashes) in &self.time_index {
            for hash in hashes {
                if let Some(tx) = self.transactions.get(hash) {
                    result.push(tx);
                    if result.len() == limit {
                        return result;
                    }
                }
            }
        }
        result
    }

    /// Pool size
    pub fn size(&self) -> usize {
        self.transactions.len()
    }
}

/// Sharded, in-RAM transaction pool for high throughput
pub struct ShardedTxPool {
    shards: Vec<Mutex<TxPool>>,
}

impl ShardedTxPool {
    pub fn new(max_size_per_shard: usize) -> Self {
        let mut shards = Vec::with_capacity(SHARD_COUNT);
        for _ in 0..SHARD_COUNT {
            shards.push(Mutex::new(TxPool::new(max_size_per_shard)));
        }
        Self { shards }
    }
    fn shard_for(&self, tx_hash: &[u8; 32]) -> usize {
        (tx_hash[0] as usize) % SHARD_COUNT
    }
    pub fn add_transaction(&self, tx: Transaction) -> bool {
        let shard = self.shard_for(&tx.hashtimer);
        self.shards[shard].lock().unwrap().add_transaction(tx)
    }
    pub fn remove_transaction(&self, tx_hash: &[u8; 32], findag_time: u64) {
        let shard = self.shard_for(tx_hash);
        self.shards[shard].lock().unwrap().remove_transaction(tx_hash, findag_time);
    }
    pub fn get_transactions(&self, limit: usize) -> Vec<Transaction> {
        // Collect up to 'limit' transactions from all shards, oldest first
        let mut all_txs = Vec::new();
        for shard in &self.shards {
            let shard_txs = shard.lock().unwrap().get_transactions(limit);
            for tx in shard_txs {
                all_txs.push(tx.clone());
                if all_txs.len() == limit {
                    return all_txs;
                }
            }
        }
        all_txs
    }
    pub fn size(&self) -> usize {
        self.shards.iter().map(|s| s.lock().unwrap().size()).sum()
    }
} 
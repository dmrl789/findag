use crate::core::types::{Transaction};
use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, Mutex};
use crate::storage::state::StateDB;
use crate::metrics;

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
    pub state_db: Arc<StateDB>,
    pub asset_whitelist: Arc<Mutex<Vec<String>>>,
}

impl TxPool {
    pub fn new(max_size: usize, state_db: Arc<StateDB>, asset_whitelist: Arc<Mutex<Vec<String>>>) -> Self {
        Self {
            transactions: HashMap::new(),
            time_index: BTreeMap::new(),
            max_size,
            state_db,
            asset_whitelist,
        }
    }

    /// Add a new transaction to the pool. Returns true if added.
    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        // Cross-shard transaction protocol (scaffold)
        if let (Some(source), Some(dest)) = (tx.source_shard, tx.dest_shard) {
            // TODO: Implement two-phase commit for cross-shard txs
            // Phase 1: Lock/prepare on source shard
            // Phase 2: Commit/acknowledge on destination shard
            // Finalize and update state on both shards
            println!("[TxPool] Received cross-shard tx: {:?} -> {:?}", source, dest);
            // For now, reject or queue cross-shard txs
            return false;
        }
        let tx_hash = tx.hashtimer; // Or use a real tx hash if available
        if self.transactions.contains_key(&tx_hash) {
            metrics::ERROR_COUNT.with_label_values(&["duplicate_tx"]).inc();
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
        // Enforce dynamic asset whitelist
        let asset = "USD"; // Default asset for now
        let whitelist = self.asset_whitelist.lock().unwrap();
        if !whitelist.contains(&asset.to_string()) {
            println!("[TxPool] Rejected tx: unsupported asset '{}'.", asset);
            metrics::ERROR_COUNT.with_label_values(&["unsupported_asset"]).inc();
            return false;
        }
        // Check sender balance before adding
        let from = tx.from.as_str();
        let amount = tx.amount;
        let bal = self.state_db.get_balance(tx.shard_id.0, from, asset);
        if bal < amount {
            println!("[TxPool] Rejected tx: insufficient funds for {} ({} {})", from, amount, asset);
            metrics::ERROR_COUNT.with_label_values(&["insufficient_funds"]).inc();
            return false;
        }
        self.time_index.entry(tx.findag_time).or_default().push(tx_hash);
        let added = self.transactions.insert(tx_hash, tx).is_none();
        if added {
            metrics::MEMPOOL_SIZE.set(self.transactions.len() as i64);
        }
        added
    }

    /// Remove a transaction (e.g., after block inclusion)
    pub fn remove_transaction(&mut self, tx_hash: &[u8; 32], findag_time: u64) {
        let removed = self.transactions.remove(tx_hash).is_some();
        if removed {
            metrics::MEMPOOL_SIZE.set(self.transactions.len() as i64);
        }
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
    shard_count: usize,
}

impl ShardedTxPool {
    pub fn new_with_whitelist_per_shard(max_size_per_shard: usize, asset_whitelist: Arc<Mutex<Vec<String>>>, shard_count: usize) -> Self {
        let mut shards = Vec::with_capacity(shard_count);
        let state_db = Arc::new(StateDB::new("state_db")); // TODO: Pass real state_db if needed
        for _ in 0..shard_count {
            shards.push(Mutex::new(TxPool::new(max_size_per_shard, state_db.clone(), asset_whitelist.clone())));
        }
        Self { shards, shard_count }
    }
    /// Route by tx.shard_id (single-shard mode: always 0)
    fn shard_for_id(&self, shard_id: u16) -> usize {
        (shard_id as usize) % self.shard_count
    }
    pub fn add_transaction(&self, tx: Transaction) -> bool {
        let shard = self.shard_for_id(tx.shard_id.0);
        self.shards[shard].lock().unwrap().add_transaction(tx)
    }
    pub fn remove_transaction(&self, tx_hash: &[u8; 32], findag_time: u64, shard_id: u16) {
        let shard = self.shard_for_id(shard_id);
        self.shards[shard].lock().unwrap().remove_transaction(tx_hash, findag_time);
    }
    pub fn get_transactions(&self, limit: usize, shard_id: u16) -> Vec<Transaction> {
        let mut txs = Vec::new();
        let shard = self.shard_for_id(shard_id);
        let binding = self.shards[shard].lock().unwrap();
        let shard_txs = binding.get_transactions(limit);
        for tx in shard_txs {
            txs.push(tx.clone());
            if txs.len() == limit {
                break;
            }
        }
        txs
    }
    pub fn size(&self, shard_id: u16) -> usize {
        let shard = self.shard_for_id(shard_id);
        self.shards[shard].lock().unwrap().size()
    }
    // For future: add multi-shard aggregation methods
} 
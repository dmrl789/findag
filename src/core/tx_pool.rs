use crate::core::types::{Transaction};
use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, Mutex};
use crate::storage::state::StateDB;
use crate::metrics;
use sha2::{Sha256, Digest};
use hex;

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

    /// Compute transaction hash from transaction data
    fn compute_tx_hash(&self, tx: &Transaction) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(tx.from.as_str().as_bytes());
        hasher.update(tx.to.as_str().as_bytes());
        hasher.update(&tx.amount.to_le_bytes());
        hasher.update(&tx.payload);
        hasher.update(&tx.findag_time.to_le_bytes());
        hasher.update(&tx.public_key.to_bytes());
        hasher.update(&tx.shard_id.0.to_le_bytes());
        hasher.finalize().into()
    }

    /// Add a new transaction to the pool. Returns true if added.
    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        println!("[DEBUG] TxPool: Attempting to add transaction: from={}, to={}, amount={}", 
                 tx.from.as_str(), tx.to.as_str(), tx.amount);
        
        // Cross-shard transaction protocol (scaffold)
        if let (Some(source), Some(dest)) = (tx.source_shard, tx.dest_shard) {
            // TODO: Implement two-phase commit for cross-shard txs
            // Phase 1: Lock/prepare on source shard
            // Phase 2: Commit/acknowledge on destination shard
            // Finalize and update state on both shards
            println!("[TxPool] Rejected cross-shard tx: {source:?} -> {dest:?}");
            // For now, reject or queue cross-shard txs
            return false;
        }
        
        let tx_hash = self.compute_tx_hash(&tx);
        if self.transactions.contains_key(&tx_hash) {
            metrics::ERROR_COUNT.with_label_values(&["duplicate_tx"]).inc();
            println!("[DEBUG] TxPool: Rejected duplicate transaction with hash: 0x{}", 
                     hex::encode(tx_hash));
            return false; // Duplicate
        }
        
        if self.transactions.len() >= self.max_size {
            // Evict oldest transaction(s) to make room
            if let Some((&oldest_time, hashes)) = self.time_index.iter_mut().next() {
                if let Some(evict_hash) = hashes.pop() {
                    self.transactions.remove(&evict_hash);
                    println!("[DEBUG] TxPool: Evicted old transaction to make room");
                }
                if hashes.is_empty() {
                    self.time_index.remove(&oldest_time);
                }
            }
        }
        
        // For now, skip asset whitelist check since we're using a default asset
        // TODO: Add proper asset field to Transaction struct
        
        // Check sender balance before adding (using USD as default asset)
        let from = tx.from.as_str();
        let amount = tx.amount;
        let bal = self.state_db.get_balance(tx.shard_id.0, from, "USD");
        println!("[DEBUG] TxPool: Balance check for {}: amount={}, balance={}, shard_id={}", from, amount, bal, tx.shard_id.0);
        if bal < amount {
            println!("[DEBUG] TxPool: Rejected tx: insufficient funds for {from} ({amount} USD, balance: {bal})");
            metrics::ERROR_COUNT.with_label_values(&["insufficient_funds"]).inc();
            return false;
        }
        
        self.time_index.entry(tx.findag_time).or_default().push(tx_hash);
        let added = self.transactions.insert(tx_hash, tx).is_none();
        if added {
            metrics::MEMPOOL_SIZE.set(self.transactions.len() as i64);
            println!("[DEBUG] TxPool: Successfully added transaction, pool size: {}", self.transactions.len());
        } else {
            println!("[DEBUG] TxPool: Failed to add transaction (insert returned Some)");
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
        println!("[DEBUG] TxPool: get_transactions called with limit={}, pool_size={}", limit, self.transactions.len());
        
        let mut result = Vec::with_capacity(limit);
        for hashes in self.time_index.values() {
            for hash in hashes {
                if let Some(tx) = self.transactions.get(hash) {
                    result.push(tx);
                    if result.len() == limit {
                        println!("[DEBUG] TxPool: Returning {} transactions (reached limit)", result.len());
                        return result;
                    }
                }
            }
        }
        println!("[DEBUG] TxPool: Returning {} transactions (all available)", result.len());
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
        Self::new_with_whitelist_per_shard_and_data_dir(max_size_per_shard, asset_whitelist, shard_count, "state_db")
    }

    pub fn new_with_whitelist_per_shard_and_data_dir(max_size_per_shard: usize, asset_whitelist: Arc<Mutex<Vec<String>>>, shard_count: usize, data_dir: &str) -> Self {
        let mut shards = Vec::with_capacity(shard_count);
        let state_db = Arc::new(StateDB::new(data_dir));
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
        println!("[DEBUG] ShardedTxPool: Adding transaction to shard {} (shard_id={})", shard, tx.shard_id.0);
        let result = self.shards[shard].lock().unwrap().add_transaction(tx);
        println!("[DEBUG] ShardedTxPool: Transaction add result: {}", result);
        result
    }
    pub fn remove_transaction(&self, tx_hash: &[u8; 32], findag_time: u64, shard_id: u16) {
        let shard = self.shard_for_id(shard_id);
        self.shards[shard].lock().unwrap().remove_transaction(tx_hash, findag_time);
    }
    pub fn get_transactions(&self, limit: usize, shard_id: u16) -> Vec<Transaction> {
        let mut txs = Vec::new();
        let shard = self.shard_for_id(shard_id);
        println!("[DEBUG] ShardedTxPool: get_transactions called with limit={}, shard_id={}, target_shard={}", 
                 limit, shard_id, shard);
        
        let binding = self.shards[shard].lock().unwrap();
        let shard_txs = binding.get_transactions(limit);
        println!("[DEBUG] ShardedTxPool: Retrieved {} transactions from shard {}", shard_txs.len(), shard);
        
        for tx in shard_txs {
            txs.push(tx.clone());
            if txs.len() == limit {
                break;
            }
        }
        println!("[DEBUG] ShardedTxPool: Returning {} transactions total", txs.len());
        txs
    }
    pub fn size(&self, shard_id: u16) -> usize {
        let shard = self.shard_for_id(shard_id);
        self.shards[shard].lock().unwrap().size()
    }
    pub fn get_balance(&self, shard_id: u16, address: &str, asset: &str) -> u64 {
        let shard = (shard_id as usize) % self.shard_count;
        self.shards[shard].lock().unwrap().state_db.get_balance(shard_id, address, asset)
    }
    // For future: add multi-shard aggregation methods
} 
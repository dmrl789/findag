//! Core database operations
//! 
//! This module provides core database operations for the Sled storage system.

use sled::{Db, Tree, IVec};
use findag_types::{FindDAGResult, FindDAGError};

/// Database manager
pub struct DatabaseManager {
    /// Sled database
    db: Db,
    /// Database trees
    trees: std::collections::HashMap<String, Tree>,
}

impl DatabaseManager {
    /// Create a new database manager
    pub fn new(db_path: &str) -> FindDAGResult<Self> {
        let db = sled::open(db_path)?;
        let mut trees = std::collections::HashMap::new();
        
        // Initialize default trees
        let tree_names = vec![
            "blocks", "rounds", "transactions", "assets", "balances",
            "wallets", "validators", "governance", "metadata",
        ];
        
        for tree_name in tree_names {
            let tree = db.open_tree(tree_name)?;
            trees.insert(tree_name.to_string(), tree);
        }
        
        Ok(Self { db, trees })
    }

    /// Get a tree by name
    pub fn get_tree(&self, name: &str) -> Option<&Tree> {
        self.trees.get(name)
    }

    /// Get a mutable tree by name
    pub fn get_tree_mut(&mut self, name: &str) -> Option<&mut Tree> {
        self.trees.get_mut(name)
    }

    /// Create a new tree
    pub fn create_tree(&mut self, name: &str) -> FindDAGResult<()> {
        if !self.trees.contains_key(name) {
            let tree = self.db.open_tree(name)?;
            self.trees.insert(name.to_string(), tree);
        }
        Ok(())
    }

    /// Delete a tree
    pub fn delete_tree(&mut self, name: &str) -> FindDAGResult<()> {
        if let Some(tree) = self.trees.remove(name) {
            self.db.drop_tree(name)?;
        }
        Ok(())
    }

    /// List all trees
    pub fn list_trees(&self) -> Vec<String> {
        self.trees.keys().cloned().collect()
    }

    /// Get database size
    pub fn get_database_size(&self) -> FindDAGResult<u64> {
        let size = self.db.size_on_disk()?;
        Ok(size)
    }

    /// Flush database
    pub fn flush(&self) -> FindDAGResult<()> {
        self.db.flush()?;
        Ok(())
    }

    /// Create a backup
    pub fn create_backup(&self, backup_path: &str) -> FindDAGResult<()> {
        // TODO: Implement backup functionality
        Ok(())
    }

    /// Restore from backup
    pub fn restore_from_backup(&mut self, backup_path: &str) -> FindDAGResult<()> {
        // TODO: Implement restore functionality
        Ok(())
    }
}

/// Key-value operations
pub trait KeyValueOperations {
    /// Insert a key-value pair
    fn insert(&self, key: &[u8], value: &[u8]) -> FindDAGResult<()>;
    
    /// Get a value by key
    fn get(&self, key: &[u8]) -> FindDAGResult<Option<Vec<u8>>>;
    
    /// Remove a key-value pair
    fn remove(&self, key: &[u8]) -> FindDAGResult<()>;
    
    /// Check if a key exists
    fn contains_key(&self, key: &[u8]) -> FindDAGResult<bool>;
    
    /// Get all key-value pairs
    fn get_all(&self) -> FindDAGResult<Vec<(Vec<u8>, Vec<u8>)>>;
    
    /// Get all keys
    fn get_keys(&self) -> FindDAGResult<Vec<Vec<u8>>>;
    
    /// Get all values
    fn get_values(&self) -> FindDAGResult<Vec<Vec<u8>>>;
}

impl KeyValueOperations for Tree {
    fn insert(&self, key: &[u8], value: &[u8]) -> FindDAGResult<()> {
        self.insert(key, value)?;
        Ok(())
    }
    
    fn get(&self, key: &[u8]) -> FindDAGResult<Option<Vec<u8>>> {
        let result = self.get(key)?;
        Ok(result.map(|v| v.to_vec()))
    }
    
    fn remove(&self, key: &[u8]) -> FindDAGResult<()> {
        self.remove(key)?;
        Ok(())
    }
    
    fn contains_key(&self, key: &[u8]) -> FindDAGResult<bool> {
        let result = self.get(key)?;
        Ok(result.is_some())
    }
    
    fn get_all(&self) -> FindDAGResult<Vec<(Vec<u8>, Vec<u8>)>> {
        let mut pairs = Vec::new();
        for result in self.iter() {
            let (key, value) = result?;
            pairs.push((key.to_vec(), value.to_vec()));
        }
        Ok(pairs)
    }
    
    fn get_keys(&self) -> FindDAGResult<Vec<Vec<u8>>> {
        let mut keys = Vec::new();
        for result in self.iter() {
            let (key, _) = result?;
            keys.push(key.to_vec());
        }
        Ok(keys)
    }
    
    fn get_values(&self) -> FindDAGResult<Vec<Vec<u8>>> {
        let mut values = Vec::new();
        for result in self.iter() {
            let (_, value) = result?;
            values.push(value.to_vec());
        }
        Ok(values)
    }
}

/// Range operations
pub trait RangeOperations {
    /// Get a range of key-value pairs
    fn get_range(&self, start: &[u8], end: &[u8]) -> FindDAGResult<Vec<(Vec<u8>, Vec<u8>)>>;
    
    /// Get a range of keys
    fn get_range_keys(&self, start: &[u8], end: &[u8]) -> FindDAGResult<Vec<Vec<u8>>>;
    
    /// Get a range of values
    fn get_range_values(&self, start: &[u8], end: &[u8]) -> FindDAGResult<Vec<Vec<u8>>>;
}

impl RangeOperations for Tree {
    fn get_range(&self, start: &[u8], end: &[u8]) -> FindDAGResult<Vec<(Vec<u8>, Vec<u8>)>> {
        let mut pairs = Vec::new();
        for result in self.range(start..end) {
            let (key, value) = result?;
            pairs.push((key.to_vec(), value.to_vec()));
        }
        Ok(pairs)
    }
    
    fn get_range_keys(&self, start: &[u8], end: &[u8]) -> FindDAGResult<Vec<Vec<u8>>> {
        let mut keys = Vec::new();
        for result in self.range(start..end) {
            let (key, _) = result?;
            keys.push(key.to_vec());
        }
        Ok(keys)
    }
    
    fn get_range_values(&self, start: &[u8], end: &[u8]) -> FindDAGResult<Vec<Vec<u8>>> {
        let mut values = Vec::new();
        for result in self.range(start..end) {
            let (_, value) = result?;
            values.push(value.to_vec());
        }
        Ok(values)
    }
} 
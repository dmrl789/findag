//! Validator storage operations
//! 
//! This module handles storage operations for validator management.

use sled::Tree;
use findag_core::{Address};
use findag_types::{Validator, FindDAGResult};

/// Validator storage manager
pub struct ValidatorStorage {
    /// Validators tree
    validators_tree: Tree,
}

impl ValidatorStorage {
    /// Create a new validator storage manager
    pub fn new(validators_tree: Tree) -> Self {
        Self { validators_tree }
    }

    /// Store a validator
    pub fn store_validator(&self, validator: &Validator) -> FindDAGResult<()> {
        let key = format!("validator:{}", validator.address);
        let value = bincode::serialize(validator)?;
        
        self.validators_tree.insert(key.as_bytes(), value)?;
        Ok(())
    }

    /// Get a validator by address
    pub fn get_validator(&self, address: &Address) -> FindDAGResult<Option<Validator>> {
        let key = format!("validator:{}", address);
        let result = self.validators_tree.get(key.as_bytes())?;
        
        if let Some(value) = result {
            let validator: Validator = bincode::deserialize(&value)?;
            Ok(Some(validator))
        } else {
            Ok(None)
        }
    }

    /// Get all validators
    pub fn get_all_validators(&self) -> FindDAGResult<Vec<Validator>> {
        let mut validators = Vec::new();
        
        for result in self.validators_tree.iter() {
            let (_, value) = result?;
            if let Ok(validator) = bincode::deserialize::<Validator>(&value) {
                validators.push(validator);
            }
        }
        
        Ok(validators)
    }
} 
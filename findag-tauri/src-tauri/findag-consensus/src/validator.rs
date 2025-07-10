//! Validator management
//! 
//! This module handles validator set management and validation.

use findag_core::{Address, Hash, HashTimer, FinDAGTime};
use findag_types::{Validator, ValidatorStatus, FindDAGResult};

/// Validator set
pub struct ValidatorSet {
    /// Active validators
    validators: Vec<Validator>,
}

impl ValidatorSet {
    /// Create a new validator set
    pub fn new() -> Self {
        Self {
            validators: vec![],
        }
    }

    /// Add a validator
    pub fn add_validator(&mut self, validator: Validator) -> FindDAGResult<()> {
        // Check if validator already exists
        if self.validators.iter().any(|v| v.address == validator.address) {
            return Err(findag_types::FindDAGError::ValidationError(
                "Validator already exists".to_string()
            ));
        }
        
        self.validators.push(validator);
        Ok(())
    }

    /// Remove a validator
    pub fn remove_validator(&mut self, address: &Address) -> FindDAGResult<()> {
        let initial_len = self.validators.len();
        self.validators.retain(|v| v.address != *address);
        
        if self.validators.len() == initial_len {
            return Err(findag_types::FindDAGError::ValidationError(
                "Validator not found".to_string()
            ));
        }
        
        Ok(())
    }

    /// Get active validators
    pub fn get_active_validators(&self) -> Vec<&Validator> {
        self.validators
            .iter()
            .filter(|v| v.status == ValidatorStatus::Active)
            .collect()
    }

    /// Get validator by address
    pub fn get_validator(&self, address: &Address) -> Option<&Validator> {
        self.validators.iter().find(|v| v.address == *address)
    }
} 
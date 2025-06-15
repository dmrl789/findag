use crate::types::transaction::{Transaction, TxType};
use crate::validation::ValidationError;
use crate::blockchain::state::State;
use crate::security::authentication::verify_transaction;
use regex::Regex;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

const AUTHORIZED_NODES: &[&str] = &[
    "1abc...", "1def...", "1bank001...", // sample authorized addresses
];

const MAX_TRANSACTION_SIZE: usize = 1024 * 1024; // 1MB
const MAX_TRANSACTION_AGE: u64 = 3600; // 1 hour in seconds
const MIN_TRANSACTION_AMOUNT: u64 = 1;

pub struct TransactionValidator {
    state: State,
    seen_transactions: HashSet<Vec<u8>>,
}

impl TransactionValidator {
    pub fn new(state: State) -> Self {
        Self {
            state,
            seen_transactions: HashSet::new(),
        }
    }

    pub fn validate_transaction(&mut self, tx: &Transaction) -> Result<(), ValidationError> {
        // Check for duplicate transactions
        if !self.seen_transactions.insert(tx.hash().to_vec()) {
            return Err(ValidationError::Custom("Duplicate transaction".into()));
        }

        // Check transaction size
        if tx.data.len() > MAX_TRANSACTION_SIZE {
            return Err(ValidationError::Custom("Transaction too large".into()));
        }

        // Check transaction age
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if current_time.saturating_sub(tx.timestamp) > MAX_TRANSACTION_AGE {
            return Err(ValidationError::Custom("Transaction too old".into()));
        }

        // Verify signature
        if !verify_transaction(tx).map_err(|e| ValidationError::Custom(e))? {
            return Err(ValidationError::InvalidSignature);
        }

        // Check minimum amount for transfers
        if let TxType::Transfer = tx.tx_type {
            if tx.amount < MIN_TRANSACTION_AMOUNT {
                return Err(ValidationError::Custom("Amount below minimum".into()));
            }
            
            let balance = self.state.get_balance(&tx.from);
            if balance < tx.amount {
                return Err(ValidationError::Custom("Insufficient balance".into()));
            }

            // Check for self-transfers
            if tx.from == tx.to {
                return Err(ValidationError::Custom("Self-transfer not allowed".into()));
            }
        }

        // Rule 1: initiator must be authorized
        if !AUTHORIZED_NODES.contains(&tx.initiator.as_str()) {
            return Err(ValidationError::UnauthorizedNode);
        }

        match &tx.tx_type {
            TxType::LoadAsset(asset) => {
                if asset.id.is_empty() {
                    return Err(ValidationError::MissingAssetId);
                }
                if asset.issuer != tx.initiator {
                    return Err(ValidationError::InvalidInitiator);
                }
                // Check if asset already exists
                if self.state.get_asset(&asset.id).is_some() {
                    return Err(ValidationError::Custom("Asset already exists".into()));
                }
            }
            TxType::UnloadAsset(asset_id) => {
                if asset_id.is_empty() {
                    return Err(ValidationError::MissingAssetId);
                }
                // Check if asset exists and belongs to initiator
                if let Some(asset) = self.state.get_asset(asset_id) {
                    if asset.owner != tx.initiator {
                        return Err(ValidationError::InvalidInitiator);
                    }
                } else {
                    return Err(ValidationError::Custom("Asset not found".into()));
                }
            }
            TxType::TransferAsset { id, from, to } => {
                if id.is_empty() || from.is_empty() || to.is_empty() {
                    return Err(ValidationError::MissingAssetId);
                }
                if from != &tx.initiator {
                    return Err(ValidationError::InvalidInitiator);
                }
                // Check if asset exists and belongs to sender
                if let Some(asset) = self.state.get_asset(id) {
                    if asset.owner != *from {
                        return Err(ValidationError::Custom("Asset not owned by sender".into()));
                    }
                } else {
                    return Err(ValidationError::Custom("Asset not found".into()));
                }
            }
            TxType::UpdateHandle { owner, new_handle } => {
                if owner != &tx.initiator {
                    return Err(ValidationError::InvalidInitiator);
                }

                let re = Regex::new(r"^@[\w\-]{3,}\.fd$").unwrap();
                if !re.is_match(new_handle) {
                    return Err(ValidationError::InvalidHandleFormat);
                }

                // Check if handle is already taken
                if self.state.get_handle_owner(new_handle).is_some() {
                    return Err(ValidationError::Custom("Handle already taken".into()));
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn clear_seen_transactions(&mut self) {
        self.seen_transactions.clear();
    }
}

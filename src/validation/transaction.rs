use crate::types::transaction::{Transaction, TxType};
use crate::validation::ValidationError;
use regex::Regex;

const AUTHORIZED_NODES: &[&str] = &[
    "1abc...", "1def...", "1bank001...", // sample authorized addresses
];

pub fn validate_transaction(tx: &Transaction) -> Result<(), ValidationError> {
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
        }
        TxType::UnloadAsset(asset_id) => {
            if asset_id.is_empty() {
                return Err(ValidationError::MissingAssetId);
            }
        }
        TxType::TransferAsset { id, from, to } => {
            if id.is_empty() || from.is_empty() || to.is_empty() {
                return Err(ValidationError::MissingAssetId);
            }
            if from != &tx.initiator {
                return Err(ValidationError::InvalidInitiator);
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
        }
    }

    Ok(())
}

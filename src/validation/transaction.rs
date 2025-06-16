use crate::types::Transaction;
use crate::blockchain::state::State;
use std::error::Error;
use regex::Regex;
use std::time::{SystemTime, UNIX_EPOCH};
use ed25519_dalek::VerifyingKey;

const MAX_TRANSACTION_AGE: u64 = 3600; // 1 hour in seconds
const MIN_TRANSACTION_AMOUNT: u64 = 1;
const AUTHORIZED_NODES: &[&str] = &["validator1", "validator2", "validator3"];

pub struct TransactionValidator {
    state: State,
}

impl TransactionValidator {
    pub fn new(state: State) -> Self {
        Self { state }
    }

    pub async fn validate(&self, tx: &Transaction) -> Result<bool, Box<dyn Error>> {
        // Check transaction age
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();
        let tx_time = tx.timestamp
            .duration_since(UNIX_EPOCH)?
            .as_secs();
        
        if current_time.saturating_sub(tx_time) > MAX_TRANSACTION_AGE {
            return Ok(false);
        }

        // Verify signature
        if !self.verify_signature(tx).await? {
            return Ok(false);
        }

        // Parse transaction data
        let data = serde_json::from_slice::<TransactionData>(&tx.data)
            .map_err(|e| format!("Failed to parse transaction data: {}", e))?;

        match data.tx_type {
            TransactionType::Transfer => {
                if data.amount < MIN_TRANSACTION_AMOUNT {
                    return Ok(false);
                }

                let from_balance = self.state.get_balance(&data.from).await;
                if from_balance < data.amount {
                    return Ok(false);
                }

                if data.from == data.to {
                    return Ok(false);
                }
            }
            TransactionType::RegisterHandle => {
                let re = Regex::new(r"^[a-zA-Z0-9_]{3,32}$")?;
                if !re.is_match(&data.handle) {
                    return Ok(false);
                }

                if self.state.get_handle_owner(&data.handle).await.is_some() {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    async fn verify_signature(&self, tx: &Transaction) -> Result<bool, Box<dyn Error>> {
        // Verify the transaction signature using the public key
        let signature = tx.signature()?;
        let public_key: [u8; 32] = tx.public_key.clone().try_into().map_err(|_| "Invalid public key length")?;
        let public_key = VerifyingKey::from_bytes(&public_key)?;
        Ok(public_key.verify_strict(&tx.data, &signature).is_ok())
    }
}

#[derive(serde::Deserialize)]
struct TransactionData {
    tx_type: TransactionType,
    from: Vec<u8>,
    to: Vec<u8>,
    amount: u64,
    handle: String,
}

#[derive(serde::Deserialize)]
enum TransactionType {
    Transfer,
    RegisterHandle,
}

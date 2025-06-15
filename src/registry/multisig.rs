use crate::storage::Storage;
use crate::types::multisig::{MultisigAccount, PendingMultisigTx};
use std::collections::HashSet;
use bincode;

pub fn create_multisig(storage: &Storage, handle: &str, owners: &[String], threshold: u8) {
    let account = MultisigAccount {
        handle: handle.to_string(),
        owners: owners.iter().cloned().collect(),
        threshold,
    };
    let key = format!("multisig:{}", handle);
    if let Ok(data) = bincode::serialize(&account) {
        let _ = storage.set(key.as_bytes(), &data);
    }
}

pub fn propose_tx(storage: &Storage, tx_id: &str, initiator: &str, handle: &str, action: &str) {
    let key = format!("multisig:{}", handle);
    if let Ok(Some(data)) = storage.get(key.as_bytes()) {
        if let Ok(account) = bincode::deserialize::<MultisigAccount>(&data) {
            if !account.owners.contains(initiator) {
                println!("Unauthorized initiator.");
                return;
            }
            let tx = PendingMultisigTx {
                tx_id: tx_id.to_string(),
                initiator: initiator.to_string(),
                target: handle.to_string(),
                action: action.to_string(),
                approvals: HashSet::from([initiator.to_string()]),
                required: account.threshold,
            };
            let tx_key = format!("pendingtx:{}", tx_id);
            if let Ok(data) = bincode::serialize(&tx) {
                let _ = storage.set(tx_key.as_bytes(), &data);
            }
        }
    }
}

pub fn approve_tx(storage: &Storage, tx_id: &str, approver: &str) {
    let tx_key = format!("pendingtx:{}", tx_id);
    if let Ok(Some(data)) = storage.get(tx_key.as_bytes()) {
        if let Ok(mut tx) = bincode::deserialize::<PendingMultisigTx>(&data) {
            tx.approvals.insert(approver.to_string());
            if tx.approvals.len() as u8 >= tx.required {
                println!("✅ Transaction {} is now approved and ready to execute: {}", tx.tx_id, tx.action);
                // Add actual action execution logic here
            } else {
                println!("🔐 Approval recorded. {} of {} collected.", tx.approvals.len(), tx.required);
            }
            if let Ok(data) = bincode::serialize(&tx) {
                let _ = storage.set(tx_key.as_bytes(), &data);
            }
        }
    }
}

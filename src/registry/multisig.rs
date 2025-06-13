use crate::storage::{read_from_store, write_to_store};
use crate::types::multisig::{MultisigAccount, PendingMultisigTx};
use std::collections::HashSet;

pub fn create_multisig(handle: &str, owners: &[String], threshold: u8) {
    let account = MultisigAccount {
        handle: handle.to_string(),
        owners: owners.iter().cloned().collect(),
        threshold,
    };
    write_to_store(&format!("multisig:{}", handle), &account);
}

pub fn propose_tx(tx_id: &str, initiator: &str, handle: &str, action: &str) {
    if let Some(account) = read_from_store::<MultisigAccount>(&format!("multisig:{}", handle)) {
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
        write_to_store(&format!("pendingtx:{}", tx_id), &tx);
    }
}

pub fn approve_tx(tx_id: &str, approver: &str) {
    if let Some(mut tx) = read_from_store::<PendingMultisigTx>(&format!("pendingtx:{}", tx_id)) {
        tx.approvals.insert(approver.to_string());
        if tx.approvals.len() as u8 >= tx.required {
            println!("✅ Transaction {} is now approved and ready to execute: {}", tx.tx_id, tx.action);
            // Add actual action execution logic here
        } else {
            println!("🔐 Approval recorded. {} of {} collected.", tx.approvals.len(), tx.required);
        }
        write_to_store(&format!("pendingtx:{}", tx_id), &tx);
    }
}

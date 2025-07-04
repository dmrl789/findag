use crate::iso20022::ISO20022Transaction;
use crate::core::types::Transaction;
use sha2::{Sha256, Digest};

use crate::core::address::Address;
use ed25519_dalek;

/// Map ISO20022Transaction into your internal Transaction format
pub fn iso20022_to_findag_tx(iso_tx: &ISO20022Transaction) -> Transaction {
    // Create addresses from the debtor and creditor strings
    let from = Address::new(iso_tx.debtor.clone());
    let to = Address::new(iso_tx.creditor.clone());
    let amount = iso_tx.amount;
    
    // Create payload with currency information
    let payload = format!("currency:{}", iso_tx.currency).into_bytes();

    // Compute simple HashTimer as SHA-256 of message_id
    let mut hasher = Sha256::new();
    hasher.update(iso_tx.message_id.as_bytes());
    let hashtimer = hasher.finalize();
    let mut hashtimer_array = [0u8; 32];
    hashtimer_array.copy_from_slice(&hashtimer);

    // Create a dummy signature and public key for now
    // In a real implementation, these would be properly signed
    let dummy_signature = ed25519_dalek::Signature::from_bytes(&[0u8; 64]).unwrap();
    let dummy_public_key = ed25519_dalek::PublicKey::from_bytes(&[0u8; 32]).unwrap();

    Transaction {
        from,
        to,
        amount,
        payload,
        findag_time: 0, // Will be set by the system
        hashtimer: hashtimer_array,
        signature: dummy_signature,
        public_key: dummy_public_key,
        shard_id: crate::core::types::ShardId(0),
        source_shard: None,
        dest_shard: None,
        target_chain: None,
        bridge_protocol: None,
    }
} 
use proptest::prelude::*;
use findag::core::types::Transaction;
use findag::core::tx_pool::TxPool;
use findag::storage::state::StateDB;
use std::sync::Arc;

proptest! {
    #[test]
    fn only_valid_transactions_are_accepted(
        from in "fdg1q[a-z0-9]{10}",
        to in "fdg1q[a-z0-9]{10}",
        amount in 1u64..1_000_000,
        asset in prop_oneof![Just("USD"), Just("EUR"), Just("BTC"), Just("FAKEASSET")]
    ) {
        let state_db = Arc::new(StateDB::default());
        let mut pool = TxPool::new(100, state_db.clone());

        // Give the sender a balance for valid assets
        if asset != "FAKEASSET" {
            state_db.set_balance(0, &from, asset, 1_000_000);
        }

        let tx = Transaction {
            from: from.clone().into(),
            to: to.clone().into(),
            amount,
            currency: asset.to_string(),
            shard_id: ShardId(0),
            // ... fill in other fields as needed ...
            ..Default::default()
        };

        let accepted = pool.add_transaction(tx.clone());
        if asset == "FAKEASSET" {
            prop_assert!(!accepted, "Should reject non-whitelisted asset");
        } else if amount > 1_000_000 {
            prop_assert!(!accepted, "Should reject if amount exceeds balance");
        } else {
            prop_assert!(accepted, "Should accept valid transaction");
        }
    }
} 
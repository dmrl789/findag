use proptest::prelude::*;
use findag::core::types::{Transaction, Block};
use findag::core::block_producer::BlockProducer;
use findag::core::dag_engine::DagEngine;
use findag::core::tx_pool::ShardedTxPool;
use findag::core::address::{generate_address, Address};
use findag::storage::state::StateDB;
use ed25519_dalek::Keypair;
use std::sync::Arc;

proptest! {
    #[test]
    fn block_producer_only_includes_valid_transactions(
        amount in 1u64..1_000_000,
        asset in prop_oneof![Just("USD"), Just("EUR"), Just("BTC")],
    ) {
        let state_db = Arc::new(StateDB::default());
        let mut dag = DagEngine::new();
        let tx_pool = ShardedTxPool::new(100);
        let (keypair, address) = generate_address();
        let bal = state_db.get_balance(0, address.as_str(), asset);
        state_db.set_balance(address.as_str(), asset, 1_000_000);

        // Add a valid transaction to the pool
        let tx = Transaction {
            from: address.clone(),
            to: Address("fdg1qtestto".to_string()),
            amount,
            currency: asset.to_string(),
            shard_id: ShardId(0),
            // ... fill in other fields as needed ...
            ..Default::default()
        };
        tx_pool.add_transaction(tx.clone());

        let mut producer = BlockProducer {
            dag: &mut dag,
            tx_pool: &tx_pool,
            proposer: address.clone(),
            keypair: &keypair,
            max_block_txs: 10,
            // ... fill in other fields as needed ...
        };

        let parent_blocks = vec![];
        let findag_time = 0;
        let hashtimer = [0u8; 32];
        let block = producer.produce_block(parent_blocks, findag_time, hashtimer);

        if let Some(block) = block {
            // All transactions in the block must be valid and unique
            let mut seen = std::collections::HashSet::new();
            for tx in &block.transactions {
                assert!(tx.amount > 0);
                assert!(seen.insert(tx.hashtimer));
            }
        }
    }
} 
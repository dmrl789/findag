mod core {
    pub mod address;
    pub mod types;
    pub mod tx_pool;
    pub mod dag_engine;
    pub mod block_producer;
    pub mod block_production_loop;
    pub mod round_checkpoint_loop;
}
mod dagtimer {
    pub mod findag_time_manager;
    pub mod hashtimer;
}
mod consensus {
    pub mod round_finalizer;
    pub mod validator_set;
    pub mod governance;
}
mod network {
    pub mod propagation;
}
mod storage {
    pub mod persistent;
}
mod metrics;
mod api;

use core::address::{generate_address, Address};
use core::types::{Transaction, ShardId};
use core::tx_pool::ShardedTxPool;
use core::dag_engine::DagEngine;
use core::block_production_loop::run_block_production_loop;
use core::round_checkpoint_loop::run_round_checkpoint_loop;
use dagtimer::findag_time_manager::FinDAGTimeManager;
use consensus::round_finalizer::{Validator, RoundFinalizer};
use network::propagation::{NetworkPropagator, GossipMsg};
use ed25519_dalek::Keypair;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::task;
use tokio::time::{sleep, Duration};
use rand::seq::IteratorRandom;
use rand::Rng;
use std::sync::atomic::{AtomicUsize, Ordering};
use storage::persistent::{PersistentStorage, PersistMsg};
use tokio::sync::mpsc;
use prometheus_exporter::prometheus_exporter::start;
use std::thread;
use consensus::validator_set::{ValidatorSet, ValidatorInfo, ValidatorStatus};
use api::http_server::{run_http_server, VALIDATOR_SET, STORAGE, GOVERNANCE_STATE};
use consensus::governance::GovernanceState;
use std::env;

// Number of bots and transactions per bot per second
const NUM_BOTS: usize = 5;
const TXS_PER_BOT_PER_SEC: usize = 100;

/// Node configuration
struct Config {
    shard_count: usize,
}

impl Config {
    fn load() -> Self {
        let shard_count = env::var("FINDAG_SHARD_COUNT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);
        Config { shard_count }
    }
}

#[tokio::main]
async fn main() {
    // --- Load config ---
    let config = Config::load();
    println!("[Config] shard_count = {}", config.shard_count);
    // --- Metrics setup ---
    metrics::register_metrics();
    thread::spawn(|| {
        let exporter = start("127.0.0.1:9898").unwrap();
        loop {
            let metric_families = metrics::REGISTRY.gather();
            let mut buffer = Vec::new();
            let encoder = prometheus::TextEncoder::new();
            encoder.encode(&metric_families, &mut buffer).unwrap();
            exporter.write_all(&buffer).unwrap();
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    });
    // --- Static config for demo ---
    let peers = vec!["127.0.0.1:9001".parse().unwrap()]; // Add more for multi-node
    let validator_keypairs: Vec<Keypair> = vec![generate_address().0];
    let validators: Vec<Validator> = validator_keypairs.iter().enumerate().map(|(i, k)| Validator {
        id: format!("v{}", i),
        public_key: k.public,
    }).collect();
    let local_keypair = validator_keypairs[0].clone();
    let local_address = Address::from_public_key(&local_keypair.public);
    let round_finalizer = Arc::new(RoundFinalizer::new(validators.clone(), local_keypair.clone()));

    // Simulate two addresses for payment test
    let (keypair1, address1) = generate_address();
    let (keypair2, address2) = generate_address();

    // --- Load or initialize asset whitelist ---
    let asset_whitelist = storage.load_asset_whitelist().unwrap_or_else(|| vec![
        "USD".to_string(), "EUR".to_string(), "BTC".to_string(), "ETH".to_string()
    ]);
    let asset_whitelist_arc = Arc::new(Mutex::new(asset_whitelist));

    // --- Core state ---
    let mut dag = DagEngine::new();
    let tx_pool = Arc::new(ShardedTxPool::new_with_whitelist_per_shard(100_000, asset_whitelist_arc.clone(), config.shard_count));
    let time_manager = FinDAGTimeManager::new();

    // --- Network propagator ---
    let propagator = Arc::new(NetworkPropagator::new("0.0.0.0:9000", peers).await.unwrap());

    // --- Spawn network listener ---
    let tx_pool_clone = tx_pool.clone();
    let propagator_clone = propagator.clone();
    let dag_arc = Arc::new(Mutex::new(&mut dag));
    let round_finalizer_clone = round_finalizer.clone();
    task::spawn(async move {
        propagator_clone.listen(move |msg| {
            match msg {
                GossipMsg::NewTransaction(tx) => {
                    tx_pool_clone.add_transaction(tx);
                },
                GossipMsg::NewBlock(block) => {
                    let mut dag = dag_arc.lock().unwrap();
                    dag.add_block(block);
                },
                GossipMsg::NewRound(round) => {
                    let mut dag = dag_arc.lock().unwrap();
                    dag.add_round(round);
                },
            }
        }).await;
    });

    // --- Per-shard block production ---
    let my_address = local_address.clone();
    let my_shards = validator_set.shards_for_validator(my_address.as_str()).cloned().unwrap_or_default();
    for shard_id in my_shards {
        let tx_pool_clone = tx_pool.clone();
        let local_address_clone = my_address.clone();
        let local_keypair_clone = local_keypair.clone();
        let time_manager_clone = time_manager.clone();
        let dag_arc2 = dag_arc.clone();
        // TODO: Pass per-shard state, mempool, and block producer
        tokio::spawn(async move {
            println!("[Shard {}] Starting block production loop", shard_id);
            // Replace with per-shard block production logic
            run_block_production_loop(
                &mut *dag_arc2.lock().unwrap(),
                &tx_pool_clone,
                local_address_clone,
                &local_keypair_clone,
                100,
                20,
                &time_manager_clone,
                // TODO: Pass shard_id to all relevant functions
            ).await;
        });
    }

    // --- Spawn round checkpoint loop ---
    let local_keypair_clone = local_keypair.clone();
    let local_address_clone = local_address.clone();
    let time_manager_clone = time_manager.clone();
    let dag_arc3 = dag_arc.clone();
    task::spawn(async move {
        run_round_checkpoint_loop(
            &mut *dag_arc3.lock().unwrap(),
            local_address_clone,
            &local_keypair_clone,
            200,
            &time_manager_clone,
        ).await;
    });

    // --- Simulate a payment transaction of 1000 USD from address1 to address2 ---
    let tx_pool_clone = tx_pool.clone();
    let propagator_clone = propagator.clone();
    let time_manager_clone = time_manager.clone();
    task::spawn(async move {
        // Use real FinDAG Time
        let findag_time = time_manager_clone.get_findag_time();
        // Compute hashtimer for the transaction
        let mut tx_content = Vec::new();
        tx_content.extend_from_slice(address1.as_str().as_bytes());
        tx_content.extend_from_slice(address2.as_str().as_bytes());
        let nonce = 0u32;
        let hashtimer = dagtimer::hashtimer::compute_hashtimer(findag_time, &tx_content, nonce);
        // Create a payment transaction
        let tx = Transaction {
            from: address1.clone(),
            to: address2.clone(),
            amount: 1000,
            currency: "USD".to_string(),
            payload: vec![],
            findag_time,
            hashtimer,
            signature: keypair1.sign(b"payment"),
            public_key: keypair1.public,
            shard_id: ShardId(0),
        };
        println!("Test payment transaction: {:?}", tx);
        tx_pool_clone.add_transaction(tx.clone());
        let tx_injected = AtomicUsize::new(0);
        tx_injected.fetch_add(1, Ordering::Relaxed);
        propagator_clone.broadcast(&GossipMsg::NewTransaction(tx)).await;
    });

    // --- After your node setup, before the main loop ...
    let mut all_addresses = vec![address1.clone(), address2.clone()];
    let mut bot_keypairs = Vec::new();
    let mut bot_addresses = Vec::new();
    for _ in 0..NUM_BOTS {
        let (bot_keypair, bot_address) = generate_address();
        all_addresses.push(bot_address.clone());
        bot_keypairs.push(bot_keypair);
        bot_addresses.push(bot_address);
    }

    for (i, bot_keypair) in bot_keypairs.into_iter().enumerate() {
        let tx_pool_clone = tx_pool.clone();
        let propagator_clone = propagator.clone();
        let time_manager_clone = time_manager.clone();
        let bot_address = all_addresses[2 + i].clone();
        let all_addresses_clone = all_addresses.clone();
        tokio::spawn(async move {
            let mut rng = rand::thread_rng();
            loop {
                // Pick a random recipient (not self)
                let recipient = all_addresses_clone
                    .iter()
                    .filter(|a| *a != &bot_address)
                    .choose(&mut rng)
                    .unwrap()
                    .clone();

                let findag_time = time_manager_clone.get_findag_time();
                let mut tx_content = Vec::new();
                tx_content.extend_from_slice(bot_address.as_str().as_bytes());
                tx_content.extend_from_slice(recipient.as_str().as_bytes());
                let nonce = rng.gen::<u32>();
                let hashtimer = dagtimer::hashtimer::compute_hashtimer(findag_time, &tx_content, nonce);

                let tx = Transaction {
                    from: bot_address.clone(),
                    to: recipient,
                    amount: rng.gen_range(1..100),
                    currency: "USD".to_string(),
                    payload: vec![],
                    findag_time,
                    hashtimer,
                    signature: bot_keypair.sign(b"bot-tx"),
                    public_key: bot_keypair.public,
                    shard_id: ShardId(0),
                };

                tx_pool_clone.add_transaction(tx.clone());
                let tx_injected = AtomicUsize::new(0);
                tx_injected.fetch_add(1, Ordering::Relaxed);
                propagator_clone.broadcast(&GossipMsg::NewTransaction(tx)).await;

                // Control rate
                tokio::time::sleep(Duration::from_millis(1000 / TXS_PER_BOT_PER_SEC as u64)).await;
            }
        });
    }

    // --- Initialize persistent storage ---
    let storage = Arc::new(PersistentStorage::new("findag_db"));
    let (persist_tx, persist_rx) = mpsc::unbounded_channel();
    storage.clone().spawn_background_writer(persist_rx);

    // --- Load or initialize validator set ---
    let mut validator_set = storage.load_validator_set().unwrap_or_else(|| {
        // If not present, create a default set with 2 validators for local testing
        let (keypair1, address1) = generate_address();
        let (keypair2, address2) = generate_address();
        let info1 = ValidatorInfo {
            address: address1.clone(),
            public_key: keypair1.public,
            status: ValidatorStatus::Active,
            metadata: Some("Test validator 1".to_string()),
        };
        let info2 = ValidatorInfo {
            address: address2.clone(),
            public_key: keypair2.public,
            status: ValidatorStatus::Active,
            metadata: Some("Test validator 2".to_string()),
        };
        let mut set = ValidatorSet::new();
        set.add_validator(info1);
        set.add_validator(info2);
        set
    });
    // Assign validators to shards
    validator_set.assign_validators_to_shards(config.shard_count as u16);
    println!("[Shard Assignment] {:?}", validator_set.shard_assignments);
    storage.save_validator_set(&validator_set);

    // --- On startup, reload all blocks and rounds from disk into the DAG engine ---
    for result in storage.db.scan_prefix(b"block:") {
        if let Ok((_, value)) = result {
            if let Ok(block) = bincode::deserialize::<Block>(&value) {
                dag.add_block(block);
            }
        }
    }
    for result in storage.db.scan_prefix(b"round:") {
        if let Ok((_, value)) = result {
            if let Ok(round) = bincode::deserialize::<Round>(&value) {
                dag.add_round(round);
            }
        }
    }

    // --- Use validator_set for consensus, etc. ---
    // Find the local validator's keypair (for demo, use the first one)
    let local_validator = validator_set.active_validators().get(0).expect("No active validator");
    // In a real deployment, match the local node's address/keypair
    let local_keypair = generate_address().0; // Replace with actual keypair management
    let round_finalizer = RoundFinalizer::new(&validator_set, local_keypair);

    // --- Validator management functions ---
    fn add_validator(
        validator_set: &mut ValidatorSet,
        storage: &PersistentStorage,
        address: Address,
        public_key: ed25519_dalek::PublicKey,
        metadata: Option<String>,
    ) {
        let info = ValidatorInfo {
            address: address.clone(),
            public_key,
            status: ValidatorStatus::Active,
            metadata,
        };
        validator_set.add_validator(info);
        // Reassign and persist
        validator_set.assign_validators_to_shards(Config::load().shard_count as u16);
        println!("[Shard Assignment] {:?}", validator_set.shard_assignments);
        storage.save_validator_set(validator_set);
    }

    fn remove_validator(
        validator_set: &mut ValidatorSet,
        storage: &PersistentStorage,
        address: &str,
    ) {
        validator_set.remove_validator(address);
        // Reassign and persist
        validator_set.assign_validators_to_shards(Config::load().shard_count as u16);
        println!("[Shard Assignment] {:?}", validator_set.shard_assignments);
        storage.save_validator_set(validator_set);
    }

    fn slash_validator(
        validator_set: &mut ValidatorSet,
        storage: &PersistentStorage,
        address: &str,
    ) {
        validator_set.set_status(address, ValidatorStatus::Slashed);
        // Reassign and persist
        validator_set.assign_validators_to_shards(Config::load().shard_count as u16);
        println!("[Shard Assignment] {:?}", validator_set.shard_assignments);
        storage.save_validator_set(validator_set);
    }
    // Example usage (call from main or API/CLI):
    // add_validator(&mut validator_set, &storage, new_address, new_pubkey, Some("New validator".to_string()));
    // remove_validator(&mut validator_set, &storage, address_str);
    // slash_validator(&mut validator_set, &storage, address_str);
    // After mutation, update consensus logic if needed.

    // --- Wrap validator set and storage for HTTP API ---
    let validator_set_arc = Arc::new(Mutex::new(validator_set));
    let storage_arc = storage.clone();
    unsafe {
        VALIDATOR_SET = Some(validator_set_arc.clone());
        STORAGE = Some(storage_arc.clone());
    }
    // --- Load or initialize governance state ---
    let governance_state = storage.load_governance_state().unwrap_or_else(|| GovernanceState::new());
    let governance_state_arc = Arc::new(Mutex::new(governance_state));
    unsafe {
        GOVERNANCE_STATE = Some(governance_state_arc.clone());
    }
    // --- Start HTTP server ---
    tokio::spawn(async move {
        run_http_server().await;
    });

    // --- Keep main alive ---
    loop {
        sleep(Duration::from_secs(60)).await;
    }
} 
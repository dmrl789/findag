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
}
mod network {
    pub mod propagation;
}

use core::address::{generate_address, Address};
use core::types::{Transaction};
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

// Number of bots and transactions per bot per second
const NUM_BOTS: usize = 5;
const TXS_PER_BOT_PER_SEC: usize = 100;

#[tokio::main]
async fn main() {
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

    // --- Core state ---
    let mut dag = DagEngine::new();
    let tx_pool = Arc::new(ShardedTxPool::new(100_000));
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

    // --- Spawn block production loop ---
    let tx_pool_clone = tx_pool.clone();
    let local_address_clone = local_address.clone();
    let local_keypair_clone = local_keypair.clone();
    let time_manager_clone = time_manager.clone();
    let dag_arc2 = dag_arc.clone();
    task::spawn(async move {
        run_block_production_loop(
            &mut *dag_arc2.lock().unwrap(),
            &tx_pool_clone,
            local_address_clone,
            &local_keypair_clone,
            100,
            20,
            &time_manager_clone,
        ).await;
    });

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

    // --- Keep main alive ---
    loop {
        sleep(Duration::from_secs(60)).await;
    }
} 
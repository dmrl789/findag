use std::sync::Arc;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    extract::State,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::Mutex;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use std::net::SocketAddr;

mod core;
mod dagtimer;
mod storage;
mod network;

use crate::core::dag_engine::DagEngine;
use crate::core::tx_pool::ShardedTxPool;
use crate::core::address::{Address, generate_address};
use crate::core::types::{Transaction, Block, ShardId, SerializableTransaction, SerializableBlock};
use crate::dagtimer::findag_time_manager::FinDAGTimeManager;
use crate::core::block_production_loop::run_block_production_loop;
use crate::network::propagation::{NetworkPropagator, GossipMsg};

#[derive(Clone)]
struct AppState {
    dag: Arc<Mutex<DagEngine>>,
    tx_pool: Arc<ShardedTxPool>,
    time_manager: Arc<FinDAGTimeManager>,
    keypair: Arc<Keypair>,
    address: Address,
    propagator: Arc<NetworkPropagator>,
}

#[tokio::main]
async fn main() {
    println!("FinDAG node is starting...");

    // --- Node identity ---
    let (keypair, address) = generate_address();
    let keypair = Arc::new(keypair);
    println!("Node address: {}", address.0);

    // --- DAG engine ---
    let dag = Arc::new(Mutex::new(DagEngine::new()));

    // --- Sharded mempool ---
    let asset_whitelist = Arc::new(std::sync::Mutex::new(vec!["USD".to_string()]));
    let tx_pool = Arc::new(ShardedTxPool::new_with_whitelist_per_shard(100_000, asset_whitelist, 1));

    // --- Time manager ---
    let time_manager = Arc::new(FinDAGTimeManager::new());

    // --- Network propagator ---
    let udp_port = std::env::var("FINDAG_UDP_PORT").unwrap_or_else(|_| "9000".to_string());
    let udp_bind = format!("0.0.0.0:{}", udp_port);
    let peer_str = std::env::var("FINDAG_PEERS").unwrap_or_else(|_| "127.0.0.1:9001".to_string());
    let peers: Vec<SocketAddr> = peer_str.split(',').filter_map(|s| s.trim().parse().ok()).collect();
    let propagator = Arc::new(NetworkPropagator::new(&udp_bind, peers).await.unwrap());

    // --- Spawn gossip listener ---
    let dag_clone = dag.clone();
    let tx_pool_clone = tx_pool.clone();
    let propagator_clone = propagator.clone();
    tokio::spawn(async move {
        propagator_clone.listen(move |msg| {
            match msg {
                GossipMsg::NewTransaction(stx) => {
                    // Convert to Transaction and add to pool
                    if let Ok(tx) = Transaction::try_from(stx) {
                        tx_pool_clone.add_transaction(tx);
                    }
                }
                GossipMsg::NewBlock(sblock) => {
                    // Convert to Block and add to DAG
                    if let Ok(block) = Block::try_from(sblock) {
                        let mut dag = dag_clone.blocking_lock();
                        let _ = dag.add_block(block);
                    }
                }
                _ => {}
            }
        }).await;
    });

    // --- Block production loop ---
    let dag_clone = dag.clone();
    let tx_pool_clone = tx_pool.clone();
    let time_manager_clone = time_manager.clone();
    let keypair_clone = keypair.clone();
    let address_clone = address.clone();
    let propagator_clone = propagator.clone();
    tokio::spawn(async move {
        let (persist_tx, _persist_rx) = unbounded_channel();
        let mut dag = dag_clone.lock().await;
        let round_finalizer = crate::core::consensus::round_finalizer::RoundFinalizer::dummy();
        loop {
            run_block_production_loop(
                &mut dag,
                &tx_pool_clone,
                address_clone.clone(),
                &keypair_clone,
                100,
                1000, // 1 block/sec for demo
                &time_manager_clone,
                persist_tx.clone(),
                0, // single-shard
                round_finalizer.clone(),
            ).await;
            // After producing a block, broadcast it
            if let Some(block) = dag.get_all_blocks().last() {
                let sblock: SerializableBlock = block.clone().into();
                let msg = GossipMsg::NewBlock(sblock);
                propagator_clone.broadcast(&msg).await;
            }
        }
    });

    // --- App state ---
    let state = AppState {
        dag: dag.clone(),
        tx_pool: tx_pool.clone(),
        time_manager: time_manager.clone(),
        keypair: keypair.clone(),
        address: address.clone(),
        propagator: propagator.clone(),
    };

    // --- API endpoints ---
    let app = Router::new()
        .route("/health", get(|| async { StatusCode::OK }))
        .route("/node/info", get({
            let state = state.clone();
            move || {
                let state = state.clone();
                async move {
                    let dag = state.dag.lock().await;
                    let block_count = dag.block_count() as u64;
                    let peers = vec![]; // TODO: add real peer info
                    Json(serde_json::json!({
                        "address": state.address.0,
                        "peers": peers,
                        "block_count": block_count,
                    }))
                }
            }
        }))
        .route("/transactions", post({
            let state = state.clone();
            move |Json(tx): Json<Transaction>| {
                let state = state.clone();
                async move {
                    // Add transaction to sharded pool and broadcast
                    let added = state.tx_pool.add_transaction(tx.clone());
                    if added {
                        let stx: SerializableTransaction = tx.into();
                        let msg = GossipMsg::NewTransaction(stx);
                        state.propagator.broadcast(&msg).await;
                        StatusCode::OK
                    } else {
                        StatusCode::BAD_REQUEST
                    }
                }
            }
        }))
        .route("/blocks", get({
            let state = state.clone();
            move || {
                let state = state.clone();
                async move {
                    let dag = state.dag.lock().await;
                    let blocks = dag.get_all_blocks();
                    Json(blocks)
                }
            }
        }))
        .route("/dag", get({
            let state = state.clone();
            move || {
                let state = state.clone();
                async move {
                    let dag = state.dag.lock().await;
                    let stats = dag.get_stats();
                    Json(stats)
                }
            }
        }));

    // --- Start HTTP server ---
    let bind_addr = std::env::var("FINDAG_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let listener = TcpListener::bind(&bind_addr).await.unwrap();
    println!("HTTP server listening on http://{}", bind_addr);
    axum::serve(listener, app).await.unwrap();
} 
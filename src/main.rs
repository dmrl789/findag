use std::sync::Arc;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    extract::State,
    debug_handler,
};
use tokio::net::TcpListener;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::Mutex;
use ed25519_dalek::Keypair;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

use findag::core::dag_engine::DagEngine;
use findag::core::tx_pool::ShardedTxPool;
use findag::core::address::{Address, generate_address};
use findag::core::types::{Transaction, Block, SerializableTransaction, SerializableBlock};
use findag::dagtimer::findag_time_manager::FinDAGTimeManager;
use findag::core::block_production_loop::{run_block_production_loop, BlockProductionConfig};
use findag::network::propagation::{NetworkPropagator, GossipMsg};
use findag::consensus::round_finalizer::RoundFinalizer;
use findag::consensus::validator_set::ValidatorSet;
use serde_json::json;
use findag::tools::run_quorum_demo;
use findag::tools::run_handle_wallet;
use std::env;

#[derive(Clone)]
struct AppState {
    dag: Arc<Mutex<DagEngine>>,
    tx_pool: Arc<ShardedTxPool>,
    address: Address,
    propagator: Arc<NetworkPropagator>,
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn node_info(State(state): State<AppState>) -> Json<serde_json::Value> {
    let dag = state.dag.lock().await;
    let block_count = dag.block_count() as u64;
    let peers: Vec<String> = vec![]; // TODO: add real peer info
    Json(json!({
        "address": state.address.0,
        "peers": peers,
        "block_count": block_count,
    }))
}

#[debug_handler]
async fn submit_transaction(
    State(state): State<AppState>, 
    Json(tx): Json<Transaction>
) -> StatusCode {
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

#[debug_handler]
async fn get_blocks(State(state): State<AppState>) -> Json<Vec<Block>> {
    let dag = state.dag.lock().await;
    let blocks = dag.get_all_blocks();
    Json(blocks)
}

#[debug_handler]
async fn get_dag(State(state): State<AppState>) -> Json<serde_json::Value> {
    let dag = state.dag.lock().await;
    let stats = dag.get_stats();
    Json(json!(stats))
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Handle CLI commands
    if args.len() > 1 {
        match args[1].as_str() {
            "quorum-demo" => {
                println!("Running FinDAG Quorum Rotation Demo...\n");
                run_quorum_demo();
                return;
            }
            "handle-wallet" => {
                println!("Running FinDAG Handle Wallet Demo...\n");
                run_handle_wallet();
                return;
            }
            "help" | "--help" | "-h" => {
                println!("FinDAG - Permissioned Asset Tracking DAG Blockchain");
                println!();
                println!("Available commands:");
                println!("  quorum-demo    - Run quorum rotation system demo");
                println!("  handle-wallet  - Run handle wallet system demo");
                println!("  help           - Show this help message");
                println!("  (no args)      - Start FinDAG node");
                println!();
                println!("Examples:");
                println!("  cargo run -- quorum-demo");
                println!("  cargo run -- handle-wallet");
                println!("  cargo run -- help");
                println!("  cargo run --");
                return;
            }
            _ => {
                println!("Unknown command: {}", args[1]);
                println!("Run 'cargo run -- help' for available commands");
                return;
            }
        }
    }

    // Start FinDAG node
    println!("FinDAG node is starting...");

    // Initialize metrics
    findag::metrics::register_metrics();

    // --- Node identity ---
    let (keypair, address) = generate_address();
    let keypair: Arc<Keypair> = Arc::new(keypair);
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
        let mut validator_set = ValidatorSet::new();
        loop {
            let round_finalizer = RoundFinalizer::dummy(&mut validator_set);
            let config = BlockProductionConfig {
                max_block_txs: 100,
                interval_ms: 1000, // 1 block/sec for demo
                shard_id: 0, // single-shard
            };
            run_block_production_loop(
                &mut *dag_clone.lock().await,
                &tx_pool_clone,
                address_clone.clone(),
                &keypair_clone,
                config,
                &time_manager_clone,
                persist_tx.clone(),
                round_finalizer,
            ).await;
            // After producing a block, broadcast it
            if let Some(block) = dag_clone.lock().await.get_all_blocks().last() {
                let sblock: SerializableBlock = block.clone().into();
                let msg = GossipMsg::NewBlock(sblock);
                propagator_clone.broadcast(&msg).await;
            }
        }
    });

    // --- HTTP API ---
    let app_state = AppState {
        dag: dag.clone(),
        tx_pool: tx_pool.clone(),
        address: address.clone(),
        propagator: propagator.clone(),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/node", get(node_info))
        .route("/tx", post(submit_transaction))
        .route("/blocks", get(get_blocks))
        .route("/dag", get(get_dag))
        .layer(cors)
        .with_state(app_state);

    let port = std::env::var("FINDAG_HTTP_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("HTTP API listening on {}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
} 
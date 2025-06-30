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
use findag::dagtimer::hashtimer::compute_hashtimer;
use findag::core::block_production_loop::{run_block_production_loop, BlockProductionConfig};
use findag::network::propagation::{NetworkPropagator, GossipMsg};
use findag::consensus::round_finalizer::RoundFinalizer;
use findag::consensus::validator_set::ValidatorSet;
use findag::consensus::mempool::Mempool;
use serde_json::json;
use findag::tools::run_quorum_demo;
use findag::tools::run_handle_wallet;
use std::env;

#[derive(Clone)]
struct AppState {
    dag: Arc<Mutex<DagEngine>>,
    tx_pool: Arc<ShardedTxPool>,
    mempool: Arc<Mempool>,
    address: Address,
    propagator: Arc<NetworkPropagator>,
    time_manager: Arc<FinDAGTimeManager>,
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
    // Add to both tx_pool and mempool
    let added_to_pool = state.tx_pool.add_transaction(tx.clone());
    let _ = state.mempool.add(tx.clone()).await;
    
    if added_to_pool {
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

#[debug_handler]
async fn get_hashtimer_status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let current_time = state.time_manager.get_findag_time();
    let round_duration_ns = 16_000_000_000u64; // 16 seconds in nanoseconds
    let current_round = current_time / round_duration_ns;
    let hashtimer = compute_hashtimer(current_time, b"status_check", 0);
    Json(json!({
        "current_round": current_round,
        "current_findag_time": current_time,
        "hashtimer_hash": format!("0x{}", hashtimer.iter().map(|b| format!("{:02x}", b)).collect::<String>()),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

#[debug_handler]
async fn get_mempool_status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let mempool_len = state.mempool.len().await;
    let tx_pool_size = state.tx_pool.size(0);
    Json(json!({
        "mempool_size": mempool_len,
        "tx_pool_size": tx_pool_size,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
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

    // --- New Mempool ---
    let mempool = Arc::new(Mempool::new());

    // --- Time manager ---
    let time_manager = Arc::new(FinDAGTimeManager::new());

    // --- Hashtimer logging task ---
    let time_manager_clone = time_manager.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            let current_time = time_manager_clone.get_findag_time();
            let round_duration_ns = 16_000_000_000u64; // 16 seconds in nanoseconds
            let current_round = current_time / round_duration_ns;
            let hashtimer = compute_hashtimer(current_time, b"periodic_log", 0);
            println!("[Round {}][HashTimer] Current FinDAG Time: {}, Hash: 0x{}", 
                current_round, current_time, 
                hashtimer.iter().map(|b| format!("{:02x}", b)).collect::<String>());
        }
    });

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
    let mempool_clone = mempool.clone();
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
                interval_ms: 100, // 10 blocks/sec for faster transactions
                shard_id: 0, // single-shard
            };
            
            // Enhanced block production with mempool
            run_block_production_loop(
                &mut *dag_clone.lock().await,
                &tx_pool_clone,
                &mempool_clone,
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
        mempool: Arc::new(Mempool::new()),
        address: address.clone(),
        propagator: propagator.clone(),
        time_manager: time_manager.clone(),
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
        .route("/hashtimer-status", get(get_hashtimer_status))
        .route("/mempool-status", get(get_mempool_status))
        .layer(cors)
        .with_state(app_state);

    let port = std::env::var("FINDAG_HTTP_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("HTTP API listening on {}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
} 
use std::sync::Arc;
use axum::{
    http::StatusCode,
    Json,
    extract::State,
    debug_handler,
};
use tokio::sync::Mutex;
use ed25519_dalek::{Verifier, VerifyingKey, Signature};
use serde::{Serialize, Deserialize};
use clap::Parser;

use findag::core::dag_engine::DagEngine;
use findag::core::tx_pool::ShardedTxPool;
use findag::core::address::Address;
use findag::core::types::{Transaction, Block, ShardId};
use findag::dagtimer::findag_time_manager::FinDAGTimeManager;
use findag::dagtimer::hashtimer::compute_hashtimer;
use findag::network::propagation::NetworkPropagator;
use findag::network::consensus_integration::ConsensusIntegration;
use findag::network::encryption::P2PEncryption;
use findag::consensus::validator_set::ValidatorSet;
use serde_json::json;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// HTTP API port
    #[arg(long, default_value = "3001")]
    port: u16,

    /// Data directory
    #[arg(long, default_value = "state_db")]
    data_dir: String,

    /// P2P network port
    #[arg(long, default_value = "9001")]
    p2p_port: u16,
}

#[derive(Clone)]
struct AppState {
    tx_pool: Arc<ShardedTxPool>,
    dag: Arc<Mutex<DagEngine>>,
    #[allow(dead_code)]
    propagator: Arc<NetworkPropagator>,
    address: Address,
    time_manager: Arc<FinDAGTimeManager>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ApiTransaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub currency: String,
    pub shard_id: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TransactionRequest {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub signature: Vec<u8>,
    pub payload: Vec<u8>,
    pub findag_time: u64,
    pub hashtimer: Vec<u8>,
    pub public_key: Vec<u8>,
    pub shard_id: u16,
}

#[debug_handler]
#[allow(dead_code)]
async fn health(State(state): State<AppState>) -> Json<serde_json::Value> {
    let dag_stats = state.dag.lock().await.get_stats().await;
    let tx_pool_size = state.tx_pool.size(0); // shard 0
    let current_time = state.time_manager.get_findag_time();
    
    Json(json!({
        "status": "healthy",
        "address": state.address.0,
        "dag_stats": dag_stats,
        "tx_pool_size": tx_pool_size,
        "current_time": current_time,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[allow(dead_code)]
async fn node_info(State(state): State<AppState>) -> Json<serde_json::Value> {
    let dag = state.dag.lock().await;
    let block_count = dag.block_count().await as u64;
    let peers: Vec<String> = vec![]; // TODO: add real peer info
    Json(json!({
        "address": state.address.0,
        "peers": peers,
        "block_count": block_count,
    }))
}

#[debug_handler]
#[allow(dead_code)]
async fn submit_transaction(State(state): State<AppState>, Json(req): Json<TransactionRequest>) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    println!("[DEBUG] HTTP API: Received transaction request: {req:?}");
    
    // Validate the transaction
    let from_address = Address(req.from.clone());
    let to_address = Address(req.to.clone());
    
    // Verify signature
    let message = format!("{}{}{}", req.from, req.to, req.amount);
    println!("[DEBUG] HTTP API: Verifying signature for message: '{message}'");
    
    // Convert Vec<u8> to [u8; 64] for signature
    let signature_bytes: [u8; 64] = match req.signature.clone().try_into() {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("[DEBUG] HTTP API: Invalid signature length: expected 64 bytes, got {}", req.signature.len());
            return Err((StatusCode::BAD_REQUEST, Json(json!({
                "error": "Invalid signature length"
            }))));
        }
    };
    
    let signature = Signature::from_bytes(&signature_bytes);
    
    // Convert Vec<u8> to [u8; 32] for public key
    let public_key_bytes: [u8; 32] = match req.public_key.clone().try_into() {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("[DEBUG] HTTP API: Invalid public key length: expected 32 bytes, got {}", req.public_key.len());
            return Err((StatusCode::BAD_REQUEST, Json(json!({
                "error": "Invalid public key length"
            }))));
        }
    };
    
    let public_key = match VerifyingKey::from_bytes(&public_key_bytes) {
        Ok(pk) => pk,
        Err(e) => {
            println!("[DEBUG] HTTP API: Invalid public key format: {e:?}");
            return Err((StatusCode::BAD_REQUEST, Json(json!({
                "error": "Invalid public key format"
            }))));
        }
    };
    
    match public_key.verify(message.as_bytes(), &signature) {
        Ok(_) => println!("[DEBUG] HTTP API: Signature verification passed"),
        Err(e) => {
            println!("[DEBUG] HTTP API: Signature verification failed: {e:?}");
            return Err((StatusCode::BAD_REQUEST, Json(json!({
                "error": "Signature verification failed"
            }))));
        }
    }
    
    // Create transaction
    let transaction = Transaction {
        from: from_address.clone(),
        to: to_address.clone(),
        amount: req.amount,
        payload: req.payload.clone(),
        findag_time: req.findag_time,
        hashtimer: {
            let mut ht = [0u8; 32];
            if req.hashtimer.len() >= 32 {
                ht.copy_from_slice(&req.hashtimer[..32]);
            } else {
                ht[..req.hashtimer.len()].copy_from_slice(&req.hashtimer);
            }
            ht
        },
        public_key,
        shard_id: ShardId(req.shard_id),
        signature,
        source_shard: None,
        dest_shard: None,
        target_chain: None,
        bridge_protocol: None,
    };
    
    println!("[DEBUG] HTTP API: Created transaction, adding to tx_pool");
    // Add to transaction pool
    let added = state.tx_pool.add_transaction(transaction);
    if added {
        println!("[DEBUG] HTTP API: Transaction successfully added to tx_pool");
        Ok(Json(json!({
            "status": "success",
            "message": "Transaction submitted successfully"
        })))
    } else {
        println!("[DEBUG] HTTP API: Transaction rejected by tx_pool (likely insufficient funds or duplicate)");
        // Print the sender's balance for debugging
        let bal = state.tx_pool.get_balance(req.shard_id, &req.from, "USD");
        println!("[DEBUG] HTTP API: Sender balance for {}: {} USD", req.from, bal);
        Err((StatusCode::BAD_REQUEST, Json(json!({
            "error": "Transaction rejected",
            "balance": bal
        }))))
    }
}

#[debug_handler]
#[allow(dead_code)]
async fn get_blocks(State(state): State<AppState>) -> Json<Vec<Block>> {
    let dag = state.dag.lock().await;
    let blocks = dag.get_all_blocks().await;
    Json(blocks)
}

#[debug_handler]
#[allow(dead_code)]
async fn get_dag(State(state): State<AppState>) -> Json<serde_json::Value> {
    let dag = state.dag.lock().await;
    let stats = dag.get_stats().await;
    Json(json!(stats))
}

#[debug_handler]
#[allow(dead_code)]
async fn get_hashtimer_status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let current_time = state.time_manager.get_findag_time();
    let round_duration_ns = 16_000_000_000u64; // 16 seconds in nanoseconds
    let current_round = current_time / round_duration_ns;
    let hashtimer = compute_hashtimer(current_time, b"status_check", 0);
    Json(json!({
        "current_round": current_round,
        "current_findag_time": current_time,
        "hashtimer_hash": format!("0x{}", hashtimer.iter().map(|b| format!("{b:02x}")).collect::<String>()),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

#[debug_handler]
#[allow(dead_code)]
async fn get_mempool_status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let tx_pool_size = state.tx_pool.size(0);
    Json(json!({
        "tx_pool_size": tx_pool_size,
        "shard_id": 0,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

#[debug_handler]
#[allow(dead_code)]
async fn get_mempool_transactions(State(state): State<AppState>) -> Json<serde_json::Value> {
    let transactions = state.tx_pool.get_transactions(1000, 0); // Get up to 1000 transactions from shard 0
    let tx_data: Vec<serde_json::Value> = transactions.iter().map(|tx| {
        json!({
            "from": tx.from.as_str(),
            "to": tx.to.as_str(),
            "amount": tx.amount,
            "findag_time": tx.findag_time,
            "hashtimer": format!("0x{}", tx.hashtimer.iter().map(|b| format!("{b:02x}")).collect::<String>()),
            "shard_id": tx.shard_id.0,
        })
    }).collect();
    Json(json!({
        "transactions": tx_data,
        "count": tx_data.len(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

#[debug_handler]
#[allow(dead_code)]
async fn get_recent_transactions(State(state): State<AppState>) -> Json<serde_json::Value> {
    let dag = state.dag.lock().await;
    let blocks = dag.get_all_blocks().await;
    // Get transactions from the last 10 blocks
    let recent_blocks = blocks.iter().rev().take(10).collect::<Vec<_>>();
    let blocks_checked = recent_blocks.len();
    let mut all_transactions = Vec::new();
    for block in &recent_blocks {
        for tx in &block.transactions {
            all_transactions.push(json!({
                "block_hash": format!("0x{}", block.hashtimer.iter().map(|b| format!("{b:02x}")).collect::<String>()),
                "from": tx.from.0,
                "to": tx.to.0,
                "amount": tx.amount,
                "findag_time": tx.findag_time,
                "hashtimer": format!("0x{}", tx.hashtimer.iter().map(|b| format!("{b:02x}")).collect::<String>()),
                "shard_id": tx.shard_id.0,
            }));
        }
    }
    Json(json!({
        "transactions": all_transactions,
        "count": all_transactions.len(),
        "blocks_checked": blocks_checked,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

#[debug_handler]
#[allow(dead_code)]
async fn get_simple_transactions(State(state): State<AppState>) -> Json<serde_json::Value> {
    // Get tx_pool transactions
    let transactions = state.tx_pool.get_transactions(1000, 0); // Get up to 1000 transactions from shard 0
    
    // Convert to JSON format
    let tx_data: Vec<serde_json::Value> = transactions.iter().map(|tx| {
        json!({
            "from": tx.from.as_str(),
            "to": tx.to.as_str(),
            "amount": tx.amount,
            "findag_time": tx.findag_time,
            "hashtimer": format!("0x{}", tx.hashtimer.iter().map(|b| format!("{b:02x}")).collect::<String>()),
            "status": "pending"
        })
    }).collect();

    Json(json!({
        "transactions": tx_data,
        "count": tx_data.len(),
        "source": "tx_pool",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

#[debug_handler]
#[allow(dead_code)]
async fn get_transaction_summary(State(state): State<AppState>) -> Json<serde_json::Value> {
    let tx_pool_size = state.tx_pool.size(0);
    
    Json(json!({
        "tx_pool_transactions": tx_pool_size,
        "total_transactions_processed": "unknown", // Would need to track this
        "node_status": "running",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

// TODO: Implement get_transactions_by_block endpoint
// This would need to be implemented to find a specific block by hash

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Initialize the node with the parsed arguments
    println!("Starting FinDAG node with the following configuration:");
    println!("HTTP Port: {}", args.port);
    println!("Data Directory: {}", args.data_dir);
    println!("P2P Port: {}", args.p2p_port);

    // Create data directory if it doesn't exist
    if let Err(e) = std::fs::create_dir_all(&args.data_dir) {
        eprintln!("Failed to create data directory '{}': {}", args.data_dir, e);
        return;
    }

    // Initialize node components
    let asset_whitelist = Arc::new(std::sync::Mutex::new(vec!["USD".to_string()]));
    let tx_pool = Arc::new(ShardedTxPool::new_with_whitelist_per_shard_and_data_dir(
        100_000, 
        asset_whitelist, 
        1, 
        &args.data_dir
    ));

    // Initialize DAG engine
    let dag = Arc::new(Mutex::new(DagEngine::new().await));
    
    // Initialize validator set
    let validator_set = Arc::new(Mutex::new(ValidatorSet::new()));
    
    // Initialize time manager
    let _time_manager = Arc::new(FinDAGTimeManager::new());
    
    // Generate local node address and keypair
    let (local_keypair, local_address) = findag::core::address::generate_address();
    
    // Initialize encryption layer
    let encryption = Arc::new(P2PEncryption::new_from_ed25519(&local_keypair));
    
    // Initialize network propagator with encryption
    let peers = vec![]; // TODO: Load from configuration
    let mut propagator = NetworkPropagator::new("0.0.0.0:9000", peers, local_address.clone()).await
        .expect("Failed to create network propagator");
    propagator.enable_encryption(encryption.clone());
    let propagator = Arc::new(propagator);
    
    // Initialize consensus integration
    let consensus_integration = ConsensusIntegration::new(
        propagator.clone(),
        validator_set.clone(),
        dag.clone(),
        tx_pool.clone(),
        local_address.clone(),
        Some(local_keypair),
    );
    
    // Start consensus integration
    consensus_integration.start().await;

    // Start HTTP server
    if let Err(e) = findag::api::http_server::start(args.port, tx_pool.clone()).await {
        eprintln!("Failed to start HTTP server: {e}");
        return;
    }

    // Start P2P network
    if let Err(e) = findag::network::p2p::start(args.p2p_port, tx_pool).await {
        eprintln!("Failed to start P2P network: {e}");
        return;
    }
    
    println!("✅ FinDAG node started successfully!");
    println!("🔗 Local address: {}", local_address.as_str());
    println!("🌐 P2P port: {}", args.p2p_port);
    println!("📡 HTTP port: {}", args.port);
    
    // Keep the main thread alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
} 
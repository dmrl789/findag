use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use std::env;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    extract::State,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub block_id: String,
    pub transactions: Vec<Transaction>,
    pub timestamp: u64,
    pub proposer: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub address: Address,
    pub peers: Vec<String>,
    pub block_count: u64,
    pub transaction_count: u64,
}

#[derive(Debug, Clone)]
pub struct SimpleStorage {
    pub transactions: Arc<Mutex<Vec<Transaction>>>,
    pub blocks: Arc<Mutex<Vec<Block>>>,
    pub peers: Arc<Mutex<Vec<String>>>,
}

impl SimpleStorage {
    pub fn new() -> Self {
        Self {
            transactions: Arc::new(Mutex::new(Vec::new())),
            blocks: Arc::new(Mutex::new(Vec::new())),
            peers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add_transaction(&self, tx: Transaction) {
        let mut txs = self.transactions.lock().unwrap();
        txs.push(tx);
        println!("Added transaction: {:?}", txs.last().unwrap());
    }

    pub async fn add_block(&self, block: Block) {
        let mut blocks = self.blocks.lock().unwrap();
        blocks.push(block);
        println!("Added block: {:?}", blocks.last().unwrap());
    }

    pub async fn get_stats(&self) -> (u64, u64) {
        let block_count = self.blocks.lock().unwrap().len() as u64;
        let tx_count = self.transactions.lock().unwrap().len() as u64;
        (block_count, tx_count)
    }
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn get_node_info(State(storage): State<Arc<SimpleStorage>>) -> Json<NodeInfo> {
    let (block_count, transaction_count) = storage.get_stats().await;
    let peers = storage.peers.lock().unwrap().clone();
    
    Json(NodeInfo {
        address: Address("fdg1testnode".to_string()),
        peers,
        block_count,
        transaction_count,
    })
}

async fn submit_transaction(
    State(storage): State<Arc<SimpleStorage>>,
    Json(tx): Json<Transaction>,
) -> StatusCode {
    storage.add_transaction(tx).await;
    StatusCode::OK
}

async fn get_transactions(State(storage): State<Arc<SimpleStorage>>) -> Json<Vec<Transaction>> {
    let txs = storage.transactions.lock().unwrap().clone();
    Json(txs)
}

async fn get_blocks(State(storage): State<Arc<SimpleStorage>>) -> Json<Vec<Block>> {
    let blocks = storage.blocks.lock().unwrap().clone();
    Json(blocks)
}

async fn block_production_loop(storage: Arc<SimpleStorage>) {
    let mut block_id = 0u64;
    let proposer = Address("fdg1testnode".to_string());
    loop {
        sleep(Duration::from_secs(5)).await;
        let block_transactions = {
            let mut txs = storage.transactions.lock().unwrap();
            if txs.is_empty() {
                None
            } else {
                let tx_count = txs.len();
                Some(txs.drain(..std::cmp::min(10, tx_count)).collect::<Vec<_>>())
            }
        };
        if let Some(block_transactions) = block_transactions {
            let block = Block {
                block_id: format!("block_{}", block_id),
                transactions: block_transactions,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                proposer: proposer.clone(),
            };
            storage.add_block(block).await;
            block_id += 1;
            println!("Produced block {}", block_id);
        }
    }
}

async fn transaction_generator(storage: Arc<SimpleStorage>) {
    let addresses = vec![
        Address("fdg1alice".to_string()),
        Address("fdg1bob".to_string()),
        Address("fdg1charlie".to_string()),
        Address("fdg1diana".to_string()),
    ];
    let rng = fastrand::Rng::new();
    loop {
        sleep(Duration::from_millis(1000)).await;
        let from = addresses[rng.usize(..addresses.len())].clone();
        let to = addresses[rng.usize(..addresses.len())].clone();
        if from.0 != to.0 {
            let tx = Transaction {
                from,
                to,
                amount: rng.u64(1..1000),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            storage.add_transaction(tx).await;
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Starting FinDAG Node...");
    
    let bind_addr = env::var("FINDAG_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let peers = env::var("FINDAG_PEERS")
        .unwrap_or_else(|_| "127.0.0.1:8081,127.0.0.1:8082".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();
    
    println!("Bind address: {}", bind_addr);
    println!("Peers: {:?}", peers);
    
    let storage = Arc::new(SimpleStorage::new());
    
    {
        let mut peer_list = storage.peers.lock().unwrap();
        peer_list.extend(peers);
    }
    
    let storage_clone = storage.clone();
    tokio::spawn(async move {
        block_production_loop(storage_clone).await;
    });
    
    let storage_clone = storage.clone();
    tokio::spawn(async move {
        transaction_generator(storage_clone).await;
    });
    
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/node/info", get(get_node_info))
        .route("/transactions", get(get_transactions))
        .route("/transactions", post(submit_transaction))
        .route("/blocks", get(get_blocks))
        .with_state(storage);
    
    let listener = TcpListener::bind(&bind_addr).await.unwrap();
    println!("HTTP server listening on http://{}", bind_addr);
    println!("Health check: http://{}/health", bind_addr);
    println!("Node info: http://{}/node/info", bind_addr);
    println!("Transactions: http://{}/transactions", bind_addr);
    println!("Blocks: http://{}/blocks", bind_addr);
    
    axum::serve(listener, app).await.unwrap();
} 
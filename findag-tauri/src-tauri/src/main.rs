// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Mock data structures for demonstration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeStatus {
    pub is_running: bool,
    pub is_connected: bool,
    pub uptime: u64,
    pub peers: u32,
    pub tps: u32,
    pub blocks_per_second: u32,
    pub mempool_size: u32,
    pub last_block_hash: Option<String>,
    pub last_round_number: Option<u64>,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeConfig {
    pub port: u16,
    pub peers: Vec<String>,
    pub data_directory: String,
    pub max_block_size: u32,
    pub block_interval: u32,
    pub round_interval: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletInfo {
    pub address: String,
    pub public_key: String,
    pub handle: Option<String>,
    pub total_balance: f64,
    pub transaction_count: u32,
    pub last_activity: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Balance {
    pub asset: String,
    pub amount: f64,
    pub available: f64,
    pub locked: f64,
    pub last_updated: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: String,
    pub transaction_type: String,
    pub asset: String,
    pub amount: f64,
    pub fee: f64,
    pub status: String,
    pub timestamp: u64,
    pub block_hash: Option<String>,
    pub round_number: Option<u64>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub memo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: String,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub quantity: f64,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub status: String,
    pub timestamp: u64,
    pub filled_quantity: Option<f64>,
    pub average_price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketData {
    pub symbol: String,
    pub last_price: f64,
    pub bid: f64,
    pub ask: f64,
    pub volume: u64,
    pub change: f64,
    pub change_percent: f64,
    pub high: f64,
    pub low: f64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub average_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Peer {
    pub id: String,
    pub address: String,
    pub port: u16,
    pub status: String,
    pub latency: u32,
    pub last_seen: u64,
    pub version: String,
    pub user_agent: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkStats {
    pub total_peers: u32,
    pub connected_peers: u32,
    pub total_bandwidth: u64,
    pub average_latency: u32,
    pub uptime: u64,
    pub blocks_received: u64,
    pub blocks_sent: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemInfo {
    pub platform: String,
    pub version: String,
    pub memory_total: u64,
    pub memory_available: u64,
    pub cpu_cores: u32,
    pub cpu_usage: f64,
    pub disk_total: u64,
    pub disk_available: u64,
    pub node_id: String,
    pub architecture: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemConfig {
    pub port: u16,
    pub max_block_size: u32,
    pub block_interval: u32,
    pub round_interval: u32,
    pub peers: Vec<String>,
    pub validator_address: String,
    pub validator_public_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemStats {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub uptime: u64,
    pub network_connections: u32,
    pub active_processes: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
    pub notifications: bool,
    pub auto_refresh: bool,
    pub refresh_interval: u32,
    pub debug_mode: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: u64,
    pub level: String,
    pub component: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

// Global state for the application
struct AppState {
    node_status: Arc<Mutex<NodeStatus>>,
    node_config: Arc<Mutex<NodeConfig>>,
    wallets: Arc<Mutex<HashMap<String, WalletInfo>>>,
    balances: Arc<Mutex<HashMap<String, Balance>>>,
    transactions: Arc<Mutex<Vec<Transaction>>>,
    orders: Arc<Mutex<Vec<Order>>>,
    market_data: Arc<Mutex<HashMap<String, MarketData>>>,
    positions: Arc<Mutex<Vec<Position>>>,
    peers: Arc<Mutex<Vec<Peer>>>,
    network_stats: Arc<Mutex<NetworkStats>>,
    logs: Arc<Mutex<Vec<LogEntry>>>,
}

impl AppState {
    fn new() -> Self {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            node_status: Arc::new(Mutex::new(NodeStatus {
                is_running: false,
                is_connected: false,
                uptime: 0,
                peers: 0,
                tps: 0,
                blocks_per_second: 0,
                mempool_size: 0,
                last_block_hash: None,
                last_round_number: None,
                version: "1.0.0".to_string(),
            })),
            node_config: Arc::new(Mutex::new(NodeConfig {
                port: 8080,
                peers: vec![],
                data_directory: "./data".to_string(),
                max_block_size: 32768,
                block_interval: 50,
                round_interval: 250,
            })),
            wallets: Arc::new(Mutex::new(HashMap::new())),
            balances: Arc::new(Mutex::new(HashMap::new())),
            transactions: Arc::new(Mutex::new(Vec::new())),
            orders: Arc::new(Mutex::new(Vec::new())),
            market_data: Arc::new(Mutex::new(HashMap::new())),
            positions: Arc::new(Mutex::new(Vec::new())),
            peers: Arc::new(Mutex::new(Vec::new())),
            network_stats: Arc::new(Mutex::new(NetworkStats {
                total_peers: 0,
                connected_peers: 0,
                total_bandwidth: 0,
                average_latency: 0,
                uptime: start_time,
                blocks_received: 0,
                blocks_sent: 0,
            })),
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

// Node Management Commands
#[tauri::command]
async fn start_findag_node(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut status = state.node_status.lock().unwrap();
    status.is_running = true;
    status.is_connected = true;
    status.uptime = 0;
    status.peers = 2;
    status.tps = 150;
    status.blocks_per_second = 8;
    status.mempool_size = 125;
    
    // Add a log entry
    let mut logs = state.logs.lock().unwrap();
    logs.push(LogEntry {
        id: format!("log_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        level: "info".to_string(),
        component: "node".to_string(),
        message: "FinDAG node started successfully".to_string(),
        details: None,
    });
    
    Ok(())
}

#[tauri::command]
async fn stop_findag_node(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut status = state.node_status.lock().unwrap();
    status.is_running = false;
    status.is_connected = false;
    status.uptime = 0;
    status.peers = 0;
    status.tps = 0;
    status.blocks_per_second = 0;
    status.mempool_size = 0;
    
    // Add a log entry
    let mut logs = state.logs.lock().unwrap();
    logs.push(LogEntry {
        id: format!("log_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        level: "info".to_string(),
        component: "node".to_string(),
        message: "FinDAG node stopped".to_string(),
        details: None,
    });
    
    Ok(())
}

#[tauri::command]
async fn get_node_status(state: tauri::State<'_, AppState>) -> Result<NodeStatus, String> {
    let status = state.node_status.lock().unwrap();
    Ok(status.clone())
}

#[tauri::command]
async fn get_node_config(state: tauri::State<'_, AppState>) -> Result<NodeConfig, String> {
    let config = state.node_config.lock().unwrap();
    Ok(config.clone())
}

#[tauri::command]
async fn update_node_config(
    state: tauri::State<'_, AppState>,
    config: NodeConfig,
) -> Result<(), String> {
    let mut current_config = state.node_config.lock().unwrap();
    *current_config = config;
    Ok(())
}

// Wallet Operations
#[tauri::command]
async fn create_wallet(state: tauri::State<'_, AppState>) -> Result<WalletInfo, String> {
    let wallet_id = format!("wallet_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
    let wallet = WalletInfo {
        address: format!("fdg1q{}", wallet_id),
        public_key: format!("public_key_{}", wallet_id),
        handle: Some(format!("@user.{}", wallet_id)),
        total_balance: 0.0,
        transaction_count: 0,
        last_activity: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
    };
    
    let mut wallets = state.wallets.lock().unwrap();
    wallets.insert(wallet_id.clone(), wallet.clone());
    
    Ok(wallet)
}

#[tauri::command]
async fn import_wallet(
    state: tauri::State<'_, AppState>,
    private_key: String,
) -> Result<WalletInfo, String> {
    let wallet_id = format!("imported_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
    let wallet = WalletInfo {
        address: format!("fdg1q{}", wallet_id),
        public_key: format!("imported_key_{}", wallet_id),
        handle: Some("@imported.user".to_string()),
        total_balance: 1000.0,
        transaction_count: 5,
        last_activity: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
    };
    
    let mut wallets = state.wallets.lock().unwrap();
    wallets.insert(wallet_id.clone(), wallet.clone());
    
    // Add some mock balances
    let mut balances = state.balances.lock().unwrap();
    balances.insert("EUR".to_string(), Balance {
        asset: "EUR".to_string(),
        amount: 10000.0,
        available: 9500.0,
        locked: 500.0,
        last_updated: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
    });
    
    Ok(wallet)
}

#[tauri::command]
async fn get_wallet_balance(
    state: tauri::State<'_, AppState>,
    wallet_address: String,
) -> Result<Vec<Balance>, String> {
    let balances = state.balances.lock().unwrap();
    Ok(balances.values().cloned().collect())
}

#[tauri::command]
async fn send_transaction(
    state: tauri::State<'_, AppState>,
    to_address: String,
    asset: String,
    amount: f64,
    memo: Option<String>,
) -> Result<Transaction, String> {
    let transaction_id = format!("tx_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
    let transaction = Transaction {
        id: transaction_id.clone(),
        transaction_type: "send".to_string(),
        asset: asset.clone(),
        amount: -amount,
        fee: 0.001,
        status: "pending".to_string(),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        block_hash: None,
        round_number: None,
        from_address: Some("fdg1qwallet123".to_string()),
        to_address: Some(to_address),
        memo,
    };
    
    let mut transactions = state.transactions.lock().unwrap();
    transactions.push(transaction.clone());
    
    // Simulate transaction confirmation after 3 seconds
    let transaction_clone = transaction.clone();
    let transactions_clone = state.transactions.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        let mut txs = transactions_clone.lock().unwrap();
        if let Some(tx) = txs.iter_mut().find(|t| t.id == transaction_clone.id) {
            tx.status = "confirmed".to_string();
            tx.block_hash = Some(format!("block_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()));
        }
    });
    
    Ok(transaction)
}

#[tauri::command]
async fn get_transaction_history(
    state: tauri::State<'_, AppState>,
    wallet_address: Option<String>,
) -> Result<Vec<Transaction>, String> {
    let transactions = state.transactions.lock().unwrap();
    Ok(transactions.clone())
}

// Trading Operations
#[tauri::command]
async fn get_trading_pairs(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    Ok(vec![
        "EUR/USD".to_string(),
        "USD/JPY".to_string(),
        "GBP/USD".to_string(),
        "USD/CHF".to_string(),
        "AUD/USD".to_string(),
    ])
}

#[tauri::command]
async fn get_market_data(
    state: tauri::State<'_, AppState>,
    symbol: String,
) -> Result<MarketData, String> {
    let market_data = MarketData {
        symbol: symbol.clone(),
        last_price: 1.0850 + (rand::random::<f64>() - 0.5) * 0.01,
        bid: 1.0848,
        ask: 1.0852,
        volume: rand::random::<u64>() % 1000000 + 100000,
        change: (rand::random::<f64>() - 0.5) * 0.01,
        change_percent: (rand::random::<f64>() - 0.5) * 2.0,
        high: 1.0900,
        low: 1.0800,
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
    };
    
    let mut market_data_map = state.market_data.lock().unwrap();
    market_data_map.insert(symbol, market_data.clone());
    
    Ok(market_data)
}

#[tauri::command]
async fn place_order(
    state: tauri::State<'_, AppState>,
    symbol: String,
    side: String,
    order_type: String,
    quantity: f64,
    price: Option<f64>,
    stop_price: Option<f64>,
) -> Result<Order, String> {
    let order_id = format!("order_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
    let order = Order {
        id: order_id.clone(),
        symbol,
        side,
        order_type,
        quantity,
        price,
        stop_price,
        status: "pending".to_string(),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        filled_quantity: None,
        average_price: None,
    };
    
    let mut orders = state.orders.lock().unwrap();
    orders.push(order.clone());
    
    // Simulate order execution after 2 seconds
    let order_clone = order.clone();
    let orders_clone = state.orders.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let mut ords = orders_clone.lock().unwrap();
        if let Some(ord) = ords.iter_mut().find(|o| o.id == order_clone.id) {
            ord.status = "filled".to_string();
            ord.filled_quantity = Some(ord.quantity);
            ord.average_price = ord.price;
        }
    });
    
    Ok(order)
}

#[tauri::command]
async fn cancel_order(
    state: tauri::State<'_, AppState>,
    order_id: String,
) -> Result<(), String> {
    let mut orders = state.orders.lock().unwrap();
    if let Some(order) = orders.iter_mut().find(|o| o.id == order_id) {
        order.status = "cancelled".to_string();
    }
    Ok(())
}

#[tauri::command]
async fn get_order_history(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<Order>, String> {
    let orders = state.orders.lock().unwrap();
    Ok(orders.clone())
}

// DAG Operations
#[tauri::command]
async fn get_dag_status(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "total_blocks": 12345,
        "total_rounds": 1234,
        "current_round": 1235,
        "dag_height": 12345,
        "last_block_hash": "block_hash_12345",
        "last_round_hash": "round_hash_1234"
    }))
}

#[tauri::command]
async fn get_dag_blocks(
    state: tauri::State<'_, AppState>,
    limit: Option<u32>,
) -> Result<Vec<serde_json::Value>, String> {
    let limit = limit.unwrap_or(10);
    let mut blocks = Vec::new();
    
    for i in 0..limit {
        blocks.push(serde_json::json!({
            "hash": format!("block_hash_{}", 12345 - i),
            "round": 1234 - (i / 10),
            "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - (i * 50),
            "transactions": rand::random::<u32>() % 100 + 10,
            "size": rand::random::<u32>() % 32000 + 1000
        }));
    }
    
    Ok(blocks)
}

#[tauri::command]
async fn get_dag_transactions(
    state: tauri::State<'_, AppState>,
    block_hash: Option<String>,
) -> Result<Vec<Transaction>, String> {
    let transactions = state.transactions.lock().unwrap();
    Ok(transactions.clone())
}

#[tauri::command]
async fn submit_dag_transaction(
    state: tauri::State<'_, AppState>,
    transaction_data: serde_json::Value,
) -> Result<String, String> {
    let transaction_id = format!("dag_tx_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
    
    // Add to mempool
    let mut status = state.node_status.lock().unwrap();
    status.mempool_size += 1;
    
    Ok(transaction_id)
}

// Network Operations
#[tauri::command]
async fn get_network_status(state: tauri::State<'_, AppState>) -> Result<NetworkStats, String> {
    let stats = state.network_stats.lock().unwrap();
    Ok(stats.clone())
}

#[tauri::command]
async fn get_peer_list(state: tauri::State<'_, AppState>) -> Result<Vec<Peer>, String> {
    let peers = state.peers.lock().unwrap();
    Ok(peers.clone())
}

#[tauri::command]
async fn add_peer(
    state: tauri::State<'_, AppState>,
    address: String,
    port: u16,
) -> Result<(), String> {
    let peer = Peer {
        id: format!("peer_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()),
        address,
        port,
        status: "connecting".to_string(),
        latency: 0,
        last_seen: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        version: "1.0.0".to_string(),
        user_agent: "FinDAG/1.0.0".to_string(),
    };
    
    let mut peers = state.peers.lock().unwrap();
    peers.push(peer);
    
    Ok(())
}

#[tauri::command]
async fn remove_peer(
    state: tauri::State<'_, AppState>,
    peer_id: String,
) -> Result<(), String> {
    let mut peers = state.peers.lock().unwrap();
    peers.retain(|p| p.id != peer_id);
    Ok(())
}

// System Operations
#[tauri::command]
async fn get_system_info(state: tauri::State<'_, AppState>) -> Result<SystemInfo, String> {
    Ok(SystemInfo {
        platform: std::env::consts::OS.to_string(),
        version: "1.0.0".to_string(),
        memory_total: 16 * 1024 * 1024 * 1024, // 16GB
        memory_available: 8 * 1024 * 1024 * 1024, // 8GB
        cpu_cores: num_cpus::get() as u32,
        cpu_usage: rand::random::<f64>() * 100.0,
        disk_total: 1000 * 1024 * 1024 * 1024, // 1TB
        disk_available: 500 * 1024 * 1024 * 1024, // 500GB
        node_id: "node_001".to_string(),
        architecture: std::env::consts::ARCH.to_string(),
    })
}

#[tauri::command]
async fn get_system_config(state: tauri::State<'_, AppState>) -> Result<SystemConfig, String> {
    let config = state.node_config.lock().unwrap();
    Ok(SystemConfig {
        port: config.port,
        max_block_size: config.max_block_size,
        block_interval: config.block_interval,
        round_interval: config.round_interval,
        peers: config.peers.clone(),
        validator_address: "validator_001".to_string(),
        validator_public_key: "validator_pub_key_001".to_string(),
    })
}

#[tauri::command]
async fn get_system_stats(state: tauri::State<'_, AppState>) -> Result<SystemStats, String> {
    let status = state.node_status.lock().unwrap();
    Ok(SystemStats {
        cpu_usage: rand::random::<f64>() * 100.0,
        memory_usage: rand::random::<f64>() * 100.0,
        disk_usage: rand::random::<f64>() * 100.0,
        uptime: status.uptime,
        network_connections: status.peers,
        active_processes: 1,
    })
}

#[tauri::command]
async fn update_system_config(
    state: tauri::State<'_, AppState>,
    config: SystemConfig,
) -> Result<(), String> {
    let mut node_config = state.node_config.lock().unwrap();
    node_config.port = config.port;
    node_config.max_block_size = config.max_block_size;
    node_config.block_interval = config.block_interval;
    node_config.round_interval = config.round_interval;
    node_config.peers = config.peers;
    Ok(())
}

#[tauri::command]
async fn update_app_settings(
    state: tauri::State<'_, AppState>,
    settings: AppSettings,
) -> Result<(), String> {
    // In a real implementation, this would save settings to a file
    // For now, we'll just return success
    Ok(())
}

#[tauri::command]
async fn restart_node(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut status = state.node_status.lock().unwrap();
    status.is_running = false;
    
    // Simulate restart delay
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    status.is_running = true;
    status.uptime = 0;
    Ok(())
}

#[tauri::command]
async fn get_logs(
    state: tauri::State<'_, AppState>,
    level: Option<String>,
    component: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<LogEntry>, String> {
    let logs = state.logs.lock().unwrap();
    let mut filtered_logs = logs.clone();
    
    if let Some(level_filter) = level {
        filtered_logs.retain(|log| log.level == level_filter);
    }
    
    if let Some(component_filter) = component {
        filtered_logs.retain(|log| log.component == component_filter);
    }
    
    if let Some(limit_count) = limit {
        filtered_logs.truncate(limit_count as usize);
    }
    
    Ok(filtered_logs)
}

#[tauri::command]
async fn export_data(
    state: tauri::State<'_, AppState>,
    data_type: String,
) -> Result<String, String> {
    match data_type.as_str() {
        "transactions" => {
            let transactions = state.transactions.lock().unwrap();
            Ok(serde_json::to_string(&*transactions).unwrap())
        }
        "orders" => {
            let orders = state.orders.lock().unwrap();
            Ok(serde_json::to_string(&*orders).unwrap())
        }
        "logs" => {
            let logs = state.logs.lock().unwrap();
            Ok(serde_json::to_string(&*logs).unwrap())
        }
        _ => Err("Unknown data type".to_string()),
    }
}

#[tauri::command]
async fn backup_wallet(
    state: tauri::State<'_, AppState>,
    wallet_address: String,
) -> Result<String, String> {
    let wallets = state.wallets.lock().unwrap();
    let balances = state.balances.lock().unwrap();
    
    let backup_data = serde_json::json!({
        "wallet": wallets.get(&wallet_address),
        "balances": balances.values().collect::<Vec<_>>(),
        "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    });
    
    Ok(serde_json::to_string(&backup_data).unwrap())
}

fn main() {
    let app_state = AppState::new();
    
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Node Management
            start_findag_node,
            stop_findag_node,
            get_node_status,
            get_node_config,
            update_node_config,
            
            // Wallet Operations
            create_wallet,
            import_wallet,
            get_wallet_balance,
            send_transaction,
            get_transaction_history,
            
            // Trading Operations
            get_trading_pairs,
            get_market_data,
            place_order,
            cancel_order,
            get_order_history,
            
            // DAG Operations
            get_dag_status,
            get_dag_blocks,
            get_dag_transactions,
            submit_dag_transaction,
            
            // Network Operations
            get_network_status,
            get_peer_list,
            add_peer,
            remove_peer,
            
            // System Operations
            get_system_info,
            get_system_config,
            get_system_stats,
            update_system_config,
            update_app_settings,
            restart_node,
            get_logs,
            export_data,
            backup_wallet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 
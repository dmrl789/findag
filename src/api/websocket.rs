use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::sync::broadcast;
use futures::{SinkExt, StreamExt};
use chrono::{DateTime, Utc};
use crate::api::http_server::AppState;
use fastrand;
use tokio::time::{sleep, Duration};

// WebSocket message types
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WebSocketMessage {
    // Price updates
    PriceUpdate {
        symbol: String,
        price: f64,
        change: f64,
        change_percent: f64,
        timestamp: DateTime<Utc>,
    },
    
    // Order book updates
    OrderBookUpdate {
        symbol: String,
        bids: Vec<OrderBookEntry>,
        asks: Vec<OrderBookEntry>,
        timestamp: DateTime<Utc>,
    },
    
    // Trade updates
    TradeUpdate {
        symbol: String,
        price: f64,
        quantity: f64,
        side: String, // "buy" or "sell"
        timestamp: DateTime<Utc>,
        trade_id: String,
    },
    
    // Market data updates
    MarketDataUpdate {
        symbol: String,
        volume_24h: f64,
        high_24h: f64,
        low_24h: f64,
        open_24h: f64,
        timestamp: DateTime<Utc>,
    },
    
    // DAG updates
    DAGUpdate {
        new_transactions: Vec<String>,
        new_blocks: Vec<String>,
        network_status: String,
        timestamp: DateTime<Utc>,
    },
    
    // System messages
    SystemMessage {
        message: String,
        level: String, // "info", "warning", "error"
        timestamp: DateTime<Utc>,
    },
    
    // Subscription management
    Subscribe {
        channels: Vec<String>,
    },
    
    Unsubscribe {
        channels: Vec<String>,
    },
    
    // Ping/Pong for connection health
    Ping,
    Pong,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderBookEntry {
    pub price: f64,
    pub quantity: f64,
    pub order_count: u32,
}

// WebSocket connection manager
pub struct WebSocketManager {
    pub connections: Arc<Mutex<HashMap<String, broadcast::Sender<WebSocketMessage>>>>,
    pub price_sender: broadcast::Sender<WebSocketMessage>,
    pub orderbook_sender: broadcast::Sender<WebSocketMessage>,
    pub trade_sender: broadcast::Sender<WebSocketMessage>,
    pub market_data_sender: broadcast::Sender<WebSocketMessage>,
    pub dag_sender: broadcast::Sender<WebSocketMessage>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        let (price_sender, _) = broadcast::channel(1000);
        let (orderbook_sender, _) = broadcast::channel(1000);
        let (trade_sender, _) = broadcast::channel(1000);
        let (market_data_sender, _) = broadcast::channel(1000);
        let (dag_sender, _) = broadcast::channel(1000);
        
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
            price_sender,
            orderbook_sender,
            market_data_sender,
            trade_sender,
            dag_sender,
        }
    }
    
    pub fn broadcast_price_update(&self, symbol: &str, price: f64, change: f64, change_percent: f64) {
        let message = WebSocketMessage::PriceUpdate {
            symbol: symbol.to_string(),
            price,
            change,
            change_percent,
            timestamp: Utc::now(),
        };
        
        let _ = self.price_sender.send(message);
    }
    
    pub fn broadcast_orderbook_update(&self, symbol: &str, bids: Vec<OrderBookEntry>, asks: Vec<OrderBookEntry>) {
        let message = WebSocketMessage::OrderBookUpdate {
            symbol: symbol.to_string(),
            bids,
            asks,
            timestamp: Utc::now(),
        };
        
        let _ = self.orderbook_sender.send(message);
    }
    
    pub fn broadcast_trade_update(&self, symbol: &str, price: f64, quantity: f64, side: &str) {
        let message = WebSocketMessage::TradeUpdate {
            symbol: symbol.to_string(),
            price,
            quantity,
            side: side.to_string(),
            timestamp: Utc::now(),
            trade_id: uuid::Uuid::new_v4().to_string(),
        };
        
        let _ = self.trade_sender.send(message);
    }
    
    pub fn broadcast_market_data_update(&self, symbol: &str, volume_24h: f64, high_24h: f64, low_24h: f64, open_24h: f64) {
        let message = WebSocketMessage::MarketDataUpdate {
            symbol: symbol.to_string(),
            volume_24h,
            high_24h,
            low_24h,
            open_24h,
            timestamp: Utc::now(),
        };
        
        let _ = self.market_data_sender.send(message);
    }
    
    pub fn broadcast_dag_update(&self, new_transactions: Vec<String>, new_blocks: Vec<String>, network_status: &str) {
        let message = WebSocketMessage::DAGUpdate {
            new_transactions,
            new_blocks,
            network_status: network_status.to_string(),
            timestamp: Utc::now(),
        };
        
        let _ = self.dag_sender.send(message);
    }
}

// WebSocket handler
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    
    // Create a unique connection ID
    let connection_id = uuid::Uuid::new_v4().to_string();
    
    // Create broadcast receivers for different channels
    let mut price_receiver = state.ws_manager.price_sender.subscribe();
    let mut orderbook_receiver = state.ws_manager.orderbook_sender.subscribe();
    let mut trade_receiver = state.ws_manager.trade_sender.subscribe();
    let mut market_data_receiver = state.ws_manager.market_data_sender.subscribe();
    let mut dag_receiver = state.ws_manager.dag_sender.subscribe();
    
    // Track subscribed channels for this connection
    let mut subscribed_channels: Vec<String> = Vec::new();
    
    // Send welcome message
    let welcome = WebSocketMessage::SystemMessage {
        message: "Connected to FinDAG WebSocket API".to_string(),
        level: "info".to_string(),
        timestamp: Utc::now(),
    };
    
    if let Ok(msg) = serde_json::to_string(&welcome) {
        let _ = sender.send(Message::Text(msg)).await;
    }
    
    // Handle incoming messages and broadcast updates
    loop {
        tokio::select! {
            // Handle incoming WebSocket messages
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(message) = serde_json::from_str::<WebSocketMessage>(&text) {
                            match message {
                                WebSocketMessage::Subscribe { channels } => {
                                    subscribed_channels.extend(channels);
                                    let response = WebSocketMessage::SystemMessage {
                                        message: format!("Subscribed to channels: {:?}", subscribed_channels),
                                        level: "info".to_string(),
                                        timestamp: Utc::now(),
                                    };
                                    if let Ok(msg) = serde_json::to_string(&response) {
                                        let _ = sender.send(Message::Text(msg)).await;
                                    }
                                },
                                WebSocketMessage::Unsubscribe { channels } => {
                                    subscribed_channels.retain(|c| !channels.contains(c));
                                    let response = WebSocketMessage::SystemMessage {
                                        message: format!("Unsubscribed from channels: {:?}", channels),
                                        level: "info".to_string(),
                                        timestamp: Utc::now(),
                                    };
                                    if let Ok(msg) = serde_json::to_string(&response) {
                                        let _ = sender.send(Message::Text(msg)).await;
                                    }
                                },
                                WebSocketMessage::Ping => {
                                    let pong = WebSocketMessage::Pong;
                                    if let Ok(msg) = serde_json::to_string(&pong) {
                                        let _ = sender.send(Message::Text(msg)).await;
                                    }
                                },
                                _ => {
                                    // Handle other message types if needed
                                }
                            }
                        }
                    },
                    Some(Ok(Message::Close(_))) => {
                        break;
                    },
                    Some(Err(_)) => {
                        break;
                    },
                    None => {
                        break;
                    },
                    _ => {}
                }
            },
            
            // Handle price updates
            Ok(msg) = price_receiver.recv() => {
                if subscribed_channels.contains(&"prices".to_string()) {
                    if let Ok(msg_str) = serde_json::to_string(&msg) {
                        let _ = sender.send(Message::Text(msg_str)).await;
                    }
                }
            },
            
            // Handle order book updates
            Ok(msg) = orderbook_receiver.recv() => {
                if subscribed_channels.contains(&"orderbook".to_string()) {
                    if let Ok(msg_str) = serde_json::to_string(&msg) {
                        let _ = sender.send(Message::Text(msg_str)).await;
                    }
                }
            },
            
            // Handle trade updates
            Ok(msg) = trade_receiver.recv() => {
                if subscribed_channels.contains(&"trades".to_string()) {
                    if let Ok(msg_str) = serde_json::to_string(&msg) {
                        let _ = sender.send(Message::Text(msg_str)).await;
                    }
                }
            },
            
            // Handle market data updates
            Ok(msg) = market_data_receiver.recv() => {
                if subscribed_channels.contains(&"market_data".to_string()) {
                    if let Ok(msg_str) = serde_json::to_string(&msg) {
                        let _ = sender.send(Message::Text(msg_str)).await;
                    }
                }
            },
            
            // Handle DAG updates
            Ok(msg) = dag_receiver.recv() => {
                if subscribed_channels.contains(&"dag".to_string()) {
                    if let Ok(msg_str) = serde_json::to_string(&msg) {
                        let _ = sender.send(Message::Text(msg_str)).await;
                    }
                }
            },
        }
    }
    
    // Clean up connection
    let mut connections = state.ws_manager.connections.lock().unwrap();
    connections.remove(&connection_id);
}

// Mock data generator for testing
pub async fn generate_mock_data(ws_manager: Arc<WebSocketManager>) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(1000));
    
    let symbols = vec!["BTC/USD", "ETH/USD", "SOL/USD", "ADA/USD", "DOT/USD"];
    let mut prices = HashMap::new();
    
    // Initialize prices
    for symbol in &symbols {
        prices.insert(*symbol, 50000.0 + fastrand::f64() * 100000.0);
    }
    
    loop {
        interval.tick().await;
        
        for symbol in &symbols {
            let current_price = prices.get_mut(symbol).unwrap();
            let change = (fastrand::f64() - 0.5) * 1000.0; // Random price change
            let new_price = (*current_price + change).max(100.0); // Ensure positive price
            let change_percent = (change / *current_price) * 100.0;
            
            // Update price
            *current_price = new_price;
            
            // Broadcast price update
            ws_manager.broadcast_price_update(symbol, new_price, change, change_percent);
            
            // Generate order book data
            let mut bids = Vec::new();
            let mut asks = Vec::new();
            
            // Generate realistic order book
            for i in 0..10 {
                let bid_price = new_price * (1.0 - (i as f64 + 1.0) * 0.001);
                let ask_price = new_price * (1.0 + (i as f64 + 1.0) * 0.001);
                
                bids.push(OrderBookEntry {
                    price: bid_price,
                    quantity: fastrand::f64() * 100.0,
                    order_count: fastrand::u32(1..50),
                });
                
                asks.push(OrderBookEntry {
                    price: ask_price,
                    quantity: fastrand::f64() * 100.0,
                    order_count: fastrand::u32(1..50),
                });
            }
            
            // Sort order book
            bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
            asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
            
            // Broadcast order book update
            ws_manager.broadcast_orderbook_update(symbol, bids, asks);
            
            // Generate trade updates (less frequent)
            if fastrand::u32(1..10) == 1 {
                let trade_price = new_price * (1.0 + (fastrand::f64() - 0.5) * 0.01);
                let trade_quantity = fastrand::f64() * 10.0;
                let trade_side = if fastrand::bool() { "buy" } else { "sell" };
                
                ws_manager.broadcast_trade_update(symbol, trade_price, trade_quantity, trade_side);
            }
            
            // Generate market data updates
            let volume_24h = fastrand::f64() * 1000000.0;
            let high_24h = new_price * (1.0 + fastrand::f64() * 0.1);
            let low_24h = new_price * (1.0 - fastrand::f64() * 0.1);
            let open_24h = new_price * (1.0 + (fastrand::f64() - 0.5) * 0.05);
            
            ws_manager.broadcast_market_data_update(symbol, volume_24h, high_24h, low_24h, open_24h);
        }
        
        // Generate DAG updates (less frequent)
        if fastrand::u32(1..20) == 1 {
            let new_transactions = vec![
                format!("0x{}", hex::encode(fastrand::u64(..).to_le_bytes())),
                format!("0x{}", hex::encode(fastrand::u64(..).to_le_bytes())),
            ];
            let new_blocks = vec![
                format!("0x{}", hex::encode(fastrand::u64(..).to_le_bytes())),
            ];
            
            ws_manager.broadcast_dag_update(new_transactions, new_blocks, "healthy");
        }
    }
} 

/// Spawns a background task to broadcast mock real-time data
pub fn spawn_realtime_mock_data(ws_manager: Arc<WebSocketManager>) {
    let symbols = vec!["BTC/USD", "ETH/USD", "SOL/USD", "USDT/USD"];
    tokio::spawn(async move {
        loop {
            for symbol in &symbols {
                // Mock price
                let price = 20000.0 + fastrand::f64() * 50000.0;
                let change = fastrand::f64() * 1000.0 - 500.0;
                let change_percent = change / price * 100.0;
                ws_manager.broadcast_price_update(symbol, price, change, change_percent);

                // Mock order book
                let mut bids = vec![];
                let mut asks = vec![];
                let base = price;
                for i in 0..10 {
                    bids.push(OrderBookEntry {
                        price: base - i as f64 * 10.0,
                        quantity: fastrand::f64() * 2.0,
                        order_count: fastrand::u32(1..10),
                    });
                    asks.push(OrderBookEntry {
                        price: base + i as f64 * 10.0,
                        quantity: fastrand::f64() * 2.0,
                        order_count: fastrand::u32(1..10),
                    });
                }
                ws_manager.broadcast_orderbook_update(symbol, bids, asks);

                // Mock trade
                let trade_price = price + fastrand::f64() * 100.0 - 50.0;
                let trade_qty = fastrand::f64() * 2.0;
                let side = if fastrand::bool() { "buy" } else { "sell" };
                ws_manager.broadcast_trade_update(symbol, trade_price, trade_qty, side);

                // Mock market data
                let volume_24h = fastrand::f64() * 10000.0;
                let high_24h = price + fastrand::f64() * 500.0;
                let low_24h = price - fastrand::f64() * 500.0;
                let open_24h = price - fastrand::f64() * 100.0;
                ws_manager.broadcast_market_data_update(symbol, volume_24h, high_24h, low_24h, open_24h);
            }
            sleep(Duration::from_secs(2)).await;
        }
    });
} 
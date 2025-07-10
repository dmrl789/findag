//! Trading type definitions
//! 
//! This module contains types related to trading operations, order management,
//! and market data.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use findag_core::{Address, Hash, HashTimer, FinDAGTime};

/// Order type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    /// Market order
    Market,
    /// Limit order
    Limit,
    /// Stop order
    Stop,
    /// Stop limit order
    StopLimit,
    /// Take profit order
    TakeProfit,
    /// Trailing stop order
    TrailingStop,
}

/// Order side
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}

/// Order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    /// Order is pending
    Pending,
    /// Order is active
    Active,
    /// Order is partially filled
    PartiallyFilled,
    /// Order is filled
    Filled,
    /// Order is cancelled
    Cancelled,
    /// Order is rejected
    Rejected,
    /// Order is expired
    Expired,
}

/// Order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Order ID
    pub order_id: String,
    /// Trading pair
    pub trading_pair: String,
    /// Order type
    pub order_type: OrderType,
    /// Order side
    pub side: OrderSide,
    /// Order quantity
    pub quantity: f64,
    /// Order price (for limit orders)
    pub price: Option<f64>,
    /// Stop price (for stop orders)
    pub stop_price: Option<f64>,
    /// Filled quantity
    pub filled_quantity: f64,
    /// Average fill price
    pub avg_fill_price: Option<f64>,
    /// Order status
    pub status: OrderStatus,
    /// Order timestamp
    pub timestamp: FinDAGTime,
    /// Order expiry (optional)
    pub expiry: Option<FinDAGTime>,
    /// Order metadata
    pub metadata: Option<String>,
}

/// Trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Trade ID
    pub trade_id: String,
    /// Trading pair
    pub trading_pair: String,
    /// Trade price
    pub price: f64,
    /// Trade quantity
    pub quantity: f64,
    /// Trade side
    pub side: OrderSide,
    /// Trade timestamp
    pub timestamp: FinDAGTime,
    /// Maker order ID
    pub maker_order_id: String,
    /// Taker order ID
    pub taker_order_id: String,
    /// Trade fee
    pub fee: f64,
    /// Trade fee asset
    pub fee_asset: String,
}

/// Trading pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingPair {
    /// Base asset
    pub base_asset: String,
    /// Quote asset
    pub quote_asset: String,
    /// Trading pair symbol
    pub symbol: String,
    /// Minimum order size
    pub min_order_size: f64,
    /// Maximum order size
    pub max_order_size: f64,
    /// Price precision
    pub price_precision: u32,
    /// Quantity precision
    pub quantity_precision: u32,
    /// Trading pair status
    pub status: TradingPairStatus,
    /// Trading pair metadata
    pub metadata: Option<String>,
}

/// Trading pair status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradingPairStatus {
    /// Active
    Active,
    /// Inactive
    Inactive,
    /// Suspended
    Suspended,
}

/// Market data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    /// Trading pair
    pub trading_pair: String,
    /// Last price
    pub last_price: f64,
    /// Bid price
    pub bid_price: f64,
    /// Ask price
    pub ask_price: f64,
    /// 24h high
    pub high_24h: f64,
    /// 24h low
    pub low_24h: f64,
    /// 24h volume
    pub volume_24h: f64,
    /// 24h change
    pub change_24h: f64,
    /// 24h change percent
    pub change_24h_percent: f64,
    /// Timestamp
    pub timestamp: FinDAGTime,
}

/// Order book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookEntry {
    /// Price
    pub price: f64,
    /// Quantity
    pub quantity: f64,
    /// Order count
    pub order_count: u32,
}

/// Order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    /// Trading pair
    pub trading_pair: String,
    /// Bids (buy orders)
    pub bids: Vec<OrderBookEntry>,
    /// Asks (sell orders)
    pub asks: Vec<OrderBookEntry>,
    /// Timestamp
    pub timestamp: FinDAGTime,
}

/// Portfolio position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Asset
    pub asset: String,
    /// Quantity
    pub quantity: f64,
    /// Average price
    pub avg_price: f64,
    /// Unrealized P&L
    pub unrealized_pnl: f64,
    /// Realized P&L
    pub realized_pnl: f64,
    /// Position value
    pub value: f64,
    /// Last update timestamp
    pub last_update: DateTime<Utc>,
}

/// Portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    /// Owner address
    pub owner: Address,
    /// Total value
    pub total_value: f64,
    /// Total P&L
    pub total_pnl: f64,
    /// Positions
    pub positions: Vec<Position>,
    /// Last update timestamp
    pub last_update: DateTime<Utc>,
}

/// Trading metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingMetrics {
    /// Total trades
    pub total_trades: u64,
    /// Total volume
    pub total_volume: f64,
    /// Total fees paid
    pub total_fees: f64,
    /// Win rate
    pub win_rate: f64,
    /// Average trade size
    pub avg_trade_size: f64,
    /// Best trade
    pub best_trade: f64,
    /// Worst trade
    pub worst_trade: f64,
    /// Trading uptime in seconds
    pub uptime_seconds: u64,
}

/// Trading event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradingEvent {
    /// Order placed
    OrderPlaced {
        order_id: String,
        trading_pair: String,
        side: OrderSide,
        quantity: f64,
        price: Option<f64>,
        timestamp: FinDAGTime,
    },
    /// Order filled
    OrderFilled {
        order_id: String,
        trading_pair: String,
        quantity: f64,
        price: f64,
        timestamp: FinDAGTime,
    },
    /// Order cancelled
    OrderCancelled {
        order_id: String,
        reason: String,
        timestamp: FinDAGTime,
    },
    /// Trade executed
    TradeExecuted {
        trade_id: String,
        trading_pair: String,
        price: f64,
        quantity: f64,
        side: OrderSide,
        timestamp: FinDAGTime,
    },
    /// Market data updated
    MarketDataUpdated {
        trading_pair: String,
        last_price: f64,
        timestamp: FinDAGTime,
    },
}

/// Trading command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradingCommand {
    /// Place order
    PlaceOrder(Order),
    /// Cancel order
    CancelOrder(String),
    /// Get order book
    GetOrderBook(String),
    /// Get market data
    GetMarketData(String),
    /// Get portfolio
    GetPortfolio(Address),
    /// Get trading history
    GetTradingHistory(Address),
} 
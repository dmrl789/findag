//! API request and response models
//! 
//! This module contains all the request and response models for the FinDAG API.

use findag_types::{Block, Round, Transaction, Asset, Balance, Wallet, Validator, GovernanceProposal};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

// Common response types

/// Error response
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error type
    pub error: String,
    /// Error message
    pub message: String,
    /// Error code
    pub code: String,
}

/// Success response
#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse {
    /// Success message
    pub message: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

// Blockchain responses

/// Blocks response
#[derive(Debug, Serialize, Deserialize)]
pub struct BlocksResponse {
    /// Blocks
    pub blocks: Vec<Block>,
    /// Total count
    pub total: u64,
    /// Limit
    pub limit: u64,
    /// Offset
    pub offset: u64,
}

/// Block response
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResponse {
    /// Block
    pub block: Block,
}

/// Rounds response
#[derive(Debug, Serialize, Deserialize)]
pub struct RoundsResponse {
    /// Rounds
    pub rounds: Vec<Round>,
    /// Total count
    pub total: u64,
    /// Limit
    pub limit: u64,
    /// Offset
    pub offset: u64,
}

/// Round response
#[derive(Debug, Serialize, Deserialize)]
pub struct RoundResponse {
    /// Round
    pub round: Round,
}

/// Transactions response
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsResponse {
    /// Transactions
    pub transactions: Vec<Transaction>,
    /// Total count
    pub total: u64,
    /// Limit
    pub limit: u64,
    /// Offset
    pub offset: u64,
}

/// Transaction response
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    /// Transaction
    pub transaction: Transaction,
}

/// Submit transaction request
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitTransactionRequest {
    /// From address
    pub from: String,
    /// To address
    pub to: String,
    /// Amount
    pub amount: u64,
    /// Asset
    pub asset: String,
    /// Fee
    pub fee: u64,
    /// Nonce
    pub nonce: u64,
    /// Signature
    pub signature: String,
}

/// Submit transaction response
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitTransactionResponse {
    /// Transaction hash
    pub transaction_hash: String,
    /// Status
    pub status: String,
    /// Message
    pub message: String,
}

// DAG responses

/// DAG status response
#[derive(Debug, Serialize, Deserialize)]
pub struct DAGStatusResponse {
    /// Total blocks
    pub total_blocks: u64,
    /// Total transactions
    pub total_transactions: u64,
    /// Latest block hash
    pub latest_block_hash: String,
    /// Latest round number
    pub latest_round_number: u64,
    /// DAG height
    pub dag_height: u64,
    /// DAG width
    pub dag_width: u64,
}

/// DAG blocks response
#[derive(Debug, Serialize, Deserialize)]
pub struct DAGBlocksResponse {
    /// Blocks
    pub blocks: Vec<Block>,
    /// Total count
    pub total: u64,
}

/// DAG transactions response
#[derive(Debug, Serialize, Deserialize)]
pub struct DAGTransactionsResponse {
    /// Transactions
    pub transactions: Vec<Transaction>,
    /// Total count
    pub total: u64,
}

// Trading responses

/// Trading pair
#[derive(Debug, Serialize, Deserialize)]
pub struct TradingPair {
    /// Base asset
    pub base: String,
    /// Quote asset
    pub quote: String,
    /// Current price
    pub price: f64,
    /// 24h change
    pub change_24h: f64,
    /// 24h volume
    pub volume_24h: f64,
}

/// Trading pairs response
#[derive(Debug, Serialize, Deserialize)]
pub struct TradingPairsResponse {
    /// Trading pairs
    pub pairs: Vec<TradingPair>,
}

/// Market data
#[derive(Debug, Serialize, Deserialize)]
pub struct MarketData {
    /// Asset
    pub asset: String,
    /// Price
    pub price: f64,
    /// Volume
    pub volume: f64,
    /// Change 24h
    pub change_24h: f64,
    /// High 24h
    pub high_24h: f64,
    /// Low 24h
    pub low_24h: f64,
}

/// Market data response
#[derive(Debug, Serialize, Deserialize)]
pub struct MarketDataResponse {
    /// Market data
    pub market_data: Vec<MarketData>,
}

/// Order
#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    /// Order ID
    pub id: String,
    /// Trading pair
    pub pair: String,
    /// Order type
    pub order_type: String,
    /// Side
    pub side: String,
    /// Amount
    pub amount: f64,
    /// Price
    pub price: f64,
    /// Status
    pub status: String,
    /// Created at
    pub created_at: DateTime<Utc>,
}

/// Orders response
#[derive(Debug, Serialize, Deserialize)]
pub struct OrdersResponse {
    /// Orders
    pub orders: Vec<Order>,
}

/// Place order request
#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    /// Trading pair
    pub pair: String,
    /// Order type
    pub order_type: String,
    /// Side
    pub side: String,
    /// Amount
    pub amount: f64,
    /// Price
    pub price: f64,
}

/// Place order response
#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceOrderResponse {
    /// Order ID
    pub order_id: String,
    /// Status
    pub status: String,
    /// Message
    pub message: String,
}

/// Cancel order response
#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrderResponse {
    /// Order ID
    pub order_id: String,
    /// Status
    pub status: String,
    /// Message
    pub message: String,
}

/// Position
#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    /// Asset
    pub asset: String,
    /// Amount
    pub amount: f64,
    /// Value
    pub value: f64,
    /// PnL
    pub pnl: f64,
}

/// Portfolio
#[derive(Debug, Serialize, Deserialize)]
pub struct Portfolio {
    /// Total value
    pub total_value: f64,
    /// Positions
    pub positions: Vec<Position>,
    /// PnL
    pub pnl: f64,
}

/// Portfolio response
#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioResponse {
    /// Portfolio
    pub portfolio: Portfolio,
}

/// Trading history
#[derive(Debug, Serialize, Deserialize)]
pub struct TradingHistory {
    /// Transaction hash
    pub transaction_hash: String,
    /// Trading pair
    pub pair: String,
    /// Side
    pub side: String,
    /// Amount
    pub amount: f64,
    /// Price
    pub price: f64,
    /// Fee
    pub fee: f64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Trading history response
#[derive(Debug, Serialize, Deserialize)]
pub struct TradingHistoryResponse {
    /// Trading history
    pub history: Vec<TradingHistory>,
}

// Wallet responses

/// Wallet balance
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletBalance {
    /// Asset
    pub asset: String,
    /// Balance
    pub balance: u64,
    /// Available balance
    pub available_balance: u64,
    /// Locked balance
    pub locked_balance: u64,
}

/// Wallet balance response
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletBalanceResponse {
    /// Balances
    pub balances: Vec<WalletBalance>,
}

/// Wallet transactions response
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionsResponse {
    /// Transactions
    pub transactions: Vec<Transaction>,
}

/// Send transaction request
#[derive(Debug, Serialize, Deserialize)]
pub struct SendTransactionRequest {
    /// To address
    pub to: String,
    /// Amount
    pub amount: u64,
    /// Asset
    pub asset: String,
    /// Fee
    pub fee: u64,
}

/// Send transaction response
#[derive(Debug, Serialize, Deserialize)]
pub struct SendTransactionResponse {
    /// Transaction hash
    pub transaction_hash: String,
    /// Status
    pub status: String,
    /// Message
    pub message: String,
}

/// Wallet addresses response
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletAddressesResponse {
    /// Addresses
    pub addresses: Vec<String>,
}

/// Create wallet request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    /// Wallet name
    pub name: String,
    /// Password
    pub password: String,
}

/// Create wallet response
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWalletResponse {
    /// Wallet ID
    pub wallet_id: String,
    /// Address
    pub address: String,
    /// Message
    pub message: String,
}

/// Import wallet request
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportWalletRequest {
    /// Wallet data
    pub wallet_data: String,
    /// Password
    pub password: String,
}

/// Import wallet response
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportWalletResponse {
    /// Wallet ID
    pub wallet_id: String,
    /// Message
    pub message: String,
}

/// Export wallet request
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportWalletRequest {
    /// Wallet ID
    pub wallet_id: String,
    /// Password
    pub password: String,
}

/// Export wallet response
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportWalletResponse {
    /// Export data
    pub export_data: String,
    /// Message
    pub message: String,
}

// Network responses

/// Network status response
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStatusResponse {
    /// Connected peers
    pub connected_peers: u64,
    /// Total peers
    pub total_peers: u64,
    /// Network height
    pub network_height: u64,
    /// Sync status
    pub sync_status: String,
}

/// Peer
#[derive(Debug, Serialize, Deserialize)]
pub struct Peer {
    /// Peer ID
    pub id: String,
    /// Address
    pub address: String,
    /// Status
    pub status: String,
    /// Last seen
    pub last_seen: DateTime<Utc>,
}

/// Peers response
#[derive(Debug, Serialize, Deserialize)]
pub struct PeersResponse {
    /// Peers
    pub peers: Vec<Peer>,
}

/// Add peer request
#[derive(Debug, Serialize, Deserialize)]
pub struct AddPeerRequest {
    /// Peer address
    pub address: String,
}

/// Add peer response
#[derive(Debug, Serialize, Deserialize)]
pub struct AddPeerResponse {
    /// Message
    pub message: String,
}

/// Remove peer response
#[derive(Debug, Serialize, Deserialize)]
pub struct RemovePeerResponse {
    /// Message
    pub message: String,
}

/// Network node
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkNode {
    /// Node ID
    pub id: String,
    /// Address
    pub address: String,
    /// Status
    pub status: String,
}

/// Network connection
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConnection {
    /// From node
    pub from: String,
    /// To node
    pub to: String,
    /// Status
    pub status: String,
}

/// Network topology
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkTopology {
    /// Nodes
    pub nodes: Vec<NetworkNode>,
    /// Connections
    pub connections: Vec<NetworkConnection>,
}

/// Network topology response
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkTopologyResponse {
    /// Network topology
    pub topology: NetworkTopology,
}

/// Network bandwidth response
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkBandwidthResponse {
    /// Bytes sent
    pub bytes_sent: u64,
    /// Bytes received
    pub bytes_received: u64,
    /// Packets sent
    pub packets_sent: u64,
    /// Packets received
    pub packets_received: u64,
}

// Admin responses

/// Validators response
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidatorsResponse {
    /// Validators
    pub validators: Vec<Validator>,
}

/// Add validator request
#[derive(Debug, Serialize, Deserialize)]
pub struct AddValidatorRequest {
    /// Validator address
    pub address: String,
    /// Public key
    pub public_key: String,
    /// Metadata
    pub metadata: String,
}

/// Add validator response
#[derive(Debug, Serialize, Deserialize)]
pub struct AddValidatorResponse {
    /// Message
    pub message: String,
}

/// Remove validator response
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveValidatorResponse {
    /// Message
    pub message: String,
}

/// Governance
#[derive(Debug, Serialize, Deserialize)]
pub struct Governance {
    /// Proposals
    pub proposals: Vec<GovernanceProposal>,
    /// Votes
    pub votes: Vec<String>,
}

/// Governance response
#[derive(Debug, Serialize, Deserialize)]
pub struct GovernanceResponse {
    /// Governance
    pub governance: Governance,
}

/// Submit proposal request
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitProposalRequest {
    /// Proposal type
    pub proposal_type: String,
    /// Proposal data
    pub proposal_data: String,
}

/// Submit proposal response
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitProposalResponse {
    /// Proposal ID
    pub proposal_id: String,
    /// Message
    pub message: String,
}

/// System status response
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatusResponse {
    /// Uptime
    pub uptime: u64,
    /// Memory usage
    pub memory_usage: u64,
    /// CPU usage
    pub cpu_usage: f64,
    /// Disk usage
    pub disk_usage: u64,
    /// Network usage
    pub network_usage: u64,
}

/// System config
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemConfig {
    /// Node ID
    pub node_id: String,
    /// Network ID
    pub network_id: String,
    /// Version
    pub version: String,
}

/// System config response
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemConfigResponse {
    /// System config
    pub config: SystemConfig,
}

/// Update system config request
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSystemConfigRequest {
    /// Config data
    pub config_data: String,
}

/// Update system config response
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSystemConfigResponse {
    /// Message
    pub message: String,
}

// Metrics responses

/// Metrics
#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    /// Total requests
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
}

/// Metrics response
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsResponse {
    /// Metrics
    pub metrics: Metrics,
}

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    /// Status
    pub status: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Status response
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    /// Status
    pub status: String,
    /// Version
    pub version: String,
    /// Uptime
    pub uptime: u64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
} 
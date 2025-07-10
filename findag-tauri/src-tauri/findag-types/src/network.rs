//! Network type definitions
//! 
//! This module contains types related to P2P networking, peer management,
//! and network communication.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use findag_core::{Address, Hash, HashTimer, FinDAGTime};

/// Network message type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkMessageType {
    /// Transaction message
    Transaction,
    /// Block message
    Block,
    /// Round message
    Round,
    /// Vote message
    Vote,
    /// Peer discovery
    PeerDiscovery,
    /// Handshake
    Handshake,
    /// Ping/Pong
    PingPong,
    /// Error message
    Error,
}

/// Network message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    /// Message type
    pub message_type: NetworkMessageType,
    /// Message ID
    pub message_id: String,
    /// Source peer ID
    pub source_peer: String,
    /// Target peer ID (optional for broadcast)
    pub target_peer: Option<String>,
    /// Message payload
    pub payload: Vec<u8>,
    /// Message timestamp
    pub timestamp: FinDAGTime,
    /// Message signature
    pub signature: Option<Vec<u8>>,
}

/// Peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    /// Peer ID
    pub peer_id: String,
    /// Peer address
    pub address: String,
    /// Peer port
    pub port: u16,
    /// Peer public key
    pub public_key: Vec<u8>,
    /// Peer version
    pub version: String,
    /// Connection status
    pub status: PeerStatus,
    /// Last seen timestamp
    pub last_seen: DateTime<Utc>,
    /// Latency in milliseconds
    pub latency_ms: Option<u64>,
    /// Peer metadata
    pub metadata: Option<String>,
}

/// Peer status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerStatus {
    /// Connected
    Connected,
    /// Connecting
    Connecting,
    /// Disconnected
    Disconnected,
    /// Failed
    Failed,
    /// Banned
    Banned,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Local port
    pub port: u16,
    /// Maximum peers
    pub max_peers: usize,
    /// Connection timeout in milliseconds
    pub connection_timeout_ms: u64,
    /// Message timeout in milliseconds
    pub message_timeout_ms: u64,
    /// Enable discovery
    pub enable_discovery: bool,
    /// Discovery interval in milliseconds
    pub discovery_interval_ms: u64,
    /// Enable metrics
    pub enable_metrics: bool,
    /// Bootstrap peers
    pub bootstrap_peers: Vec<String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            max_peers: 50,
            connection_timeout_ms: 5000,
            message_timeout_ms: 10000,
            enable_discovery: true,
            discovery_interval_ms: 30000,
            enable_metrics: true,
            bootstrap_peers: vec![],
        }
    }
}

/// Network metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Connected peers count
    pub connected_peers: usize,
    /// Total peers discovered
    pub total_peers_discovered: usize,
    /// Messages sent
    pub messages_sent: u64,
    /// Messages received
    pub messages_received: u64,
    /// Bytes sent
    pub bytes_sent: u64,
    /// Bytes received
    pub bytes_received: u64,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// Network uptime in seconds
    pub uptime_seconds: u64,
}

/// Handshake message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeMessage {
    /// Node ID
    pub node_id: String,
    /// Node version
    pub version: String,
    /// Node capabilities
    pub capabilities: Vec<String>,
    /// Node public key
    pub public_key: Vec<u8>,
    /// Timestamp
    pub timestamp: FinDAGTime,
}

/// Ping/Pong message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingPongMessage {
    /// Message ID
    pub message_id: String,
    /// Timestamp
    pub timestamp: FinDAGTime,
    /// Payload (for ping)
    pub payload: Option<Vec<u8>>,
}

/// Peer discovery message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerDiscoveryMessage {
    /// Known peers
    pub known_peers: Vec<PeerInfo>,
    /// Timestamp
    pub timestamp: FinDAGTime,
}

/// Network event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkEvent {
    /// Peer connected
    PeerConnected {
        peer_id: String,
        address: String,
        timestamp: FinDAGTime,
    },
    /// Peer disconnected
    PeerDisconnected {
        peer_id: String,
        reason: String,
        timestamp: FinDAGTime,
    },
    /// Message received
    MessageReceived {
        peer_id: String,
        message_type: NetworkMessageType,
        message_id: String,
        timestamp: FinDAGTime,
    },
    /// Message sent
    MessageSent {
        peer_id: String,
        message_type: NetworkMessageType,
        message_id: String,
        timestamp: FinDAGTime,
    },
    /// Network error
    NetworkError {
        error: String,
        timestamp: FinDAGTime,
    },
}

/// Network command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkCommand {
    /// Start network
    Start,
    /// Stop network
    Stop,
    /// Connect to peer
    ConnectPeer(String),
    /// Disconnect from peer
    DisconnectPeer(String),
    /// Send message
    SendMessage(NetworkMessage),
    /// Update configuration
    UpdateConfig(NetworkConfig),
    /// Ban peer
    BanPeer(String, String), // peer_id, reason
    /// Unban peer
    UnbanPeer(String),
} 
use crate::storage::types::PeerId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    AssetBroadcast,
    AssetRequest,
    AssetResponse,
    PeerDiscovery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub sender: PeerId,
    pub recipient: PeerId,
    pub message_type: MessageType,
    pub payload: String,
    pub timestamp: DateTime<Utc>,
} 
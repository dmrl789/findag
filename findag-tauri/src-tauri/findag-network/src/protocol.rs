//! Custom protocols
//! 
//! This module implements custom protocols for FinDAG networking.

use libp2p::{
    core::{upgrade, InboundUpgrade, OutboundUpgrade},
    swarm::StreamProtocol,
};
use findag_types::{NetworkMessage, FindDAGResult};

/// Custom protocol for FinDAG
pub struct FinDAGProtocol;

impl FinDAGProtocol {
    /// Protocol name
    pub const PROTOCOL_NAME: &'static str = "/findag/1.0.0";
    
    /// Create protocol upgrade
    pub fn upgrade() -> upgrade::VersionedUpgrade<upgrade::Ready<Vec<u8>>, StreamProtocol> {
        upgrade::VersionedUpgrade::new(
            StreamProtocol::new(Self::PROTOCOL_NAME),
            upgrade::Ready::new(vec![]),
        )
    }
}

/// Protocol message
#[derive(Debug, Clone)]
pub struct ProtocolMessage {
    /// Message type
    pub message_type: String,
    /// Message payload
    pub payload: Vec<u8>,
    /// Message timestamp
    pub timestamp: u64,
}

impl ProtocolMessage {
    /// Create a new protocol message
    pub fn new(message_type: String, payload: Vec<u8>) -> Self {
        Self {
            message_type,
            payload,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Serialize message
    pub fn serialize(&self) -> FindDAGResult<Vec<u8>> {
        Ok(bincode::serialize(self)?)
    }

    /// Deserialize message
    pub fn deserialize(data: &[u8]) -> FindDAGResult<Self> {
        Ok(bincode::deserialize(data)?)
    }
}

/// Protocol handler
pub struct ProtocolHandler {
    /// Supported protocols
    protocols: std::collections::HashMap<String, Box<dyn ProtocolHandlerTrait>>,
}

/// Protocol handler trait
pub trait ProtocolHandlerTrait: Send + Sync {
    /// Handle incoming message
    fn handle_message(&self, message: ProtocolMessage) -> FindDAGResult<()>;
    
    /// Get protocol name
    fn protocol_name(&self) -> &str;
}

impl ProtocolHandler {
    /// Create a new protocol handler
    pub fn new() -> Self {
        Self {
            protocols: std::collections::HashMap::new(),
        }
    }

    /// Register a protocol handler
    pub fn register_handler(&mut self, handler: Box<dyn ProtocolHandlerTrait>) {
        let protocol_name = handler.protocol_name().to_string();
        self.protocols.insert(protocol_name, handler);
    }

    /// Handle incoming message
    pub fn handle_message(&self, protocol_name: &str, message: ProtocolMessage) -> FindDAGResult<()> {
        if let Some(handler) = self.protocols.get(protocol_name) {
            handler.handle_message(message)?;
        }
        Ok(())
    }
} 
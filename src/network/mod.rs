use std::sync::Arc;
use tokio::sync::Mutex;
use crate::storage::types::PeerId;

pub mod discovery;
pub mod message;
pub mod peer;
pub mod transport;

pub struct Network {
    discovery: Arc<discovery::DiscoveryManager>,
    transport: Arc<transport::TransportManager>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            discovery: Arc::new(discovery::DiscoveryManager::new()),
            transport: Arc::new(transport::TransportManager::new()),
        }
    }

    pub async fn add_peer(&self, peer_id: PeerId, address: String) -> Result<(), String> {
        self.discovery.add_peer(peer_id.clone(), address.clone()).await?;
        self.transport.connect(peer_id, address).await
    }

    pub async fn remove_peer(&self, peer_id: &PeerId) -> Result<(), String> {
        self.discovery.remove_peer(peer_id).await?;
        self.transport.disconnect(peer_id).await
    }

    pub async fn get_peers(&self) -> Vec<PeerId> {
        self.discovery.get_peers().await
    }

    pub async fn is_peer_connected(&self, peer_id: &PeerId) -> bool {
        self.transport.is_connected(peer_id).await
    }
}

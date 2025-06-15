use libp2p::PeerId;
use libp2p::swarm::Swarm;
use crate::network::{Network, NetworkEvent};
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::StreamExt;
use tracing::{info, warn, error, debug};
use crate::network::error::{NetworkError, NetworkResult};
use crate::network::config::NetworkConfig;
use crate::network::connection::ConnectionInfo;

pub struct P2PNode {
    peer_id: PeerId,
    network: Arc<Mutex<Network>>,
    event_sender: tokio::sync::mpsc::Sender<NetworkEvent>,
}

impl P2PNode {
    pub async fn new(config: NetworkConfig) -> NetworkResult<Self> {
        info!("Creating new P2P node");
        let network = Network::new(config)?;
        let peer_id = network.swarm.lock().await.local_peer_id().clone();
        info!("Node created with peer ID: {}", peer_id);
        
        let (event_sender, mut event_receiver) = tokio::sync::mpsc::channel(100);
        let network = Arc::new(Mutex::new(network));
        let network_clone = network.clone();

        tokio::spawn(async move {
            while let Some(event) = event_receiver.recv().await {
                let mut network = network_clone.lock().await;
                if let Err(e) = network.handle_swarm_event(event).await {
                    error!("Error handling network event: {}", e);
                }
            }
        });

        Ok(Self {
            peer_id,
            network,
            event_sender,
        })
    }

    pub async fn start(&self) -> NetworkResult<()> {
        info!("Starting P2P node");
        let network = self.network.lock().await;
        network.start().await?;

        let swarm = network.swarm.clone();
        let event_sender = self.event_sender.clone();

        tokio::spawn(async move {
            let mut swarm = swarm.lock().await;
            while let Some(event) = swarm.next().await {
                if let Err(e) = event_sender.send(event).await {
                    error!("Error sending event: {}", e);
                    break;
                }
            }
        });

        info!("P2P node started successfully");
        Ok(())
    }

    pub async fn stop(&self) -> NetworkResult<()> {
        info!("Stopping P2P node");
        // Cleanup can be added here if needed
        Ok(())
    }

    pub async fn send_message(&self, topic: &str, message: &[u8]) -> NetworkResult<()> {
        debug!("Sending message to topic: {}", topic);
        let network = self.network.lock().await;
        network.get_protocol().lock().await.broadcast_message(topic, message).await
    }

    pub async fn subscribe(&self, topic: &str) -> NetworkResult<()> {
        debug!("Subscribing to topic: {}", topic);
        let network = self.network.lock().await;
        network.get_protocol().lock().await.subscribe_to_protocol(topic).await
    }

    pub async fn unsubscribe(&self, topic: &str) -> NetworkResult<()> {
        debug!("Unsubscribing from topic: {}", topic);
        let network = self.network.lock().await;
        network.get_protocol().lock().await.unsubscribe_from_protocol(topic).await
    }

    pub async fn get_peers(&self) -> NetworkResult<Vec<PeerId>> {
        debug!("Getting connected peers");
        let network = self.network.lock().await;
        network.get_connection_manager().get_active_connections().await
    }

    pub async fn connect_to_peer(&self, addr: &str) -> NetworkResult<()> {
        debug!("Connecting to peer at address: {}", addr);
        let network = self.network.lock().await;
        let addr = addr.parse().map_err(|e| NetworkError::InvalidAddress(e.to_string()))?;
        network.get_discovery().lock().await.connect_to_peer(addr).await
    }

    pub fn get_peer_id(&self) -> PeerId {
        self.peer_id
    }

    pub async fn get_peer_info(&self, peer_id: PeerId) -> Option<ConnectionInfo> {
        let network = self.network.lock().await;
        network.get_connection_manager().get_connection_info(peer_id).await
    }

    pub async fn get_connection_stats(&self) -> NetworkResult<()> {
        let network = self.network.lock().await;
        let active_connections = network.get_connection_manager().get_active_connections().await;
        info!("Active connections: {}", active_connections.len());
        for peer_id in active_connections {
            if let Some(info) = network.get_connection_manager().get_connection_info(peer_id).await {
                info!("Peer {}: connected for {:?}, last activity {:?} ago",
                    peer_id,
                    info.connected_at.elapsed(),
                    info.last_activity.elapsed()
                );
            }
        }
        Ok(())
    }
} 
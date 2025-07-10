//! FinDAG P2P networking with libp2p
//! 
//! This crate implements P2P networking for FinDAG using libp2p,
//! providing peer discovery, message propagation, and network management.

pub mod swarm;
pub mod discovery;
pub mod gossipsub;
pub mod kad;
pub mod transport;
pub mod protocol;
pub mod metrics;

pub use swarm::*;
pub use discovery::*;
pub use gossipsub::*;
pub use kad::*;
pub use transport::*;
pub use protocol::*;
pub use metrics::*;

use findag_core::{Address, Hash, HashTimer, FinDAGTime};
use findag_types::{
    NetworkMessage, NetworkMessageType, PeerInfo, PeerStatus, NetworkConfig,
    NetworkMetrics, HandshakeMessage, PingPongMessage, PeerDiscoveryMessage,
    NetworkEvent, NetworkCommand, FindDAGResult, FindDAGError,
};

use libp2p::{
    core::{upgrade, transport, PeerId},
    gossipsub::{self, MessageId, ValidationMode},
    kad::{Kademlia, KademliaEvent, QueryResult},
    mdns::{Mdns, MdnsEvent},
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    tcp, noise, yamux, mplex,
    identity, multiaddr,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tracing::{info, warn, error, debug};
use metrics::{counter, gauge, histogram};

/// Network behaviour combining all protocols
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "NetworkEvent")]
pub struct FinDAGBehaviour {
    /// GossipSub for message propagation
    gossipsub: gossipsub::Behaviour,
    /// Kademlia for peer discovery
    kad: Kademlia,
    /// mDNS for local peer discovery
    mdns: Mdns,
}

/// Network manager
pub struct NetworkManager {
    /// libp2p swarm
    swarm: Swarm<FinDAGBehaviour>,
    /// Network configuration
    config: NetworkConfig,
    /// Connected peers
    peers: Arc<RwLock<HashMap<PeerId, PeerInfo>>>,
    /// Event sender
    event_sender: mpsc::Sender<NetworkEvent>,
    /// Command receiver
    command_receiver: mpsc::Receiver<NetworkCommand>,
    /// Metrics
    metrics: NetworkMetrics,
    /// Local peer ID
    local_peer_id: PeerId,
}

impl NetworkManager {
    /// Create a new network manager
    pub fn new(
        config: NetworkConfig,
        event_sender: mpsc::Sender<NetworkEvent>,
        command_receiver: mpsc::Receiver<NetworkCommand>,
    ) -> FindDAGResult<Self> {
        // Generate local identity
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        // Create transport
        let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(&local_key)
            .expect("Signing libp2p-noise static DH keypair failed.");
        
        let transport = transport::TokioTcpConfig::new()
            .nodelay(true)
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseAuthenticated::xx(noise_keys).unwrap())
            .multiplex(yamux::YamuxConfig::default())
            .boxed();
        
        // Create GossipSub
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(std::time::Duration::from_secs(1))
            .validation_mode(ValidationMode::Strict)
            .message_id_fn(|msg| {
                let mut s = DefaultHasher::new();
                s.write(msg.data.as_ref());
                MessageId(s.finish())
            })
            .build()
            .expect("Valid config");
        
        let mut gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )?;
        
        // Subscribe to topics
        gossipsub.subscribe(&gossipsub::IdentTopic::new("findag-transactions"))?;
        gossipsub.subscribe(&gossipsub::IdentTopic::new("findag-blocks"))?;
        gossipsub.subscribe(&gossipsub::IdentTopic::new("findag-rounds"))?;
        
        // Create Kademlia
        let kad_config = kad::Config::default();
        let kad = Kademlia::new(local_peer_id, kad_config);
        
        // Create mDNS
        let mdns = Mdns::new(Default::default())?;
        
        // Create behaviour
        let behaviour = FinDAGBehaviour {
            gossipsub,
            kad,
            mdns,
        };
        
        // Create swarm
        let swarm = Swarm::new(transport, behaviour, local_peer_id);
        
        let peers = Arc::new(RwLock::new(HashMap::new()));
        let metrics = NetworkMetrics::default();
        
        Ok(Self {
            swarm,
            config,
            peers,
            event_sender,
            command_receiver,
            metrics,
            local_peer_id,
        })
    }

    /// Start the network manager
    pub async fn start(&mut self) -> FindDAGResult<()> {
        info!("Starting network manager");
        
        // Listen on configured address
        let addr = format!("/ip4/0.0.0.0/tcp/{}", self.config.port);
        let addr: multiaddr::Multiaddr = addr.parse()?;
        self.swarm.listen_on(addr)?;
        
        // Initialize metrics
        self.initialize_metrics();
        
        // Start network loop
        self.network_loop().await?;
        
        Ok(())
    }

    /// Stop the network manager
    pub async fn stop(&mut self) -> FindDAGResult<()> {
        info!("Stopping network manager");
        Ok(())
    }

    /// Get network metrics
    pub async fn get_metrics(&self) -> NetworkMetrics {
        self.metrics.clone()
    }

    /// Get connected peers
    pub async fn get_peers(&self) -> Vec<PeerInfo> {
        let peers = self.peers.read().await;
        peers.values().cloned().collect()
    }

    /// Send a message to a specific peer
    pub async fn send_message(&mut self, peer_id: PeerId, message: NetworkMessage) -> FindDAGResult<()> {
        let topic = self.get_topic_for_message(&message.message_type);
        let data = bincode::serialize(&message)?;
        
        self.swarm.behaviour_mut().gossipsub.publish(topic, data)?;
        
        counter!("findag_network_messages_sent", 1);
        self.metrics.messages_sent += 1;
        
        Ok(())
    }

    /// Broadcast a message to all peers
    pub async fn broadcast_message(&mut self, message: NetworkMessage) -> FindDAGResult<()> {
        let topic = self.get_topic_for_message(&message.message_type);
        let data = bincode::serialize(&message)?;
        
        self.swarm.behaviour_mut().gossipsub.publish(topic, data)?;
        
        counter!("findag_network_messages_broadcast", 1);
        self.metrics.messages_sent += 1;
        
        Ok(())
    }

    /// Connect to a peer
    pub async fn connect_peer(&mut self, addr: &str) -> FindDAGResult<()> {
        let addr: multiaddr::Multiaddr = addr.parse()?;
        self.swarm.dial(addr)?;
        
        info!("Connecting to peer: {}", addr);
        Ok(())
    }

    /// Disconnect from a peer
    pub async fn disconnect_peer(&mut self, peer_id: &PeerId) -> FindDAGResult<()> {
        self.swarm.disconnect_peer_id(*peer_id);
        
        let mut peers = self.peers.write().await;
        peers.remove(peer_id);
        
        info!("Disconnected from peer: {}", peer_id);
        Ok(())
    }

    /// Get topic for message type
    fn get_topic_for_message(&self, message_type: &NetworkMessageType) -> gossipsub::IdentTopic {
        match message_type {
            NetworkMessageType::Transaction => gossipsub::IdentTopic::new("findag-transactions"),
            NetworkMessageType::Block => gossipsub::IdentTopic::new("findag-blocks"),
            NetworkMessageType::Round => gossipsub::IdentTopic::new("findag-rounds"),
            NetworkMessageType::Vote => gossipsub::IdentTopic::new("findag-votes"),
            NetworkMessageType::PeerDiscovery => gossipsub::IdentTopic::new("findag-discovery"),
            NetworkMessageType::Handshake => gossipsub::IdentTopic::new("findag-handshake"),
            NetworkMessageType::PingPong => gossipsub::IdentTopic::new("findag-ping"),
            NetworkMessageType::Error => gossipsub::IdentTopic::new("findag-error"),
        }
    }

    /// Initialize metrics
    fn initialize_metrics(&self) {
        gauge!("findag_network_connected_peers", 0.0);
        gauge!("findag_network_total_peers_discovered", 0.0);
        counter!("findag_network_messages_sent", 0);
        counter!("findag_network_messages_received", 0);
        counter!("findag_network_bytes_sent", 0);
        counter!("findag_network_bytes_received", 0);
        histogram!("findag_network_latency_ms", 0.0);
    }

    /// Main network loop
    async fn network_loop(&mut self) -> FindDAGResult<()> {
        info!("Starting network loop");
        
        loop {
            tokio::select! {
                swarm_event = self.swarm.next() => {
                    match swarm_event {
                        Some(SwarmEvent::Behaviour(NetworkEvent::Gossipsub(gossipsub::Event::Message {
                            propagation_source: peer_id,
                            message_id: id,
                            message,
                        }))) => {
                            self.handle_gossipsub_message(peer_id, id, message).await?;
                        }
                        Some(SwarmEvent::Behaviour(NetworkEvent::Kad(KademliaEvent::OutboundQueryCompleted {
                            result: QueryResult::Bootstrap(ok),
                            ..
                        }))) => {
                            if ok {
                                info!("Bootstrap completed successfully");
                            }
                        }
                        Some(SwarmEvent::Behaviour(NetworkEvent::Mdns(MdnsEvent::Discovered(list)))) => {
                            for (peer_id, multiaddr) in list {
                                info!("mDNS discovered peer: {} at {}", peer_id, multiaddr);
                                self.swarm.dial(multiaddr)?;
                            }
                        }
                        Some(SwarmEvent::Behaviour(NetworkEvent::Mdns(MdnsEvent::Expired(list)))) => {
                            for (peer_id, _multiaddr) in list {
                                info!("mDNS expired peer: {}", peer_id);
                                let mut peers = self.peers.write().await;
                                peers.remove(&peer_id);
                            }
                        }
                        Some(SwarmEvent::NewListenAddr { address, .. }) => {
                            info!("Listening on: {}", address);
                        }
                        Some(SwarmEvent::ConnectionEstablished { peer_id, .. }) => {
                            info!("Connected to peer: {}", peer_id);
                            let mut peers = self.peers.write().await;
                            peers.insert(peer_id, PeerInfo {
                                peer_id: peer_id.to_string(),
                                address: "".to_string(),
                                port: 0,
                                public_key: vec![],
                                version: "".to_string(),
                                status: PeerStatus::Connected,
                                last_seen: chrono::Utc::now(),
                                latency_ms: None,
                                metadata: None,
                            });
                        }
                        Some(SwarmEvent::ConnectionClosed { peer_id, .. }) => {
                            info!("Disconnected from peer: {}", peer_id);
                            let mut peers = self.peers.write().await;
                            peers.remove(&peer_id);
                        }
                        _ => {}
                    }
                }
                command = self.command_receiver.recv() => {
                    if let Some(command) = command {
                        self.handle_command(command).await?;
                    } else {
                        break;
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Handle GossipSub message
    async fn handle_gossipsub_message(
        &mut self,
        peer_id: PeerId,
        _message_id: MessageId,
        message: gossipsub::Message,
    ) -> FindDAGResult<()> {
        let network_message: NetworkMessage = bincode::deserialize(&message.data)?;
        
        counter!("findag_network_messages_received", 1);
        self.metrics.messages_received += 1;
        self.metrics.bytes_received += message.data.len() as u64;
        
        // Send event
        let _ = self.event_sender.send(NetworkEvent::MessageReceived {
            peer_id: peer_id.to_string(),
            message_type: network_message.message_type,
            message_id: network_message.message_id,
            timestamp: network_message.timestamp,
        }).await;
        
        Ok(())
    }

    /// Handle network command
    async fn handle_command(&mut self, command: NetworkCommand) -> FindDAGResult<()> {
        match command {
            NetworkCommand::Start => {
                info!("Network manager started");
            }
            NetworkCommand::Stop => {
                info!("Network manager stopped");
                break;
            }
            NetworkCommand::ConnectPeer(addr) => {
                self.connect_peer(&addr).await?;
            }
            NetworkCommand::DisconnectPeer(peer_id) => {
                let peer_id: PeerId = peer_id.parse()?;
                self.disconnect_peer(&peer_id).await?;
            }
            NetworkCommand::SendMessage(message) => {
                // TODO: Implement direct message sending
                self.broadcast_message(message).await?;
            }
            NetworkCommand::UpdateConfig(config) => {
                info!("Updating network configuration");
                self.config = config;
            }
            NetworkCommand::BanPeer(peer_id, reason) => {
                info!("Banning peer {}: {}", peer_id, reason);
                // TODO: Implement peer banning
            }
            NetworkCommand::UnbanPeer(peer_id) => {
                info!("Unbanning peer: {}", peer_id);
                // TODO: Implement peer unbanning
            }
        }
        
        Ok(())
    }
}

impl Default for NetworkMetrics {
    fn default() -> Self {
        Self {
            connected_peers: 0,
            total_peers_discovered: 0,
            messages_sent: 0,
            messages_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
            avg_latency_ms: 0.0,
            uptime_seconds: 0,
        }
    }
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct MessageId(u64);

impl From<MessageId> for gossipsub::MessageId {
    fn from(id: MessageId) -> Self {
        gossipsub::MessageId::from(id.0.to_be_bytes())
    }
} 
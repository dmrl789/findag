use crate::core::types::{SerializableTransaction, SerializableBlock, SerializableRound};
use crate::core::address::Address;
use crate::network::encryption::{P2PEncryption, EncryptedMessage};
use serde::{Serialize, Deserialize};
use tokio::net::UdpSocket;
use std::collections::{HashSet, HashMap};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::metrics;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GossipMsg {
    NewTransaction(SerializableTransaction),
    NewBlock(SerializableBlock),
    NewRound(SerializableRound),
}

/// Network message wrapper that can be encrypted or plain
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NetworkMessage {
    Plain(GossipMsg),
    Encrypted(EncryptedMessage),
}

pub struct NetworkPropagator {
    socket: Arc<UdpSocket>,
    peers: Vec<SocketAddr>,
    peer_addresses: Arc<Mutex<HashMap<SocketAddr, Address>>>, // Map socket addresses to peer addresses
    seen_hashes: Arc<Mutex<HashSet<Vec<u8>>>>, // For deduplication
    encryption: Option<Arc<P2PEncryption>>, // Optional encryption layer
    #[allow(dead_code)]
    local_address: Address,
}

impl NetworkPropagator {
    pub async fn new(bind_addr: &str, peers: Vec<SocketAddr>, local_address: Address) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(bind_addr).await?;
        metrics::PEER_COUNT.set(peers.len() as i64);
        Ok(Self {
            socket: Arc::new(socket),
            peers,
            peer_addresses: Arc::new(Mutex::new(HashMap::new())),
            seen_hashes: Arc::new(Mutex::new(HashSet::new())),
            encryption: None,
            local_address,
        })
    }

    /// Create a new NetworkPropagator with encryption enabled
    pub async fn new_with_encryption(
        bind_addr: &str, 
        peers: Vec<SocketAddr>, 
        local_address: Address,
        encryption: Arc<P2PEncryption>
    ) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(bind_addr).await?;
        metrics::PEER_COUNT.set(peers.len() as i64);
        Ok(Self {
            socket: Arc::new(socket),
            peers,
            peer_addresses: Arc::new(Mutex::new(HashMap::new())),
            seen_hashes: Arc::new(Mutex::new(HashSet::new())),
            encryption: Some(encryption),
            local_address,
        })
    }

    /// Broadcast a message to all peers
    pub async fn broadcast(&self, msg: &GossipMsg) {
        if let Some(encryption) = &self.encryption {
            // Encrypt the message for each peer
            for peer_socket in &self.peers {
                if let Some(peer_address) = self.get_peer_address(peer_socket).await {
                    if encryption.has_encryption_session(&peer_address).await {
                        // Encrypt and send
                        match encryption.encrypt_message(&peer_address, &bincode::serialize(msg).unwrap(), "gossip").await {
                            Ok(encrypted_msg) => {
                                let network_msg = NetworkMessage::Encrypted(encrypted_msg);
                                let buf = bincode::serialize(&network_msg).unwrap();
                                let _ = self.socket.send_to(&buf, peer_socket).await;
                            }
                            Err(e) => {
                                println!("⚠️ Failed to encrypt message for peer {}: {}", peer_address.as_str(), e);
                            }
                        }
                    } else {
                        // Send plain message if no encryption session
                        let network_msg = NetworkMessage::Plain(msg.clone());
                        let buf = bincode::serialize(&network_msg).unwrap();
                        let _ = self.socket.send_to(&buf, peer_socket).await;
                    }
                } else {
                    // Send plain message if peer address unknown
                    let network_msg = NetworkMessage::Plain(msg.clone());
                    let buf = bincode::serialize(&network_msg).unwrap();
                    let _ = self.socket.send_to(&buf, peer_socket).await;
                }
            }
        } else {
            // No encryption, send plain messages
            let network_msg = NetworkMessage::Plain(msg.clone());
            let buf = bincode::serialize(&network_msg).unwrap();
            for peer in &self.peers {
                let _ = self.socket.send_to(&buf, peer).await;
            }
        }
    }

    /// Listen for incoming gossip messages and handle them
    pub async fn listen<F>(&self, mut handler: F)
    where
        F: FnMut(GossipMsg) + Send + 'static,
    {
        let mut buf = [0u8; 65536];
        loop {
            if let Ok((len, peer_socket)) = self.socket.recv_from(&mut buf).await {
                // Try to deserialize as NetworkMessage first
                if let Ok(network_msg) = bincode::deserialize::<NetworkMessage>(&buf[..len]) {
                    match network_msg {
                        NetworkMessage::Plain(gossip_msg) => {
                            self.process_gossip_message(gossip_msg, &mut handler).await;
                        }
                        NetworkMessage::Encrypted(encrypted_msg) => {
                            if let Some(encryption) = &self.encryption {
                                if let Some(peer_address) = self.get_peer_address(&peer_socket).await {
                                    match encryption.decrypt_message(&peer_address, &encrypted_msg).await {
                                        Ok(decrypted_data) => {
                                            if let Ok(gossip_msg) = bincode::deserialize::<GossipMsg>(&decrypted_data) {
                                                self.process_gossip_message(gossip_msg, &mut handler).await;
                                            }
                                        }
                                        Err(e) => {
                                            println!("❌ Failed to decrypt message from {}: {}", peer_address.as_str(), e);
                                        }
                                    }
                                } else {
                                    println!("⚠️ Received encrypted message from unknown peer: {}", peer_socket);
                                }
                            } else {
                                println!("⚠️ Received encrypted message but encryption is disabled");
                            }
                        }
                    }
                } else {
                    // Fallback: try to deserialize as plain GossipMsg (backward compatibility)
                    if let Ok(msg) = bincode::deserialize::<GossipMsg>(&buf[..len]) {
                        self.process_gossip_message(msg, &mut handler).await;
                    }
                }
            }
        }
    }

    /// Process a gossip message (deduplication and handling)
    async fn process_gossip_message<F>(&self, msg: GossipMsg, handler: &mut F)
    where
        F: FnMut(GossipMsg) + Send + 'static,
    {
        let hash = match &msg {
            GossipMsg::NewTransaction(tx) => tx.hashtimer.to_vec(),
            GossipMsg::NewBlock(block) => block.block_id.to_vec(),
            GossipMsg::NewRound(round) => round.round_number.to_be_bytes().to_vec(),
        };
        let mut seen = self.seen_hashes.lock().unwrap();
        if seen.insert(hash) {
            drop(seen);
            handler(msg);
        }
    }

    /// Get peer address for a socket address
    async fn get_peer_address(&self, socket_addr: &SocketAddr) -> Option<Address> {
        let peer_addresses = self.peer_addresses.lock().unwrap();
        peer_addresses.get(socket_addr).cloned()
    }

    /// Register a peer address for a socket address
    pub async fn register_peer(&self, socket_addr: SocketAddr, peer_address: Address) {
        let mut peer_addresses = self.peer_addresses.lock().unwrap();
        peer_addresses.insert(socket_addr, peer_address.clone());
        println!("📝 Registered peer {} at {}", peer_address.as_str(), socket_addr);
    }

    /// Enable encryption for this propagator
    pub fn enable_encryption(&mut self, encryption: Arc<P2PEncryption>) {
        self.encryption = Some(encryption);
        println!("🔐 Encryption enabled for network propagator");
    }
}

// Example usage (in your main):
//
// use std::net::SocketAddr;
// use network::propagation::{NetworkPropagator, GossipMsg};
//
// #[tokio::main]
// async fn main() {
//     let peers = vec!["127.0.0.1:9001".parse().unwrap()];
//     let propagator = NetworkPropagator::new("0.0.0.0:9000", peers).await.unwrap();
//     // Spawn listener
//     tokio::spawn(async move {
//         propagator.listen(|msg| {
//             match msg {
//                 GossipMsg::NewTransaction(tx) => {/* add to mempool */},
//                 GossipMsg::NewBlock(block) => {/* add to DAG */},
//                 GossipMsg::NewRound(round) => {/* add to DAG */},
//             }
//         }).await;
//     });
//     // To broadcast:
//     // propagator.broadcast(&GossipMsg::NewTransaction(tx)).await;
// } 
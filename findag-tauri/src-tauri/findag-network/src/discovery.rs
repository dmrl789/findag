//! Peer discovery
//! 
//! This module handles peer discovery using mDNS and Kademlia.

use libp2p::{
    core::PeerId,
    mdns::{Mdns, MdnsEvent},
    kad::{Kademlia, KademliaEvent, QueryResult},
    swarm::NetworkBehaviour,
};
use findag_types::{PeerInfo, FindDAGResult};

/// Discovery manager
pub struct DiscoveryManager {
    /// mDNS for local discovery
    mdns: Mdns,
    /// Kademlia for DHT discovery
    kad: Kademlia,
    /// Discovered peers
    peers: std::collections::HashMap<PeerId, PeerInfo>,
}

impl DiscoveryManager {
    /// Create a new discovery manager
    pub fn new() -> FindDAGResult<Self> {
        let mdns = Mdns::new(Default::default())?;
        let kad = Kademlia::new(PeerId::random(), Default::default());
        
        Ok(Self {
            mdns,
            kad,
            peers: std::collections::HashMap::new(),
        })
    }

    /// Handle mDNS events
    pub fn handle_mdns_event(&mut self, event: MdnsEvent) -> FindDAGResult<()> {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer_id, multiaddr) in list {
                    let peer_info = PeerInfo {
                        peer_id: peer_id.to_string(),
                        address: multiaddr.to_string(),
                        port: 0, // TODO: Extract port from multiaddr
                        public_key: vec![],
                        version: "".to_string(),
                        status: findag_types::PeerStatus::Disconnected,
                        last_seen: chrono::Utc::now(),
                        latency_ms: None,
                        metadata: None,
                    };
                    self.peers.insert(peer_id, peer_info);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer_id, _multiaddr) in list {
                    self.peers.remove(&peer_id);
                }
            }
        }
        
        Ok(())
    }

    /// Handle Kademlia events
    pub fn handle_kad_event(&mut self, event: KademliaEvent) -> FindDAGResult<()> {
        match event {
            KademliaEvent::OutboundQueryCompleted { result, .. } => {
                match result {
                    QueryResult::Bootstrap(ok) => {
                        if ok {
                            tracing::info!("Kademlia bootstrap completed successfully");
                        }
                    }
                    QueryResult::GetProviders(Ok(providers)) => {
                        for (peer_id, _) in providers {
                            // Handle discovered providers
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        
        Ok(())
    }

    /// Get discovered peers
    pub fn get_peers(&self) -> Vec<&PeerInfo> {
        self.peers.values().collect()
    }
} 
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use tracing::{info, debug};
use crate::storage::types::PeerId;

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub peer_id: PeerId,
    pub addresses: Vec<String>,
    pub last_seen: DateTime<Utc>,
}

pub struct DiscoveryManager {
    known_peers: Arc<Mutex<HashSet<PeerId>>>,
    peer_info: Arc<Mutex<HashMap<PeerId, PeerInfo>>>,
}

impl DiscoveryManager {
    pub fn new() -> Self {
        Self {
            known_peers: Arc::new(Mutex::new(HashSet::new())),
            peer_info: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_peer(&self, peer_id: PeerId, addr: String) -> Result<(), String> {
        let mut known_peers = self.known_peers.lock().await;
        let mut peer_info = self.peer_info.lock().await;

        known_peers.insert(peer_id.clone());
        peer_info.insert(peer_id.clone(), PeerInfo {
            peer_id: peer_id.clone(),
            addresses: vec![addr],
            last_seen: Utc::now(),
        });

        info!("Successfully added peer: {}", peer_id.clone());
        Ok(())
    }

    pub async fn remove_peer(&self, peer_id: &PeerId) -> Result<(), String> {
        let mut known_peers = self.known_peers.lock().await;
        let mut peer_info = self.peer_info.lock().await;

        known_peers.remove(peer_id);
        peer_info.remove(peer_id);

        info!("Successfully removed peer: {}", peer_id);
        Ok(())
    }

    pub async fn get_peers(&self) -> Vec<PeerId> {
        let known_peers = self.known_peers.lock().await;
        known_peers.iter().cloned().collect()
    }

    pub async fn is_peer_known(&self, peer_id: &PeerId) -> bool {
        let known_peers = self.known_peers.lock().await;
        known_peers.contains(peer_id)
    }

    pub async fn update_peer_info(&self, peer_id: PeerId, addr: String) -> Result<(), String> {
        let mut peer_info = self.peer_info.lock().await;
        let info = peer_info.entry(peer_id.clone()).or_insert(PeerInfo {
            peer_id: peer_id.clone(),
            addresses: Vec::new(),
            last_seen: Utc::now(),
        });

        if !info.addresses.contains(&addr) {
            info.addresses.push(addr);
        }
        info.last_seen = Utc::now();
        Ok(())
    }

    pub async fn get_peer_info(&self, peer_id: &PeerId) -> Option<PeerInfo> {
        let peer_info = self.peer_info.lock().await;
        peer_info.get(peer_id).cloned()
    }
} 
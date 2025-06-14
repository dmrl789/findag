use std::error::Error;
use libp2p::PeerId;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub struct Discovery {
    known_peers: Arc<Mutex<HashSet<PeerId>>>,
}

impl Discovery {
    pub fn new() -> Self {
        Self {
            known_peers: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn add_peer(&self, peer_id: PeerId) {
        let mut peers = self.known_peers.lock().unwrap();
        peers.insert(peer_id);
    }

    pub fn remove_peer(&self, peer_id: &PeerId) {
        let mut peers = self.known_peers.lock().unwrap();
        peers.remove(peer_id);
    }

    pub fn get_peers(&self) -> Vec<PeerId> {
        let peers = self.known_peers.lock().unwrap();
        peers.iter().cloned().collect()
    }
} 
use crate::core::types::{Transaction, Block, Round};
use serde::{Serialize, Deserialize};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::metrics;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GossipMsg {
    NewTransaction(Transaction),
    NewBlock(Block),
    NewRound(Round),
}

pub struct NetworkPropagator {
    socket: Arc<UdpSocket>,
    peers: Vec<SocketAddr>,
    seen_hashes: Arc<Mutex<HashSet<Vec<u8>>>>, // For deduplication
}

impl NetworkPropagator {
    pub async fn new(bind_addr: &str, peers: Vec<SocketAddr>) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(bind_addr).await?;
        metrics::PEER_COUNT.set(peers.len() as i64);
        Ok(Self {
            socket: Arc::new(socket),
            peers,
            seen_hashes: Arc::new(Mutex::new(HashSet::new())),
        })
    }

    /// Broadcast a message to all peers
    pub async fn broadcast(&self, msg: &GossipMsg) {
        let buf = bincode::serialize(msg).unwrap();
        for peer in &self.peers {
            let _ = self.socket.send_to(&buf, peer).await;
        }
    }

    /// Listen for incoming gossip messages and handle them
    pub async fn listen<F>(&self, mut handler: F)
    where
        F: FnMut(GossipMsg) + Send + 'static,
    {
        let mut buf = [0u8; 65536];
        loop {
            if let Ok((len, _peer)) = self.socket.recv_from(&mut buf).await {
                if let Ok(msg) = bincode::deserialize::<GossipMsg>(&buf[..len]) {
                    let hash = match &msg {
                        GossipMsg::NewTransaction(tx) => tx.hashtimer.to_vec(),
                        GossipMsg::NewBlock(block) => block.block_id.to_vec(),
                        GossipMsg::NewRound(round) => round.round_id.to_be_bytes().to_vec(),
                    };
                    let mut seen = self.seen_hashes.lock().unwrap();
                    if seen.insert(hash) {
                        drop(seen);
                        handler(msg);
                    }
                }
            }
        }
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
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::blockchain::block::Block;
use crate::utils::time::get_findag_time_micro;

type PeerList = Arc<Mutex<HashMap<String, tokio_tungstenite::WebSocketStream<TcpStream>>>>;

#[derive(Clone)]
pub struct RelayService {
    pub peers: PeerList,
}

impl RelayService {
    pub fn new() -> Self {
        Self {
            peers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn connect_peer(&self, peer_url: &str) {
        if let Ok((ws_stream, _)) = connect_async(peer_url).await {
            self.peers.lock().unwrap().insert(peer_url.to_string(), ws_stream);
            println!("Connected to peer: {}", peer_url);
        } else {
            eprintln!("Failed to connect to peer: {}", peer_url);
        }
    }

    pub async fn relay_block(&self, block: &Block) {
        let message = serde_json::to_string(&block).unwrap();
        for (url, peer) in self.peers.lock().unwrap().iter_mut() {
            if let Err(e) = peer.send(Message::Text(message.clone())).await {
                eprintln!("Error relaying to {}: {}", url, e);
            }
        }
    }

    pub async fn start_listener(&self, port: u16) {
        let addr = format!("127.0.0.1:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind");

        println!("Relay listener running on {}", addr);
        while let Ok((stream, _)) = listener.accept().await {
            let peer_list = self.peers.clone();
            tokio::spawn(async move {
                let ws_stream = tokio_tungstenite::accept_async(stream).await.unwrap();
                let (mut write, mut read) = ws_stream.split();

                while let Some(msg) = read.next().await {
                    if let Ok(Message::Text(data)) = msg {
                        println!("Received: {}", data);
                        // Optionally: parse and validate block
                    }
                }
            });
        }
    }
}

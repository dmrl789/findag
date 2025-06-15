use std::error::Error;
use std::sync::Arc;
use std::collections::HashMap;
use std::fmt;
use tokio::sync::Mutex;
use async_trait::async_trait;
use libp2p::{
    floodsub::{Floodsub, FloodsubEvent, Topic},
    mdns::{tokio::Behaviour as Mdns, Event as MdnsEvent},
    swarm::Swarm,
    PeerId,
};
use crate::network::MyNetworkBehaviour;

#[async_trait]
pub trait ProtocolHandler: Send + Sync {
    async fn handle_message(&self, message: &[u8]) -> Result<(), Box<dyn Error>>;
}

pub struct Protocol {
    handlers: Arc<Mutex<HashMap<String, Box<dyn ProtocolHandler>>>>,
    swarm: Arc<Mutex<Swarm<MyNetworkBehaviour>>>,
}

impl fmt::Debug for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Protocol")
            .field("handlers", &"<opaque>")
            .finish()
    }
}

impl Clone for Protocol {
    fn clone(&self) -> Self {
        Self {
            handlers: Arc::clone(&self.handlers),
            swarm: Arc::clone(&self.swarm),
        }
    }
}

impl Protocol {
    pub fn new(swarm: Arc<Mutex<Swarm<MyNetworkBehaviour>>>) -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
            swarm,
        }
    }

    pub async fn register_handler<H>(&self, protocol: &str, handler: H)
    where
        H: ProtocolHandler + 'static,
    {
        let mut handlers = self.handlers.lock().await;
        handlers.insert(protocol.to_string(), Box::new(handler));
    }

    pub async fn handle_message(&self, protocol: &str, message: &[u8]) -> Result<(), Box<dyn Error>> {
        let handlers = self.handlers.lock().await;
        if let Some(handler) = handlers.get(protocol) {
            handler.handle_message(message).await
        } else {
            Err("Protocol handler not found".into())
        }
    }

    pub async fn broadcast_message(&self, topic: &str, message: &[u8]) -> Result<(), Box<dyn Error>> {
        let mut swarm = self.swarm.lock().await;
        let topic = Topic::new(topic);
        swarm.behaviour_mut().floodsub.publish(topic, message.to_vec());
        Ok(())
    }

    pub async fn subscribe_to_topic(&self, topic: &str) -> Result<(), Box<dyn Error>> {
        let mut swarm = self.swarm.lock().await;
        let topic = Topic::new(topic);
        swarm.behaviour_mut().floodsub.subscribe(topic);
        Ok(())
    }

    pub async fn unsubscribe_from_topic(&self, topic: &str) -> Result<(), Box<dyn Error>> {
        let mut swarm = self.swarm.lock().await;
        let topic = Topic::new(topic);
        swarm.behaviour_mut().floodsub.unsubscribe(topic);
        Ok(())
    }

    pub async fn handle_floodsub_event(&self, event: FloodsubEvent) -> Result<(), Box<dyn Error>> {
        match event {
            FloodsubEvent::Message(message) => {
                // Handle incoming message
                println!("Received message from {:?}: {:?}", message.source, message.data);
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn handle_mdns_event(&self, event: MdnsEvent) -> Result<(), Box<dyn Error>> {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer_id, _addr) in list {
                    println!("Discovered peer: {}", peer_id);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer_id, _addr) in list {
                    println!("Expired peer: {}", peer_id);
                }
            }
        }
        Ok(())
    }

    pub async fn get_connected_peers(&self) -> Result<Vec<PeerId>, Box<dyn Error>> {
        let swarm = self.swarm.lock().await;
        Ok(swarm.connected_peers().cloned().collect())
    }

    pub async fn get_peer_addresses(&self) -> Vec<(PeerId, String)> {
        let swarm = self.swarm.lock().await;
        swarm.connected_peers()
            .cloned()
            .filter_map(|peer_id| {
                swarm.external_addresses()
                    .next()
                    .map(|addr| (peer_id, addr.to_string()))
            })
            .collect()
    }
} 
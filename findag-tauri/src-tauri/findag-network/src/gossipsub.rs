//! GossipSub message propagation
//! 
//! This module handles message propagation using GossipSub protocol.

use libp2p::{
    core::PeerId,
    gossipsub::{self, MessageId, ValidationMode, IdentTopic},
    swarm::NetworkBehaviour,
};
use findag_types::{NetworkMessage, NetworkMessageType, FindDAGResult};

/// GossipSub manager
pub struct GossipSubManager {
    /// GossipSub behaviour
    gossipsub: gossipsub::Behaviour,
    /// Subscribed topics
    topics: std::collections::HashMap<String, IdentTopic>,
}

impl GossipSubManager {
    /// Create a new GossipSub manager
    pub fn new() -> FindDAGResult<Self> {
        let config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(std::time::Duration::from_secs(1))
            .validation_mode(ValidationMode::Strict)
            .message_id_fn(|msg| {
                let mut s = std::collections::hash_map::DefaultHasher::new();
                s.write(msg.data.as_ref());
                MessageId(s.finish())
            })
            .build()
            .expect("Valid config");
        
        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Anonymous,
            config,
        )?;
        
        let mut topics = std::collections::HashMap::new();
        topics.insert("transactions".to_string(), IdentTopic::new("findag-transactions"));
        topics.insert("blocks".to_string(), IdentTopic::new("findag-blocks"));
        topics.insert("rounds".to_string(), IdentTopic::new("findag-rounds"));
        topics.insert("votes".to_string(), IdentTopic::new("findag-votes"));
        
        Ok(Self {
            gossipsub,
            topics,
        })
    }

    /// Subscribe to a topic
    pub fn subscribe(&mut self, topic_name: &str) -> FindDAGResult<()> {
        if let Some(topic) = self.topics.get(topic_name) {
            self.gossipsub.subscribe(topic)?;
        }
        Ok(())
    }

    /// Publish a message to a topic
    pub fn publish(&mut self, topic_name: &str, message: NetworkMessage) -> FindDAGResult<()> {
        if let Some(topic) = self.topics.get(topic_name) {
            let data = bincode::serialize(&message)?;
            self.gossipsub.publish(topic.clone(), data)?;
        }
        Ok(())
    }

    /// Get topic for message type
    pub fn get_topic_for_message(&self, message_type: &NetworkMessageType) -> Option<&IdentTopic> {
        let topic_name = match message_type {
            NetworkMessageType::Transaction => "transactions",
            NetworkMessageType::Block => "blocks",
            NetworkMessageType::Round => "rounds",
            NetworkMessageType::Vote => "votes",
            _ => return None,
        };
        
        self.topics.get(topic_name)
    }

    /// Handle GossipSub events
    pub fn handle_event(&mut self, event: gossipsub::Event) -> FindDAGResult<()> {
        match event {
            gossipsub::Event::Message {
                propagation_source: peer_id,
                message_id: _id,
                message,
            } => {
                // Handle received message
                let _network_message: NetworkMessage = bincode::deserialize(&message.data)?;
                tracing::info!("Received message from peer: {}", peer_id);
            }
            gossipsub::Event::Subscribed { peer_id, topic } => {
                tracing::info!("Peer {} subscribed to topic: {}", peer_id, topic);
            }
            gossipsub::Event::Unsubscribed { peer_id, topic } => {
                tracing::info!("Peer {} unsubscribed from topic: {}", peer_id, topic);
            }
            _ => {}
        }
        
        Ok(())
    }
} 
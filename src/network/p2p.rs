use libp2p::{
    identity, PeerId, Multiaddr,
    gossipsub::{self, Gossipsub, GossipsubEvent, MessageAuthenticity, IdentTopic},
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    kad::{Kademlia, KademliaConfig, store::MemoryStore, record::store::MemoryStore as RecordStore, KademliaEvent},
    noise::{Keypair as NoiseKeypair, X25519Spec, NoiseConfig, AuthenticKeypair, NoiseError},
    swarm::{Swarm, SwarmEvent, NetworkBehaviour},
    futures::StreamExt,
};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use tokio::sync::mpsc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum P2PMsg {
    NewTransaction(Vec<u8>), // Replace Vec<u8> with your Transaction type
    NewBlock(Vec<u8>),       // Replace Vec<u8> with your Block type
    NewRound(Vec<u8>),       // Replace Vec<u8> with your Round type
}

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
pub struct MyBehaviour {
    pub gossipsub: Gossipsub,
    pub mdns: Mdns,
    pub kademlia: Kademlia<MemoryStore>,
}

#[derive(Debug)]
pub enum MyBehaviourEvent {
    Gossipsub(GossipsubEvent),
    Mdns(MdnsEvent),
    Kademlia(KademliaEvent),
}

impl From<GossipsubEvent> for MyBehaviourEvent {
    fn from(event: GossipsubEvent) -> Self {
        MyBehaviourEvent::Gossipsub(event)
    }
}
impl From<MdnsEvent> for MyBehaviourEvent {
    fn from(event: MdnsEvent) -> Self {
        MyBehaviourEvent::Mdns(event)
    }
}
impl From<KademliaEvent> for MyBehaviourEvent {
    fn from(event: KademliaEvent) -> Self {
        MyBehaviourEvent::Kademlia(event)
    }
}

pub async fn run_p2p_node(topic_str: &str, mut rx: mpsc::UnboundedReceiver<P2PMsg>) {
    // Generate a random identity for this node
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    // Set up Noise for secure transport
    let noise_keys = NoiseKeypair::<X25519Spec>::new().into_authentic(&id_keys).unwrap();
    let transport = libp2p::tokio_development_transport(id_keys.clone()).await.unwrap();

    // Set up gossipsub
    let gossipsub_config = gossipsub::GossipsubConfig::default();
    let mut gossipsub = Gossipsub::new(MessageAuthenticity::Signed(id_keys.clone()), gossipsub_config).unwrap();
    let topic = IdentTopic::new(topic_str);
    gossipsub.subscribe(&topic).unwrap();

    // Set up mDNS for local peer discovery
    let mdns = Mdns::new(MdnsConfig::default()).await.unwrap();

    // Set up Kademlia for global peer discovery
    let store = MemoryStore::new(peer_id.clone());
    let mut kademlia = Kademlia::with_config(peer_id.clone(), store, KademliaConfig::default());

    // Combine behaviours
    let behaviour = MyBehaviour { gossipsub, mdns, kademlia };
    let mut swarm = Swarm::new(transport, behaviour, peer_id);

    // Listen on all interfaces and a random OS-assigned port
    let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
    swarm.listen_on(listen_addr).unwrap();

    // Main event loop
    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                // Serialize and publish to gossipsub
                let data = bincode::serialize(&msg).unwrap();
                // TODO: Validate message before publishing
                swarm.behaviour_mut().gossipsub.publish(topic.clone(), data).unwrap();
            }
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(GossipsubEvent::Message { message, .. })) => {
                        // Deserialize and validate incoming message
                        if let Ok(msg) = bincode::deserialize::<P2PMsg>(&message.data) {
                            // TODO: Validate message (signature, replay, etc.)
                            println!("Received P2PMsg: {:?}", msg);
                            // TODO: Integrate with mempool, DAG, etc.
                        }
                    }
                    SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(MdnsEvent::Discovered(peers))) => {
                        for (peer, _addr) in peers {
                            swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer);
                        }
                    }
                    SwarmEvent::Behaviour(MyBehaviourEvent::Kademlia(_event)) => {
                        // Handle Kademlia events (peer discovery, etc.)
                    }
                    _ => {}
                }
            }
        }
    }
}

// Example usage (in your main):
//
// use tokio::sync::mpsc;
// use network::p2p::{run_p2p_node, P2PMsg};
//
// #[tokio::main]
// async fn main() {
//     let (tx, rx) = mpsc::unbounded_channel();
//     tokio::spawn(async move {
//         run_p2p_node("findag-main", rx).await;
//     });
//     // To broadcast:
//     // tx.send(P2PMsg::NewTransaction(...)).unwrap();
// } 
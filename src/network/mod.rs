use libp2p::kad::{Behaviour as Kademlia, Config as KademliaConfig, Event as KademliaEvent, store::MemoryStore};
use libp2p::mdns::{tokio::Behaviour as Mdns, Config as MdnsConfig, Event as MdnsEvent};
use libp2p::tcp::tokio::Transport as TokioTcpTransport;
use libp2p::noise::Config as NoiseConfig;
use libp2p::yamux::Config as YamuxConfig;
use libp2p::swarm::{Swarm, SwarmEvent, NetworkBehaviour};
use libp2p::identity::Keypair;
use libp2p::PeerId;
use std::error::Error;
use std::time::Duration;
use crate::types::finality::FinalityVote;
use libp2p_swarm_derive::NetworkBehaviour;
use futures::StreamExt;

pub mod p2p;
pub mod discovery;
pub mod protocol;

pub fn broadcast_vote(vote: &FinalityVote) {
    // Serialize and send vote through P2P layer
    println!("Broadcasting finality vote: {:?}", vote);
}

// (You would call this function when a node votes to finalize a block)

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "NodeBehaviourEvent")]
pub struct NodeBehaviour {
    pub kademlia: Kademlia<MemoryStore>,
    pub mdns: Mdns,
}

#[derive(Debug)]
pub enum NodeBehaviourEvent {
    Kademlia(KademliaEvent),
    Mdns(MdnsEvent),
}

impl From<KademliaEvent> for NodeBehaviourEvent {
    fn from(event: KademliaEvent) -> Self {
        NodeBehaviourEvent::Kademlia(event)
    }
}

impl From<MdnsEvent> for NodeBehaviourEvent {
    fn from(event: MdnsEvent) -> Self {
        NodeBehaviourEvent::Mdns(event)
    }
}

pub struct Node {
    pub swarm: Swarm<NodeBehaviour>,
}

impl Node {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let local_keys = Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_keys.public());

        let transport = TokioTcpTransport::default()
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(NoiseConfig::new(&local_keys)?)
            .multiplex(YamuxConfig::default())
            .boxed();

        let store = MemoryStore::new(local_peer_id);
        let mut kad_config = KademliaConfig::default();
        kad_config.set_query_timeout(Duration::from_secs(30));
        let kademlia = Kademlia::with_config(local_peer_id, store, kad_config);

        let mdns = Mdns::new(MdnsConfig::default(), local_peer_id).expect("Could not start mDNS");

        let behaviour = NodeBehaviour {
            kademlia,
            mdns,
        };

        let swarm = Swarm::new(
            transport,
            behaviour,
            local_peer_id,
            libp2p::swarm::Config::default(),
        );

        Ok(Self { swarm })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            // Example event loop, update as needed
            // match self.swarm.next().await { ... }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

pub fn setup_network() {
    println!("Setting up network...");
    // TODO: Implement network setup
}

pub fn start_network() -> Result<(), Box<dyn Error>> {
    println!("Starting network...");
    Ok(())
}

pub fn stop_network() -> Result<(), Box<dyn Error>> {
    println!("Stopping network...");
    Ok(())
}

pub async fn run_network() -> Result<(), Box<dyn Error>> {
    let local_keys = Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_keys.public());
    println!("🧬 Local peer id: {local_peer_id}");

    let transport = TokioTcpTransport::default()
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(NoiseConfig::new(&local_keys)?)
        .multiplex(YamuxConfig::default())
        .boxed();

    let store = MemoryStore::new(local_peer_id);
    let mut kad_config = KademliaConfig::default();
    kad_config.set_query_timeout(Duration::from_secs(30));
    let kademlia = Kademlia::with_config(local_peer_id, store, kad_config);

    let mdns = Mdns::new(MdnsConfig::default(), local_peer_id).expect("Could not start mDNS");

    let behaviour = NodeBehaviour { kademlia, mdns };

    let mut swarm = Swarm::new(
        transport,
        behaviour,
        local_peer_id,
        libp2p::swarm::Config::with_executor(Box::new(|fut| {
            tokio::spawn(fut);
        })),
    );

    loop {
        match swarm.next().await {
            Some(SwarmEvent::Behaviour(NodeBehaviourEvent::Kademlia(KademliaEvent::InboundRequest { .. }))) => {
                println!("📡 Received inbound DHT request");
            }
            Some(SwarmEvent::Behaviour(NodeBehaviourEvent::Mdns(MdnsEvent::Discovered(peers)))) => {
                for (peer, _) in peers {
                    println!("🔍 mDNS discovered: {peer}");
                    swarm.behaviour_mut().kademlia.add_address(&peer, "/ip4/127.0.0.1/tcp/4001".parse().unwrap());
                }
            }
            _ => {}
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

use libp2p::{
    kad::{record::store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent, QueryId, record::Key, Record},
    identity,
    swarm::{SwarmBuilder, SwarmEvent},
    tcp::TokioTcpConfig,
    noise,
    core::upgrade,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    yamux::YamuxConfig,
    gossipsub::{Gossipsub, GossipsubConfig, MessageAuthenticity, IdentTopic as Topic},
    PeerId, Multiaddr, Swarm, Transport,
    NetworkBehaviour,
    futures::StreamExt,
};
use std::time::Duration;
use tokio::time::sleep;
use crate::types::finality::FinalityVote;

pub fn broadcast_vote(vote: &FinalityVote) {
    // Serialize and send vote through P2P layer
    println!("Broadcasting finality vote: {:?}", vote);
}

// (You would call this function when a node votes to finalize a block)



#[derive(NetworkBehaviour)]
pub struct NodeBehaviour {
    pub kademlia: Kademlia<MemoryStore>,
    pub mdns: Mdns,
}

pub fn setup_network() {
    // Generate identity keys
    let local_keys = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_keys.public());
    println!("🧬 Local peer id: {local_peer_id}");

    // Base transport + noise + yamux
    let transport = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseAuthenticated::xx(&local_keys).unwrap())
        .multiplex(YamuxConfig::default())
        .boxed();

    // Kademlia DHT setup
    let store = MemoryStore::new(local_peer_id);
    let mut kad_config = KademliaConfig::default();
    kad_config.set_query_timeout(Duration::from_secs(30));
    let kademlia = Kademlia::with_config(local_peer_id, store, kad_config);

    // MDNS for local peer discovery
    let mdns = Mdns::new(MdnsConfig::default()).expect("Could not start mDNS");

    // Build full behaviour
    let behaviour = NodeBehaviour {
        kademlia,
        mdns,
    };

    let mut swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, local_peer_id)
        .build();

    // Start loop
    tokio::spawn(async move {
        loop {
            match swarm.select_next_some().await {
                SwarmEvent::Behaviour(event) => match event {
                    NodeBehaviourEvent::Kademlia(KademliaEvent::InboundRequest { .. }) => {
                        println!("📡 Received inbound DHT request");
                    }
                    NodeBehaviourEvent::Mdns(MdnsEvent::Discovered(peers)) => {
                        for (peer, _) in peers {
                            println!("🔍 mDNS discovered: {peer}");
                            swarm.behaviour_mut().kademlia.add_address(&peer, "/ip4/127.0.0.1/tcp/4001".parse().unwrap());
                        }
                    }
                    _ => {}
                },
                _ => {}
            }

            sleep(Duration::from_millis(100)).await;
        }
    });

    println!("✅ DHT + mDNS network running...");
}

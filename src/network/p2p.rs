use libp2p::{
    identity, PeerId, Multiaddr,
    gossipsub::{self, Gossipsub, GossipsubEvent, MessageAuthenticity, IdentTopic},
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    kad::{Kademlia, KademliaConfig, store::MemoryStore, KademliaEvent},
    noise::{Keypair as NoiseKeypair, X25519Spec},
    swarm::{Swarm, SwarmEvent, NetworkBehaviour},
    futures::StreamExt,
};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use tokio::sync::mpsc;
use crate::core::types::{Transaction, Block, Round};
use crate::core::tx_pool::ShardedTxPool;
use crate::core::dag_engine::DagEngine;
use ed25519_dalek::{Verifier, Signature, PublicKey};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum P2PMsg {
    NewTransaction(Transaction),
    NewBlock(Block),
    NewRound(Round),
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

fn verify_transaction(tx: &Transaction) -> bool {
    // Check signature matches public key and transaction content
    let msg = b"mock-tx"; // Replace with real tx serialization if needed
    tx.public_key.verify_strict(msg, &tx.signature).is_ok()
}

fn verify_block(block: &Block) -> bool {
    // Check block is signed by proposer
    let msg = &block.block_id;
    block.public_key.verify_strict(msg, &block.signature).is_ok()
}

fn verify_round(round: &Round) -> bool {
    // Check round is signed by proposer/finalizer
    let mut msg = round.round_id.to_le_bytes().to_vec();
    msg.extend_from_slice(&round.hashtimer);
    round.public_key.verify_strict(&msg, &round.signature).is_ok()
}

/// Run the P2P node and integrate with mempool, DAG, and round logic
pub async fn run_p2p_node(
    topic_str: &str,
    mut rx: mpsc::UnboundedReceiver<P2PMsg>,
    tx_pool: ShardedTxPool,
    dag: &mut DagEngine,
    seen_hashes: &mut HashSet<Vec<u8>>,
) {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    let noise_keys = NoiseKeypair::<X25519Spec>::new().into_authentic(&id_keys).unwrap();
    let transport = libp2p::tokio_development_transport(id_keys.clone()).await.unwrap();

    let gossipsub_config = gossipsub::GossipsubConfig::default();
    let mut gossipsub = Gossipsub::new(MessageAuthenticity::Signed(id_keys.clone()), gossipsub_config).unwrap();
    let topic = IdentTopic::new(topic_str);
    gossipsub.subscribe(&topic).unwrap();

    let mdns = Mdns::new(MdnsConfig::default()).await.unwrap();
    let store = MemoryStore::new(peer_id.clone());
    let kademlia = Kademlia::with_config(peer_id.clone(), store, KademliaConfig::default());

    let behaviour = MyBehaviour { gossipsub, mdns, kademlia };
    let mut swarm = Swarm::new(transport, behaviour, peer_id);
    let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
    swarm.listen_on(listen_addr).unwrap();

    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                let data = bincode::serialize(&msg).unwrap();
                swarm.behaviour_mut().gossipsub.publish(topic.clone(), data).unwrap();
            }
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(GossipsubEvent::Message { message, .. })) => {
                        if let Ok(msg) = bincode::deserialize::<P2PMsg>(&message.data) {
                            let hash = match &msg {
                                P2PMsg::NewTransaction(tx) => tx.hashtimer.to_vec(),
                                P2PMsg::NewBlock(block) => block.block_id.to_vec(),
                                P2PMsg::NewRound(round) => round.round_id.to_be_bytes().to_vec(),
                            };
                            if seen_hashes.insert(hash) {
                                // Signature and structure validation
                                let valid = match &msg {
                                    P2PMsg::NewTransaction(tx) => verify_transaction(tx),
                                    P2PMsg::NewBlock(block) => verify_block(block),
                                    P2PMsg::NewRound(round) => verify_round(round),
                                };
                                if !valid {
                                    println!("[P2P] Dropped invalid message: signature or structure check failed");
                                    continue;
                                }
                                match msg {
                                    P2PMsg::NewTransaction(tx) => {
                                        tx_pool.add_transaction(tx);
                                    }
                                    P2PMsg::NewBlock(block) => {
                                        if !block.validate_merkle_root() {
                                            println!("Rejected block with invalid Merkle root: {:?}", block.block_id);
                                            continue;
                                        }
                                        dag.add_block(block);
                                    }
                                    P2PMsg::NewRound(round) => {
                                        dag.add_round(round);
                                    }
                                }
                            }
                        }
                    }
                    SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(MdnsEvent::Discovered(peers))) => {
                        for (peer, _addr) in peers {
                            swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer);
                        }
                    }
                    SwarmEvent::Behaviour(MyBehaviourEvent::Kademlia(_event)) => {
                        // Handle Kademlia events
                    }
                    _ => {}
                }
            }
        }
    }
}

// Usage:
// Call run_p2p_node from your main, passing the mempool, DAG, and a seen_hashes set.
// When you create a new tx/block/round locally, send it to the P2P network via the channel.

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
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use crate::network::{Network, NetworkEvent};
use libp2p::PeerId;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn test_network_creation() -> Result<(), Box<dyn Error>> {
    let network = Network::new()?;
    // Test that we can get the protocol and discovery instances
    let _protocol = network.get_protocol();
    let _discovery = network.get_discovery();
    Ok(())
}

#[tokio::test]
async fn test_network_start() -> Result<(), Box<dyn Error>> {
    let network = Network::new()?;
    network.start("/ip4/127.0.0.1/tcp/0").await?;
    Ok(())
}

#[tokio::test]
async fn test_peer_discovery() -> Result<(), Box<dyn Error>> {
    // Create two networks
    let network1 = Network::new()?;
    let network2 = Network::new()?;

    // Start both networks
    network1.start("/ip4/127.0.0.1/tcp/0").await?;
    network2.start("/ip4/127.0.0.1/tcp/0").await?;

    // Wait for mDNS discovery
    sleep(Duration::from_secs(2)).await;

    // Check if peers are discovered
    let peers1 = network1.get_discovery().lock().await.get_peers().await?;
    let peers2 = network2.get_discovery().lock().await.get_peers().await?;

    assert!(!peers1.is_empty() || !peers2.is_empty(), "Peers should be discovered");
    Ok(())
}

#[tokio::test]
async fn test_message_broadcast() -> Result<(), Box<dyn Error>> {
    // Create two networks
    let network1 = Network::new()?;
    let network2 = Network::new()?;

    // Start both networks
    network1.start("/ip4/127.0.0.1/tcp/0").await?;
    network2.start("/ip4/127.0.0.1/tcp/0").await?;

    // Subscribe to a test topic
    let topic = "test_topic";
    network1.get_protocol().lock().await.subscribe_to_topic(topic).await?;
    network2.get_protocol().lock().await.subscribe_to_topic(topic).await?;

    // Wait for peers to connect
    sleep(Duration::from_secs(2)).await;

    // Broadcast a message
    let message = b"Hello, P2P!";
    network1.get_protocol().lock().await.broadcast_message(topic, message).await?;

    // Wait for message propagation
    sleep(Duration::from_secs(1)).await;

    Ok(())
}

// Helper function to create a test network with a specific port
async fn create_test_network(port: u16) -> Result<Network, Box<dyn Error>> {
    let network = Network::new()?;
    network.start(&format!("/ip4/127.0.0.1/tcp/{}", port)).await?;
    Ok(network)
}

#[tokio::test]
async fn test_multiple_networks() -> Result<(), Box<dyn Error>> {
    // Create three networks on different ports
    let network1 = create_test_network(8001).await?;
    let network2 = create_test_network(8002).await?;
    let network3 = create_test_network(8003).await?;

    // Wait for peer discovery
    sleep(Duration::from_secs(2)).await;

    // Check peer connections
    let peers1 = network1.get_discovery().lock().await.get_peers().await?;
    let peers2 = network2.get_discovery().lock().await.get_peers().await?;
    let peers3 = network3.get_discovery().lock().await.get_peers().await?;

    assert!(!peers1.is_empty() || !peers2.is_empty() || !peers3.is_empty(), 
        "Networks should discover each other");

    Ok(())
} 
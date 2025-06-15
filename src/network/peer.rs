use crate::{
    network::{
        discovery::DiscoveryManager,
        message::{Message, MessageType},
        transport::TransportManager,
    },
    storage::{
        asset::AssetManager,
        types::{AssetId, AssetType, PeerId},
    },
    utils::crypto::signature::SignatureManager,
};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Represents a peer in the network
pub struct Peer {
    id: PeerId,
    address: String,
    transport: Arc<TransportManager>,
    discovery: Arc<DiscoveryManager>,
    asset_manager: Arc<AssetManager>,
    signature_manager: Arc<SignatureManager>,
    connected_peers: Arc<RwLock<Vec<PeerId>>>,
}

impl Peer {
    pub fn new(
        id: PeerId,
        address: String,
        transport: Arc<TransportManager>,
        discovery: Arc<DiscoveryManager>,
        asset_manager: Arc<AssetManager>,
        signature_manager: Arc<SignatureManager>,
    ) -> Self {
        Self {
            id,
            address,
            transport,
            discovery,
            asset_manager,
            signature_manager,
            connected_peers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn connect(&self, peer_id: PeerId) -> Result<(), String> {
        let mut peers = self.connected_peers.write().await;
        if !peers.contains(&peer_id) {
            peers.push(peer_id);
        }
        Ok(())
    }

    pub async fn disconnect(&self, peer_id: PeerId) -> Result<(), String> {
        let mut peers = self.connected_peers.write().await;
        peers.retain(|p| p != &peer_id);
        Ok(())
    }

    pub async fn broadcast_asset(&self, asset_id: AssetId) -> Result<(), String> {
        let peers = self.connected_peers.read().await;
        for peer_id in peers.iter() {
            self.send_message(
                peer_id.clone(),
                MessageType::AssetBroadcast,
                asset_id.to_string(),
            )
            .await?;
        }
        Ok(())
    }

    pub async fn send_message(
        &self,
        peer_id: PeerId,
        message_type: MessageType,
        payload: String,
    ) -> Result<(), String> {
        let message = Message {
            sender: self.id.clone(),
            recipient: peer_id.clone(),
            message_type,
            payload,
            timestamp: chrono::Utc::now(),
        };

        self.transport.send_message(peer_id.clone(), message).await
    }

    pub async fn handle_message(&self, message: Message) -> Result<(), String> {
        match message.message_type {
            MessageType::AssetBroadcast => {
                let asset_id = AssetId::from_str(&message.payload)
                    .map_err(|_| "Invalid asset ID".to_string())?;
                self.asset_manager.get_asset(asset_id).await?;
            }
            MessageType::AssetRequest => {
                let asset_id = AssetId::from_str(&message.payload)
                    .map_err(|_| "Invalid asset ID".to_string())?;
                if let Ok(asset) = self.asset_manager.get_asset(asset_id).await {
                    self.send_message(
                        message.sender,
                        MessageType::AssetResponse,
                        serde_json::to_string(&asset).map_err(|e| e.to_string())?,
                    )
                    .await?;
                }
            }
            MessageType::AssetResponse => {
                let asset: AssetType = serde_json::from_str(&message.payload)
                    .map_err(|e| e.to_string())?;
                self.asset_manager.store_asset(asset).await?;
            }
            MessageType::PeerDiscovery => {
                let peer_id = PeerId::from_str(&message.payload)
                    .map_err(|_| "Invalid peer ID".to_string())?;
                self.discovery.add_peer(peer_id, String::new()).await?;
            }
        }
        Ok(())
    }
}

use std::str::FromStr;

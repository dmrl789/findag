use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time;
use libp2p::PeerId;
use tracing::{info, warn, debug};
use crate::network::error::{NetworkError, NetworkResult};

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub peer_id: PeerId,
    pub connected_at: Instant,
    pub last_activity: Instant,
    pub connection_attempts: u32,
    pub is_connected: bool,
}

pub struct ConnectionManager {
    connections: Arc<Mutex<HashMap<PeerId, ConnectionInfo>>>,
    max_connections: usize,
    connection_timeout: Duration,
    cleanup_interval: Duration,
    expiry_timeout: Duration,
}

impl ConnectionManager {
    pub fn new(
        max_connections: usize,
        connection_timeout: Duration,
        cleanup_interval: Duration,
        expiry_timeout: Duration,
    ) -> Self {
        let manager = Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
            max_connections,
            connection_timeout,
            cleanup_interval,
            expiry_timeout,
        };

        // Start cleanup task
        let manager_clone = manager.clone();
        tokio::spawn(async move {
            manager_clone.cleanup_task().await;
        });

        manager
    }

    pub async fn add_connection(&self, peer_id: PeerId) -> NetworkResult<()> {
        let mut connections = self.connections.lock().await;
        
        if connections.len() >= self.max_connections {
            return Err(NetworkError::Connection("Maximum connections reached".to_string()));
        }

        let info = ConnectionInfo {
            peer_id,
            connected_at: Instant::now(),
            last_activity: Instant::now(),
            connection_attempts: 1,
            is_connected: true,
        };

        connections.insert(peer_id, info);
        debug!("Added connection for peer: {}", peer_id);
        Ok(())
    }

    pub async fn remove_connection(&self, peer_id: PeerId) -> NetworkResult<()> {
        let mut connections = self.connections.lock().await;
        if connections.remove(&peer_id).is_some() {
            debug!("Removed connection for peer: {}", peer_id);
            Ok(())
        } else {
            Err(NetworkError::PeerNotFound(peer_id.to_string()))
        }
    }

    pub async fn update_activity(&self, peer_id: PeerId) -> NetworkResult<()> {
        let mut connections = self.connections.lock().await;
        if let Some(info) = connections.get_mut(&peer_id) {
            info.last_activity = Instant::now();
            debug!("Updated activity for peer: {}", peer_id);
            Ok(())
        } else {
            Err(NetworkError::PeerNotFound(peer_id.to_string()))
        }
    }

    pub async fn increment_connection_attempts(&self, peer_id: PeerId) -> NetworkResult<()> {
        let mut connections = self.connections.lock().await;
        if let Some(info) = connections.get_mut(&peer_id) {
            info.connection_attempts += 1;
            debug!("Incremented connection attempts for peer: {}", peer_id);
            Ok(())
        } else {
            Err(NetworkError::PeerNotFound(peer_id.to_string()))
        }
    }

    pub async fn get_connection_info(&self, peer_id: PeerId) -> Option<ConnectionInfo> {
        self.connections.lock().await.get(&peer_id).cloned()
    }

    pub async fn get_active_connections(&self) -> Vec<PeerId> {
        self.connections.lock().await
            .iter()
            .filter(|(_, info)| info.is_connected)
            .map(|(peer_id, _)| *peer_id)
            .collect()
    }

    async fn cleanup_task(&self) {
        let mut interval = time::interval(self.cleanup_interval);
        loop {
            interval.tick().await;
            self.cleanup_expired_connections().await;
        }
    }

    async fn cleanup_expired_connections(&self) {
        let mut connections = self.connections.lock().await;
        let now = Instant::now();
        
        connections.retain(|peer_id, info| {
            let should_remove = !info.is_connected && 
                (now.duration_since(info.last_activity) > self.expiry_timeout ||
                 info.connection_attempts > 5);
            
            if should_remove {
                debug!("Cleaning up expired connection for peer: {}", peer_id);
            }
            
            !should_remove
        });
    }
}

impl Clone for ConnectionManager {
    fn clone(&self) -> Self {
        Self {
            connections: self.connections.clone(),
            max_connections: self.max_connections,
            connection_timeout: self.connection_timeout,
            cleanup_interval: self.cleanup_interval,
            expiry_timeout: self.expiry_timeout,
        }
    }
} 
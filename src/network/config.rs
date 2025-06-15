use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub listen_addr: String,
    pub bootstrap_peers: Vec<String>,
    pub mdns_enabled: bool,
    pub floodsub_enabled: bool,
    pub connection_timeout: Duration,
    pub max_connections: usize,
    pub peer_cleanup_interval: Duration,
    pub peer_expiry_timeout: Duration,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_addr: "/ip4/0.0.0.0/tcp/0".to_string(),
            bootstrap_peers: vec![],
            mdns_enabled: true,
            floodsub_enabled: true,
            connection_timeout: Duration::from_secs(30),
            max_connections: 100,
            peer_cleanup_interval: Duration::from_secs(300),
            peer_expiry_timeout: Duration::from_secs(3600),
        }
    }
}

impl NetworkConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_listen_addr(mut self, addr: String) -> Self {
        self.listen_addr = addr;
        self
    }

    pub fn with_bootstrap_peers(mut self, peers: Vec<String>) -> Self {
        self.bootstrap_peers = peers;
        self
    }

    pub fn with_mdns(mut self, enabled: bool) -> Self {
        self.mdns_enabled = enabled;
        self
    }

    pub fn with_floodsub(mut self, enabled: bool) -> Self {
        self.floodsub_enabled = enabled;
        self
    }

    pub fn with_connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }

    pub fn with_max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }

    pub fn with_peer_cleanup_interval(mut self, interval: Duration) -> Self {
        self.peer_cleanup_interval = interval;
        self
    }

    pub fn with_peer_expiry_timeout(mut self, timeout: Duration) -> Self {
        self.peer_expiry_timeout = timeout;
        self
    }
} 
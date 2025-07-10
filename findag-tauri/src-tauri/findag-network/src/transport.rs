//! Network transport
//! 
//! This module handles network transport configuration and management.

use libp2p::{
    core::{upgrade, transport},
    tcp, noise, yamux, mplex,
    identity,
};
use findag_types::FindDAGResult;

/// Transport manager
pub struct TransportManager {
    /// Transport configuration
    config: TransportConfig,
}

/// Transport configuration
pub struct TransportConfig {
    /// Enable TCP
    pub enable_tcp: bool,
    /// Enable noise encryption
    pub enable_noise: bool,
    /// Enable yamux multiplexing
    pub enable_yamux: bool,
    /// Enable mplex multiplexing
    pub enable_mplex: bool,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            enable_tcp: true,
            enable_noise: true,
            enable_yamux: true,
            enable_mplex: false,
        }
    }
}

impl TransportManager {
    /// Create a new transport manager
    pub fn new(config: TransportConfig) -> Self {
        Self { config }
    }

    /// Create a transport
    pub fn create_transport(&self, local_key: &identity::Keypair) -> FindDAGResult<transport::Boxed<(PeerId, libp2p::core::muxing::StreamMuxerBox)>> {
        let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(local_key)
            .expect("Signing libp2p-noise static DH keypair failed.");
        
        let transport = transport::TokioTcpConfig::new()
            .nodelay(true)
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseAuthenticated::xx(noise_keys).unwrap())
            .multiplex(yamux::YamuxConfig::default())
            .boxed();
        
        Ok(transport)
    }

    /// Get transport configuration
    pub fn get_config(&self) -> &TransportConfig {
        &self.config
    }
} 
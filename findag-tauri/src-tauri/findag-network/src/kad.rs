//! Kademlia DHT
//! 
//! This module handles distributed hash table operations using Kademlia.

use libp2p::{
    core::PeerId,
    kad::{Kademlia, KademliaEvent, QueryResult, Record, RecordKey},
    swarm::NetworkBehaviour,
};
use findag_types::FindDAGResult;

/// Kademlia manager
pub struct KademliaManager {
    /// Kademlia behaviour
    kad: Kademlia,
}

impl KademliaManager {
    /// Create a new Kademlia manager
    pub fn new(local_peer_id: PeerId) -> Self {
        let kad = Kademlia::new(local_peer_id, Default::default());
        
        Self { kad }
    }

    /// Bootstrap the DHT
    pub fn bootstrap(&mut self) -> FindDAGResult<()> {
        self.kad.bootstrap()?;
        Ok(())
    }

    /// Store a record in the DHT
    pub fn store_record(&mut self, key: RecordKey, value: Vec<u8>) -> FindDAGResult<()> {
        let record = Record {
            key,
            value,
            publisher: None,
            expires: None,
        };
        
        self.kad.start_providing(record.key.clone())?;
        self.kad.put_record(record, libp2p::kad::Quorum::One)?;
        
        Ok(())
    }

    /// Get a record from the DHT
    pub fn get_record(&mut self, key: RecordKey) -> FindDAGResult<()> {
        self.kad.get_record(key, libp2p::kad::Quorum::One)?;
        Ok(())
    }

    /// Handle Kademlia events
    pub fn handle_event(&mut self, event: KademliaEvent) -> FindDAGResult<()> {
        match event {
            KademliaEvent::OutboundQueryCompleted { result, .. } => {
                match result {
                    QueryResult::Bootstrap(ok) => {
                        if ok {
                            tracing::info!("Kademlia bootstrap completed successfully");
                        }
                    }
                    QueryResult::GetRecord(Ok(records)) => {
                        for (key, record) in records {
                            tracing::info!("Retrieved record: {:?}", key);
                        }
                    }
                    QueryResult::PutRecord(Ok(_)) => {
                        tracing::info!("Record stored successfully");
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        
        Ok(())
    }
} 
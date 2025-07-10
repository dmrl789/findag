//! HashTimer implementation
//! 
//! This module implements the HashTimer functionality for deterministic
//! ordering in the FinDAG blockchain.

use findag_core::{Address, Hash, FinDAGTime};
use findag_types::{FindDAGResult, FindDAGError};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use metrics::{counter, histogram};
use rand::Rng;
use chrono::{DateTime, Utc};

/// HashTimer structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HashTimer {
    /// FinDAG time component
    pub findag_time: u64,
    /// Content hash component
    pub content_hash: Hash,
    /// Nonce component for uniqueness
    pub nonce: u32,
    /// Final hash
    pub hash: Hash,
}

/// HashTimer Manager
pub struct HashTimerManager {
    /// Current FinDAG time
    current_time: Arc<RwLock<FinDAGTime>>,
    /// Nonce range for HashTimer generation
    nonce_range: u32,
    /// HashTimer generation rate
    generation_rate: Arc<RwLock<f64>>,
    /// Last generation time
    last_generation: Arc<RwLock<DateTime<Utc>>>,
    /// Total HashTimers generated
    total_generated: Arc<RwLock<u64>>,
}

impl HashTimerManager {
    /// Create a new HashTimer Manager
    pub fn new(config: crate::TimeManagerConfig) -> Self {
        let current_time = Arc::new(RwLock::new(FinDAGTime::now()));
        let generation_rate = Arc::new(RwLock::new(0.0));
        let last_generation = Arc::new(RwLock::new(Utc::now()));
        let total_generated = Arc::new(RwLock::new(0));
        
        Self {
            current_time,
            nonce_range: config.hashtimer_nonce_range,
            generation_rate,
            last_generation,
            total_generated,
        }
    }

    /// Start the HashTimer manager
    pub async fn start(&self) -> FindDAGResult<()> {
        info!("Starting HashTimer Manager");
        
        // Initialize metrics
        gauge!("findag_hashtimer_generation_rate", 0.0);
        gauge!("findag_hashtimer_total_generated", 0.0);
        
        Ok(())
    }

    /// Stop the HashTimer manager
    pub async fn stop(&self) -> FindDAGResult<()> {
        info!("Stopping HashTimer Manager");
        
        // Update final metrics
        let total = *self.total_generated.read().await;
        gauge!("findag_hashtimer_total_generated", total as f64);
        
        Ok(())
    }

    /// Generate a new HashTimer
    pub async fn generate_hashtimer(&self, content_hash: Hash) -> FindDAGResult<HashTimer> {
        let start_time = std::time::Instant::now();
        
        // Get current FinDAG time
        let findag_time = {
            let time = self.current_time.read().await;
            time.as_u64()
        };
        
        // Generate random nonce
        let nonce = rand::thread_rng().gen_range(0..self.nonce_range);
        
        // Create HashTimer
        let hashtimer = HashTimer::new(findag_time, content_hash, nonce)?;
        
        // Update metrics
        let generation_time = start_time.elapsed();
        histogram!("findag_hashtimer_generation_time_ms", generation_time.as_millis() as f64);
        
        // Update generation rate
        {
            let mut rate = self.generation_rate.write().await;
            let mut last_gen = self.last_generation.write().await;
            let now = Utc::now();
            
            let time_diff = (now - *last_gen).num_milliseconds() as f64 / 1000.0;
            if time_diff > 0.0 {
                *rate = 1.0 / time_diff;
            }
            
            *last_gen = now;
        }
        
        // Update total generated
        {
            let mut total = self.total_generated.write().await;
            *total += 1;
            gauge!("findag_hashtimer_total_generated", *total as f64);
        }
        
        // Update generation rate metric
        {
            let rate = self.generation_rate.read().await;
            gauge!("findag_hashtimer_generation_rate", *rate);
        }
        
        debug!("Generated HashTimer: {:?}", hashtimer);
        
        Ok(hashtimer)
    }

    /// Verify a HashTimer
    pub async fn verify_hashtimer(&self, hashtimer: &HashTimer) -> FindDAGResult<bool> {
        let start_time = std::time::Instant::now();
        
        // Recompute hash to verify
        let computed_hash = hashtimer.compute_hash()?;
        let is_valid = hashtimer.hash == computed_hash;
        
        // Record verification time
        let verification_time = start_time.elapsed();
        histogram!("findag_hashtimer_verification_time_ms", verification_time.as_millis() as f64);
        
        if is_valid {
            counter!("findag_hashtimer_verifications_successful", 1);
        } else {
            counter!("findag_hashtimer_verifications_failed", 1);
        }
        
        Ok(is_valid)
    }

    /// Get HashTimer generation rate
    pub async fn get_generation_rate(&self) -> f64 {
        let rate = self.generation_rate.read().await;
        *rate
    }

    /// Get total HashTimers generated
    pub async fn get_total_generated(&self) -> u64 {
        let total = self.total_generated.read().await;
        *total
    }

    /// Update current FinDAG time
    pub async fn update_current_time(&self, time: FinDAGTime) {
        let mut current_time = self.current_time.write().await;
        *current_time = time;
    }
}

impl HashTimer {
    /// Create a new HashTimer
    pub fn new(findag_time: u64, content_hash: Hash, nonce: u32) -> FindDAGResult<Self> {
        let mut hashtimer = Self {
            findag_time,
            content_hash,
            nonce,
            hash: Hash::default(),
        };
        
        // Compute hash
        hashtimer.hash = hashtimer.compute_hash()?;
        
        Ok(hashtimer)
    }

    /// Compute the HashTimer hash
    pub fn compute_hash(&self) -> FindDAGResult<Hash> {
        let mut hasher = Sha256::new();
        
        // Hash the three components: FinDAG time, content hash, and nonce
        hasher.update(self.findag_time.to_le_bytes());
        hasher.update(self.content_hash.as_bytes());
        hasher.update(self.nonce.to_le_bytes());
        
        let result = hasher.finalize();
        let hash_bytes: [u8; 32] = result.into();
        
        Ok(Hash::from_bytes(hash_bytes))
    }

    /// Get the FinDAG time component
    pub fn get_findag_time(&self) -> u64 {
        self.findag_time
    }

    /// Get the content hash component
    pub fn get_content_hash(&self) -> Hash {
        self.content_hash
    }

    /// Get the nonce component
    pub fn get_nonce(&self) -> u32 {
        self.nonce
    }

    /// Get the final hash
    pub fn get_hash(&self) -> Hash {
        self.hash
    }

    /// Convert to FinDAG time
    pub fn to_findag_time(&self) -> FinDAGTime {
        FinDAGTime::from_u64(self.findag_time)
    }

    /// Check if this HashTimer is valid
    pub fn is_valid(&self) -> FindDAGResult<bool> {
        let computed_hash = self.compute_hash()?;
        Ok(self.hash == computed_hash)
    }

    /// Get HashTimer as bytes for serialization
    pub fn to_bytes(&self) -> FindDAGResult<Vec<u8>> {
        let mut bytes = Vec::new();
        
        // Serialize components
        bytes.extend_from_slice(&self.findag_time.to_le_bytes());
        bytes.extend_from_slice(self.content_hash.as_bytes());
        bytes.extend_from_slice(&self.nonce.to_le_bytes());
        bytes.extend_from_slice(self.hash.as_bytes());
        
        Ok(bytes)
    }

    /// Create HashTimer from bytes
    pub fn from_bytes(bytes: &[u8]) -> FindDAGResult<Self> {
        if bytes.len() != 72 { // 8 + 32 + 4 + 32 = 76 bytes
            return Err(FindDAGError::InvalidFormat("Invalid HashTimer bytes length".to_string()));
        }
        
        let findag_time = u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]
        ]);
        
        let content_hash_bytes: [u8; 32] = bytes[8..40].try_into()
            .map_err(|_| FindDAGError::InvalidFormat("Invalid content hash bytes".to_string()))?;
        let content_hash = Hash::from_bytes(content_hash_bytes);
        
        let nonce = u32::from_le_bytes([
            bytes[40], bytes[41], bytes[42], bytes[43]
        ]);
        
        let hash_bytes: [u8; 32] = bytes[44..76].try_into()
            .map_err(|_| FindDAGError::InvalidFormat("Invalid hash bytes".to_string()))?;
        let hash = Hash::from_bytes(hash_bytes);
        
        Ok(Self {
            findag_time,
            content_hash,
            nonce,
            hash,
        })
    }
}

impl std::fmt::Display for HashTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HashTimer(time={}, content={}, nonce={}, hash={})",
            self.findag_time,
            self.content_hash,
            self.nonce,
            self.hash
        )
    }
}

impl PartialOrd for HashTimer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HashTimer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary ordering by FinDAG time
        match self.findag_time.cmp(&other.findag_time) {
            std::cmp::Ordering::Equal => {
                // Secondary ordering by content hash
                match self.content_hash.cmp(&other.content_hash) {
                    std::cmp::Ordering::Equal => {
                        // Tertiary ordering by nonce
                        self.nonce.cmp(&other.nonce)
                    }
                    other => other,
                }
            }
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use findag_core::Hash;

    #[test]
    fn test_hashtimer_creation() {
        let content_hash = Hash::random();
        let hashtimer = HashTimer::new(12345, content_hash, 42).unwrap();
        
        assert_eq!(hashtimer.findag_time, 12345);
        assert_eq!(hashtimer.content_hash, content_hash);
        assert_eq!(hashtimer.nonce, 42);
        assert!(hashtimer.is_valid().unwrap());
    }

    #[test]
    fn test_hashtimer_ordering() {
        let content_hash = Hash::random();
        
        let ht1 = HashTimer::new(1000, content_hash, 1).unwrap();
        let ht2 = HashTimer::new(1000, content_hash, 2).unwrap();
        let ht3 = HashTimer::new(1001, content_hash, 1).unwrap();
        
        assert!(ht1 < ht2);
        assert!(ht2 < ht3);
        assert!(ht1 < ht3);
    }

    #[test]
    fn test_hashtimer_serialization() {
        let content_hash = Hash::random();
        let original = HashTimer::new(12345, content_hash, 42).unwrap();
        
        let bytes = original.to_bytes().unwrap();
        let deserialized = HashTimer::from_bytes(&bytes).unwrap();
        
        assert_eq!(original, deserialized);
    }
} 
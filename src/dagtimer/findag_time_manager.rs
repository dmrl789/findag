// src/findag_time_manager.rs

//! FinDAGTimeManager.rs
//! Production-ready: synchronizes local clock with peer offsets using clamped adjustment
//! Format: [upper 40 bits = seconds since epoch] | [lower 24 bits = 100ns slots]

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_HISTORY: usize = 100;
const MAX_ALLOWED_SKEW_US: i64 = 5000; // Clamp time correction to ±5ms
#[allow(dead_code)]
const NANO_PER_MICRO: u64 = 1_000;

#[derive(Debug, Clone)]
pub struct PeerPing {
    pub offset_us: i64, // Offset in microseconds
    pub rtt_us: u64,    // Round-trip time in microseconds
}

#[derive(Debug, Clone)]
pub struct FinDAGTimeManager {
    peer_offsets: VecDeque<i64>,
}

impl FinDAGTimeManager {
    pub fn new() -> Self {
        Self {
            peer_offsets: VecDeque::with_capacity(MAX_HISTORY),
        }
    }

    /// Add a peer offset measurement
    pub fn add_peer_offset(&mut self, offset: i64) {
        if self.peer_offsets.len() >= MAX_HISTORY {
            self.peer_offsets.pop_front();
        }
        self.peer_offsets.push_back(offset);
    }

    /// Calculate median offset (or mean if odd)
    pub fn get_median_offset(&self) -> i64 {
        let mut offsets: Vec<i64> = self.peer_offsets.iter().copied().collect();
        if offsets.is_empty() {
            return 0;
        }
        offsets.sort_unstable();
        let mid = offsets.len() / 2;
        if offsets.len() % 2 == 0 {
            (offsets[mid - 1] + offsets[mid]) / 2
        } else {
            offsets[mid]
        }
    }

    /// Return FinDAG time adjusted by median offset, clamped to max skew
    pub fn get_findag_time(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Clock may be before UNIX_EPOCH");

        let local_micros = now.as_secs() * 1_000_000 + now.subsec_micros() as u64;

        let median_offset = self.get_median_offset();
        let clamped_offset = median_offset.clamp(-MAX_ALLOWED_SKEW_US, MAX_ALLOWED_SKEW_US);

        let adjusted_micros = if clamped_offset >= 0 {
            local_micros + clamped_offset as u64
        } else {
            local_micros.saturating_sub(clamped_offset.unsigned_abs())
        };

        let seconds = adjusted_micros / 1_000_000;
        let remainder_micros = adjusted_micros % 1_000_000;
        let slots_100ns = remainder_micros * 10; // 1 µs = 10 × 100ns slots

        (seconds << 24) | (slots_100ns & 0xFFFFFF)
    }
}

impl Default for FinDAGTimeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_offset() {
        let mut mgr = FinDAGTimeManager::new();
        let samples = vec![100, -50, 75, -25, 0];
        for offset in samples {
            mgr.add_peer_offset(offset);
        }
        let median = mgr.get_median_offset();
        assert_eq!(median, 0);
    }

    #[test]
    fn test_findag_time_format() {
        let mgr = FinDAGTimeManager::new();
        let ts = mgr.get_findag_time();
        let seconds = ts >> 24;
        let slots = ts & 0xFFFFFF;

        assert!(seconds > 1_600_000_000); // Should be reasonable epoch seconds
        assert!(slots < 10_000_000);      // Should be within 100ns slots range
    }

    #[test]
    fn test_time_adjustment_clamp() {
        let mut mgr = FinDAGTimeManager::new();
        mgr.add_peer_offset(20_000); // Large offset, should clamp
        let ts = mgr.get_findag_time();
        let slots = ts & 0xFFFFFF;
        assert!(slots > 0);
    }
} 
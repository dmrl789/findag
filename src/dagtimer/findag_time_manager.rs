// FinDAGTimeManager.rs
// A simplified version of FinDAG Time with peer offset tracking, median filtering, and time smoothing

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_HISTORY: usize = 16;
const MAX_ALLOWED_SKEW_US: i64 = 5000; // Maximum allowed time adjustment in microseconds
const NANO_PER_MICRO: u64 = 1_000;
const NANO_PER_100NS: u64 = 100;

#[derive(Debug, Clone)]
pub struct PeerPing {
    pub offset_us: i64, // Microseconds offset from local clock
    pub rtt_us: u64,    // Round-trip time in microseconds
}

#[derive(Debug, Clone)]
pub struct FinDAGTimeManager {
    peer_offsets: VecDeque<i64>,
    last_adjusted_time: SystemTime,
}

impl FinDAGTimeManager {
    pub fn new() -> Self {
        Self {
            peer_offsets: VecDeque::with_capacity(MAX_HISTORY),
            last_adjusted_time: SystemTime::now(),
        }
    }

    pub fn record_peer_ping(&mut self, ping: PeerPing) {
        if ping.rtt_us > 5000 {
            return; // Ignore unstable peers
        }

        if self.peer_offsets.len() == MAX_HISTORY {
            self.peer_offsets.pop_front();
        }
        self.peer_offsets.push_back(ping.offset_us);
    }

    pub fn get_findag_time(&self) -> u64 {
        let median_offset = self.compute_median_offset();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time before UNIX EPOCH");

        let mut adjusted_us = (now.as_secs() as i64 * 1_000_000) + (now.subsec_micros() as i64);

        // Apply bounded offset adjustment
        let bounded_offset = median_offset.clamp(-MAX_ALLOWED_SKEW_US, MAX_ALLOWED_SKEW_US);
        adjusted_us += bounded_offset;

        let seconds = (adjusted_us / 1_000_000) as u64;
        let hundred_ns_slot = ((adjusted_us % 1_000_000) as u64 * 10) % (1 << 24);

        (seconds << 24) | (hundred_ns_slot & 0xFFFFFF)
    }

    fn compute_median_offset(&self) -> i64 {
        let mut offsets: Vec<i64> = self.peer_offsets.clone().into();
        if offsets.is_empty() {
            return 0;
        }
        offsets.sort();
        let mid = offsets.len() / 2;
        if offsets.len() % 2 == 0 {
            (offsets[mid - 1] + offsets[mid]) / 2
        } else {
            offsets[mid]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_offset_calculation() {
        let mut mgr = FinDAGTimeManager::new();
        let samples = vec![-20, -10, -5, 0, 5, 10, 10000];

        for offset in samples {
            mgr.record_peer_ping(PeerPing {
                offset_us: offset,
                rtt_us: 100,
            });
        }

        let median = mgr.compute_median_offset();
        assert_eq!(median, 0);
    }

    #[test]
    fn test_findag_time_format() {
        let mgr = FinDAGTimeManager::new();
        let ts = mgr.get_findag_time();
        let seconds = ts >> 24;
        let hundred_ns = ts & 0xFFFFFF;

        assert!(seconds > 1_600_000_000); // Reasonable timestamp
        assert!(hundred_ns < 10_000_000); // 100ns slots
    }
} 
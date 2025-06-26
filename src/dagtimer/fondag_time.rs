use std::time::{SystemTime, UNIX_EPOCH};

/// FinDAG Time is a 64-bit timestamp:
/// [ upper 40 bits = seconds since epoch ]
/// [ lower 24 bits = 100ns slots within the second (max 10M per second) ]
pub fn get_findag_time() -> u64 {
    // Get time since UNIX epoch
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX_EPOCH");

    let seconds = now.as_secs();                  // u64: seconds since epoch
    let nanos = now.subsec_nanos() as u64;        // nanoseconds part (0..1_000_000_000)

    let hundred_ns_slots = nanos / 100;           // Convert to 100ns intervals (0..10_000_000)

    // Combine seconds and sub-second slot into 64-bit timestamp
    (seconds << 24) | (hundred_ns_slots & 0xFFFFFF)
} 
use sha2::{Sha256, Digest};

/// Converts an IPFS CID and FinDAG time (in microseconds) into a 32-byte HashTimer
pub fn generate_hashtimer(ipfs_cid: &str, time_microseconds: u128) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(ipfs_cid.as_bytes());
    hasher.update(&time_microseconds.to_be_bytes());
    let result = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(&result[..]);
    output
}

/// Converts a HashTimer to its hexadecimal representation
pub fn hashtimer_to_hex(timer: &[u8; 32]) -> String {
    timer.iter().map(|b| format!("{:02x}", b)).collect()
}

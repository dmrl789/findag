use sha2::{Sha256, Digest};

/// Computes a HashTimer value:
/// - `findag_time`: 64-bit FinDAG Time (see findag_time.rs)
/// - `tx_hash`: transaction hash or arbitrary data
/// - `nonce`: optional nonce for uniqueness
/// Returns a 32-byte SHA-256 hash
pub fn compute_hashtimer(findag_time: u64, tx_hash: &[u8], nonce: u32) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&findag_time.to_be_bytes());
    hasher.update(tx_hash);
    hasher.update(&nonce.to_be_bytes());
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
} 
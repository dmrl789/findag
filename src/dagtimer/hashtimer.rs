use sha2::{Sha256, Digest};

/// Computes a HashTimer value based on FinDAG Time, content, and nonce.
///
/// Returns a 32-byte SHA-256 hash
pub fn compute_hashtimer(findag_time: u64, content: &[u8], nonce: u32) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(findag_time.to_be_bytes());
    hasher.update(content);
    hasher.update(nonce.to_be_bytes());
    let result = hasher.finalize();
    let mut hashtimer = [0u8; 32];
    hashtimer.copy_from_slice(&result);
    hashtimer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashtimer_deterministic() {
        let t = 1234567890u64;
        let content = b"example";
        let nonce = 42u32;
        let hash1 = compute_hashtimer(t, content, nonce);
        let hash2 = compute_hashtimer(t, content, nonce);
        assert_eq!(hash1, hash2);
    }
} 
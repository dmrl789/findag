use anyhow::Result;

/// Trait for a generic cross-chain settlement proof.
pub trait SettlementProof {
    /// Verify the proof cryptographically.
    fn verify(&self, trusted_pubkey: &[u8]) -> Result<()>;

    /// Return the state hash or root that the proof represents.
    fn state_hash(&self) -> &[u8];
} 
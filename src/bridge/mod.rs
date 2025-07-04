// src/bridge/mod.rs

pub mod api;
pub mod corda;
pub mod fabric;
pub mod proofs;

#[cfg(test)]
mod tests {
    use super::corda::CordaSettlementProof;
    use super::fabric::FabricEndorsementProof;
    use super::proofs::SettlementProof;

    #[test]
    fn test_corda_proof() {
        let proof = CordaSettlementProof {
            state_hash: vec![1, 2, 3, 4],
            notary_signature: vec![9, 9, 9],
        };
        let trusted_key = vec![0u8; 32];
        proof.verify(&trusted_key).unwrap();
        assert_eq!(proof.state_hash(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_fabric_proof() {
        let proof = FabricEndorsementProof {
            state_root: vec![5, 6, 7, 8],
            endorsement_signatures: vec![vec![8, 8, 8]],
        };
        let trusted_keys = vec![1u8; 32];
        proof.verify(&trusted_keys).unwrap();
        assert_eq!(proof.state_hash(), &[5, 6, 7, 8]);
    }
} 
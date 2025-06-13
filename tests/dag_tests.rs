#[cfg(test)]
mod tests {
    use findag_core::blockchain::dag::DagState;
    use findag_core::types::block::{Block, RoundHash};

    #[test]
    fn test_dag_integration() {
        let mut dag = DagState::new();

        let block1 = Block {
            hash: "hash1".into(),
            parents: vec![],
            timestamp: 1,
            data: "genesis".into(),
            signature: "sig1".into(),
        };

        let block2 = Block {
            hash: "hash2".into(),
            parents: vec!["hash1".into()],
            timestamp: 2,
            data: "second block".into(),
            signature: "sig2".into(),
        };

        dag.insert_block(block1.clone());
        dag.insert_block(block2.clone());

        assert_eq!(dag.blocks.len(), 2);
        assert_eq!(dag.edges.get("hash1").unwrap().contains("hash2"), true);

        let round = RoundHash {
            round_id: 1,
            block_hashes: vec!["hash1".into(), "hash2".into()],
            timestamp: 3,
            validator_signature: "sigR".into(),
        };

        dag.insert_round(round);
        assert_eq!(dag.round_hashes.len(), 1);
    }
}

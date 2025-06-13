#[cfg(test)]
mod tests {
    use findag_core::blockchain::block::Block;
    use findag_core::consensus::round::Round;

    #[test]
    fn test_round_validation() {
        let block = Block::new("123".into(), vec![]);
        let round = Round::new("round1".into(), "validator1".into(), vec![block]);
        assert!(round.validate_blocks());
    }

    #[test]
    fn test_round_serialization() {
        let block = Block::new("123".into(), vec![]);
        let round = Round::new("round2".into(), "validator2".into(), vec![block]);
        let bytes = round.to_bytes();
        let decoded = Round::from_bytes(&bytes);
        assert_eq!(decoded.round_id, "round2");
    }
}

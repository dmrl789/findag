use proptest::prelude::*;
use findag::consensus::validator_set::{ValidatorSet, ValidatorInfo, ValidatorStatus};
use findag::consensus::round_finalizer::RoundFinalizer;
use findag::core::address::{generate_address, Address};
use ed25519_dalek::Keypair;

proptest! {
    #[test]
    fn round_assignment_is_deterministic_and_fair(
        num_validators in 2usize..10,
        round_number in 0u64..1000,
    ) {
        let mut set = ValidatorSet::new();
        let mut keypairs = Vec::new();
        for _ in 0..num_validators {
            let (keypair, address) = generate_address();
            set.add_validator(ValidatorInfo {
                address: address.clone(),
                public_key: keypair.public,
                status: ValidatorStatus::Active,
                metadata: None,
            });
            keypairs.push((keypair, address));
        }
        // Use the first validator as local for the test
        let (local_keypair, _local_address) = &keypairs[0];
        let round_finalizer = RoundFinalizer::new(&set, local_keypair.clone());
        // Assignment should be deterministic
        let active = set.active_validators();
        let assigned_index = (round_number as usize) % active.len();
        let assigned_address = &active[assigned_index].address;
        // The round_finalizer should agree
        let is_finalizer = round_finalizer.is_finalizer_for_round(round_number);
        let local_id = round_finalizer.local_id();
        let expected = assigned_address.as_str() == local_id;
        assert_eq!(is_finalizer, expected);
    }
} 
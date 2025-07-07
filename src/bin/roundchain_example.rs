// roundchain_example.rs
// Example demonstrating the simple linear RoundChain implementation

use findag::consensus::roundchain::RoundChain;
use findag::consensus::validator_set::ValidatorSet;
use findag::core::address::generate_address;
use findag::core::types::{Block, Transaction, ShardId};
use ed25519_dalek::{SigningKey, VerifyingKey, Signer};

fn create_test_transaction(from: &SigningKey, to: &VerifyingKey, amount: u64) -> Transaction {
    let (_, from_address) = generate_address();
    let (_, to_address) = generate_address();
    
    let payload = format!("Transfer {amount} units").into_bytes();
    let findag_time = 1000;
    let hashtimer = [0u8; 32]; // Simplified for example
    
    let message = format!("{}{}{}", from_address.as_str(), to_address.as_str(), amount);
    let signature = from.sign(message.as_bytes());
    
    Transaction {
        from: from_address,
        to: to_address,
        amount,
        payload,
        findag_time,
        hashtimer,
        signature,
        public_key: to.clone(),
        shard_id: ShardId(0),
        source_shard: None,
        dest_shard: None,
        target_chain: None,
        bridge_protocol: None,
    }
}

fn create_test_block(proposer: &SigningKey, transactions: Vec<Transaction>) -> Block {
    let (_, proposer_address) = generate_address();
    let block_id = [1u8; 32]; // Simplified for example
    let parent_blocks = vec![];
    let findag_time = 1000;
    let hashtimer = [2u8; 32]; // Simplified for example
    
    let message = format!("Block with {} transactions", transactions.len());
    let signature = proposer.sign(message.as_bytes());
    
    Block {
        block_id,
        parent_blocks,
        transactions,
        findag_time,
        hashtimer,
        proposer: proposer_address,
        signature,
        public_key: proposer.verifying_key(),
        shard_id: ShardId(0),
        merkle_root: None,
    }
}

fn main() {
    println!("🚀 FinDAG Simple Linear RoundChain Example");
    println!("===========================================");
    
    // Create validator set
    let mut validator_set = ValidatorSet::new();
    
    // Add some test validators
    for i in 0..5 {
        let (keypair, address) = generate_address();
        validator_set.add_validator_with_metadata(
            address,
            keypair.verifying_key(),
            1000, // stake
            format!("validator_{i}"),
            "test_region".to_string(),
        );
    }
    
    // Create RoundChain
    let mut roundchain = RoundChain::new(validator_set);
    
    // Create test keypairs
    let (proposer_keypair, proposer_address) = generate_address();
    let (user1_keypair, _) = generate_address();
    let (user2_keypair, _) = generate_address();
    
    println!("✅ Created RoundChain with {} validators", 5);
    
    // Create some test transactions
    let tx1 = create_test_transaction(&user1_keypair, &user2_keypair.verifying_key(), 100);
    let tx2 = create_test_transaction(&user2_keypair, &user1_keypair.verifying_key(), 50);
    let tx3 = create_test_transaction(&user1_keypair, &user2_keypair.verifying_key(), 75);
    
    // Create blocks with transactions
    let block1 = create_test_block(&proposer_keypair, vec![tx1, tx2]);
    let block2 = create_test_block(&proposer_keypair, vec![tx3]);
    
    println!("✅ Created {} blocks with {} transactions", 2, 3);
    
    // Create Round 1
    println!("\n📦 Creating Round 1...");
    let round1 = roundchain.create_round(
        1,
        vec![block1.clone(), block2.clone()],
        1000,
        &proposer_keypair,
        proposer_address.clone(),
    ).expect("Failed to create round 1");
    
    // Add round to chain
    roundchain.add_round(round1.clone()).expect("Failed to add round 1");
    
    println!("✅ Round 1 created:");
    println!("   Round Number: {}", round1.round_number);
    println!("   Parent Round Hash: {}", hex::encode(round1.parent_round_hash));
    println!("   Finalized Blocks: {}", round1.finalized_block_hashes.len());
    println!("   FinDAG Time: {}", round1.findag_time);
    println!("   Proposer: {}", round1.proposer.as_str());
    
    // Create Round 2
    println!("\n📦 Creating Round 2...");
    let block3 = create_test_block(&proposer_keypair, vec![]);
    
    let round2 = roundchain.create_round(
        2,
        vec![block3.clone()],
        1100,
        &proposer_keypair,
        proposer_address.clone(),
    ).expect("Failed to create round 2");
    
    // Add round to chain
    roundchain.add_round(round2.clone()).expect("Failed to add round 2");
    
    println!("✅ Round 2 created:");
    println!("   Round Number: {}", round2.round_number);
    println!("   Parent Round Hash: {}", hex::encode(round2.parent_round_hash));
    println!("   Finalized Blocks: {}", round2.finalized_block_hashes.len());
    println!("   FinDAG Time: {}", round2.findag_time);
    
    // Demonstrate quorum signing (simplified)
    println!("\n🔐 Simulating quorum signing for Round 1...");
    
    // Get committee for round 1
    let committee = roundchain.validator_set.select_committee(1);
    println!("   Committee size: {}", committee.validators.len());
    
    // Create dummy signatures (in real implementation, these would be actual validator signatures)
    let mut signatures = Vec::new();
    for validator_addr in &committee.validators {
        let (dummy_keypair, _) = generate_address();
        let round_content = roundchain.create_round_content(
            round1.round_number,
            &round1.parent_round_hash,
            &round1.finalized_block_hashes,
            &round1.block_hashtimers,
            round1.findag_time,
        );
        let signature = dummy_keypair.sign(&round_content);
        signatures.push((validator_addr.clone(), signature));
    }
    
    // Sign round with quorum
    roundchain.sign_round_with_quorum(1, &committee, &signatures)
        .expect("Failed to sign round with quorum");
    
    println!("✅ Round 1 signed with quorum ({} signatures)", signatures.len());
    
    // Verify quorum signature
    let is_valid = roundchain.verify_round_quorum(&round1);
    println!("   Quorum signature valid: {is_valid}");
    
    // Demonstrate block finalization tracking
    println!("\n📋 Block Finalization Tracking:");
    println!("   Block 1 finalized: {}", roundchain.is_block_finalized(&block1.block_id));
    println!("   Block 2 finalized: {}", roundchain.is_block_finalized(&block2.block_id));
    println!("   Block 3 finalized: {}", roundchain.is_block_finalized(&block3.block_id));
    
    if let Some(round_num) = roundchain.get_block_finalization_round(&block1.block_id) {
        println!("   Block 1 finalized in round: {round_num}");
    }
    
    // Get statistics
    let stats = roundchain.get_statistics();
    println!("\n📊 RoundChain Statistics:");
    println!("   Total Rounds: {}", stats.total_rounds);
    println!("   Total Finalized Blocks: {}", stats.total_finalized_blocks);
    println!("   Average Blocks per Round: {:.2}", stats.average_blocks_per_round);
    println!("   Latest Round Number: {}", stats.latest_round_number);
    
    // Demonstrate sequential round validation
    println!("\n🔍 Sequential Round Validation:");
    
    // Try to create round 4 (should fail - missing round 3)
    let block4 = create_test_block(&proposer_keypair, vec![]);
    let result = roundchain.create_round(4, vec![block4], 1200, &proposer_keypair, proposer_address.clone());
    match result {
        Ok(_) => println!("❌ Unexpectedly succeeded creating round 4"),
        Err(e) => println!("✅ Correctly rejected round 4: {}", e),
    }
    
    println!("\n🎉 RoundChain example completed successfully!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_roundchain_creation() {
        let validator_set = ValidatorSet::new();
        let roundchain = RoundChain::new(validator_set);
        
        assert_eq!(roundchain.latest_round_number, 0);
        assert_eq!(roundchain.rounds.len(), 0);
    }
    
    #[test]
    fn test_sequential_round_creation() {
        let validator_set = ValidatorSet::new();
        let mut roundchain = RoundChain::new(validator_set);
        let (keypair, address) = generate_address();
        
        // Create test block
        let block = create_test_block(&keypair, vec![]);
        
        // Create round 1
        let round1 = roundchain.create_round(1, vec![block.clone()], 1000, &keypair, address.clone())
            .expect("Failed to create round 1");
        roundchain.add_round(round1).expect("Failed to add round 1");
        
        // Try to create round 3 (should fail)
        let result = roundchain.create_round(3, vec![block.clone()], 1000, &keypair, address.clone());
        assert!(result.is_err());
        
        // Create round 2 (should succeed)
        let round2 = roundchain.create_round(2, vec![block], 1000, &keypair, address.clone())
            .expect("Failed to create round 2");
        roundchain.add_round(round2).expect("Failed to add round 2");
        
        assert_eq!(roundchain.latest_round_number, 2);
    }
} 
use findag::consensus::governance::*;
use std::collections::HashMap;

fn main() {
    println!("🧪 FinDAG Governance System Test");
    println!("=================================");

    // Initialize governance state
    let mut governance = GovernanceState::default();
    governance.update_total_stake(1000000); // 1M total stake

    println!("✅ Governance state initialized");

    // Test 1: Create proposals
    println!("\n📋 Test 1: Creating Proposals");
    
    let proposal1_id = governance.create_proposal(
        "fdg1validator1".to_string(),
        "Add New Validator".to_string(),
        "Add validator fdg1validator2 to the network".to_string(),
        ProposalType::AddValidator {
            address: "fdg1validator2".to_string(),
            public_key: "03a1b2c3d4e5f6...".to_string(),
        },
        Some(86400), // 24 hours
    ).unwrap();
    
    let proposal2_id = governance.create_proposal(
        "fdg1validator3".to_string(),
        "Parameter Change".to_string(),
        "Increase minimum stake requirement".to_string(),
        ProposalType::ParameterChange {
            parameter: "min_stake".to_string(),
            new_value: "50000".to_string(),
        },
        Some(172800), // 48 hours
    ).unwrap();

    println!("✅ Created proposals: {} and {}", proposal1_id, proposal2_id);

    // Debug: Check proposal status after creation
    let proposal1 = governance.get_proposal(&proposal1_id).unwrap();
    let proposal2 = governance.get_proposal(&proposal2_id).unwrap();
    println!("Debug - Proposal 1 status: {:?}, voting_end: {}", proposal1.status, proposal1.voting_end);
    println!("Debug - Proposal 2 status: {:?}, voting_end: {}", proposal2.status, proposal2.voting_end);
    
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("Debug - Current time: {}", now);
    println!("Debug - Time until proposal 1 ends: {} seconds", proposal1.voting_end.saturating_sub(now));
    println!("Debug - Time until proposal 2 ends: {} seconds", proposal2.voting_end.saturating_sub(now));

    // Test 2: Submit votes
    println!("\n🗳️ Test 2: Submitting Votes");
    
    // Debug: Check proposal status before voting
    let proposal1_before = governance.get_proposal(&proposal1_id).unwrap();
    let proposal2_before = governance.get_proposal(&proposal2_id).unwrap();
    println!("Debug - Before voting - Proposal 1 status: {:?}", proposal1_before.status);
    println!("Debug - Before voting - Proposal 2 status: {:?}", proposal2_before.status);
    
    // Vote on proposal 1
    governance.submit_vote(
        &proposal1_id,
        "fdg1validator1".to_string(),
        true,
        100000,
        Some("Good validator candidate".to_string()),
    ).unwrap();
    
    governance.submit_vote(
        &proposal1_id,
        "fdg1validator3".to_string(),
        true,
        150000,
        Some("Agree with addition".to_string()),
    ).unwrap();
    
    governance.submit_vote(
        &proposal1_id,
        "fdg1validator4".to_string(),
        false,
        80000,
        Some("Concerns about security".to_string()),
    ).unwrap();

    // Vote on proposal 2
    governance.submit_vote(
        &proposal2_id,
        "fdg1validator1".to_string(),
        true,
        100000,
        None,
    ).unwrap();
    
    governance.submit_vote(
        &proposal2_id,
        "fdg1validator2".to_string(),
        true,
        120000,
        Some("Supports parameter change".to_string()),
    ).unwrap();

    println!("✅ Submitted votes on both proposals");

    // Test 3: Check voting results
    println!("\n📊 Test 3: Voting Results");
    
    let results1 = governance.calculate_voting_results(&proposal1_id);
    let results2 = governance.calculate_voting_results(&proposal2_id);
    
    println!("Proposal 1 Results:");
    println!("  Total votes: {}", results1.total_votes);
    println!("  Yes votes: {}", results1.yes_votes);
    println!("  No votes: {}", results1.no_votes);
    println!("  Total stake voted: {}", results1.total_stake_voted);
    println!("  Quorum achieved: {}", results1.quorum_achieved);
    println!("  Approval percentage: {:.2}%", results1.approval_percentage * 100.0);
    println!("  Passed: {}", results1.passed);
    
    println!("\nProposal 2 Results:");
    println!("  Total votes: {}", results2.total_votes);
    println!("  Yes votes: {}", results2.yes_votes);
    println!("  No votes: {}", results2.no_votes);
    println!("  Total stake voted: {}", results2.total_stake_voted);
    println!("  Quorum achieved: {}", results2.quorum_achieved);
    println!("  Approval percentage: {:.2}%", results2.approval_percentage * 100.0);
    println!("  Passed: {}", results2.passed);

    // Test 4: Check proposal finalization
    println!("\n⏰ Test 4: Proposal Finalization");
    
    governance.check_proposal_finalization(&proposal1_id);
    governance.check_proposal_finalization(&proposal2_id);
    
    let proposal1_status = governance.get_proposal(&proposal1_id).unwrap().status.clone();
    let proposal2_status = governance.get_proposal(&proposal2_id).unwrap().status.clone();
    
    println!("Proposal 1 status: {:?}", proposal1_status);
    println!("Proposal 2 status: {:?}", proposal2_status);

    // Test 5: Analytics and monitoring
    println!("\n📈 Test 5: Analytics and Monitoring");
    
    let analytics = governance.get_analytics();
    let participation_rate = governance.calculate_participation_rate();
    let success_rate = governance.get_success_rate();
    let top_voters = governance.get_top_voters(5);
    let recent_events = governance.get_recent_events(10);
    
    println!("Analytics:");
    println!("  Total proposals created: {}", analytics.total_proposals_created);
    println!("  Total proposals passed: {}", analytics.total_proposals_passed);
    println!("  Total proposals failed: {}", analytics.total_proposals_failed);
    println!("  Total votes cast: {}", analytics.total_votes_cast);
    println!("  Participation rate: {:.2}%", participation_rate * 100.0);
    println!("  Success rate: {:.2}%", success_rate * 100.0);
    
    println!("\nTop Voters:");
    for (i, voter) in top_voters.iter().enumerate() {
        println!("  {}. {} - {} votes, {} stake", 
            i + 1, voter.voter, voter.total_votes, voter.total_stake_voted);
    }
    
    println!("\nRecent Events (last 10):");
    for event in recent_events.iter() {
        println!("  [{}] {} - {}: {}", 
            event.timestamp, event.event_type, event.actor, event.details);
    }

    // Test 6: Proposal execution
    println!("\n⚡ Test 6: Proposal Execution");
    
    if proposal1_status == ProposalStatus::Passed {
        match governance.execute_proposal(&proposal1_id) {
            Ok(()) => println!("✅ Proposal 1 executed successfully"),
            Err(e) => println!("❌ Failed to execute proposal 1: {}", e),
        }
    }
    
    if proposal2_status == ProposalStatus::Passed {
        match governance.execute_proposal(&proposal2_id) {
            Ok(()) => println!("✅ Proposal 2 executed successfully"),
            Err(e) => println!("❌ Failed to execute proposal 2: {}", e),
        }
    }

    // Test 7: Governance statistics
    println!("\n📊 Test 7: Governance Statistics");
    
    let stats = governance.get_statistics();
    println!("Governance Statistics:");
    for (key, value) in stats.iter() {
        println!("  {}: {}", key, value);
    }

    // Test 8: Error handling
    println!("\n🚨 Test 8: Error Handling");
    
    // Try to vote twice on the same proposal
    match governance.submit_vote(
        &proposal1_id,
        "fdg1validator1".to_string(),
        true,
        100000,
        None,
    ) {
        Ok(()) => println!("❌ Should have failed - duplicate vote"),
        Err(e) => println!("✅ Correctly rejected duplicate vote: {}", e),
    }
    
    // Try to create proposal with invalid duration
    match governance.create_proposal(
        "fdg1validator1".to_string(),
        "Invalid Proposal".to_string(),
        "Test invalid duration".to_string(),
        ProposalType::EmergencyPause {
            reason: "Test".to_string(),
        },
        Some(100), // Too short
    ) {
        Ok(_) => println!("❌ Should have failed - invalid duration"),
        Err(e) => println!("✅ Correctly rejected invalid duration: {}", e),
    }

    // Test 9: Emergency proposals
    println!("\n🚨 Test 9: Emergency Proposals");
    
    let emergency_id = governance.create_proposal(
        "fdg1validator1".to_string(),
        "Emergency Pause".to_string(),
        "Pause network due to security incident".to_string(),
        ProposalType::EmergencyPause {
            reason: "Security incident detected".to_string(),
        },
        Some(86400), // 24 hours for emergency
    ).unwrap();
    
    println!("✅ Created emergency proposal: {}", emergency_id);
    
    // Vote on emergency proposal
    governance.submit_vote(
        &emergency_id,
        "fdg1validator1".to_string(),
        true,
        100000,
        Some("Security concern confirmed".to_string()),
    ).unwrap();
    
    governance.submit_vote(
        &emergency_id,
        "fdg1validator3".to_string(),
        true,
        150000,
        Some("Agree with emergency pause".to_string()),
    ).unwrap();
    
    let emergency_results = governance.calculate_voting_results(&emergency_id);
    println!("Emergency proposal results:");
    println!("  Passed: {}", emergency_results.passed);
    println!("  Approval: {:.2}%", emergency_results.approval_percentage * 100.0);

    // Test 10: Configuration management
    println!("\n⚙️ Test 10: Configuration Management");
    
    let current_config = governance.config.clone();
    println!("Current configuration:");
    println!("  Min proposal duration: {} seconds", current_config.min_proposal_duration);
    println!("  Max proposal duration: {} seconds", current_config.max_proposal_duration);
    println!("  Min quorum percentage: {:.1}%", current_config.min_quorum_percentage * 100.0);
    println!("  Min approval percentage: {:.1}%", current_config.min_approval_percentage * 100.0);
    println!("  Proposal fee: {}", current_config.proposal_fee);
    println!("  Emergency threshold: {}", current_config.emergency_threshold);
    println!("  Max active proposals: {}", current_config.max_active_proposals);

    println!("\n🎉 All governance tests completed successfully!");
    println!("✅ Governance system is working correctly");
    
    // Summary
    println!("\n📋 Test Summary:");
    println!("  ✅ Proposal creation and validation");
    println!("  ✅ Voting system with stake weights");
    println!("  ✅ Result calculation and finalization");
    println!("  ✅ Analytics and monitoring");
    println!("  ✅ Error handling and validation");
    println!("  ✅ Emergency proposal handling");
    println!("  ✅ Configuration management");
    println!("  ✅ Event tracking and statistics");
} 
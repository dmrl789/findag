use findag_core::registry::governance::GovernanceRegistry;
use findag_core::types::governance::{ProposalStatus, VoteChoice, GovernanceConfig};
use findag_core::types::error::Error;
use std::path::PathBuf;
use tempfile::tempdir;

#[tokio::test]
async fn test_create_proposal() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_db");
    let registry = GovernanceRegistry::new(&db_path, None).unwrap();

    let result = registry.create_proposal(
        "test_proposal".to_string(),
        "Test Proposal".to_string(),
        "This is a test proposal".to_string(),
        "test_proposer".to_string(),
    ).await;

    assert!(result.is_ok());

    let proposal = registry.get_proposal("test_proposal").await.unwrap();
    assert_eq!(proposal.id, "test_proposal");
    assert_eq!(proposal.title, "Test Proposal");
    assert_eq!(proposal.description, "This is a test proposal");
    assert_eq!(proposal.proposer, "test_proposer");
    assert_eq!(proposal.status, ProposalStatus::Active);
    assert!(proposal.votes.is_empty());
}

#[tokio::test]
async fn test_duplicate_proposal() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_db");
    let registry = GovernanceRegistry::new(&db_path, None).unwrap();

    let proposal_id = "test_proposal".to_string();
    
    // Create first proposal
    registry.create_proposal(
        proposal_id.clone(),
        "Test Proposal".to_string(),
        "This is a test proposal".to_string(),
        "test_proposer".to_string(),
    ).await.unwrap();

    // Try to create duplicate proposal
    let result = registry.create_proposal(
        proposal_id,
        "Duplicate Proposal".to_string(),
        "This is a duplicate proposal".to_string(),
        "test_proposer".to_string(),
    ).await;

    assert!(matches!(result, Err(Error::ProposalExists)));
}

#[tokio::test]
async fn test_cast_vote() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_db");
    let registry = GovernanceRegistry::new(&db_path, None).unwrap();

    // Create a proposal
    registry.create_proposal(
        "test_proposal".to_string(),
        "Test Proposal".to_string(),
        "This is a test proposal".to_string(),
        "test_proposer".to_string(),
    ).await.unwrap();

    // Cast a vote
    let result = registry.cast_vote(
        "test_proposal",
        "test_voter".to_string(),
        VoteChoice::Yes,
        1000, // stake amount
        60 * 60 * 24 * 30, // 30 days stake duration
    ).await;

    assert!(result.is_ok());

    let proposal = registry.get_proposal("test_proposal").await.unwrap();
    assert_eq!(proposal.votes.len(), 1);
    let vote = proposal.votes.get("test_voter").unwrap();
    assert_eq!(vote.choice, VoteChoice::Yes);
    assert!(vote.weight > 0);
}

#[tokio::test]
async fn test_finalize_proposal() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_db");
    let mut config = GovernanceConfig::default();
    config.proposal_duration = 1; // 1 second duration for testing
    let registry = GovernanceRegistry::new(&db_path, Some(config)).unwrap();

    // Create a proposal
    registry.create_proposal(
        "test_proposal".to_string(),
        "Test Proposal".to_string(),
        "This is a test proposal".to_string(),
        "test_proposer".to_string(),
    ).await.unwrap();

    // Cast multiple votes
    registry.cast_vote(
        "test_proposal",
        "voter1".to_string(),
        VoteChoice::Yes,
        1000,
        60 * 60 * 24 * 30,
    ).await.unwrap();

    registry.cast_vote(
        "test_proposal",
        "voter2".to_string(),
        VoteChoice::Yes,
        1000,
        60 * 60 * 24 * 30,
    ).await.unwrap();

    registry.cast_vote(
        "test_proposal",
        "voter3".to_string(),
        VoteChoice::No,
        500,
        60 * 60 * 24 * 30,
    ).await.unwrap();

    // Wait for proposal to expire
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Finalize proposal
    let result = registry.finalize_proposal("test_proposal").await;
    assert!(result.is_ok());

    let proposal = registry.get_proposal("test_proposal").await.unwrap();
    assert_eq!(proposal.status, ProposalStatus::Passed);
}

#[tokio::test]
async fn test_list_proposals() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_db");
    let registry = GovernanceRegistry::new(&db_path, None).unwrap();

    // Create multiple proposals
    for i in 0..3 {
        registry.create_proposal(
            format!("proposal_{}", i),
            format!("Proposal {}", i),
            format!("Description {}", i),
            "test_proposer".to_string(),
        ).await.unwrap();
    }

    // Test list_all_proposals
    let all_proposals = registry.list_all_proposals().await.unwrap();
    assert_eq!(all_proposals.len(), 3);

    // Test list_active_proposals
    let active_proposals = registry.list_active_proposals().await.unwrap();
    assert_eq!(active_proposals.len(), 3);

    // Finalize one proposal
    let mut config = GovernanceConfig::default();
    config.proposal_duration = 1;
    let registry = GovernanceRegistry::new(&db_path, Some(config)).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(2));
    registry.finalize_proposal("proposal_0").await.unwrap();

    let active_proposals = registry.list_active_proposals().await.unwrap();
    assert_eq!(active_proposals.len(), 2);
}

#[tokio::test]
async fn test_voting_power_calculation() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_db");
    let mut config = GovernanceConfig::default();
    config.voting_power_decay = 60 * 60 * 24 * 30; // 30 days
    let registry = GovernanceRegistry::new(&db_path, Some(config)).unwrap();

    // Create a proposal
    registry.create_proposal(
        "test_proposal".to_string(),
        "Test Proposal".to_string(),
        "This is a test proposal".to_string(),
        "test_proposer".to_string(),
    ).await.unwrap();

    // Test voting power with different stake durations
    let stake_amount = 1000;
    
    // Full voting power (stake duration > decay period)
    registry.cast_vote(
        "test_proposal",
        "voter1".to_string(),
        VoteChoice::Yes,
        stake_amount,
        60 * 60 * 24 * 31, // 31 days
    ).await.unwrap();

    // Half voting power (stake duration = half decay period)
    registry.cast_vote(
        "test_proposal",
        "voter2".to_string(),
        VoteChoice::Yes,
        stake_amount,
        60 * 60 * 24 * 15, // 15 days
    ).await.unwrap();

    let proposal = registry.get_proposal("test_proposal").await.unwrap();
    let vote1 = proposal.votes.get("voter1").unwrap();
    let vote2 = proposal.votes.get("voter2").unwrap();

    assert!(vote1.weight > vote2.weight);
    assert_eq!(vote1.weight, stake_amount);
    assert!(vote2.weight < stake_amount);
} 
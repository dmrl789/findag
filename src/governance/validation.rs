use crate::governance::types::{Proposal, ProposalStatus, GovernanceConfig};
use std::error::Error;

pub fn validate_proposal(proposal: &Proposal, config: &GovernanceConfig) -> Result<(), Box<dyn Error>> {
    // Validate proposal title
    if proposal.title.is_empty() {
        return Err("Proposal title cannot be empty".into());
    }

    // Validate proposal description
    if proposal.description.is_empty() {
        return Err("Proposal description cannot be empty".into());
    }

    // Validate proposal duration
    let duration = proposal.updated_at.duration_since(proposal.created_at)
        .map_err(|_| "Invalid proposal duration")?;
    
    if duration.as_secs() < config.min_proposal_duration {
        return Err("Proposal duration is too short".into());
    }
    
    if duration.as_secs() > config.max_proposal_duration {
        return Err("Proposal duration is too long".into());
    }

    Ok(())
}

pub fn validate_proposal_status(
    current_status: &ProposalStatus,
    new_status: &ProposalStatus,
) -> Result<(), Box<dyn Error>> {
    match (current_status, new_status) {
        (ProposalStatus::Draft, ProposalStatus::Active) => Ok(()),
        (ProposalStatus::Active, ProposalStatus::Passed) => Ok(()),
        (ProposalStatus::Active, ProposalStatus::Failed) => Ok(()),
        (ProposalStatus::Passed, ProposalStatus::Executed) => Ok(()),
        (ProposalStatus::Active, ProposalStatus::Cancelled) => Ok(()),
        (ProposalStatus::Draft, ProposalStatus::Cancelled) => Ok(()),
        _ => Err("Invalid status transition".into()),
    }
} 
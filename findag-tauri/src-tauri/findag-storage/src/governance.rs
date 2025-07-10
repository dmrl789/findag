//! Governance storage operations
//! 
//! This module handles storage operations for governance management.

use sled::Tree;
use findag_core::{Address};
use findag_types::{GovernanceProposal, FindDAGResult};

/// Governance storage manager
pub struct GovernanceStorage {
    /// Governance tree
    governance_tree: Tree,
}

impl GovernanceStorage {
    /// Create a new governance storage manager
    pub fn new(governance_tree: Tree) -> Self {
        Self { governance_tree }
    }

    /// Store a governance proposal
    pub fn store_proposal(&self, proposal: &GovernanceProposal) -> FindDAGResult<()> {
        let key = format!("proposal:{}", proposal.proposal_id);
        let value = bincode::serialize(proposal)?;
        
        self.governance_tree.insert(key.as_bytes(), value)?;
        Ok(())
    }

    /// Get a governance proposal by ID
    pub fn get_proposal(&self, proposal_id: &str) -> FindDAGResult<Option<GovernanceProposal>> {
        let key = format!("proposal:{}", proposal_id);
        let result = self.governance_tree.get(key.as_bytes())?;
        
        if let Some(value) = result {
            let proposal: GovernanceProposal = bincode::deserialize(&value)?;
            Ok(Some(proposal))
        } else {
            Ok(None)
        }
    }

    /// Get all proposals
    pub fn get_all_proposals(&self) -> FindDAGResult<Vec<GovernanceProposal>> {
        let mut proposals = Vec::new();
        
        for result in self.governance_tree.iter() {
            let (key, value) = result?;
            let key_str = String::from_utf8_lossy(&key);
            
            if key_str.starts_with("proposal:") {
                if let Ok(proposal) = bincode::deserialize::<GovernanceProposal>(&value) {
                    proposals.push(proposal);
                }
            }
        }
        
        Ok(proposals)
    }
} 
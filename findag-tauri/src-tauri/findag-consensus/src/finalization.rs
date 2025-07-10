//! Block finalization
//! 
//! This module handles block finalization and proof generation.

use findag_core::{Address, Hash, HashTimer, FinDAGTime};
use findag_types::{Block, BlockFinalization, FinalizationProof, FindDAGResult};

/// Block finalizer
pub struct BlockFinalizer {
    /// Finalized blocks
    finalized_blocks: std::collections::HashMap<Hash, BlockFinalization>,
}

impl BlockFinalizer {
    /// Create a new block finalizer
    pub fn new() -> Self {
        Self {
            finalized_blocks: std::collections::HashMap::new(),
        }
    }

    /// Finalize a block
    pub fn finalize_block(
        &mut self,
        block: &Block,
        proof: FinalizationProof,
    ) -> FindDAGResult<()> {
        let finalization = BlockFinalization {
            block_hash: block.header.hash.clone(),
            round_number: 0, // TODO: Get from consensus state
            finalization_timestamp: FinDAGTime::now(),
            proof,
        };
        
        self.finalized_blocks.insert(block.header.hash.clone(), finalization);
        Ok(())
    }

    /// Check if a block is finalized
    pub fn is_finalized(&self, hash: &Hash) -> bool {
        self.finalized_blocks.contains_key(hash)
    }

    /// Get finalization proof
    pub fn get_finalization_proof(&self, hash: &Hash) -> Option<&BlockFinalization> {
        self.finalized_blocks.get(hash)
    }
} 
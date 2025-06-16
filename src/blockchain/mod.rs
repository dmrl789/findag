use crate::blockchain::block::Block;
use crate::blockchain::dag::Dag;
use crate::blockchain::state::State;
use crate::blockchain::assembler::Assembler;
use crate::blockchain::validator::Validator;
use crate::blockchain::finality::FinalityManager;
use crate::blockchain::error::BlockchainError;
use crate::blockchain::types::BlockAnalysis;
use std::error::Error;

pub mod block;
pub mod dag;
pub mod state;
pub mod assembler;
pub mod validator;
pub mod finality;
pub mod error;
pub mod types;
pub mod transaction;
pub mod auto_transaction;
pub mod auto_transaction_executor;

pub struct Blockchain {
    pub dag: Dag,
    pub state: State,
    pub assembler: Assembler,
    pub validator: Validator,
    pub finality_manager: FinalityManager,
}

impl Blockchain {
    pub fn new() -> Result<Self, BlockchainError> {
        let dag = Dag::new();
        let state = State::new();
        let assembler = Assembler::new(state.clone());
        let validator = Validator::new();
        let finality_manager = FinalityManager::new();
        Ok(Self {
            dag,
            state,
            assembler,
            validator,
            finality_manager,
        })
    }

    pub fn analyze_block(&self, _content: &str) -> Result<BlockAnalysis, Box<dyn Error>> {
        // Implement block analysis logic here
        Ok(BlockAnalysis::default())
    }
}

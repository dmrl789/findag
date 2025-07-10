//! FinDAG RoundChain consensus engine
//! 
//! This crate implements the RoundChain consensus algorithm for FinDAG,
//! providing deterministic finality with parallel block processing.

pub mod roundchain;
pub mod validator;
pub mod mempool;
pub mod finalization;
pub mod metrics;

pub use roundchain::*;
pub use validator::*;
pub use mempool::*;
pub use finalization::*;
pub use metrics::*;

use findag_core::{Address, Hash, HashTimer, FinDAGTime};
use findag_types::{
    Block, Round, Transaction, ConsensusState, ConsensusStatus, ConsensusConfig,
    ConsensusEvent, ConsensusCommand, RoundState, ValidatorVote, VoteType,
    BlockFinalization, FinalizationProof, ConsensusMetrics, ValidatorAssignment,
    AssignmentStatus, FindDAGResult, FindDAGError,
};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tracing::{info, warn, error, debug};
use metrics::{counter, gauge, histogram};

/// Consensus engine
pub struct ConsensusEngine {
    /// Consensus state
    state: Arc<RwLock<ConsensusState>>,
    /// Configuration
    config: ConsensusConfig,
    /// Validator set
    validators: Arc<RwLock<HashMap<Address, Validator>>>,
    /// Round states
    rounds: Arc<RwLock<HashMap<u64, RoundState>>>,
    /// Transaction mempool
    mempool: Arc<RwLock<TransactionMempool>>,
    /// Event sender
    event_sender: mpsc::Sender<ConsensusEvent>,
    /// Command receiver
    command_receiver: mpsc::Receiver<ConsensusCommand>,
    /// Metrics
    metrics: ConsensusMetrics,
}

impl ConsensusEngine {
    /// Create a new consensus engine
    pub fn new(
        config: ConsensusConfig,
        event_sender: mpsc::Sender<ConsensusEvent>,
        command_receiver: mpsc::Receiver<ConsensusCommand>,
    ) -> Self {
        let state = Arc::new(RwLock::new(ConsensusState {
            current_round: 0,
            latest_finalized_round: 0,
            current_block: 0,
            active_validators: vec![],
            status: ConsensusStatus::Running,
            last_update: chrono::Utc::now(),
        }));

        let validators = Arc::new(RwLock::new(HashMap::new()));
        let rounds = Arc::new(RwLock::new(HashMap::new()));
        let mempool = Arc::new(RwLock::new(TransactionMempool::new()));

        Self {
            state,
            config,
            validators,
            rounds,
            mempool,
            event_sender,
            command_receiver,
            metrics: ConsensusMetrics::default(),
        }
    }

    /// Start the consensus engine
    pub async fn start(&mut self) -> FindDAGResult<()> {
        info!("Starting consensus engine");
        
        // Initialize metrics
        self.initialize_metrics();
        
        // Start consensus loop
        self.consensus_loop().await?;
        
        Ok(())
    }

    /// Stop the consensus engine
    pub async fn stop(&mut self) -> FindDAGResult<()> {
        info!("Stopping consensus engine");
        
        let mut state = self.state.write().await;
        state.status = ConsensusStatus::Failed;
        state.last_update = chrono::Utc::now();
        
        Ok(())
    }

    /// Get consensus state
    pub async fn get_state(&self) -> ConsensusState {
        self.state.read().await.clone()
    }

    /// Get consensus metrics
    pub async fn get_metrics(&self) -> ConsensusMetrics {
        self.metrics.clone()
    }

    /// Submit a transaction to the mempool
    pub async fn submit_transaction(&self, transaction: Transaction) -> FindDAGResult<()> {
        let mut mempool = self.mempool.write().await;
        mempool.add_transaction(transaction).await?;
        
        counter!("findag_consensus_transactions_submitted", 1);
        Ok(())
    }

    /// Submit a block for consensus
    pub async fn submit_block(&self, block: Block) -> FindDAGResult<()> {
        let mut rounds = self.rounds.write().await;
        let round_number = self.get_round_number(&block).await?;
        
        if let Some(round_state) = rounds.get_mut(&round_number) {
            round_state.blocks.push(block.header.hash.clone());
        } else {
            // Create new round state
            let round_state = RoundState {
                round_number,
                start_timestamp: block.header.timestamp,
                end_timestamp: None,
                blocks: vec![block.header.hash],
                status: findag_types::RoundStatus::Active,
                votes: vec![],
                signature: None,
            };
            rounds.insert(round_number, round_state);
        }
        
        counter!("findag_consensus_blocks_submitted", 1);
        Ok(())
    }

    /// Submit a validator vote
    pub async fn submit_vote(&self, vote: ValidatorVote) -> FindDAGResult<()> {
        let mut rounds = self.rounds.write().await;
        let round_number = vote.vote_data.round_number;
        
        if let Some(round_state) = rounds.get_mut(&round_number) {
            round_state.votes.push(vote);
            
            // Check if we have enough votes to finalize
            if round_state.votes.len() >= self.config.min_validators {
                self.finalize_round(round_number).await?;
            }
        }
        
        counter!("findag_consensus_votes_submitted", 1);
        Ok(())
    }

    /// Get round number for a block
    async fn get_round_number(&self, block: &Block) -> FindDAGResult<u64> {
        let state = self.state.read().await;
        let round_interval = self.config.round_interval_ms as u64;
        let block_timestamp = block.header.timestamp.0;
        let round_number = block_timestamp / round_interval;
        Ok(round_number)
    }

    /// Finalize a round
    async fn finalize_round(&self, round_number: u64) -> FindDAGResult<()> {
        let mut rounds = self.rounds.write().await;
        let mut state = self.state.write().await;
        
        if let Some(round_state) = rounds.get_mut(&round_number) {
            round_state.status = findag_types::RoundStatus::Finalized;
            round_state.end_timestamp = Some(FinDAGTime::now());
            
            // Update consensus state
            state.latest_finalized_round = round_number;
            state.last_update = chrono::Utc::now();
            
            // Update metrics
            self.metrics.rounds_per_sec += 1.0;
            self.metrics.total_votes = round_state.votes.len();
            
            // Send event
            let _ = self.event_sender.send(ConsensusEvent::RoundFinalized {
                round_number,
                timestamp: FinDAGTime::now(),
                blocks: round_state.blocks.clone(),
            }).await;
            
            info!("Round {} finalized with {} blocks", round_number, round_state.blocks.len());
        }
        
        Ok(())
    }

    /// Initialize metrics
    fn initialize_metrics(&self) {
        gauge!("findag_consensus_active_validators", 0.0);
        gauge!("findag_consensus_current_round", 0.0);
        gauge!("findag_consensus_latest_finalized_round", 0.0);
        counter!("findag_consensus_transactions_submitted", 0);
        counter!("findag_consensus_blocks_submitted", 0);
        counter!("findag_consensus_votes_submitted", 0);
        histogram!("findag_consensus_round_latency_seconds", 0.0);
    }

    /// Main consensus loop
    async fn consensus_loop(&mut self) -> FindDAGResult<()> {
        info!("Starting consensus loop");
        
        while let Some(command) = self.command_receiver.recv().await {
            match command {
                ConsensusCommand::Start => {
                    info!("Consensus engine started");
                    let mut state = self.state.write().await;
                    state.status = ConsensusStatus::Running;
                }
                ConsensusCommand::Stop => {
                    info!("Consensus engine stopped");
                    let mut state = self.state.write().await;
                    state.status = ConsensusStatus::Failed;
                    break;
                }
                ConsensusCommand::UpdateConfig(config) => {
                    info!("Updating consensus configuration");
                    self.config = config;
                }
                ConsensusCommand::AddValidator(address) => {
                    info!("Adding validator: {}", address);
                    let mut validators = self.validators.write().await;
                    validators.insert(address.clone(), Validator {
                        address,
                        public_key: vec![],
                        metadata: None,
                        status: findag_types::ValidatorStatus::Active,
                        registered_at: chrono::Utc::now(),
                        last_active: chrono::Utc::now(),
                    });
                }
                ConsensusCommand::RemoveValidator(address) => {
                    info!("Removing validator: {}", address);
                    let mut validators = self.validators.write().await;
                    validators.remove(&address);
                }
                ConsensusCommand::ForceFinalizeRound(round_number) => {
                    info!("Force finalizing round: {}", round_number);
                    self.finalize_round(round_number).await?;
                }
            }
        }
        
        Ok(())
    }
}

impl Default for ConsensusMetrics {
    fn default() -> Self {
        Self {
            rounds_per_sec: 0.0,
            blocks_per_sec: 0.0,
            avg_round_latency_ms: 0.0,
            avg_finalization_time_ms: 0.0,
            active_validators_count: 0,
            total_votes: 0,
            uptime_seconds: 0,
        }
    }
}

/// Transaction mempool
pub struct TransactionMempool {
    /// Pending transactions
    transactions: HashMap<Hash, Transaction>,
    /// Transaction order (by timestamp)
    order: Vec<Hash>,
}

impl TransactionMempool {
    /// Create a new mempool
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
            order: vec![],
        }
    }

    /// Add a transaction to the mempool
    pub async fn add_transaction(&mut self, transaction: Transaction) -> FindDAGResult<()> {
        let hash = transaction.hash.clone();
        
        if !self.transactions.contains_key(&hash) {
            self.transactions.insert(hash.clone(), transaction);
            self.order.push(hash);
            
            // Sort by timestamp
            self.order.sort_by(|a, b| {
                let tx_a = &self.transactions[a];
                let tx_b = &self.transactions[b];
                tx_a.timestamp.cmp(&tx_b.timestamp)
            });
        }
        
        Ok(())
    }

    /// Get transactions for a block
    pub fn get_transactions(&self, max_count: usize) -> Vec<Transaction> {
        self.order
            .iter()
            .take(max_count)
            .filter_map(|hash| self.transactions.get(hash).cloned())
            .collect()
    }

    /// Remove transactions from mempool
    pub fn remove_transactions(&mut self, hashes: &[Hash]) {
        for hash in hashes {
            self.transactions.remove(hash);
        }
        
        self.order.retain(|hash| self.transactions.contains_key(hash));
    }

    /// Get mempool size
    pub fn size(&self) -> usize {
        self.transactions.len()
    }
} 
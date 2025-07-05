use crate::network::propagation::{NetworkPropagator, GossipMsg};
use crate::consensus::validator_set::{ValidatorSet, ValidatorReputation};
use crate::core::types::{SerializableTransaction, SerializableBlock, SerializableRound, Transaction, Block, Round};
use crate::core::dag_engine::DagEngine;
use crate::core::tx_pool::ShardedTxPool;
use crate::core::address::Address;
use ed25519_dalek::{Keypair, PublicKey, Verifier};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Instant, Duration};


/// Peer scoring and reputation tracking
#[derive(Debug, Clone)]
pub struct PeerScore {
    pub address: Address,
    pub score: f64, // 0.0 to 1.0
    pub message_count: u64,
    pub invalid_messages: u64,
    pub last_seen: Instant,
    pub response_time_ms: u64,
}

impl Default for PeerScore {
    fn default() -> Self {
        Self {
            address: Address("unknown".to_string()),
            score: 1.0,
            message_count: 0,
            invalid_messages: 0,
            last_seen: Instant::now(),
            response_time_ms: 0,
        }
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub max_messages_per_second: u32,
    pub max_messages_per_minute: u32,
    pub penalty_threshold: u32,
    pub recovery_time_ms: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_messages_per_second: 100,
            max_messages_per_minute: 1000,
            penalty_threshold: 10,
            recovery_time_ms: 60000, // 1 minute
        }
    }
}

/// Consensus integration manager
pub struct ConsensusIntegration {
    propagator: Arc<NetworkPropagator>,
    validator_set: Arc<Mutex<ValidatorSet>>,
    dag: Arc<Mutex<DagEngine>>,
    tx_pool: Arc<ShardedTxPool>,
    peer_scores: Arc<Mutex<HashMap<Address, PeerScore>>>,
    rate_limits: Arc<Mutex<HashMap<Address, (Instant, u32)>>>,
    rate_config: RateLimitConfig,
    local_address: Address,
    #[allow(dead_code)]
    local_keypair: Option<Keypair>,
}

#[allow(dead_code)]
impl ConsensusIntegration {
    pub fn new(
        propagator: Arc<NetworkPropagator>,
        validator_set: Arc<Mutex<ValidatorSet>>,
        dag: Arc<Mutex<DagEngine>>,
        tx_pool: Arc<ShardedTxPool>,
        local_address: Address,
        local_keypair: Option<Keypair>,
    ) -> Self {
        Self {
            propagator,
            validator_set,
            dag,
            tx_pool,
            peer_scores: Arc::new(Mutex::new(HashMap::new())),
            rate_limits: Arc::new(Mutex::new(HashMap::new())),
            rate_config: RateLimitConfig::default(),
            local_address,
            local_keypair,
        }
    }

    /// Start the consensus integration
    pub async fn start(&self) {
        println!("🚀 Starting consensus integration...");
        
        // Spawn message listener
        let propagator = self.propagator.clone();
        let consensus_integration = self.clone();
        
        tokio::spawn(async move {
            propagator.listen(move |msg| {
                let integration = consensus_integration.clone();
                match msg {
                    GossipMsg::NewTransaction(tx) => {
                        // No spawn needed, just call directly
                        tokio::spawn(async move {
                            integration.handle_new_transaction(tx, &integration.local_address).await;
                        });
                    }
                    GossipMsg::NewBlock(serializable_block) => {
                        let try_block: Result<_, _> = serializable_block.try_into();
                        match try_block {
                            Ok(block) => {
                                tokio::spawn(async move {
                                    integration.handle_new_block_converted(block, &integration.local_address).await;
                                });
                            }
                            Err(_) => {
                                println!("❌ Failed to convert block from peer");
                            }
                        }
                    }
                    GossipMsg::NewRound(serializable_round) => {
                        let try_round: Result<_, _> = serializable_round.try_into();
                        match try_round {
                            Ok(round) => {
                                tokio::spawn(async move {
                                    integration.handle_new_round_converted(round, &integration.local_address).await;
                                });
                            }
                            Err(_) => {
                                println!("❌ Failed to convert round from peer");
                            }
                        }
                    }
                }
            }).await;
        });

        // Spawn periodic tasks
        self.spawn_periodic_tasks().await;
    }

    /// Handle incoming gossip messages
    async fn handle_gossip_message(&self, msg: GossipMsg) {
        let start_time = Instant::now();
        
        // Extract sender address (in a real implementation, this would come from the network layer)
        let sender_address = self.extract_sender_address(&msg);
        
        // Check rate limiting
        if !self.check_rate_limit(&sender_address).await {
            println!("⚠️ Rate limit exceeded for peer: {}", sender_address.as_str());
            return;
        }

        // Validate message
        let validation_result = self.validate_message(&msg).await;
        if !validation_result.is_valid {
            let reason = validation_result.reason.clone();
            self.penalize_peer(&sender_address, reason.clone()).await;
            println!("❌ Invalid message from {}: {}", sender_address.as_str(), reason);
            return;
        }

        // Process message based on type
        match msg {
            GossipMsg::NewTransaction(tx) => {
                self.handle_new_transaction(tx, &sender_address).await;
            }
            GossipMsg::NewBlock(block) => {
                // Convert before async to avoid Send issues
                match block.try_into() {
                    Ok(block) => {
                        self.handle_new_block_converted(block, &sender_address).await;
                    }
                    Err(_) => {
                        println!("❌ Failed to convert block from peer {}", sender_address.as_str());
                    }
                }
            }
            GossipMsg::NewRound(round) => {
                // Convert before async to avoid Send issues
                match round.try_into() {
                    Ok(round) => {
                        self.handle_new_round_converted(round, &sender_address).await;
                    }
                    Err(_) => {
                        println!("❌ Failed to convert round from peer {}", sender_address.as_str());
                    }
                }
            }
        }

        // Update peer score
        let response_time = start_time.elapsed().as_millis() as u64;
        self.update_peer_score(&sender_address, response_time, true).await;
    }

    /// Validate incoming messages
    async fn validate_message(&self, msg: &GossipMsg) -> MessageValidationResult {
        match msg {
            GossipMsg::NewTransaction(tx) => {
                self.validate_transaction(tx).await
            }
            GossipMsg::NewBlock(block) => {
                self.validate_block(block).await
            }
            GossipMsg::NewRound(round) => {
                self.validate_round(round).await
            }
        }
    }

    /// Validate transaction message
    async fn validate_transaction(&self, tx: &SerializableTransaction) -> MessageValidationResult {
        // Basic validation
        if tx.amount == 0 {
            return MessageValidationResult {
                is_valid: false,
                reason: "Transaction amount cannot be zero".to_string(),
            };
        }

        // Validate addresses
        if !self.is_valid_address(&tx.from) || !self.is_valid_address(&tx.to) {
            return MessageValidationResult {
                is_valid: false,
                reason: "Invalid address format".to_string(),
            };
        }

        // Validate signature
        if let Err(_) = self.verify_transaction_signature(tx).await {
            return MessageValidationResult {
                is_valid: false,
                reason: "Invalid transaction signature".to_string(),
            };
        }

        MessageValidationResult {
            is_valid: true,
            reason: "Valid".to_string(),
        }
    }

    /// Validate block message
    async fn validate_block(&self, block: &SerializableBlock) -> MessageValidationResult {
        // Basic validation
        if block.transactions.is_empty() {
            return MessageValidationResult {
                is_valid: false,
                reason: "Block cannot be empty".to_string(),
            };
        }

        // Validate proposer signature
        if let Err(_) = self.verify_block_signature(block).await {
            return MessageValidationResult {
                is_valid: false,
                reason: "Invalid block signature".to_string(),
            };
        }

        MessageValidationResult {
            is_valid: true,
            reason: "Valid".to_string(),
        }
    }

    /// Validate round message
    async fn validate_round(&self, round: &SerializableRound) -> MessageValidationResult {
        // Basic validation
        if round.finalized_block_hashes.is_empty() {
            return MessageValidationResult {
                is_valid: false,
                reason: "Round cannot be empty".to_string(),
            };
        }

        // Validate that block_hashtimers matches finalized_block_hashes length
        if round.finalized_block_hashes.len() != round.block_hashtimers.len() {
            return MessageValidationResult {
                is_valid: false,
                reason: "Block hashes and hashtimers count mismatch".to_string(),
            };
        }

        // Validate proposer signature
        if let Err(_) = self.verify_round_signature(round).await {
            return MessageValidationResult {
                is_valid: false,
                reason: "Invalid round signature".to_string(),
            };
        }

        MessageValidationResult {
            is_valid: true,
            reason: "Valid".to_string(),
        }
    }

    /// Handle new transaction from network
    async fn handle_new_transaction(&self, tx: SerializableTransaction, sender: &Address) {
        // Convert to core transaction
        if let Ok(transaction) = tx.try_into() {
            // Add to transaction pool
            let added = self.tx_pool.add_transaction(transaction);
            if added {
                println!("✅ Added transaction from peer {} to pool", sender.as_str());
            } else {
                println!("⚠️ Transaction from peer {} rejected by pool", sender.as_str());
            }
        }
    }

    /// Handle new block from network (converted)
    async fn handle_new_block_converted(&self, block: Block, sender: &Address) {
        // Add to DAG
        let dag = self.dag.lock().await;
        if let Err(e) = dag.add_block(block).await {
            println!("❌ Failed to add block from peer {}: {}", sender.as_str(), e);
        } else {
            println!("✅ Added block from peer {} to DAG", sender.as_str());
        }
    }

    /// Handle new round from network (converted)
    async fn handle_new_round_converted(&self, round: Round, sender: &Address) {
        // Add to DAG
        let mut dag = self.dag.lock().await;
        dag.add_round(round).await;
        println!("✅ Added round from peer {} to DAG", sender.as_str());
    }

    /// Broadcast new transaction to network
    pub async fn broadcast_transaction(&self, transaction: Transaction) {
        let serializable_tx: SerializableTransaction = transaction.into();
        let msg = GossipMsg::NewTransaction(serializable_tx);
        self.propagator.broadcast(&msg).await;
        println!("📡 Broadcasted transaction to network");
    }

    /// Broadcast new block to network
    pub async fn broadcast_block(&self, block: Block) {
        let serializable_block: SerializableBlock = block.into();
        let msg = GossipMsg::NewBlock(serializable_block);
        self.propagator.broadcast(&msg).await;
        println!("📡 Broadcasted block to network");
    }

    /// Broadcast new round to network
    pub async fn broadcast_round(&self, round: Round) {
        let serializable_round: SerializableRound = round.into();
        let msg = GossipMsg::NewRound(serializable_round);
        self.propagator.broadcast(&msg).await;
        println!("📡 Broadcasted round to network");
    }

    /// Check rate limiting for a peer
    async fn check_rate_limit(&self, peer: &Address) -> bool {
        let mut limits = self.rate_limits.lock().await;
        let now = Instant::now();
        
        if let Some((last_request, count)) = limits.get_mut(peer) {
            if now.duration_since(*last_request) > Duration::from_secs(1) {
                *last_request = now;
                *count = 1;
                true
            } else if *count < self.rate_config.max_messages_per_second {
                *count += 1;
                true
            } else {
                false
            }
        } else {
            limits.insert(peer.clone(), (now, 1));
            true
        }
    }

    /// Update peer score
    async fn update_peer_score(&self, peer: &Address, response_time: u64, success: bool) {
        let mut scores = self.peer_scores.lock().await;
        let score = scores.entry(peer.clone()).or_default();
        
        score.message_count += 1;
        score.last_seen = Instant::now();
        score.response_time_ms = response_time;
        
        if success {
            score.score = (score.score + 0.01).min(1.0);
        } else {
            score.invalid_messages += 1;
            score.score = (score.score - 0.1).max(0.0);
        }
    }

    /// Penalize peer for invalid behavior
    async fn penalize_peer(&self, peer: &Address, reason: String) {
        let mut scores = self.peer_scores.lock().await;
        let score = scores.entry(peer.clone()).or_default();
        
        score.invalid_messages += 1;
        score.score = (score.score - 0.2).max(0.0);
        
        println!("⚠️ Penalized peer {} for: {}", peer.as_str(), reason);
    }

    /// Extract sender address from message (placeholder implementation)
    fn extract_sender_address(&self, _msg: &GossipMsg) -> Address {
        // In a real implementation, this would extract the sender from the network layer
        Address("unknown_peer".to_string())
    }

    /// Check if address is valid
    fn is_valid_address(&self, address: &Address) -> bool {
        address.as_str().starts_with("fdg1") && address.as_str().len() >= 10
    }

    /// Verify transaction signature
    async fn verify_transaction_signature(&self, tx: &SerializableTransaction) -> Result<(), String> {
        // Convert signature bytes to signature
        let signature = ed25519_dalek::Signature::from_bytes(&tx.signature_bytes)
            .map_err(|_| "Invalid signature format".to_string())?;
        
        // Convert public key bytes to public key
        let public_key = PublicKey::from_bytes(&tx.public_key_bytes)
            .map_err(|_| "Invalid public key format".to_string())?;
        
        // Create message to verify
        let message = format!("{}{}{}", tx.from.as_str(), tx.to.as_str(), tx.amount);
        
        // Verify signature
        public_key.verify(message.as_bytes(), &signature)
            .map_err(|_| "Signature verification failed".to_string())?;
        
        Ok(())
    }

    /// Verify block signature
    async fn verify_block_signature(&self, block: &SerializableBlock) -> Result<(), String> {
        // Convert signature bytes to signature
        let signature = ed25519_dalek::Signature::from_bytes(&block.signature_bytes)
            .map_err(|_| "Invalid signature format".to_string())?;
        
        // Convert public key bytes to public key
        let public_key = PublicKey::from_bytes(&block.public_key_bytes)
            .map_err(|_| "Invalid public key format".to_string())?;
        
        // Verify signature against block ID
        public_key.verify(&block.block_id, &signature)
            .map_err(|_| "Block signature verification failed".to_string())?;
        
        Ok(())
    }

    /// Verify round signature
    async fn verify_round_signature(&self, round: &SerializableRound) -> Result<(), String> {
        // Convert signature bytes to signature
        let signature = ed25519_dalek::Signature::from_bytes(&round.proposer_signature_bytes)
            .map_err(|_| "Invalid signature format".to_string())?;
        
        // Convert public key bytes to public key
        let public_key = PublicKey::from_bytes(&round.proposer_public_key_bytes)
            .map_err(|_| "Invalid public key format".to_string())?;
        
        // Create message to verify (round content)
        let mut message = Vec::new();
        message.extend_from_slice(&round.round_number.to_be_bytes());
        message.extend_from_slice(&round.parent_round_hash);
        message.extend_from_slice(&(round.finalized_block_hashes.len() as u32).to_be_bytes());
        for hash in &round.finalized_block_hashes {
            message.extend_from_slice(hash);
        }
        for hashtimer in &round.block_hashtimers {
            message.extend_from_slice(hashtimer);
        }
        message.extend_from_slice(&round.findag_time.to_be_bytes());
        
        // Verify signature
        public_key.verify(&message, &signature)
            .map_err(|_| "Round signature verification failed".to_string())?;
        
        Ok(())
    }

    /// Spawn periodic tasks for maintenance
    async fn spawn_periodic_tasks(&self) {
        let _peer_scores = self.peer_scores.clone();
        let rate_limits = self.rate_limits.clone();
        
        // Clean up old rate limits every minute
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                let mut limits = rate_limits.lock().await;
                let now = Instant::now();
                limits.retain(|_, (last_request, _)| {
                    now.duration_since(*last_request) < Duration::from_secs(60)
                });
            }
        });

        // Log peer statistics every 5 minutes
        let peer_scores_clone = self.peer_scores.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(300));
            loop {
                interval.tick().await;
                let scores = peer_scores_clone.lock().await;
                println!("📊 Peer Statistics:");
                for (peer, score) in scores.iter() {
                    println!("  {}: score={:.2}, messages={}, invalid={}", 
                        peer.as_str(), score.score, score.message_count, score.invalid_messages);
                }
            }
        });
    }

    /// Get peer statistics
    pub async fn get_peer_stats(&self) -> HashMap<Address, PeerScore> {
        self.peer_scores.lock().await.clone()
    }

    /// Get validator set statistics
    pub async fn get_validator_stats(&self) -> HashMap<Address, ValidatorReputation> {
        let validator_set = self.validator_set.lock().await;
        validator_set.get_validator_stats()
    }
}

impl Clone for ConsensusIntegration {
    fn clone(&self) -> Self {
        Self {
            propagator: self.propagator.clone(),
            validator_set: self.validator_set.clone(),
            dag: self.dag.clone(),
            tx_pool: self.tx_pool.clone(),
            peer_scores: self.peer_scores.clone(),
            rate_limits: self.rate_limits.clone(),
            rate_config: self.rate_config.clone(),
            local_address: self.local_address.clone(),
            local_keypair: None, // Keypair doesn't implement Clone, so we can't clone it
        }
    }
}

/// Message validation result
#[derive(Debug)]
#[allow(dead_code)]
struct MessageValidationResult {
    is_valid: bool,
    reason: String,
} 
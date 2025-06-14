use crate::types::finality::{FinalityVote, Justification};
use crate::security::{SecurityManager, SecurityConfig, SecurityAuditLog};
use crate::lib::ai::{AIService, AIRequest};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteAnalysis {
    pub trust_score: f32,
    pub risk_level: RiskLevel,
    pub voting_pattern: String,
    pub anomalies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

pub struct AISecureConsensus {
    security_manager: Arc<SecurityManager>,
    ai_service: Arc<AIService>,
    votes: HashMap<String, Vec<FinalityVote>>,
    finalized_blocks: HashSet<String>,
    justifications: HashMap<String, Justification>,
    vote_analyses: HashMap<String, VoteAnalysis>,
    finality_threshold: usize,
}

impl AISecureConsensus {
    pub fn new(security_config: SecurityConfig, threshold: usize) -> Self {
        Self {
            security_manager: Arc::new(SecurityManager::new(security_config)),
            ai_service: Arc::new(AIService::new(security_config)),
            votes: HashMap::new(),
            finalized_blocks: HashSet::new(),
            justifications: HashMap::new(),
            vote_analyses: HashMap::new(),
            finality_threshold: threshold,
        }
    }

    pub async fn submit_vote(&mut self, vote: FinalityVote) -> Result<(), String> {
        // Security checks
        self.security_manager.check_rate_limit(&vote.signer).await?;

        // Analyze vote
        let analysis = self.analyze_vote(&vote).await?;

        // Check for high-risk votes
        if matches!(analysis.risk_level, RiskLevel::Critical) {
            return Err("Vote rejected due to critical risk level".to_string());
        }

        // Store vote and analysis
        self.votes.entry(vote.block_hash.clone())
            .or_default()
            .push(vote.clone());
        self.vote_analyses.insert(vote.signer.clone(), analysis);

        // Log audit
        self.security_manager.log_audit(SecurityAuditLog {
            timestamp: crate::utils::time::get_findag_time_micro(),
            action: "VOTE_SUBMITTED".to_string(),
            content_hash: self.hash_vote(&vote),
            user_id: Some(vote.signer.clone()),
            status: "SUCCESS".to_string(),
            details: HashMap::new(),
        }).await;

        Ok(())
    }

    async fn analyze_vote(&self, vote: &FinalityVote) -> Result<VoteAnalysis, String> {
        let prompt = format!(
            "Analyze the following finality vote for security risks and trustworthiness:\n\
             Block Hash: {}\n\
             Signer: {}\n\
             \n\
             Please provide:\n\
             1. A trust score (0-1)\n\
             2. A risk level (Low/Medium/High/Critical)\n\
             3. Voting pattern analysis\n\
             4. Any anomalies detected\n\
             Format the response as JSON with these fields: trust_score, risk_level, voting_pattern, anomalies",
            vote.block_hash,
            vote.signer
        );

        let request = AIRequest {
            prompt,
            model: Some("gpt-3.5-turbo".to_string()),
            temperature: Some(0.3),
            max_tokens: Some(500),
        };

        match self.ai_service.generate_response(request, &vote.signer).await {
            Ok(response) => {
                match serde_json::from_str::<VoteAnalysis>(&response.text) {
                    Ok(analysis) => Ok(analysis),
                    Err(e) => Err(format!("Failed to parse vote analysis: {}", e)),
                }
            }
            Err(e) => Err(format!("Vote analysis failed: {}", e)),
        }
    }

    pub async fn try_finalize(&mut self, block_hash: &str) -> Option<Justification> {
        let votes = match self.votes.get(block_hash) {
            Some(v) => v,
            None => return None,
        };

        // Check if we have enough votes
        if votes.len() < self.finality_threshold {
            return None;
        }

        // Analyze voting patterns
        let mut total_trust_score = 0.0;
        let mut high_risk_votes = 0;

        for vote in votes {
            if let Some(analysis) = self.vote_analyses.get(&vote.signer) {
                total_trust_score += analysis.trust_score;
                if matches!(analysis.risk_level, RiskLevel::High | RiskLevel::Critical) {
                    high_risk_votes += 1;
                }
            }
        }

        // Calculate average trust score
        let avg_trust_score = total_trust_score / votes.len() as f32;

        // Only finalize if trust score is high enough and not too many high-risk votes
        if avg_trust_score >= 0.7 && high_risk_votes < self.finality_threshold / 3 {
            let justification = Justification {
                block_hash: block_hash.to_string(),
                signers: votes.iter().map(|v| v.signer.clone()).collect(),
            };
            self.finalized_blocks.insert(block_hash.to_string());
            self.justifications.insert(block_hash.to_string(), justification.clone());
            Some(justification)
        } else {
            None
        }
    }

    fn hash_vote(&self, vote: &FinalityVote) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(vote.block_hash.as_bytes());
        hasher.update(vote.signer.as_bytes());
        hex::encode(hasher.finalize().as_bytes())
    }

    pub fn is_finalized(&self, block_hash: &str) -> bool {
        self.finalized_blocks.contains(block_hash)
    }

    pub fn get_justification(&self, block_hash: &str) -> Option<&Justification> {
        self.justifications.get(block_hash)
    }

    pub fn get_vote_analysis(&self, signer: &str) -> Option<&VoteAnalysis> {
        self.vote_analyses.get(signer)
    }
} 
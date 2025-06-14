use libp2p::{
    core::upgrade,
    identity,
    PeerId,
    Multiaddr,
};
use crate::security::{SecurityManager, SecurityConfig, SecurityAuditLog};
use crate::lib::ai::{AIService, AIRequest};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub content: String,
    pub sender: PeerId,
    pub timestamp: u64,
    pub message_type: MessageType,
    pub ai_analysis: Option<MessageAnalysis>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Block,
    Transaction,
    Vote,
    AIRequest,
    AIResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageAnalysis {
    pub relevance_score: f32,
    pub risk_level: RiskLevel,
    pub category: String,
    pub suggested_actions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

pub struct AISecureNetwork {
    security_manager: Arc<SecurityManager>,
    ai_service: Arc<AIService>,
    message_history: Arc<RwLock<HashMap<String, NetworkMessage>>>,
}

impl AISecureNetwork {
    pub fn new(security_config: SecurityConfig) -> Self {
        Self {
            security_manager: Arc::new(SecurityManager::new(security_config)),
            ai_service: Arc::new(AIService::new(security_config)),
            message_history: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn process_message(&self, message: NetworkMessage) -> Result<(), String> {
        // Security checks
        self.security_manager.validate_content(&message.content).await?;
        self.security_manager.check_rate_limit(&message.sender.to_string()).await?;

        // AI analysis
        let analysis = self.analyze_message(&message).await?;
        
        // Risk assessment
        if matches!(analysis.risk_level, RiskLevel::Critical) {
            return Err("Message rejected due to critical risk level".to_string());
        }

        // Store message with analysis
        let mut history = self.message_history.write().await;
        let mut message = message;
        message.ai_analysis = Some(analysis);
        history.insert(message.sender.to_string(), message);

        // Log audit
        self.security_manager.log_audit(SecurityAuditLog {
            timestamp: crate::utils::time::get_findag_time_micro(),
            action: "MESSAGE_PROCESSED".to_string(),
            content_hash: self.hash_content(&message.content),
            user_id: Some(message.sender.to_string()),
            status: "SUCCESS".to_string(),
            details: HashMap::new(),
        }).await;

        Ok(())
    }

    async fn analyze_message(&self, message: &NetworkMessage) -> Result<MessageAnalysis, String> {
        let prompt = format!(
            "Analyze the following network message for security risks and relevance:\n\
             Content: {}\n\
             Type: {:?}\n\
             Sender: {}\n\
             \n\
             Please provide:\n\
             1. A relevance score (0-1)\n\
             2. A risk level (Low/Medium/High/Critical)\n\
             3. A category\n\
             4. Suggested actions\n\
             Format the response as JSON with these fields: relevance_score, risk_level, category, suggested_actions",
            message.content,
            message.message_type,
            message.sender
        );

        let request = AIRequest {
            prompt,
            model: Some("gpt-3.5-turbo".to_string()),
            temperature: Some(0.3),
            max_tokens: Some(500),
        };

        match self.ai_service.generate_response(request, &message.sender.to_string()).await {
            Ok(response) => {
                match serde_json::from_str::<MessageAnalysis>(&response.text) {
                    Ok(analysis) => Ok(analysis),
                    Err(e) => Err(format!("Failed to parse AI analysis: {}", e)),
                }
            }
            Err(e) => Err(format!("AI analysis failed: {}", e)),
        }
    }

    fn hash_content(&self, content: &str) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(content.as_bytes());
        hex::encode(hasher.finalize().as_bytes())
    }

    pub async fn get_message_history(&self) -> Vec<NetworkMessage> {
        let history = self.message_history.read().await;
        history.values().cloned().collect()
    }

    pub async fn get_high_risk_messages(&self) -> Vec<NetworkMessage> {
        let history = self.message_history.read().await;
        history.values()
            .filter(|msg| {
                if let Some(analysis) = &msg.ai_analysis {
                    matches!(analysis.risk_level, RiskLevel::High | RiskLevel::Critical)
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    }
} 
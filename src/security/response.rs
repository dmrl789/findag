use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use crate::security::monitoring::{ThreatAlert, ThreatSeverity};
use crate::security::ai_analysis::{SecurityPattern, ThreatIntelligence};
use crate::lib::ai::AIService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseAction {
    pub action_id: String,
    pub name: String,
    pub description: String,
    pub severity: ThreatSeverity,
    pub steps: Vec<String>,
    pub timeout_seconds: u64,
    pub requires_confirmation: bool,
    pub success_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseResult {
    pub action_id: String,
    pub threat_id: String,
    pub timestamp: u64,
    pub status: ResponseStatus,
    pub details: HashMap<String, String>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    RequiresConfirmation,
}

pub struct SecurityResponder {
    ai_service: Arc<AIService>,
    actions: Arc<RwLock<HashMap<String, ResponseAction>>>,
    results: Arc<RwLock<Vec<ResponseResult>>>,
    patterns: Arc<RwLock<Vec<SecurityPattern>>>,
    threats: Arc<RwLock<HashMap<String, ThreatIntelligence>>>,
}

impl SecurityResponder {
    pub fn new(ai_service: Arc<AIService>) -> Self {
        Self {
            ai_service,
            actions: Arc::new(RwLock::new(HashMap::new())),
            results: Arc::new(RwLock::new(Vec::new())),
            patterns: Arc::new(RwLock::new(Vec::new())),
            threats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_action(&self, action: ResponseAction) {
        let mut actions = self.actions.write().await;
        actions.insert(action.action_id.clone(), action);
    }

    pub async fn handle_threat(&self, alert: ThreatAlert) -> Result<ResponseResult, String> {
        // Find matching patterns and threats
        let patterns = self.patterns.read().await;
        let threats = self.threats.read().await;

        // Prepare context for AI analysis
        let patterns_str = serde_json::to_string(&*patterns).map_err(|e| e.to_string())?;
        let threats_str = serde_json::to_string(&*threats).map_err(|e| e.to_string())?;
        let alert_str = serde_json::to_string(&alert).map_err(|e| e.to_string())?;

        let prompt = format!(
            "Analyze the following security threat and determine appropriate response actions:\n\
             Alert: {}\n\
             Known Patterns: {}\n\
             Threat Intelligence: {}\n\
             \n\
             Please provide:\n\
             1. Recommended response actions\n\
             2. Action priority\n\
             3. Required confirmation\n\
             4. Success criteria\n\
             Format the response as JSON with these fields: action_id, name, description, severity, steps, timeout_seconds, requires_confirmation, success_criteria",
            alert_str,
            patterns_str,
            threats_str
        );

        let request = crate::lib::ai::AIRequest {
            prompt,
            model: Some("gpt-3.5-turbo".to_string()),
            temperature: Some(0.3),
            max_tokens: Some(1000),
        };

        match self.ai_service.generate_response(request, "security_responder").await {
            Ok(response) => {
                if let Ok(action) = serde_json::from_str::<ResponseAction>(&response.text) {
                    // Execute the response action
                    let result = self.execute_action(action, alert).await?;
                    
                    // Store the result
                    let mut results = self.results.write().await;
                    results.push(result.clone());
                    
                    Ok(result)
                } else {
                    Err("Failed to parse AI response".to_string())
                }
            }
            Err(e) => Err(format!("AI analysis failed: {}", e)),
        }
    }

    async fn execute_action(&self, action: ResponseAction, alert: ThreatAlert) -> Result<ResponseResult, String> {
        let start_time = std::time::Instant::now();
        let mut details = HashMap::new();

        // Execute each step of the action
        for (i, step) in action.steps.iter().enumerate() {
            details.insert(format!("step_{}", i + 1), step.clone());
            
            // Simulate step execution (replace with actual implementation)
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        let execution_time = start_time.elapsed().as_millis() as u64;

        Ok(ResponseResult {
            action_id: action.action_id,
            threat_id: alert.source,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            status: if action.requires_confirmation {
                ResponseStatus::RequiresConfirmation
            } else {
                ResponseStatus::Completed
            },
            details,
            execution_time_ms: execution_time,
        })
    }

    pub async fn get_action_results(&self) -> Vec<ResponseResult> {
        self.results.read().await.clone()
    }

    pub async fn update_patterns(&self, patterns: Vec<SecurityPattern>) {
        let mut current_patterns = self.patterns.write().await;
        *current_patterns = patterns;
    }

    pub async fn update_threats(&self, threats: HashMap<String, ThreatIntelligence>) {
        let mut current_threats = self.threats.write().await;
        *current_threats = threats;
    }

    pub async fn confirm_action(&self, action_id: &str) -> Result<ResponseResult, String> {
        let mut results = self.results.write().await;
        if let Some(result) = results.iter_mut().find(|r| r.action_id == action_id) {
            if result.status == ResponseStatus::RequiresConfirmation {
                result.status = ResponseStatus::Completed;
                Ok(result.clone())
            } else {
                Err("Action does not require confirmation".to_string())
            }
        } else {
            Err("Action not found".to_string())
        }
    }
} 
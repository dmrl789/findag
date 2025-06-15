use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use crate::security::audit::{SecurityAuditLog, SecuritySeverity};
use crate::security::monitoring::{ThreatAlert, ThreatSeverity};
use crate::security::response::{ResponseAction, ResponseResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModelConfig {
    pub model_name: String,
    pub version: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub allowed_categories: Vec<String>,
    pub blocked_categories: Vec<String>,
    pub max_prompt_length: usize,
    pub min_prompt_length: usize,
    pub require_content_filtering: bool,
    pub require_safety_checks: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIPromptValidation {
    pub prompt: String,
    pub category: String,
    pub user_id: String,
    pub timestamp: u64,
    pub validation_status: ValidationStatus,
    pub risk_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    Valid,
    Invalid,
    RequiresReview,
    Blocked,
}

#[derive(Debug, Clone)]
pub struct AISecurityManager {
    model_configs: HashMap<String, AIModelConfig>,
    prompt_history: Arc<RwLock<Vec<AIPromptValidation>>>,
    risk_patterns: Vec<String>,
    blocked_patterns: Vec<String>,
}

impl AISecurityManager {
    pub fn new() -> Self {
        Self {
            model_configs: HashMap::new(),
            prompt_history: Arc::new(RwLock::new(Vec::new())),
            risk_patterns: vec![
                r"(?i)password|secret|key|token".to_string(),
                r"(?i)exploit|hack|bypass".to_string(),
                r"(?i)admin|root|system".to_string(),
            ],
            blocked_patterns: vec![
                r"(?i)malware|virus|trojan".to_string(),
                r"(?i)illegal|criminal|fraud".to_string(),
                r"(?i)explicit|adult|nsfw".to_string(),
            ],
        }
    }

    pub async fn register_model(&mut self, config: AIModelConfig) -> Result<(), String> {
        if config.model_name.is_empty() {
            return Err("Model name cannot be empty".to_string());
        }
        self.model_configs.insert(config.model_name.clone(), config);
        Ok(())
    }

    pub async fn validate_prompt(&self, prompt: &str, category: &str, user_id: &str) -> Result<AIPromptValidation, String> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Check prompt length
        if prompt.len() < 10 {
            return Err("Prompt too short".to_string());
        }
        if prompt.len() > 10000 {
            return Err("Prompt too long".to_string());
        }

        // Check for blocked patterns
        for pattern in &self.blocked_patterns {
            if regex::Regex::new(pattern)
                .unwrap()
                .is_match(prompt)
            {
                return Ok(AIPromptValidation {
                    prompt: prompt.to_string(),
                    category: category.to_string(),
                    user_id: user_id.to_string(),
                    timestamp,
                    validation_status: ValidationStatus::Blocked,
                    risk_score: 1.0,
                });
            }
        }

        // Calculate risk score
        let mut risk_score = 0.0;
        for pattern in &self.risk_patterns {
            if regex::Regex::new(pattern)
                .unwrap()
                .is_match(prompt)
            {
                risk_score += 0.2;
            }
        }

        let validation_status = if risk_score > 0.8 {
            ValidationStatus::Blocked
        } else if risk_score > 0.5 {
            ValidationStatus::RequiresReview
        } else {
            ValidationStatus::Valid
        };

        let validation = AIPromptValidation {
            prompt: prompt.to_string(),
            category: category.to_string(),
            user_id: user_id.to_string(),
            timestamp,
            validation_status: validation_status.clone(),
            risk_score,
        };

        // Store validation result
        let mut history = self.prompt_history.write().await;
        history.push(validation.clone());

        Ok(validation)
    }

    pub async fn get_prompt_history(&self, user_id: Option<&str>) -> Vec<AIPromptValidation> {
        let history = self.prompt_history.read().await;
        if let Some(user_id) = user_id {
            history
                .iter()
                .filter(|v| v.user_id == user_id)
                .cloned()
                .collect()
        } else {
            history.clone()
        }
    }

    pub async fn analyze_threats(&self, prompt: &str) -> Vec<ThreatAlert> {
        let mut alerts = Vec::new();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Check for high-risk patterns
        for pattern in &self.risk_patterns {
            if regex::Regex::new(pattern)
                .unwrap()
                .is_match(prompt)
            {
                alerts.push(ThreatAlert {
                    timestamp,
                    source: "AI Security".to_string(),
                    severity: ThreatSeverity::High,
                    description: format!("High-risk pattern detected: {}", pattern),
                });
            }
        }

        // Check for blocked patterns
        for pattern in &self.blocked_patterns {
            if regex::Regex::new(pattern)
                .unwrap()
                .is_match(prompt)
            {
                alerts.push(ThreatAlert {
                    timestamp,
                    source: "AI Security".to_string(),
                    severity: ThreatSeverity::Critical,
                    description: format!("Blocked pattern detected: {}", pattern),
                });
            }
        }

        alerts
    }

    pub async fn get_model_config(&self, model_name: &str) -> Option<&AIModelConfig> {
        self.model_configs.get(model_name)
    }

    pub async fn update_model_config(&mut self, model_name: &str, config: AIModelConfig) -> Result<(), String> {
        if !self.model_configs.contains_key(model_name) {
            return Err("Model not found".to_string());
        }
        self.model_configs.insert(model_name.to_string(), config);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_prompt_validation() {
        let mut manager = AISecurityManager::new();
        let result = manager
            .validate_prompt("Hello, how are you?", "general", "user1")
            .await
            .unwrap();
        assert_eq!(result.validation_status, ValidationStatus::Valid);
        assert!(result.risk_score < 0.5);
    }

    #[tokio::test]
    async fn test_blocked_pattern() {
        let manager = AISecurityManager::new();
        let result = manager
            .validate_prompt("How to hack the system", "general", "user1")
            .await
            .unwrap();
        assert_eq!(result.validation_status, ValidationStatus::Blocked);
        assert!(result.risk_score > 0.8);
    }
} 
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use crate::security::{SecurityManager, SecurityConfig, SecurityAuditLog};
use std::collections::HashMap;
use blake3::Hasher;

pub struct AIService {
    security_manager: Arc<SecurityManager>,
}

impl AIService {
    pub fn new(security_config: SecurityConfig) -> Self {
        Self {
            security_manager: Arc::new(SecurityManager::new(security_config)),
        }
    }

    pub async fn generate_response(&self, request: AIRequest, user_id: &str) -> Result<AIResponse, String> {
        // Security checks
        self.security_manager.check_rate_limit(user_id).await?;
        self.security_manager.validate_content(&request.prompt).await?;

        // Verify API key
        let api_key = match env::var("OPENAI_API_KEY") {
            Ok(key) => key,
            Err(_) => return Err("OpenAI API key not found in environment variables".to_string()),
        };

        if !SecurityManager::verify_api_key(&api_key).await {
            return Err("Invalid API key format".to_string());
        }

        // Generate content hash for audit
        let content_hash = {
            let mut hasher = Hasher::new();
            hasher.update(request.prompt.as_bytes());
            hex::encode(hasher.finalize().as_bytes())
        };

        // Create audit log
        let audit_log = SecurityAuditLog {
            timestamp: crate::utils::time::get_findag_time_micro(),
            action: "AI_REQUEST".to_string(),
            content_hash: content_hash.clone(),
            user_id: Some(user_id.to_string()),
            status: "PENDING".to_string(),
            details: HashMap::new(),
        };
        self.security_manager.log_audit(audit_log).await;

        // Make API request
        let client = reqwest::Client::new();
        let chat_request = ChatRequest {
            model: request.model.unwrap_or_else(|| DEFAULT_MODEL.to_string()),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: request.prompt,
            }],
            temperature: request.temperature.unwrap_or(DEFAULT_TEMPERATURE),
            max_tokens: request.max_tokens.unwrap_or(DEFAULT_MAX_TOKENS),
        };

        let response = match client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&chat_request)
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                // Log failed request
                let error_log = SecurityAuditLog {
                    timestamp: crate::utils::time::get_findag_time_micro(),
                    action: "AI_REQUEST_FAILED".to_string(),
                    content_hash,
                    user_id: Some(user_id.to_string()),
                    status: "ERROR".to_string(),
                    details: {
                        let mut details = HashMap::new();
                        details.insert("error".to_string(), e.to_string());
                        details
                    },
                };
                self.security_manager.log_audit(error_log).await;
                return Err(format!("Request failed: {}", e));
            }
        };

        match response.json::<ChatResponse>().await {
            Ok(chat_response) => {
                let ai_response = AIResponse {
                    text: chat_response.choices[0].message.content.clone(),
                    error: None,
                };

                // Log successful response
                let success_log = SecurityAuditLog {
                    timestamp: crate::utils::time::get_findag_time_micro(),
                    action: "AI_REQUEST_SUCCESS".to_string(),
                    content_hash,
                    user_id: Some(user_id.to_string()),
                    status: "SUCCESS".to_string(),
                    details: HashMap::new(),
                };
                self.security_manager.log_audit(success_log).await;

                Ok(ai_response)
            }
            Err(e) => {
                // Log parsing error
                let error_log = SecurityAuditLog {
                    timestamp: crate::utils::time::get_findag_time_micro(),
                    action: "AI_RESPONSE_PARSE_ERROR".to_string(),
                    content_hash,
                    user_id: Some(user_id.to_string()),
                    status: "ERROR".to_string(),
                    details: {
                        let mut details = HashMap::new();
                        details.insert("error".to_string(), e.to_string());
                        details
                    },
                };
                self.security_manager.log_audit(error_log).await;
                Err(format!("Failed to parse response: {}", e))
            }
        }
    }

    pub async fn get_audit_logs(&self) -> Vec<SecurityAuditLog> {
        self.security_manager.get_audit_logs().await
    }
}

// Types
#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}

pub struct AIResponse {
    pub text: String,
    pub error: Option<String>,
}

pub struct AIRequest {
    pub prompt: String,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

// Default configuration
const DEFAULT_MODEL: &str = "gpt-3.5-turbo";
const DEFAULT_TEMPERATURE: f32 = 0.7;
const DEFAULT_MAX_TOKENS: u32 = 1000;

pub fn is_ai_configured() -> bool {
    env::var("OPENAI_API_KEY").is_ok()
} 
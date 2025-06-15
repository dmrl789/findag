use std::sync::Arc;
use serde::{Serialize, Deserialize};
use crate::security::SecurityConfig;

mod request;
mod response;

pub use request::AIRequest;
pub use response::AIResponse;

#[derive(Clone)]
pub struct AIService {
    config: Arc<SecurityConfig>,
}

impl AIService {
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    pub async fn generate_response(&self, request: AIRequest) -> Result<AIResponse, String> {
        // TODO: Implement actual AI service integration
        // For now, return a mock response
        Ok(AIResponse {
            text: "Mock AI response".to_string(),
            error: None,
        })
    }
}

pub async fn generate_response(request: AIRequest) -> Result<AIResponse, String> {
    let config = SecurityConfig::default();
    let service = AIService::new(config);
    service.generate_response(request).await
} 
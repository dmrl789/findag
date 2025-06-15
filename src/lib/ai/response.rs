use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub text: String,
    pub error: Option<String>,
} 
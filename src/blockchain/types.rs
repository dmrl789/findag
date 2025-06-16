use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockAnalysis {
    pub relevance_score: f32,
    pub category: String,
    pub metadata: serde_json::Value,
} 
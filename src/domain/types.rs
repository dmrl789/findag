use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub name: String,
    pub description: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainAnalysis {
    pub relevance_score: f32,
    pub category: String,
    pub keywords: Vec<String>,
    pub summary: String,
    pub suggested_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainSearchQuery {
    pub query: String,
    pub filters: HashMap<String, String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainSearchResult {
    pub domains: Vec<Domain>,
    pub total: usize,
    pub page: usize,
    pub has_more: bool,
} 
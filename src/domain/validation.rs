use crate::ai::{generate_response, AIRequest};
use crate::domain::types::{Domain, DomainAnalysis, DomainValidationResult};
use std::collections::HashMap;

pub async fn validate_domain(domain: &Domain) -> DomainValidationResult {
    let mut result = DomainValidationResult {
        is_valid: true,
        errors: Vec::new(),
        warnings: Vec::new(),
        suggestions: Vec::new(),
    };

    // Basic validation
    if domain.name.is_empty() {
        result.is_valid = false;
        result.errors.push("Domain name cannot be empty".to_string());
    }

    if domain.description.is_empty() {
        result.warnings.push("Domain description is empty".to_string());
    }

    // AI-powered validation
    if let Ok(analysis) = analyze_domain(domain).await {
        // Check relevance score
        if analysis.relevance_score < 0.5 {
            result.warnings.push("Domain has low relevance score".to_string());
        }

        // Add AI suggestions
        if !analysis.suggested_tags.is_empty() {
            result.suggestions.push(format!(
                "Consider adding these tags: {}",
                analysis.suggested_tags.join(", ")
            ));
        }

        // Add category suggestion
        result.suggestions.push(format!(
            "This domain appears to be in the '{}' category",
            analysis.category
        ));
    }

    result
}

pub async fn analyze_domain(domain: &Domain) -> Result<DomainAnalysis, String> {
    let prompt = format!(
        "Analyze the following domain and provide a detailed analysis:\n\
         Name: {}\n\
         Description: {}\n\
         Metadata: {:?}\n\
         \n\
         Please provide:\n\
         1. A relevance score (0-1)\n\
         2. A category\n\
         3. Key keywords\n\
         4. A brief summary\n\
         5. Suggested tags\n\
         Format the response as JSON with these fields: relevance_score, category, keywords, summary, suggested_tags",
        domain.name,
        domain.description,
        domain.metadata
    );

    let request = AIRequest {
        data: "placeholder_data".to_string(),
        prompt,
        model: Some("gpt-3.5-turbo".to_string()),
        temperature: Some(0.3),
        max_tokens: Some(500),
    };

    match generate_response(request).await {
        Ok(response) if response.error.is_none() => {
            match serde_json::from_str::<DomainAnalysis>(&response.text) {
                Ok(analysis) => Ok(analysis),
                Err(e) => Err(format!("Failed to parse AI response: {}", e)),
            }
        }
        Ok(response) => Err(response.error.unwrap_or_else(|| "Unknown error".to_string())),
        Err(e) => Err(format!("AI analysis failed: {}", e)),
    }
}

pub async fn search_domains(
    query: &str,
    filters: &HashMap<String, String>,
) -> Result<Vec<Domain>, String> {
    let prompt = format!(
        "Search for domains matching the following criteria:\n\
         Query: {}\n\
         Filters: {:?}\n\
         \n\
         Please provide a list of relevant domains with their details.\
         Format the response as a JSON array of domain objects.",
        query, filters
    );

    let request = AIRequest {
        data: "placeholder_data".to_string(),
        prompt,
        model: Some("gpt-3.5-turbo".to_string()),
        temperature: Some(0.3),
        max_tokens: Some(1000),
    };

    match generate_response(request).await {
        Ok(response) if response.error.is_none() => {
            match serde_json::from_str::<Vec<Domain>>(&response.text) {
                Ok(domains) => Ok(domains),
                Err(e) => Err(format!("Failed to parse AI response: {}", e)),
            }
        }
        Ok(response) => Err(response.error.unwrap_or_else(|| "Unknown error".to_string())),
        Err(e) => Err(format!("Domain search failed: {}", e)),
    }
} 
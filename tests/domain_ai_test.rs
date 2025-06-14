use findag_core::domain::types::{Domain, DomainAnalysis};
use findag_core::domain::validation::{analyze_domain, validate_domain};
use std::collections::HashMap;

#[tokio::test]
async fn test_domain_analysis() {
    // Skip test if API key is not configured
    if !findag_core::ai::is_ai_configured() {
        println!("Skipping domain AI test - OPENAI_API_KEY not configured");
        return;
    }

    let mut metadata = HashMap::new();
    metadata.insert("type".to_string(), "technology".to_string());
    metadata.insert("language".to_string(), "en".to_string());

    let domain = Domain {
        name: "AI Research Hub".to_string(),
        description: "A platform for artificial intelligence research and collaboration".to_string(),
        metadata,
        ai_analysis: None,
    };

    // Test domain analysis
    let analysis = analyze_domain(&domain).await;
    assert!(analysis.is_ok(), "Domain analysis failed: {:?}", analysis.err());
    
    let analysis = analysis.unwrap();
    assert!(analysis.relevance_score >= 0.0 && analysis.relevance_score <= 1.0);
    assert!(!analysis.category.is_empty());
    assert!(!analysis.keywords.is_empty());
    assert!(!analysis.summary.is_empty());
    assert!(!analysis.suggested_tags.is_empty());

    // Test domain validation
    let validation = validate_domain(&domain).await;
    assert!(validation.is_valid);
    assert!(validation.errors.is_empty());
    assert!(!validation.suggestions.is_empty());
}

#[tokio::test]
async fn test_domain_search() {
    // Skip test if API key is not configured
    if !findag_core::ai::is_ai_configured() {
        println!("Skipping domain search test - OPENAI_API_KEY not configured");
        return;
    }

    let mut filters = HashMap::new();
    filters.insert("category".to_string(), "technology".to_string());
    filters.insert("language".to_string(), "en".to_string());

    let domains = findag_core::domain::validation::search_domains("AI research", &filters).await;
    assert!(domains.is_ok(), "Domain search failed: {:?}", domains.err());
    
    let domains = domains.unwrap();
    assert!(!domains.is_empty());
    
    for domain in domains {
        assert!(!domain.name.is_empty());
        assert!(!domain.description.is_empty());
    }
} 
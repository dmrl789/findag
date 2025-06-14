use findag_core::lib::ai::{AIService, AIRequest};
use findag_core::security::SecurityConfig;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_security_features() {
    // Skip test if API key is not configured
    if !findag_core::lib::ai::is_ai_configured() {
        println!("Skipping AI security test - OPENAI_API_KEY not configured");
        return;
    }

    let config = SecurityConfig {
        max_requests_per_minute: 2,
        max_content_length: 100,
        allowed_categories: vec!["technology".to_string()],
        blocked_keywords: vec!["malware".to_string()],
        require_content_verification: true,
    };

    let ai_service = AIService::new(config);

    // Test rate limiting
    let request = AIRequest {
        prompt: "What is blockchain?".to_string(),
        model: None,
        temperature: None,
        max_tokens: None,
    };

    // First request should succeed
    let result1 = ai_service.generate_response(request.clone(), "test_user").await;
    assert!(result1.is_ok(), "First request should succeed");

    // Second request should succeed
    let result2 = ai_service.generate_response(request.clone(), "test_user").await;
    assert!(result2.is_ok(), "Second request should succeed");

    // Third request should fail due to rate limit
    let result3 = ai_service.generate_response(request.clone(), "test_user").await;
    assert!(result3.is_err(), "Third request should fail due to rate limit");
    assert!(result3.unwrap_err().contains("Rate limit exceeded"));

    // Test content validation
    let malicious_request = AIRequest {
        prompt: "How to create malware?".to_string(),
        model: None,
        temperature: None,
        max_tokens: None,
    };

    let result = ai_service.generate_response(malicious_request, "test_user").await;
    assert!(result.is_err(), "Malicious request should be blocked");
    assert!(result.unwrap_err().contains("blocked keywords"));

    // Test content length limit
    let long_request = AIRequest {
        prompt: "x".repeat(200), // Exceeds max_content_length
        model: None,
        temperature: None,
        max_tokens: None,
    };

    let result = ai_service.generate_response(long_request, "test_user").await;
    assert!(result.is_err(), "Long request should be blocked");
    assert!(result.unwrap_err().contains("maximum length"));

    // Test audit logging
    let logs = ai_service.get_audit_logs().await;
    assert!(!logs.is_empty(), "Audit logs should not be empty");

    // Verify log contents
    let success_logs: Vec<_> = logs.iter()
        .filter(|log| log.status == "SUCCESS")
        .collect();
    assert_eq!(success_logs.len(), 2, "Should have 2 successful requests logged");

    let error_logs: Vec<_> = logs.iter()
        .filter(|log| log.status == "ERROR")
        .collect();
    assert!(error_logs.len() >= 2, "Should have at least 2 error logs");
}

#[tokio::test]
async fn test_api_key_validation() {
    // Test with invalid API key
    std::env::set_var("OPENAI_API_KEY", "invalid_key");
    
    let config = SecurityConfig::default();
    let ai_service = AIService::new(config);

    let request = AIRequest {
        prompt: "Test".to_string(),
        model: None,
        temperature: None,
        max_tokens: None,
    };

    let result = ai_service.generate_response(request, "test_user").await;
    assert!(result.is_err(), "Invalid API key should be rejected");
    assert!(result.unwrap_err().contains("Invalid API key format"));
} 
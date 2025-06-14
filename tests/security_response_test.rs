use findag_core::security::response::{SecurityResponder, ResponseAction, ResponseStatus};
use findag_core::security::monitoring::{ThreatAlert, ThreatSeverity};
use findag_core::security::ai_analysis::{SecurityPattern, ThreatIntelligence};
use findag_core::lib::ai::AIService;
use findag_core::security::SecurityConfig;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::test]
async fn test_action_registration() {
    let config = SecurityConfig::default();
    let ai_service = Arc::new(AIService::new(config));
    let responder = SecurityResponder::new(ai_service);

    let action = ResponseAction {
        action_id: "TEST-001".to_string(),
        name: "Test Action".to_string(),
        description: "Test response action".to_string(),
        severity: ThreatSeverity::Critical,
        steps: vec!["Step 1".to_string(), "Step 2".to_string()],
        timeout_seconds: 30,
        requires_confirmation: true,
        success_criteria: vec!["Criteria 1".to_string()],
    };

    responder.register_action(action).await;
    let results = responder.get_action_results().await;
    assert!(results.is_empty());
}

#[tokio::test]
async fn test_threat_handling() {
    let config = SecurityConfig::default();
    let ai_service = Arc::new(AIService::new(config));
    let responder = SecurityResponder::new(ai_service);

    // Set up test patterns and threats
    let pattern = SecurityPattern {
        pattern_type: "resource_exhaustion".to_string(),
        description: "High CPU usage pattern".to_string(),
        indicators: vec!["cpu_usage > 0.9".to_string()],
        confidence: 0.9,
        severity: ThreatSeverity::Critical,
        mitigation_steps: vec!["Scale resources".to_string()],
    };

    let threat = ThreatIntelligence {
        threat_id: "THREAT-001".to_string(),
        name: "Resource Exhaustion Attack".to_string(),
        description: "Attempt to exhaust system resources".to_string(),
        indicators: vec!["high_cpu_usage".to_string()],
        severity: ThreatSeverity::Critical,
        category: "DoS".to_string(),
        source: "Internal".to_string(),
        last_seen: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        confidence: 0.9,
    };

    let mut threats = HashMap::new();
    threats.insert(threat.threat_id.clone(), threat);

    responder.update_patterns(vec![pattern]).await;
    responder.update_threats(threats).await;

    // Create a test alert
    let alert = ThreatAlert {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        severity: ThreatSeverity::Critical,
        source: "THREAT-001".to_string(),
        description: "High CPU usage detected".to_string(),
        affected_components: vec!["network".to_string()],
        recommended_actions: vec!["Scale resources".to_string()],
        confidence_score: 0.9,
    };

    let result = responder.handle_threat(alert).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.threat_id, "THREAT-001");
}

#[tokio::test]
async fn test_action_confirmation() {
    let config = SecurityConfig::default();
    let ai_service = Arc::new(AIService::new(config));
    let responder = SecurityResponder::new(ai_service);

    // Create a test action that requires confirmation
    let action = ResponseAction {
        action_id: "TEST-002".to_string(),
        name: "Test Action".to_string(),
        description: "Test response action".to_string(),
        severity: ThreatSeverity::Critical,
        steps: vec!["Step 1".to_string()],
        timeout_seconds: 30,
        requires_confirmation: true,
        success_criteria: vec!["Criteria 1".to_string()],
    };

    // Create a test alert
    let alert = ThreatAlert {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        severity: ThreatSeverity::Critical,
        source: "TEST-002".to_string(),
        description: "Test alert".to_string(),
        affected_components: vec!["test".to_string()],
        recommended_actions: vec!["Test action".to_string()],
        confidence_score: 0.9,
    };

    // Handle the threat
    let result = responder.handle_threat(alert).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.status, ResponseStatus::RequiresConfirmation);

    // Confirm the action
    let confirmed = responder.confirm_action("TEST-002").await;
    assert!(confirmed.is_ok());
    let confirmed = confirmed.unwrap();
    assert_eq!(confirmed.status, ResponseStatus::Completed);
}

#[tokio::test]
async fn test_action_execution() {
    let config = SecurityConfig::default();
    let ai_service = Arc::new(AIService::new(config));
    let responder = SecurityResponder::new(ai_service);

    // Create a test action that doesn't require confirmation
    let action = ResponseAction {
        action_id: "TEST-003".to_string(),
        name: "Test Action".to_string(),
        description: "Test response action".to_string(),
        severity: ThreatSeverity::Critical,
        steps: vec!["Step 1".to_string(), "Step 2".to_string()],
        timeout_seconds: 30,
        requires_confirmation: false,
        success_criteria: vec!["Criteria 1".to_string()],
    };

    // Create a test alert
    let alert = ThreatAlert {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        severity: ThreatSeverity::Critical,
        source: "TEST-003".to_string(),
        description: "Test alert".to_string(),
        affected_components: vec!["test".to_string()],
        recommended_actions: vec!["Test action".to_string()],
        confidence_score: 0.9,
    };

    // Handle the threat
    let result = responder.handle_threat(alert).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.status, ResponseStatus::Completed);
    assert!(result.execution_time_ms > 0);
} 
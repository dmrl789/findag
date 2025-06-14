use findag_core::security::audit::{AuditManager, AuditEvent, AuditEventType, ComplianceRequirement, ComplianceStatus};
use findag_core::security::monitoring::{ThreatAlert, ThreatSeverity};
use findag_core::security::response::{ResponseAction, ResponseResult, ResponseStatus};
use findag_core::lib::ai::AIService;
use findag_core::security::SecurityConfig;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};

#[tokio::test]
async fn test_event_logging() {
    let config = SecurityConfig::default();
    let ai_service = Arc::new(AIService::new(config));
    let audit_manager = AuditManager::new(ai_service);

    let event = AuditEvent {
        event_id: "TEST-001".to_string(),
        timestamp: Utc::now(),
        event_type: AuditEventType::SecurityAlert,
        severity: ThreatSeverity::Critical,
        source: "test_source".to_string(),
        user_id: None,
        details: {
            let mut details = HashMap::new();
            details.insert("test_key".to_string(), "test_value".to_string());
            details
        },
        related_events: Vec::new(),
        compliance_tags: Vec::new(),
        context: HashMap::new(),
    };

    audit_manager.log_event(event).await;

    let events = audit_manager.get_events(None, None, None, None).await;
    assert!(!events.is_empty());
    assert_eq!(events[0].event_id, "TEST-001");
}

#[tokio::test]
async fn test_security_alert_logging() {
    let config = SecurityConfig::default();
    let ai_service = Arc::new(AIService::new(config));
    let audit_manager = AuditManager::new(ai_service);

    let alert = ThreatAlert {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        severity: ThreatSeverity::Critical,
        source: "test_source".to_string(),
        description: "Test security alert".to_string(),
        affected_components: vec!["test_component".to_string()],
        recommended_actions: vec!["test_action".to_string()],
        confidence_score: 0.9,
    };

    audit_manager.log_security_alert(alert).await;

    let events = audit_manager
        .get_events(Some(AuditEventType::SecurityAlert), None, None, None)
        .await;
    assert!(!events.is_empty());
    assert_eq!(events[0].event_type, AuditEventType::SecurityAlert);
}

#[tokio::test]
async fn test_response_action_logging() {
    let config = SecurityConfig::default();
    let ai_service = Arc::new(AIService::new(config));
    let audit_manager = AuditManager::new(ai_service);

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

    let result = ResponseResult {
        action_id: "TEST-002".to_string(),
        threat_id: "test_threat".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        status: ResponseStatus::Completed,
        details: {
            let mut details = HashMap::new();
            details.insert("test_key".to_string(), "test_value".to_string());
            details
        },
        execution_time_ms: 100,
    };

    audit_manager.log_response_action(action, result).await;

    let events = audit_manager
        .get_events(Some(AuditEventType::ResponseAction), None, None, None)
        .await;
    assert!(!events.is_empty());
    assert_eq!(events[0].event_type, AuditEventType::ResponseAction);
}

#[tokio::test]
async fn test_compliance_management() {
    let config = SecurityConfig::default();
    let ai_service = Arc::new(AIService::new(config));
    let audit_manager = AuditManager::new(ai_service);

    let requirement = ComplianceRequirement {
        requirement_id: "REQ-001".to_string(),
        name: "Test Requirement".to_string(),
        description: "Test compliance requirement".to_string(),
        standard: "Test Standard".to_string(),
        category: "Security".to_string(),
        controls: vec!["Control 1".to_string()],
        audit_frequency: "Monthly".to_string(),
        last_audit: Some(Utc::now()),
        status: ComplianceStatus::Compliant,
    };

    audit_manager.add_compliance_requirement(requirement).await;

    let status = audit_manager.get_compliance_status().await;
    assert!(!status.is_empty());
    assert_eq!(status.get("REQ-001").unwrap(), &ComplianceStatus::Compliant);
}

#[tokio::test]
async fn test_event_filtering() {
    let config = SecurityConfig::default();
    let ai_service = Arc::new(AIService::new(config));
    let audit_manager = AuditManager::new(ai_service);

    // Log events of different types and severities
    let event1 = AuditEvent {
        event_id: "TEST-003".to_string(),
        timestamp: Utc::now(),
        event_type: AuditEventType::SecurityAlert,
        severity: ThreatSeverity::Critical,
        source: "test_source".to_string(),
        user_id: None,
        details: HashMap::new(),
        related_events: Vec::new(),
        compliance_tags: Vec::new(),
        context: HashMap::new(),
    };

    let event2 = AuditEvent {
        event_id: "TEST-004".to_string(),
        timestamp: Utc::now(),
        event_type: AuditEventType::SystemChange,
        severity: ThreatSeverity::Info,
        source: "test_source".to_string(),
        user_id: None,
        details: HashMap::new(),
        related_events: Vec::new(),
        compliance_tags: Vec::new(),
        context: HashMap::new(),
    };

    audit_manager.log_event(event1).await;
    audit_manager.log_event(event2).await;

    // Test filtering by event type
    let security_events = audit_manager
        .get_events(Some(AuditEventType::SecurityAlert), None, None, None)
        .await;
    assert_eq!(security_events.len(), 1);
    assert_eq!(security_events[0].event_type, AuditEventType::SecurityAlert);

    // Test filtering by severity
    let critical_events = audit_manager
        .get_events(None, Some(ThreatSeverity::Critical), None, None)
        .await;
    assert_eq!(critical_events.len(), 1);
    assert_eq!(critical_events[0].severity, ThreatSeverity::Critical);
}

#[tokio::test]
async fn test_compliance_report_generation() {
    let config = SecurityConfig::default();
    let ai_service = Arc::new(AIService::new(config));
    let audit_manager = AuditManager::new(ai_service);

    // Add a compliance requirement
    let requirement = ComplianceRequirement {
        requirement_id: "REQ-002".to_string(),
        name: "Test Requirement".to_string(),
        description: "Test compliance requirement".to_string(),
        standard: "Test Standard".to_string(),
        category: "Security".to_string(),
        controls: vec!["Control 1".to_string()],
        audit_frequency: "Monthly".to_string(),
        last_audit: Some(Utc::now()),
        status: ComplianceStatus::Compliant,
    };

    audit_manager.add_compliance_requirement(requirement).await;

    // Log some events
    let event = AuditEvent {
        event_id: "TEST-005".to_string(),
        timestamp: Utc::now(),
        event_type: AuditEventType::ComplianceCheck,
        severity: ThreatSeverity::Info,
        source: "test_source".to_string(),
        user_id: None,
        details: HashMap::new(),
        related_events: Vec::new(),
        compliance_tags: vec!["REQ-002".to_string()],
        context: HashMap::new(),
    };

    audit_manager.log_event(event).await;

    // Generate compliance report
    let report = audit_manager.generate_compliance_report().await;
    assert!(!report.is_empty());
    assert!(report.contains("Overall compliance status"));
    assert!(report.contains("Requirement-specific status"));
} 
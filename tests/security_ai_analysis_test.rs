use findag_core::security::ai_analysis::{SecurityAnalyzer, SecurityPattern, ThreatIntelligence};
use findag_core::security::monitoring::{SystemMetrics, ThreatSeverity};
use findag_core::lib::ai::AIService;
use findag_core::security::SecurityConfig;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::test]
async fn test_metrics_analysis() {
    let config = SecurityConfig::default();
    let ai_service = Arc::new(AIService::new(config));
    let analyzer = SecurityAnalyzer::new(ai_service);

    let mut metrics = HashMap::new();
    metrics.insert("cpu_usage".to_string(), 0.95);
    metrics.insert("error_rate".to_string(), 0.15);

    let system_metrics = SystemMetrics {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        component: "test_component".to_string(),
        metrics,
        anomalies: vec!["high_cpu_usage".to_string(), "high_error_rate".to_string()],
    };

    let alerts = analyzer.analyze_metrics(system_metrics).await;
    assert!(alerts.is_ok());
    let alerts = alerts.unwrap();
    assert!(!alerts.is_empty());
}

#[tokio::test]
async fn test_pattern_management() {
    let config = SecurityConfig::default();
    let ai_service = Arc::new(AIService::new(config));
    let analyzer = SecurityAnalyzer::new(ai_service);

    let pattern = SecurityPattern {
        pattern_type: "resource_exhaustion".to_string(),
        description: "High CPU usage pattern".to_string(),
        indicators: vec!["cpu_usage > 0.9".to_string()],
        confidence: 0.9,
        severity: ThreatSeverity::Critical,
        mitigation_steps: vec!["Scale resources".to_string()],
    };

    analyzer.add_pattern(pattern.clone()).await;
    let patterns = analyzer.get_patterns().await;
    assert!(!patterns.is_empty());
    assert_eq!(patterns[0].pattern_type, pattern.pattern_type);
}

#[tokio::test]
async fn test_threat_intelligence() {
    let config = SecurityConfig::default();
    let ai_service = Arc::new(AIService::new(config));
    let analyzer = SecurityAnalyzer::new(ai_service);

    let threat = ThreatIntelligence {
        threat_id: "THREAT-001".to_string(),
        name: "Resource Exhaustion Attack".to_string(),
        description: "Attempt to exhaust system resources".to_string(),
        indicators: vec!["high_cpu_usage".to_string(), "high_memory_usage".to_string()],
        severity: ThreatSeverity::Critical,
        category: "DoS".to_string(),
        source: "Internal".to_string(),
        last_seen: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        confidence: 0.9,
    };

    analyzer.update_threat_intelligence(threat.clone()).await;
    let threats = analyzer.get_threat_intelligence().await;
    assert!(!threats.is_empty());
    assert_eq!(threats[0].threat_id, threat.threat_id);
}

#[tokio::test]
async fn test_historical_analysis() {
    let config = SecurityConfig::default();
    let ai_service = Arc::new(AIService::new(config));
    let analyzer = SecurityAnalyzer::new(ai_service);

    // Record some historical metrics
    for i in 0..5 {
        let mut metrics = HashMap::new();
        metrics.insert("cpu_usage".to_string(), 0.8 + (i as f64 * 0.05));
        metrics.insert("error_rate".to_string(), 0.1 + (i as f64 * 0.02));

        let system_metrics = SystemMetrics {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            component: "test_component".to_string(),
            metrics,
            anomalies: vec!["high_cpu_usage".to_string()],
        };

        analyzer.analyze_metrics(system_metrics).await.unwrap();
    }

    let patterns = analyzer.analyze_historical_patterns().await;
    assert!(patterns.is_ok());
    let patterns = patterns.unwrap();
    assert!(!patterns.is_empty());
}

#[tokio::test]
async fn test_alert_filtering() {
    let config = SecurityConfig::default();
    let ai_service = Arc::new(AIService::new(config));
    let analyzer = SecurityAnalyzer::new(ai_service);

    // Record metrics that should trigger alerts
    let mut metrics = HashMap::new();
    metrics.insert("cpu_usage".to_string(), 0.95);
    metrics.insert("error_rate".to_string(), 0.15);

    let system_metrics = SystemMetrics {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        component: "test_component".to_string(),
        metrics,
        anomalies: vec!["high_cpu_usage".to_string(), "high_error_rate".to_string()],
    };

    analyzer.analyze_metrics(system_metrics).await.unwrap();

    // Get all alerts
    let all_alerts = analyzer.get_recent_alerts(None).await;
    assert!(!all_alerts.is_empty());

    // Get only critical alerts
    let critical_alerts = analyzer.get_recent_alerts(Some(ThreatSeverity::Critical)).await;
    assert!(critical_alerts.iter().all(|alert| alert.severity >= ThreatSeverity::Critical));
} 
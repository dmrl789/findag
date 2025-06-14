use findag_core::security::monitoring::{SecurityMonitor, SystemMetrics, ThreatSeverity};
use findag_core::security::SecurityConfig;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::test]
async fn test_metrics_recording() {
    let config = SecurityConfig::default();
    let monitor = SecurityMonitor::new(config);

    let mut metrics = HashMap::new();
    metrics.insert("cpu_usage".to_string(), 0.95);
    metrics.insert("memory_usage".to_string(), 0.85);

    let system_metrics = SystemMetrics {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        component: "test_component".to_string(),
        metrics,
        anomalies: vec!["high_cpu_usage".to_string()],
    };

    let result = monitor.record_metrics(system_metrics).await;
    assert!(result.is_ok());

    let component_metrics = monitor.get_component_metrics("test_component").await;
    assert!(!component_metrics.is_empty());
    assert_eq!(component_metrics[0].component, "test_component");
}

#[tokio::test]
async fn test_threshold_checking() {
    let config = SecurityConfig::default();
    let monitor = SecurityMonitor::new(config);

    // Test CPU usage threshold
    assert!(monitor.check_thresholds("test", "cpu_usage", 0.95).await);
    assert!(!monitor.check_thresholds("test", "cpu_usage", 0.5).await);

    // Test memory usage threshold
    assert!(monitor.check_thresholds("test", "memory_usage", 0.85).await);
    assert!(!monitor.check_thresholds("test", "memory_usage", 0.5).await);
}

#[tokio::test]
async fn test_alert_filtering() {
    let config = SecurityConfig::default();
    let monitor = SecurityMonitor::new(config);

    // Record some metrics that should trigger alerts
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

    monitor.record_metrics(system_metrics).await.unwrap();

    // Wait for analysis
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Get all alerts
    let all_alerts = monitor.get_alerts(None).await;
    assert!(!all_alerts.is_empty());

    // Get only critical alerts
    let critical_alerts = monitor.get_alerts(Some(ThreatSeverity::Critical)).await;
    assert!(critical_alerts.iter().all(|alert| alert.severity >= ThreatSeverity::Critical));
}

#[tokio::test]
async fn test_security_event_logging() {
    let config = SecurityConfig::default();
    let monitor = SecurityMonitor::new(config);

    let alert = findag_core::security::monitoring::ThreatAlert {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        severity: ThreatSeverity::Critical,
        source: "test_source".to_string(),
        description: "Test security threat".to_string(),
        affected_components: vec!["test_component".to_string()],
        recommended_actions: vec!["investigate".to_string()],
        confidence_score: 0.9,
    };

    monitor.log_security_event(alert).await;
} 
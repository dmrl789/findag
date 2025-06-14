pub mod monitoring;
pub mod encryption;
pub mod authentication;
pub mod authorization;
pub mod audit;
pub mod ai_analysis;
pub mod response;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};
use regex::Regex;
use serde::{Serialize, Deserialize};
use crate::lib::ai::AIService;

pub use monitoring::{SecurityMonitor, SystemMetrics, ThreatAlert, ThreatSeverity};
pub use ai_analysis::{AISecurityAnalyzer, SecurityPattern, ThreatIntelligence};
pub use response::{SecurityResponder, ResponseAction, ResponseResult, ResponseStatus};
pub use audit::{AuditManager, AuditEvent, AuditEventType, ComplianceRequirement, ComplianceStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub encryption_key: String,
    pub jwt_secret: String,
    pub rate_limit: u32,
    pub max_retries: u32,
    pub session_timeout: u64,
    pub ai_model: String,
    pub monitoring_interval: u64,
    pub alert_thresholds: HashMap<String, f64>,
    pub pattern_confidence_threshold: f32,
    pub threat_confidence_threshold: f32,
    pub auto_response_enabled: bool,
    pub response_timeout_seconds: u64,
    pub audit_log_retention_days: u64,
    pub compliance_check_frequency_days: u64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        let mut alert_thresholds = HashMap::new();
        alert_thresholds.insert("error_rate".to_string(), 0.1);
        alert_thresholds.insert("response_time".to_string(), 1000.0);
        alert_thresholds.insert("memory_usage".to_string(), 0.8);
        alert_thresholds.insert("cpu_usage".to_string(), 0.9);

        Self {
            encryption_key: "default_encryption_key".to_string(),
            jwt_secret: "default_jwt_secret".to_string(),
            rate_limit: 100,
            max_retries: 3,
            session_timeout: 3600,
            ai_model: "gpt-3.5-turbo".to_string(),
            monitoring_interval: 60,
            alert_thresholds,
            pattern_confidence_threshold: 0.7,
            threat_confidence_threshold: 0.8,
            auto_response_enabled: true,
            response_timeout_seconds: 30,
            audit_log_retention_days: 90,
            compliance_check_frequency_days: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditLog {
    pub timestamp: u64,
    pub action: String,
    pub content_hash: String,
    pub user_id: Option<String>,
    pub status: String,
    pub details: HashMap<String, String>,
}

pub struct SecurityManager {
    monitor: Arc<SecurityMonitor>,
    analyzer: Arc<AISecurityAnalyzer>,
    responder: Arc<SecurityResponder>,
    audit_manager: Arc<AuditManager>,
    config: SecurityConfig,
}

impl SecurityManager {
    pub fn new(config: SecurityConfig) -> Self {
        let monitor = Arc::new(SecurityMonitor::new());
        let analyzer = Arc::new(AISecurityAnalyzer::new());
        let responder = Arc::new(SecurityResponder::new());
        let audit_manager = Arc::new(AuditManager::new(analyzer.ai_service.clone()));

        Self {
            monitor,
            analyzer,
            responder,
            audit_manager,
            config,
        }
    }

    pub async fn process_metrics(&self, metrics: SystemMetrics) -> Result<Vec<ThreatAlert>, String> {
        // Record metrics and get alerts
        let alerts = self.monitor.record_metrics(metrics).await?;

        // Log alerts to audit
        for alert in &alerts {
            self.audit_manager.log_security_alert(alert.clone()).await;
        }

        // Analyze metrics for patterns
        let analysis = self.analyzer.analyze_metrics(metrics).await?;

        // Handle any critical threats
        for alert in &alerts {
            if alert.severity == ThreatSeverity::Critical {
                let threat_id = self.responder.handle_threat(alert.clone()).await?;
                
                if self.config.auto_response_enabled {
                    self.responder.execute_action(threat_id).await?;
                }
            }
        }

        Ok(alerts)
    }

    pub async fn log_audit_event(&self, event: AuditEvent) -> Result<(), String> {
        self.audit_manager.log_event(event).await;
        Ok(())
    }

    pub async fn get_audit_events(
        &self,
        event_type: Option<AuditEventType>,
        severity: Option<ThreatSeverity>,
        start_time: Option<chrono::DateTime<chrono::Utc>>,
        end_time: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Vec<AuditEvent> {
        self.audit_manager.get_events(event_type, severity, start_time, end_time).await
    }

    pub async fn add_compliance_requirement(&self, requirement: ComplianceRequirement) -> Result<(), String> {
        self.audit_manager.add_compliance_requirement(requirement).await;
        Ok(())
    }

    pub async fn get_compliance_status(&self) -> HashMap<String, ComplianceStatus> {
        self.audit_manager.get_compliance_status().await
    }

    pub async fn generate_compliance_report(&self) -> String {
        self.audit_manager.generate_compliance_report().await
    }
}

pub async fn validate_content(content: &str) -> Result<(), String> {
    // Check content length
    if content.len() > 10000 {
        return Err("Content exceeds maximum length".to_string());
    }

    // Check for blocked keywords
    let content_filter = Regex::new(r"(?i)(malware|exploit|hack|attack)").unwrap();
    if content_filter.is_match(content) {
        return Err("Content contains blocked keywords".to_string());
    }

    Ok(())
}

pub async fn check_rate_limit(user_id: &str) -> Result<(), String> {
    // Implementation of check_rate_limit function
    Ok(())
}

pub fn validate_category(category: &str) -> bool {
    // Implementation of validate_category function
    false
}

pub async fn verify_api_key(api_key: &str) -> bool {
    // In a real implementation, this would verify the API key format and validity
    // For now, just check if it's not empty and has a reasonable length
    !api_key.is_empty() && api_key.len() >= 32
} 
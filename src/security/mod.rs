pub mod ai_analysis;
pub mod ai_security;
pub mod audit;
pub mod authentication;
pub mod authorization;
pub mod encryption;
pub mod monitoring;
pub mod response;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use crate::security::audit::{SecurityAuditLog as AuditLog, SecuritySeverity};
// use crate::types::governance::ComplianceStatus;

pub use monitoring::{SecurityMonitor, SystemMetrics, ThreatAlert, ThreatSeverity};
pub use response::{SecurityResponder, ResponseAction, ResponseResult, ResponseStatus};
pub use audit::{AuditManager};

#[derive(Clone, Debug)]
pub struct SecurityConfig {
    pub max_alerts: usize,
    pub alert_ttl: Duration,
    pub threat_threshold: f64,
    pub alert_thresholds: HashMap<String, f64>,
    pub audit_log_retention_days: u32,
    pub max_requests_per_second: u32,
    pub auto_response_enabled: bool,
    pub compliance_requirements: Vec<String>,
    pub monitoring_interval: Duration,
    pub max_failed_attempts: u32,
    pub lockout_duration: Duration,
    pub encryption_enabled: bool,
    pub backup_enabled: bool,
    pub backup_interval: Duration,
    pub max_backup_size: usize,
    pub alert_notification_channels: Vec<String>,
    pub alert_threshold: f64,
    pub block_threshold: f64,
    pub response_timeout: u64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            max_alerts: 1000,
            alert_ttl: Duration::from_secs(3600),
            threat_threshold: 0.8,
            alert_thresholds: HashMap::new(),
            audit_log_retention_days: 30,
            max_requests_per_second: 100,
            auto_response_enabled: false,
            compliance_requirements: Vec::new(),
            monitoring_interval: Duration::from_secs(60),
            max_failed_attempts: 5,
            lockout_duration: Duration::from_secs(300),
            encryption_enabled: true,
            backup_enabled: true,
            backup_interval: Duration::from_secs(86400),
            max_backup_size: 1024 * 1024 * 100, // 100MB
            alert_notification_channels: vec!["email".to_string()],
            alert_threshold: 0.5,
            block_threshold: 0.8,
            response_timeout: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditLog {
    pub timestamp: u64,
    pub source: String,
    pub severity: SecuritySeverity,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub count: u32,
    pub last_reset: u64,
}

#[derive(Debug)]
pub struct SecurityManager {
    monitor: Arc<SecurityMonitor>,
    responder: Arc<SecurityResponder>,
    audit_manager: Arc<Mutex<AuditManager>>,
    records: HashMap<String, Vec<ThreatAlert>>,
    config: SecurityConfig,
    audit_logs: Arc<Mutex<Vec<SecurityAuditLog>>>,
    rate_limits: HashMap<String, u32>,
    compliance_status: Arc<Mutex<HashMap<String, bool>>>,
}

impl SecurityManager {
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            monitor: Arc::new(SecurityMonitor::new()),
            responder: Arc::new(SecurityResponder::new()),
            audit_manager: Arc::new(Mutex::new(AuditManager::new())),
            records: HashMap::new(),
            config,
            audit_logs: Arc::new(Mutex::new(Vec::new())),
            rate_limits: HashMap::new(),
            compliance_status: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn process_alerts(&self, alerts: Vec<ThreatAlert>) -> Result<Vec<ThreatAlert>, String> {
        let mut processed_alerts = Vec::new();
        for alert in alerts {
            let action = self.responder.determine_action(&alert).await;
            let result = match action {
                ResponseAction::Block => self.responder.block_threat(&alert).await,
                ResponseAction::Alert => self.responder.send_alert(&alert).await,
                ResponseAction::Monitor => self.responder.monitor_threat(&alert).await,
            };

            if result.success {
                self.log_audit_event(
                    alert.source.clone(),
                    SecuritySeverity::High,
                    format!("Action taken: {:?}", action),
                ).await;
                processed_alerts.push(alert);
            }
        }
        Ok(processed_alerts)
    }

    pub async fn log_audit_event(&self, source: String, severity: SecuritySeverity, message: String) -> Result<(), String> {
        let log = SecurityAuditLog {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            source,
            severity,
            message,
        };
        let mut logs = self.audit_logs.lock().map_err(|e| e.to_string())?;
        logs.push(log);
        Ok(())
    }

    pub async fn get_audit_logs(&self) -> Result<Vec<SecurityAuditLog>, String> {
        let logs = self.audit_logs.lock().map_err(|e| e.to_string())?;
        Ok(logs.clone())
    }

    pub async fn check_rate_limit(&mut self, user_id: &str) -> Result<(), String> {
        let current = self.rate_limits.get(user_id).copied().unwrap_or(0);
        if current >= 100 {
            return Err("Rate limit exceeded".to_string());
        }
        self.rate_limits.insert(user_id.to_string(), current + 1);
        Ok(())
    }

    pub async fn process_metrics(&self, metrics: SystemMetrics) -> Result<Vec<ThreatAlert>, String> {
        // Record metrics and get alerts
        let alerts = self.monitor.record_metrics(metrics).await?;

        // Log alerts to audit
        for alert in &alerts {
            let audit_log = AuditLog {
                timestamp: alert.timestamp,
                source: alert.source.clone(),
                severity: match alert.severity {
                    ThreatSeverity::Low => SecuritySeverity::Low,
                    ThreatSeverity::Medium => SecuritySeverity::Medium,
                    ThreatSeverity::High => SecuritySeverity::High,
                    ThreatSeverity::Critical => SecuritySeverity::Critical,
                },
                message: alert.description.clone(),
            };
            let mut audit_manager = self.audit_manager.lock().unwrap();
            audit_manager.log_security_event(audit_log).await;
        }

        // Handle any critical threats
        for alert in &alerts {
            if alert.severity == ThreatSeverity::Critical {
                let threat_id = self.responder.handle_threat(alert.clone()).await?;
                
                if self.config.auto_response_enabled {
                    self.responder.execute_action(alert).await?;
                }
            }
        }

        Ok(alerts)
    }

    pub async fn add_compliance_requirement(&self, requirement: String, description: String) -> Result<(), String> {
        let mut audit_manager = self.audit_manager.lock().unwrap();
        audit_manager.add_compliance_requirement(requirement, description).await;
        Ok(())
    }

    pub async fn check_compliance(&self, requirement: &str) -> Result<bool, String> {
        let status = self.compliance_status.lock().map_err(|e| e.to_string())?;
        Ok(status.get(requirement).copied().unwrap_or(false))
    }

    pub async fn update_compliance(&self, requirement: &str, status: bool) -> Result<(), String> {
        let mut compliance = self.compliance_status.lock().map_err(|e| e.to_string())?;
        compliance.insert(requirement.to_string(), status);
        Ok(())
    }

    pub async fn generate_compliance_report(&self) -> String {
        let audit_manager = self.audit_manager.lock().unwrap();
        audit_manager.generate_compliance_report().await
    }
}

pub async fn validate_content(content: &str) -> Result<(), String> {
    // Basic content validation
    if content.is_empty() {
        return Err("Content cannot be empty".to_string());
    }
    if content.len() > 1000000 {
        return Err("Content too large".to_string());
    }
    Ok(())
}

pub async fn check_rate_limit(user_id: &str) -> Result<(), String> {
    // Basic rate limiting
    Ok(())
}

pub fn validate_category(category: &str) -> bool {
    // Basic category validation
    !category.is_empty() && category.len() <= 50
}

pub async fn verify_api_key(api_key: &str) -> bool {
    // Basic API key validation
    !api_key.is_empty() && api_key.len() >= 32
} 
use crate::audit::audit_logger::{AuditEvent, AuditSeverity, AuditCategory};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Audit trail analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTrailAnalysis {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_events: u64,
    pub events_by_severity: HashMap<String, u64>,
    pub events_by_category: HashMap<String, u64>,
    pub events_by_actor: HashMap<String, u64>,
    pub success_rate: f64,
    pub error_rate: f64,
    pub security_events: u64,
    pub compliance_events: u64,
    pub anomalies: Vec<AuditAnomaly>,
    pub risk_score: f64,
}

/// Detected audit anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditAnomaly {
    pub anomaly_type: String,
    pub description: String,
    pub severity: AuditSeverity,
    pub timestamp: DateTime<Utc>,
    pub actor: Option<String>,
    pub confidence: f64,
    pub details: HashMap<String, String>,
}

/// Audit trail patterns for anomaly detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditPattern {
    pub pattern_type: String,
    pub description: String,
    pub conditions: Vec<AuditCondition>,
    pub risk_score: f64,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditCondition {
    pub field: String,
    pub operator: String,
    pub value: String,
}

/// Audit trail analyzer
#[allow(dead_code)]
pub struct AuditTrailAnalyzer {
    #[allow(dead_code)]
    patterns: Vec<AuditPattern>,
    #[allow(dead_code)]
    baseline: Arc<RwLock<HashMap<String, f64>>>,
}

impl AuditTrailAnalyzer {
    pub fn new() -> Self {
        let patterns = Self::load_default_patterns();
        Self {
            patterns,
            baseline: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Analyze audit events for a given time period
    pub async fn analyze_trail(
        &self,
        events: &[AuditEvent],
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> AuditTrailAnalysis {
        let mut analysis = AuditTrailAnalysis {
            period_start,
            period_end,
            total_events: events.len() as u64,
            events_by_severity: HashMap::new(),
            events_by_category: HashMap::new(),
            events_by_actor: HashMap::new(),
            success_rate: 0.0,
            error_rate: 0.0,
            security_events: 0,
            compliance_events: 0,
            anomalies: Vec::new(),
            risk_score: 0.0,
        };

        if events.is_empty() {
            return analysis;
        }

        let mut success_count = 0;
        let mut error_count = 0;

        for event in events {
            // Count by severity
            let severity_key = format!("{:?}", event.severity);
            *analysis.events_by_severity.entry(severity_key).or_insert(0) += 1;

            // Count by category
            let category_key = format!("{:?}", event.category);
            *analysis.events_by_category.entry(category_key).or_insert(0) += 1;

            // Count by actor
            if let Some(ref actor) = event.actor {
                *analysis.events_by_actor.entry(actor.clone()).or_insert(0) += 1;
            }

            // Count success/failure
            if event.success {
                success_count += 1;
            } else {
                error_count += 1;
            }

            // Count security and compliance events
            if event.category == AuditCategory::Security {
                analysis.security_events += 1;
            }
            if event.category == AuditCategory::Compliance {
                analysis.compliance_events += 1;
            }
        }

        // Calculate rates
        analysis.success_rate = success_count as f64 / events.len() as f64;
        analysis.error_rate = error_count as f64 / events.len() as f64;

        // Detect anomalies
        analysis.anomalies = self.detect_anomalies(events).await;

        // Calculate risk score
        analysis.risk_score = self.calculate_risk_score(&analysis);

        analysis
    }

    /// Detect anomalies in audit events
    async fn detect_anomalies(&self, events: &[AuditEvent]) -> Vec<AuditAnomaly> {
        let mut anomalies = Vec::new();

        // Check for failed authentication attempts
        let failed_auth = events.iter()
            .filter(|e| e.category == AuditCategory::Authentication && !e.success)
            .count();
        
        if failed_auth > 5 {
            anomalies.push(AuditAnomaly {
                anomaly_type: "High Failed Authentication".to_string(),
                description: format!("{} failed authentication attempts detected", failed_auth),
                severity: AuditSeverity::Warning,
                timestamp: Utc::now(),
                actor: None,
                confidence: 0.8,
                details: HashMap::new(),
            });
        }

        // Check for unusual activity patterns
        let mut actor_activity = HashMap::new();
        for event in events {
            if let Some(ref actor) = event.actor {
                *actor_activity.entry(actor.clone()).or_insert(0) += 1;
            }
        }

        let avg_activity = actor_activity.values().sum::<u64>() as f64 / actor_activity.len() as f64;
        for (actor, count) in actor_activity {
            if count as f64 > avg_activity * 3.0 {
                anomalies.push(AuditAnomaly {
                    anomaly_type: "Unusual Activity".to_string(),
                    description: format!("Actor {} has {} events ({}x average)", actor, count, (count as f64 / avg_activity) as u64),
                    severity: AuditSeverity::Warning,
                    timestamp: Utc::now(),
                    actor: Some(actor),
                    confidence: 0.7,
                    details: HashMap::new(),
                });
            }
        }

        // Check for security events
        let security_events = events.iter()
            .filter(|e| e.category == AuditCategory::Security)
            .count();
        
        if security_events > 10 {
            anomalies.push(AuditAnomaly {
                anomaly_type: "High Security Events".to_string(),
                description: format!("{} security events detected", security_events),
                severity: AuditSeverity::Error,
                timestamp: Utc::now(),
                actor: None,
                confidence: 0.9,
                details: HashMap::new(),
            });
        }

        anomalies
    }

    /// Calculate risk score based on analysis
    fn calculate_risk_score(&self, analysis: &AuditTrailAnalysis) -> f64 {
        let mut risk_score = 0.0;

        // Base risk from error rate
        risk_score += analysis.error_rate * 50.0;

        // Risk from security events
        risk_score += (analysis.security_events as f64 / analysis.total_events as f64) * 100.0;

        // Risk from anomalies
        risk_score += analysis.anomalies.len() as f64 * 10.0;

        // Risk from high severity events
        if let Some(critical_count) = analysis.events_by_severity.get("Critical") {
            risk_score += *critical_count as f64 * 20.0;
        }

        if let Some(error_count) = analysis.events_by_severity.get("Error") {
            risk_score += *error_count as f64 * 10.0;
        }

        risk_score.min(100.0)
    }

    /// Load default audit patterns
    fn load_default_patterns() -> Vec<AuditPattern> {
        vec![
            AuditPattern {
                pattern_type: "Failed Authentication".to_string(),
                description: "Multiple failed authentication attempts".to_string(),
                conditions: vec![
                    AuditCondition {
                        field: "category".to_string(),
                        operator: "equals".to_string(),
                        value: "Authentication".to_string(),
                    },
                    AuditCondition {
                        field: "success".to_string(),
                        operator: "equals".to_string(),
                        value: "false".to_string(),
                    },
                ],
                risk_score: 0.8,
                action: "Alert security team".to_string(),
            },
            AuditPattern {
                pattern_type: "Data Access".to_string(),
                description: "Sensitive data access outside business hours".to_string(),
                conditions: vec![
                    AuditCondition {
                        field: "category".to_string(),
                        operator: "equals".to_string(),
                        value: "DataAccess".to_string(),
                    },
                ],
                risk_score: 0.6,
                action: "Review access logs".to_string(),
            },
            AuditPattern {
                pattern_type: "Configuration Change".to_string(),
                description: "System configuration changes".to_string(),
                conditions: vec![
                    AuditCondition {
                        field: "category".to_string(),
                        operator: "equals".to_string(),
                        value: "Configuration".to_string(),
                    },
                ],
                risk_score: 0.7,
                action: "Verify change authorization".to_string(),
            },
        ]
    }

    /// Generate compliance report
    pub async fn generate_compliance_report(
        &self,
        events: &[AuditEvent],
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> String {
        let analysis = self.analyze_trail(events, period_start, period_end).await;
        
        let mut report = String::new();
        report.push_str("=== AUDIT COMPLIANCE REPORT ===\n");
        report.push_str(&format!("Period: {} to {}\n", period_start, period_end));
        report.push_str(&format!("Total Events: {}\n", analysis.total_events));
        report.push_str(&format!("Success Rate: {:.2}%\n", analysis.success_rate * 100.0));
        report.push_str(&format!("Risk Score: {:.1}/100\n", analysis.risk_score));
        report.push_str(&format!("Security Events: {}\n", analysis.security_events));
        report.push_str(&format!("Compliance Events: {}\n", analysis.compliance_events));
        report.push_str(&format!("Anomalies Detected: {}\n", analysis.anomalies.len()));
        
        if !analysis.anomalies.is_empty() {
            report.push_str("\n=== ANOMALIES ===\n");
            for anomaly in &analysis.anomalies {
                report.push_str(&format!("- {}: {} (Confidence: {:.1}%)\n", 
                    anomaly.anomaly_type, anomaly.description, anomaly.confidence * 100.0));
            }
        }

        report.push_str("\n=== RECOMMENDATIONS ===\n");
        if analysis.risk_score > 70.0 {
            report.push_str("- HIGH RISK: Immediate security review required\n");
        } else if analysis.risk_score > 40.0 {
            report.push_str("- MEDIUM RISK: Enhanced monitoring recommended\n");
        } else {
            report.push_str("- LOW RISK: Normal operations\n");
        }

        if analysis.error_rate > 0.1 {
            report.push_str("- High error rate detected, investigate system issues\n");
        }

        if analysis.security_events > 0 {
            report.push_str("- Security events detected, review security policies\n");
        }

        report
    }
} 
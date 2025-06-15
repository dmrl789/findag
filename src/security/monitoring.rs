use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
// use crate::lib::ai::{AIService, AIRequest};
use crate::security::{SecurityManager, SecurityConfig, SecurityAuditLog};
use std::fmt;
use crate::security::audit::SecuritySeverity;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum ThreatSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for ThreatSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThreatSeverity::Low => write!(f, "Low"),
            ThreatSeverity::Medium => write!(f, "Medium"),
            ThreatSeverity::High => write!(f, "High"),
            ThreatSeverity::Critical => write!(f, "Critical"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAlert {
    pub source: String,
    pub description: String,
    pub severity: ThreatSeverity,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: u64,
    pub component: String,
    pub metrics: HashMap<String, f64>,
    pub anomalies: Vec<String>,
}

#[derive(Debug)]
pub struct SecurityMonitor {
    alerts: Arc<RwLock<VecDeque<ThreatAlert>>>,
    metrics_history: Arc<RwLock<HashMap<String, VecDeque<SystemMetrics>>>>,
    alert_thresholds: HashMap<String, f64>,
    last_analysis: Arc<RwLock<Instant>>,
}

impl SecurityMonitor {
    pub fn new() -> Self {
        let mut alert_thresholds = HashMap::new();
        alert_thresholds.insert("error_rate".to_string(), 0.1);
        alert_thresholds.insert("response_time".to_string(), 1000.0);
        alert_thresholds.insert("memory_usage".to_string(), 0.8);
        alert_thresholds.insert("cpu_usage".to_string(), 0.9);

        Self {
            alerts: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
            metrics_history: Arc::new(RwLock::new(HashMap::new())),
            alert_thresholds,
            last_analysis: Arc::new(RwLock::new(Instant::now())),
        }
    }

    pub async fn record_metrics(&self, metrics: SystemMetrics) -> Result<Vec<ThreatAlert>, String> {
        // Store metrics
        let mut history = self.metrics_history.write().await;
        let component_history = history
            .entry(metrics.component.clone())
            .or_insert_with(|| VecDeque::with_capacity(100));
        
        component_history.push_back(metrics.clone());
        if component_history.len() > 100 {
            component_history.pop_front();
        }

        // Check if it's time for analysis
        let last_analysis = *self.last_analysis.read().await;
        if last_analysis.elapsed() > Duration::from_secs(60) {
            self.analyze_metrics().await?;
            *self.last_analysis.write().await = Instant::now();
        }

        // Return current alerts
        let alerts = self.alerts.read().await;
        Ok(alerts.iter().cloned().collect())
    }

    async fn analyze_metrics(&self) -> Result<(), String> {
        let history = self.metrics_history.read().await;
        let mut alerts = self.alerts.write().await;

        for (component, metrics) in history.iter() {
            if metrics.is_empty() {
                continue;
            }

            // Check thresholds for each metric
            for (metric_name, value) in &metrics.back().unwrap().metrics {
                if let Some(threshold) = self.alert_thresholds.get(metric_name) {
                    if value > threshold {
                        let alert = ThreatAlert {
                            source: component.clone(),
                            description: format!("{} exceeded threshold: {} > {}", metric_name, value, threshold),
                            severity: ThreatSeverity::High,
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        };
                        alerts.push_back(alert);
                    }
                }
            }
        }

        // Trim alerts if needed
        while alerts.len() > 1000 {
            alerts.pop_front();
        }

        Ok(())
    }

    pub async fn get_alerts(&self, min_severity: Option<ThreatSeverity>) -> Vec<ThreatAlert> {
        let alerts = self.alerts.read().await;
        if let Some(severity) = min_severity {
            alerts.iter()
                .filter(|alert| alert.severity >= severity)
                .cloned()
                .collect()
        } else {
            alerts.iter().cloned().collect()
        }
    }

    pub async fn add_alert(&self, alert: ThreatAlert) {
        let mut alerts = self.alerts.write().await;
        alerts.push_back(alert);
        while alerts.len() > 1000 {
            alerts.pop_front();
        }
    }

    pub async fn get_component_metrics(&self, component: &str) -> Vec<SystemMetrics> {
        let history = self.metrics_history.read().await;
        history.get(component)
            .map(|metrics| metrics.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn check_thresholds(&self, metric: &str, value: f64) -> bool {
        if let Some(threshold) = self.alert_thresholds.get(metric) {
            value < *threshold
        } else {
            true
        }
    }

    pub fn set_threshold(&mut self, metric: &str, threshold: f64) {
        self.alert_thresholds.insert(metric.to_string(), threshold);
    }

    pub async fn log_security_event(&self, alert: ThreatAlert) {
        let mut alerts = self.alerts.write().await;
        alerts.push_back(alert);
        while alerts.len() > 1000 {
            alerts.pop_front();
        }
    }

    fn hash_alert(&self, alert: &ThreatAlert) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(alert.description.as_bytes());
        hasher.update(alert.source.as_bytes());
        hex::encode(hasher.finalize().as_bytes())
    }
} 
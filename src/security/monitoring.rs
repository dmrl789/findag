use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use crate::lib::ai::{AIService, AIRequest};
use crate::security::{SecurityManager, SecurityConfig, SecurityAuditLog};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAlert {
    pub timestamp: u64,
    pub severity: ThreatSeverity,
    pub source: String,
    pub description: String,
    pub affected_components: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub confidence_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThreatSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: u64,
    pub component: String,
    pub metrics: HashMap<String, f64>,
    pub anomalies: Vec<String>,
}

pub struct SecurityMonitor {
    security_manager: Arc<SecurityManager>,
    ai_service: Arc<AIService>,
    alerts: Arc<RwLock<VecDeque<ThreatAlert>>>,
    metrics_history: Arc<RwLock<HashMap<String, VecDeque<SystemMetrics>>>>,
    alert_thresholds: HashMap<String, f64>,
    last_analysis: Arc<RwLock<Instant>>,
}

impl SecurityMonitor {
    pub fn new(security_config: SecurityConfig) -> Self {
        let mut alert_thresholds = HashMap::new();
        alert_thresholds.insert("error_rate".to_string(), 0.1);
        alert_thresholds.insert("response_time".to_string(), 1000.0);
        alert_thresholds.insert("memory_usage".to_string(), 0.8);
        alert_thresholds.insert("cpu_usage".to_string(), 0.9);

        Self {
            security_manager: Arc::new(SecurityManager::new(security_config.clone())),
            ai_service: Arc::new(AIService::new(security_config)),
            alerts: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
            metrics_history: Arc::new(RwLock::new(HashMap::new())),
            alert_thresholds,
            last_analysis: Arc::new(RwLock::new(Instant::now())),
        }
    }

    pub async fn record_metrics(&self, metrics: SystemMetrics) -> Result<(), String> {
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

        Ok(())
    }

    async fn analyze_metrics(&self) -> Result<(), String> {
        let history = self.metrics_history.read().await;
        let mut alerts = self.alerts.write().await;

        for (component, metrics) in history.iter() {
            if metrics.is_empty() {
                continue;
            }

            // Prepare metrics for AI analysis
            let metrics_str = serde_json::to_string(&metrics).map_err(|e| e.to_string())?;
            
            let prompt = format!(
                "Analyze the following system metrics for security threats and anomalies:\n\
                 Component: {}\n\
                 Metrics: {}\n\
                 \n\
                 Please provide:\n\
                 1. Threat severity (Info/Warning/Critical/Emergency)\n\
                 2. Description of potential threats\n\
                 3. Affected components\n\
                 4. Recommended actions\n\
                 5. Confidence score (0-1)\n\
                 Format the response as JSON with these fields: severity, description, affected_components, recommended_actions, confidence_score",
                component,
                metrics_str
            );

            let request = AIRequest {
                prompt,
                model: Some("gpt-3.5-turbo".to_string()),
                temperature: Some(0.3),
                max_tokens: Some(500),
            };

            match self.ai_service.generate_response(request, "security_monitor").await {
                Ok(response) => {
                    if let Ok(alert) = serde_json::from_str::<ThreatAlert>(&response.text) {
                        if alert.confidence_score >= 0.7 {
                            alerts.push_back(alert);
                            if alerts.len() > 1000 {
                                alerts.pop_front();
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to analyze metrics: {}", e);
                }
            }
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

    pub async fn get_component_metrics(&self, component: &str) -> Vec<SystemMetrics> {
        let history = self.metrics_history.read().await;
        history.get(component)
            .map(|metrics| metrics.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub async fn check_thresholds(&self, component: &str, metric_name: &str, value: f64) -> bool {
        if let Some(threshold) = self.alert_thresholds.get(metric_name) {
            value > *threshold
        } else {
            false
        }
    }

    pub async fn log_security_event(&self, alert: ThreatAlert) {
        self.security_manager.log_audit(SecurityAuditLog {
            timestamp: alert.timestamp,
            action: "THREAT_DETECTED".to_string(),
            content_hash: self.hash_alert(&alert),
            user_id: None,
            status: format!("{:?}", alert.severity),
            details: {
                let mut details = HashMap::new();
                details.insert("description".to_string(), alert.description);
                details.insert("confidence".to_string(), alert.confidence_score.to_string());
                details
            },
        }).await;
    }

    fn hash_alert(&self, alert: &ThreatAlert) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(alert.description.as_bytes());
        hasher.update(alert.source.as_bytes());
        hex::encode(hasher.finalize().as_bytes())
    }
} 
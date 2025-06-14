use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use crate::lib::ai::{AIService, AIRequest};
use crate::security::monitoring::{SystemMetrics, ThreatAlert, ThreatSeverity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPattern {
    pub pattern_type: String,
    pub description: String,
    pub indicators: Vec<String>,
    pub confidence: f32,
    pub severity: ThreatSeverity,
    pub mitigation_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligence {
    pub threat_id: String,
    pub name: String,
    pub description: String,
    pub indicators: Vec<String>,
    pub severity: ThreatSeverity,
    pub category: String,
    pub source: String,
    pub last_seen: u64,
    pub confidence: f32,
}

pub struct SecurityAnalyzer {
    ai_service: Arc<AIService>,
    patterns: Arc<RwLock<VecDeque<SecurityPattern>>>,
    threat_intelligence: Arc<RwLock<HashMap<String, ThreatIntelligence>>>,
    metrics_history: Arc<RwLock<HashMap<String, VecDeque<SystemMetrics>>>>,
    analysis_results: Arc<RwLock<VecDeque<ThreatAlert>>>,
}

impl SecurityAnalyzer {
    pub fn new(ai_service: Arc<AIService>) -> Self {
        Self {
            ai_service,
            patterns: Arc::new(RwLock::new(VecDeque::with_capacity(100))),
            threat_intelligence: Arc::new(RwLock::new(HashMap::new())),
            metrics_history: Arc::new(RwLock::new(HashMap::new())),
            analysis_results: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
        }
    }

    pub async fn analyze_metrics(&self, metrics: SystemMetrics) -> Result<Vec<ThreatAlert>, String> {
        // Store metrics in history
        let mut history = self.metrics_history.write().await;
        let component_history = history
            .entry(metrics.component.clone())
            .or_insert_with(|| VecDeque::with_capacity(100));
        
        component_history.push_back(metrics.clone());
        if component_history.len() > 100 {
            component_history.pop_front();
        }

        // Prepare context for AI analysis
        let metrics_str = serde_json::to_string(&metrics).map_err(|e| e.to_string())?;
        let patterns = self.patterns.read().await;
        let patterns_str = serde_json::to_string(&*patterns).map_err(|e| e.to_string())?;
        let threats = self.threat_intelligence.read().await;
        let threats_str = serde_json::to_string(&*threats).map_err(|e| e.to_string())?;

        let prompt = format!(
            "Analyze the following system metrics for security threats and patterns:\n\
             Metrics: {}\n\
             Known Patterns: {}\n\
             Threat Intelligence: {}\n\
             \n\
             Please provide:\n\
             1. Threat severity (Info/Warning/Critical/Emergency)\n\
             2. Description of potential threats\n\
             3. Affected components\n\
             4. Recommended actions\n\
             5. Confidence score (0-1)\n\
             6. Pattern matches\n\
             7. Threat intelligence matches\n\
             Format the response as JSON with these fields: severity, description, affected_components, recommended_actions, confidence_score, pattern_matches, threat_matches",
            metrics_str,
            patterns_str,
            threats_str
        );

        let request = AIRequest {
            prompt,
            model: Some("gpt-3.5-turbo".to_string()),
            temperature: Some(0.3),
            max_tokens: Some(1000),
        };

        match self.ai_service.generate_response(request, "security_analyzer").await {
            Ok(response) => {
                if let Ok(alerts) = serde_json::from_str::<Vec<ThreatAlert>>(&response.text) {
                    let mut analysis_results = self.analysis_results.write().await;
                    for alert in alerts.clone() {
                        analysis_results.push_back(alert);
                        if analysis_results.len() > 1000 {
                            analysis_results.pop_front();
                        }
                    }
                    Ok(alerts)
                } else {
                    Err("Failed to parse AI response".to_string())
                }
            }
            Err(e) => Err(format!("AI analysis failed: {}", e)),
        }
    }

    pub async fn add_pattern(&self, pattern: SecurityPattern) {
        let mut patterns = self.patterns.write().await;
        patterns.push_back(pattern);
        if patterns.len() > 100 {
            patterns.pop_front();
        }
    }

    pub async fn update_threat_intelligence(&self, threat: ThreatIntelligence) {
        let mut threats = self.threat_intelligence.write().await;
        threats.insert(threat.threat_id.clone(), threat);
    }

    pub async fn get_recent_alerts(&self, min_severity: Option<ThreatSeverity>) -> Vec<ThreatAlert> {
        let alerts = self.analysis_results.read().await;
        if let Some(severity) = min_severity {
            alerts.iter()
                .filter(|alert| alert.severity >= severity)
                .cloned()
                .collect()
        } else {
            alerts.iter().cloned().collect()
        }
    }

    pub async fn get_patterns(&self) -> Vec<SecurityPattern> {
        self.patterns.read().await.iter().cloned().collect()
    }

    pub async fn get_threat_intelligence(&self) -> Vec<ThreatIntelligence> {
        self.threat_intelligence.read().await.values().cloned().collect()
    }

    pub async fn analyze_historical_patterns(&self) -> Result<Vec<SecurityPattern>, String> {
        let history = self.metrics_history.read().await;
        let history_str = serde_json::to_string(&*history).map_err(|e| e.to_string())?;

        let prompt = format!(
            "Analyze the following historical metrics for security patterns:\n\
             History: {}\n\
             \n\
             Please identify:\n\
             1. Recurring patterns\n\
             2. Anomalies\n\
             3. Potential security implications\n\
             4. Confidence level\n\
             5. Recommended monitoring steps\n\
             Format the response as JSON array of SecurityPattern objects",
            history_str
        );

        let request = AIRequest {
            prompt,
            model: Some("gpt-3.5-turbo".to_string()),
            temperature: Some(0.3),
            max_tokens: Some(1000),
        };

        match self.ai_service.generate_response(request, "pattern_analyzer").await {
            Ok(response) => {
                if let Ok(patterns) = serde_json::from_str::<Vec<SecurityPattern>>(&response.text) {
                    let mut current_patterns = self.patterns.write().await;
                    for pattern in patterns.clone() {
                        current_patterns.push_back(pattern);
                        if current_patterns.len() > 100 {
                            current_patterns.pop_front();
                        }
                    }
                    Ok(patterns)
                } else {
                    Err("Failed to parse AI response".to_string())
                }
            }
            Err(e) => Err(format!("Pattern analysis failed: {}", e)),
        }
    }
} 
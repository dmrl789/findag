use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEvent {
    LoginSuccess { username: String, ip: String, timestamp: DateTime<Utc> },
    LoginFailure { username: String, ip: String, timestamp: DateTime<Utc> },
    RateLimitExceeded { client_id: String, endpoint: String, timestamp: DateTime<Utc> },
    InvalidToken { token_hash: String, ip: String, timestamp: DateTime<Utc> },
    SuspiciousActivity { activity: String, ip: String, timestamp: DateTime<Utc> },
    AdminAction { action: String, user: String, timestamp: DateTime<Utc> },
    TransactionSubmit { from: String, amount: u64, timestamp: DateTime<Utc> },
}

#[derive(Debug, Clone)]
pub struct SecurityMetrics {
    pub failed_login_attempts: u32,
    pub successful_logins: u32,
    pub rate_limit_violations: u32,
    pub invalid_tokens: u32,
    pub suspicious_activities: u32,
    pub admin_actions: u32,
    pub transactions_submitted: u32,
}

#[derive(Debug)]
pub struct SecurityMonitor {
    events: Arc<Mutex<Vec<SecurityEvent>>>,
    failed_logins: Arc<Mutex<HashMap<String, (u32, Instant)>>>,
    rate_limit_violations: Arc<Mutex<HashMap<String, (u32, Instant)>>>,
    suspicious_ips: Arc<Mutex<HashMap<String, (u32, Instant)>>>,
    metrics: Arc<Mutex<SecurityMetrics>>,
    config: SecurityConfig,
}

#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub failed_login_threshold: u32,
    pub rate_limit_threshold: u32,
    pub suspicious_activity_threshold: u32,
    pub alert_on_failed_login: bool,
    pub alert_on_rate_limit: bool,
    pub alert_on_suspicious_activity: bool,
    pub retention_days: u32,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            failed_login_threshold: 10,
            rate_limit_threshold: 5,
            suspicious_activity_threshold: 5,
            alert_on_failed_login: true,
            alert_on_rate_limit: true,
            alert_on_suspicious_activity: true,
            retention_days: 90,
        }
    }
}

impl SecurityMonitor {
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            failed_logins: Arc::new(Mutex::new(HashMap::new())),
            rate_limit_violations: Arc::new(Mutex::new(HashMap::new())),
            suspicious_ips: Arc::new(Mutex::new(HashMap::new())),
            metrics: Arc::new(Mutex::new(SecurityMetrics {
                failed_login_attempts: 0,
                successful_logins: 0,
                rate_limit_violations: 0,
                invalid_tokens: 0,
                suspicious_activities: 0,
                admin_actions: 0,
                transactions_submitted: 0,
            })),
            config,
        }
    }

    pub fn record_event(&self, event: SecurityEvent) {
        // Add event to history
        {
            let mut events = self.events.lock().unwrap();
            events.push(event.clone());
            
            // Clean up old events
            let cutoff = Utc::now() - chrono::Duration::days(self.config.retention_days as i64);
            events.retain(|e| {
                match e {
                    SecurityEvent::LoginSuccess { timestamp, .. } => *timestamp > cutoff,
                    SecurityEvent::LoginFailure { timestamp, .. } => *timestamp > cutoff,
                    SecurityEvent::RateLimitExceeded { timestamp, .. } => *timestamp > cutoff,
                    SecurityEvent::InvalidToken { timestamp, .. } => *timestamp > cutoff,
                    SecurityEvent::SuspiciousActivity { timestamp, .. } => *timestamp > cutoff,
                    SecurityEvent::AdminAction { timestamp, .. } => *timestamp > cutoff,
                    SecurityEvent::TransactionSubmit { timestamp, .. } => *timestamp > cutoff,
                }
            });
        }

        // Update metrics
        {
            let mut metrics = self.metrics.lock().unwrap();
            match event {
                SecurityEvent::LoginSuccess { .. } => metrics.successful_logins += 1,
                SecurityEvent::LoginFailure { .. } => metrics.failed_login_attempts += 1,
                SecurityEvent::RateLimitExceeded { .. } => metrics.rate_limit_violations += 1,
                SecurityEvent::InvalidToken { .. } => metrics.invalid_tokens += 1,
                SecurityEvent::SuspiciousActivity { .. } => metrics.suspicious_activities += 1,
                SecurityEvent::AdminAction { .. } => metrics.admin_actions += 1,
                SecurityEvent::TransactionSubmit { .. } => metrics.transactions_submitted += 1,
            }
        }

        // Check for security alerts
        self.check_security_alerts(&event);
    }

    pub fn record_login_failure(&self, username: &str, ip: &str) {
        let event = SecurityEvent::LoginFailure {
            username: username.to_string(),
            ip: ip.to_string(),
            timestamp: Utc::now(),
        };
        self.record_event(event);

        // Track failed login attempts
        {
            let mut failed_logins = self.failed_logins.lock().unwrap();
            let now = Instant::now();
            let (count, _) = failed_logins.entry(username.to_string()).or_insert((0, now));
            *count += 1;
        }
    }

    pub fn record_login_success(&self, username: &str, ip: &str) {
        let event = SecurityEvent::LoginSuccess {
            username: username.to_string(),
            ip: ip.to_string(),
            timestamp: Utc::now(),
        };
        self.record_event(event);

        // Reset failed login counter
        {
            let mut failed_logins = self.failed_logins.lock().unwrap();
            failed_logins.remove(username);
        }
    }

    pub fn record_rate_limit_violation(&self, client_id: &str, endpoint: &str) {
        let event = SecurityEvent::RateLimitExceeded {
            client_id: client_id.to_string(),
            endpoint: endpoint.to_string(),
            timestamp: Utc::now(),
        };
        self.record_event(event);

        // Track rate limit violations
        {
            let mut violations = self.rate_limit_violations.lock().unwrap();
            let now = Instant::now();
            let (count, _) = violations.entry(client_id.to_string()).or_insert((0, now));
            *count += 1;
        }
    }

    pub fn record_suspicious_activity(&self, activity: &str, ip: &str) {
        let event = SecurityEvent::SuspiciousActivity {
            activity: activity.to_string(),
            ip: ip.to_string(),
            timestamp: Utc::now(),
        };
        self.record_event(event);

        // Track suspicious IPs
        {
            let mut suspicious_ips = self.suspicious_ips.lock().unwrap();
            let now = Instant::now();
            let (count, _) = suspicious_ips.entry(ip.to_string()).or_insert((0, now));
            *count += 1;
        }
    }

    pub fn is_ip_blocked(&self, ip: &str) -> bool {
        let suspicious_ips = self.suspicious_ips.lock().unwrap();
        if let Some((count, timestamp)) = suspicious_ips.get(ip) {
            if *count >= self.config.suspicious_activity_threshold {
                // Check if within recent time window (1 hour)
                if timestamp.elapsed() < Duration::from_secs(3600) {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_user_locked(&self, username: &str) -> bool {
        let failed_logins = self.failed_logins.lock().unwrap();
        if let Some((count, timestamp)) = failed_logins.get(username) {
            if *count >= self.config.failed_login_threshold {
                // Check if within recent time window (15 minutes)
                if timestamp.elapsed() < Duration::from_secs(900) {
                    return true;
                }
            }
        }
        false
    }

    fn check_security_alerts(&self, event: &SecurityEvent) {
        match event {
            SecurityEvent::LoginFailure { username, ip, .. } => {
                if self.config.alert_on_failed_login {
                    let failed_logins = self.failed_logins.lock().unwrap();
                    if let Some((count, _)) = failed_logins.get(username) {
                        if *count >= self.config.failed_login_threshold {
                            self.trigger_alert("Failed Login Threshold Exceeded", 
                                &format!("User {} has {} failed login attempts from IP {}", username, count, ip));
                        }
                    }
                }
            },
            SecurityEvent::RateLimitExceeded { client_id, endpoint, .. } => {
                if self.config.alert_on_rate_limit {
                    let violations = self.rate_limit_violations.lock().unwrap();
                    if let Some((count, _)) = violations.get(client_id) {
                        if *count >= self.config.rate_limit_threshold {
                            self.trigger_alert("Rate Limit Threshold Exceeded", 
                                &format!("Client {} has {} rate limit violations on endpoint {}", client_id, count, endpoint));
                        }
                    }
                }
            },
            SecurityEvent::SuspiciousActivity { activity, ip, .. } => {
                if self.config.alert_on_suspicious_activity {
                    let suspicious_ips = self.suspicious_ips.lock().unwrap();
                    if let Some((count, _)) = suspicious_ips.get(ip) {
                        if *count >= self.config.suspicious_activity_threshold {
                            self.trigger_alert("Suspicious Activity Detected", 
                                &format!("IP {} has {} suspicious activities: {}", ip, count, activity));
                        }
                    }
                }
            },
            _ => {}
        }
    }

    fn trigger_alert(&self, alert_type: &str, message: &str) {
        // Log security alert
        eprintln!("🚨 SECURITY ALERT [{}]: {}", alert_type, message);
        
        // In production, this would send to monitoring system, email, Slack, etc.
        // For now, just log to stderr
    }

    pub fn get_metrics(&self) -> SecurityMetrics {
        self.metrics.lock().unwrap().clone()
    }

    pub fn get_recent_events(&self, hours: u32) -> Vec<SecurityEvent> {
        let events = self.events.lock().unwrap();
        let cutoff = Utc::now() - chrono::Duration::hours(hours as i64);
        
        events.iter()
            .filter(|e| {
                match e {
                    SecurityEvent::LoginSuccess { timestamp, .. } => *timestamp > cutoff,
                    SecurityEvent::LoginFailure { timestamp, .. } => *timestamp > cutoff,
                    SecurityEvent::RateLimitExceeded { timestamp, .. } => *timestamp > cutoff,
                    SecurityEvent::InvalidToken { timestamp, .. } => *timestamp > cutoff,
                    SecurityEvent::SuspiciousActivity { timestamp, .. } => *timestamp > cutoff,
                    SecurityEvent::AdminAction { timestamp, .. } => *timestamp > cutoff,
                    SecurityEvent::TransactionSubmit { timestamp, .. } => *timestamp > cutoff,
                }
            })
            .cloned()
            .collect()
    }

    pub fn export_events(&self) -> Vec<SecurityEvent> {
        self.events.lock().unwrap().clone()
    }

    pub fn clear_events(&self) {
        self.events.lock().unwrap().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_monitor() {
        let config = SecurityConfig::default();
        let monitor = SecurityMonitor::new(config);

        // Test login failure tracking
        monitor.record_login_failure("testuser", "192.168.1.1");
        assert!(!monitor.is_user_locked("testuser"));

        // Test multiple failures
        for _ in 0..10 {
            monitor.record_login_failure("testuser", "192.168.1.1");
        }
        assert!(monitor.is_user_locked("testuser"));

        // Test metrics
        let metrics = monitor.get_metrics();
        assert_eq!(metrics.failed_login_attempts, 11);
    }

    #[test]
    fn test_suspicious_activity() {
        let config = SecurityConfig::default();
        let monitor = SecurityMonitor::new(config);

        // Test suspicious activity tracking
        monitor.record_suspicious_activity("SQL injection attempt", "192.168.1.2");
        assert!(!monitor.is_ip_blocked("192.168.1.2"));

        // Test multiple suspicious activities
        for _ in 0..5 {
            monitor.record_suspicious_activity("XSS attempt", "192.168.1.2");
        }
        assert!(monitor.is_ip_blocked("192.168.1.2"));
    }
} 
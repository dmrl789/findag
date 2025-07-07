use crate::core::address::Address;
use chrono::{DateTime, Utc};
use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Verifier, Signature};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use uuid::Uuid;
use base64::Engine;
use rand::Rng;

/// Audit event severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuditSeverity {
    Info,
    Warning,
    Error,
    Critical,
    Security,
}

/// Audit event categories for compliance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditCategory {
    Authentication,
    Authorization,
    Transaction,
    Governance,
    System,
    Security,
    Compliance,
    DataAccess,
    Configuration,
    Network,
}

/// Comprehensive audit event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub severity: AuditSeverity,
    pub category: AuditCategory,
    pub event_type: String,
    pub actor: Option<String>,
    pub actor_address: Option<Address>,
    pub resource: Option<String>,
    pub action: String,
    pub details: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
    pub session_id: Option<String>,
    pub request_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
    pub signature: Option<String>,
    pub previous_hash: Option<String>,
    pub hash: String,
}

/// Audit log entry with cryptographic integrity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub event: AuditEvent,
    pub signature: String,
    pub timestamp: u64,
    pub sequence_number: u64,
    pub merkle_root: Option<String>,
}

/// Audit logger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLoggerConfig {
    pub enabled: bool,
    pub log_level: AuditSeverity,
    pub max_entries: usize,
    pub retention_days: u32,
    pub enable_signatures: bool,
    pub enable_encryption: bool,
    pub storage_path: String,
    pub export_format: ExportFormat,
    pub compliance_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    JSON,
    CSV,
    XML,
    PDF,
}

impl Default for AuditLoggerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_level: AuditSeverity::Info,
            max_entries: 1_000_000, // 1M entries
            retention_days: 2555, // 7 years for compliance
            enable_signatures: true,
            enable_encryption: false,
            storage_path: "audit_logs".to_string(),
            export_format: ExportFormat::JSON,
            compliance_mode: true,
        }
    }
}

/// Main audit logger with cryptographic integrity
pub struct AuditLogger {
    config: AuditLoggerConfig,
    entries: Arc<RwLock<Vec<AuditLogEntry>>>,
    signing_key: Option<SigningKey>,
    verifying_key: Option<VerifyingKey>,
    sequence_counter: Arc<Mutex<u64>>,
    merkle_tree: Arc<Mutex<Vec<String>>>,
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new(config: AuditLoggerConfig) -> Result<Self, String> {
        let signing_key = if config.enable_signatures {
            let mut secret_key_bytes = [0u8; 32];
            rand::thread_rng().fill(&mut secret_key_bytes);
            Some(SigningKey::from_bytes(&secret_key_bytes))
        } else {
            None
        };

        let verifying_key = signing_key.as_ref().map(|sk| sk.verifying_key());

        Ok(Self {
            config,
            entries: Arc::new(RwLock::new(Vec::new())),
            signing_key,
            verifying_key,
            sequence_counter: Arc::new(Mutex::new(0)),
            merkle_tree: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Log an audit event with full compliance features
    pub async fn log_event(
        &self,
        severity: AuditSeverity,
        category: AuditCategory,
        event_type: String,
        actor: Option<String>,
        actor_address: Option<Address>,
        resource: Option<String>,
        action: String,
        details: HashMap<String, String>,
        metadata: HashMap<String, String>,
        session_id: Option<String>,
        request_id: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        success: bool,
        error_message: Option<String>,
    ) -> Result<String, String> {
        if !self.config.enabled {
            return Ok("Audit logging disabled".to_string());
        }

        if severity < self.config.log_level {
            return Ok("Event below log level".to_string());
        }

        let now = Utc::now();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Get sequence number
        let sequence_number = {
            let mut counter = self.sequence_counter.lock().unwrap();
            *counter += 1;
            *counter
        };

        // Get previous hash for chain integrity
        let previous_hash = {
            let entries = self.entries.read().await;
            if let Some(last_entry) = entries.last() {
                Some(last_entry.event.hash.clone())
            } else {
                None
            }
        };

        // Create audit event
        let event = AuditEvent {
            id: Uuid::new_v4().to_string(),
            timestamp: now,
            severity,
            category,
            event_type: event_type.clone(),
            actor,
            actor_address,
            resource,
            action,
            details,
            metadata,
            session_id,
            request_id,
            ip_address,
            user_agent,
            success,
            error_message,
            signature: None, // Will be set after signing
            previous_hash,
            hash: String::new(), // Will be calculated
        };

        // Calculate hash
        let event_json = serde_json::to_string(&event)
            .map_err(|e| format!("Failed to serialize event: {}", e))?;
        let hash = self.calculate_hash(&event_json);

        // Create signed event
        let event_id = event.id.clone();
        let mut signed_event = event;
        signed_event.hash = hash.clone();

        // Sign the event if enabled
        let signature = if let Some(ref signing_key) = self.signing_key {
            let data_to_sign = format!("{}:{}:{}", signed_event.id, timestamp, hash);
            let sig = signing_key.sign(data_to_sign.as_bytes());
            Some(base64::engine::general_purpose::STANDARD.encode(sig.to_bytes()))
        } else {
            None
        };

        signed_event.signature = signature.clone();

        // Create audit log entry
        let entry = AuditLogEntry {
            event: signed_event,
            signature: signature.unwrap_or_default(),
            timestamp,
            sequence_number,
            merkle_root: None, // Will be updated in batch
        };

        // Store the entry
        {
            let mut entries = self.entries.write().await;
            entries.push(entry.clone());

            // Maintain size limit
            if entries.len() > self.config.max_entries {
                entries.remove(0);
            }
        }

        // Update Merkle tree
        self.update_merkle_tree(&hash).await;

        Ok(event_id)
    }

    /// Calculate cryptographic hash of event data
    fn calculate_hash(&self, data: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Update Merkle tree for batch integrity
    async fn update_merkle_tree(&self, hash: &str) {
        let mut tree = self.merkle_tree.lock().unwrap();
        tree.push(hash.to_string());

        // Recalculate Merkle root when we have a complete level
        if tree.len() > 1 && (tree.len() & (tree.len() - 1)) == 0 {
            self.recalculate_merkle_root(&mut tree);
        }
    }

    /// Recalculate Merkle root for integrity verification
    fn recalculate_merkle_root(&self, tree: &mut Vec<String>) {
        let mut level = tree.clone();
        
        while level.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in level.chunks(2) {
                let combined = if chunk.len() == 2 {
                    format!("{}{}", chunk[0], chunk[1])
                } else {
                    chunk[0].clone()
                };
                next_level.push(self.calculate_hash(&combined));
            }
            level = next_level;
        }

        if !level.is_empty() {
            // Update all entries with new Merkle root
            // In a real implementation, this would be done more efficiently
            tree.clear();
            tree.push(level[0].clone());
        }
    }

    /// Get audit events with filtering
    pub async fn get_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        severity: Option<AuditSeverity>,
        category: Option<AuditCategory>,
        actor: Option<String>,
        limit: Option<usize>,
    ) -> Result<Vec<AuditEvent>, String> {
        let entries = self.entries.read().await;
        let mut filtered_events = Vec::new();

        for entry in entries.iter() {
            let event = &entry.event;

            // Apply filters
            if let Some(start) = start_time {
                if event.timestamp < start {
                    continue;
                }
            }

            if let Some(end) = end_time {
                if event.timestamp > end {
                    continue;
                }
            }

            if let Some(ref sev) = severity {
                if event.severity < *sev {
                    continue;
                }
            }

            if let Some(ref cat) = category {
                if event.category != *cat {
                    continue;
                }
            }

            if let Some(ref act) = actor {
                if let Some(ref event_actor) = event.actor {
                    if event_actor != act {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            filtered_events.push(event.clone());
        }

        // Apply limit
        if let Some(limit_val) = limit {
            filtered_events.truncate(limit_val);
        }

        Ok(filtered_events)
    }

    /// Export audit logs in specified format
    pub async fn export_logs(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        format: ExportFormat,
    ) -> Result<String, String> {
        let events = self.get_events(start_time, end_time, None, None, None, None).await?;

        match format {
            ExportFormat::JSON => {
                serde_json::to_string_pretty(&events)
                    .map_err(|e| format!("Failed to export JSON: {}", e))
            }
            ExportFormat::CSV => {
                self.export_csv(&events)
            }
            ExportFormat::XML => {
                self.export_xml(&events)
            }
            ExportFormat::PDF => {
                self.export_pdf(&events)
            }
        }
    }

    /// Export as CSV format
    fn export_csv(&self, events: &[AuditEvent]) -> Result<String, String> {
        let mut csv = String::new();
        csv.push_str("Timestamp,Severity,Category,Event Type,Actor,Action,Resource,Success,Error Message\n");

        for event in events {
            csv.push_str(&format!(
                "{},{:?},{:?},{},{},{},{},{},{}\n",
                event.timestamp,
                event.severity,
                event.category,
                event.event_type,
                event.actor.as_deref().unwrap_or(""),
                event.action,
                event.resource.as_deref().unwrap_or(""),
                event.success,
                event.error_message.as_deref().unwrap_or("")
            ));
        }

        Ok(csv)
    }

    /// Export as XML format
    fn export_xml(&self, events: &[AuditEvent]) -> Result<String, String> {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<audit_logs>\n");

        for event in events {
            xml.push_str(&format!(
                "  <event>\n    <timestamp>{}</timestamp>\n    <severity>{:?}</severity>\n    <category>{:?}</category>\n    <event_type>{}</event_type>\n    <actor>{}</actor>\n    <action>{}</action>\n    <success>{}</success>\n  </event>\n",
                event.timestamp,
                event.severity,
                event.category,
                event.event_type,
                event.actor.as_deref().unwrap_or(""),
                event.action,
                event.success
            ));
        }

        xml.push_str("</audit_logs>");
        Ok(xml)
    }

    /// Export as PDF format (simplified)
    fn export_pdf(&self, events: &[AuditEvent]) -> Result<String, String> {
        // In a real implementation, this would generate an actual PDF
        // For now, return a placeholder
        Ok(format!("PDF export would contain {} events", events.len()))
    }

    /// Verify integrity of audit log
    pub async fn verify_integrity(&self) -> Result<bool, String> {
        let entries = self.entries.read().await;
        
        if entries.is_empty() {
            return Ok(true);
        }

        for (i, entry) in entries.iter().enumerate() {
            // Verify hash chain
            if i > 0 {
                let previous_entry = &entries[i - 1];
                if entry.event.previous_hash.as_ref() != Some(&previous_entry.event.hash) {
                    return Ok(false);
                }
            }

            // Verify signature if enabled
            if self.config.enable_signatures {
                if let Some(ref verifying_key) = self.verifying_key {
                    let data_to_verify = format!("{}:{}:{}", 
                        entry.event.id, entry.timestamp, entry.event.hash);
                    
                    if let Ok(signature_bytes) = base64::engine::general_purpose::STANDARD.decode(&entry.signature) {
                        if signature_bytes.len() == 64 {
                            let mut signature_array = [0u8; 64];
                            signature_array.copy_from_slice(&signature_bytes);
                            let signature = Signature::from_bytes(&signature_array);
                            if verifying_key.verify(data_to_verify.as_bytes(), &signature).is_err() {
                                return Ok(false);
                            }
                        } else {
                            return Ok(false);
                        }
                    } else {
                        return Ok(false);
                    }
                }
            }
        }

        Ok(true)
    }

    /// Get audit statistics
    pub async fn get_statistics(&self) -> Result<HashMap<String, u64>, String> {
        let entries = self.entries.read().await;
        let mut stats = HashMap::new();

        stats.insert("total_events".to_string(), entries.len() as u64);

        let mut severity_counts = HashMap::new();
        let mut category_counts = HashMap::new();
        let mut success_count = 0;

        for entry in entries.iter() {
            *severity_counts.entry(format!("{:?}", entry.event.severity)).or_insert(0) += 1;
            *category_counts.entry(format!("{:?}", entry.event.category)).or_insert(0) += 1;
            
            if entry.event.success {
                success_count += 1;
            }
        }

        stats.insert("successful_events".to_string(), success_count);
        stats.insert("failed_events".to_string(), entries.len() as u64 - success_count);

        for (severity, count) in severity_counts {
            stats.insert(format!("severity_{}", severity.to_lowercase()), count);
        }

        for (category, count) in category_counts {
            stats.insert(format!("category_{}", category.to_lowercase()), count);
        }

        Ok(stats)
    }

    /// Clear audit logs (use with caution)
    pub async fn clear_logs(&self) -> Result<(), String> {
        if !self.config.compliance_mode {
            let mut entries = self.entries.write().await;
            entries.clear();
            
            let mut counter = self.sequence_counter.lock().unwrap();
            *counter = 0;
            
            let mut tree = self.merkle_tree.lock().unwrap();
            tree.clear();
        } else {
            return Err("Cannot clear logs in compliance mode".to_string());
        }
        Ok(())
    }
} 
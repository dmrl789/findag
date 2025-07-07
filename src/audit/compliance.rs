use crate::audit::audit_logger::{AuditEvent, AuditSeverity};
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Compliance framework types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplianceFramework {
    GDPR,
    SOX,
    PciDss,
    HIPAA,
    ISO27001,
    FINRA,
    BaselIII,
    Custom(String),
}

/// Compliance requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    pub id: String,
    pub framework: ComplianceFramework,
    pub title: String,
    pub description: String,
    pub category: String,
    pub severity: AuditSeverity,
    pub required_controls: Vec<String>,
    pub audit_frequency: String,
    pub last_audit: Option<DateTime<Utc>>,
    pub next_audit: Option<DateTime<Utc>>,
    pub status: ComplianceStatus,
    pub evidence: Vec<ComplianceEvidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    Partial,
    NotApplicable,
    UnderReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceEvidence {
    pub id: String,
    pub requirement_id: String,
    pub evidence_type: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub data: HashMap<String, String>,
    pub attachments: Vec<String>,
}

/// Compliance assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAssessment {
    pub framework: ComplianceFramework,
    pub assessment_date: DateTime<Utc>,
    pub assessor: String,
    pub scope: String,
    pub requirements: Vec<ComplianceRequirement>,
    pub overall_status: ComplianceStatus,
    pub compliance_score: f64,
    pub findings: Vec<ComplianceFinding>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFinding {
    pub id: String,
    pub requirement_id: String,
    pub severity: AuditSeverity,
    pub description: String,
    pub impact: String,
    pub remediation: String,
    pub due_date: Option<DateTime<Utc>>,
    pub status: FindingStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FindingStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
}

/// Compliance manager
pub struct ComplianceManager {
    requirements: Arc<RwLock<HashMap<String, ComplianceRequirement>>>,
    assessments: Arc<RwLock<Vec<ComplianceAssessment>>>,
    frameworks: Vec<ComplianceFramework>,
}

impl ComplianceManager {
    pub fn new() -> Self {
        let frameworks = vec![
            ComplianceFramework::GDPR,
            ComplianceFramework::SOX,
            ComplianceFramework::PciDss,
            ComplianceFramework::HIPAA,
            ComplianceFramework::ISO27001,
            ComplianceFramework::FINRA,
            ComplianceFramework::BaselIII,
        ];

        Self {
            requirements: Arc::new(RwLock::new(HashMap::new())),
            assessments: Arc::new(RwLock::new(Vec::new())),
            frameworks,
        }
    }

    /// Load default GDPR requirements
    pub async fn load_gdpr_requirements(&self) -> Result<(), String> {
        let mut requirements = self.requirements.write().await;
        
        let gdpr_reqs = vec![
            ComplianceRequirement {
                id: "GDPR-001".to_string(),
                framework: ComplianceFramework::GDPR,
                title: "Data Processing Lawfulness".to_string(),
                description: "Ensure all data processing has a legal basis".to_string(),
                category: "Data Processing".to_string(),
                severity: AuditSeverity::Critical,
                required_controls: vec![
                    "Consent management".to_string(),
                    "Legal basis documentation".to_string(),
                    "Processing records".to_string(),
                ],
                audit_frequency: "Quarterly".to_string(),
                last_audit: None,
                next_audit: None,
                status: ComplianceStatus::UnderReview,
                evidence: Vec::new(),
            },
            ComplianceRequirement {
                id: "GDPR-002".to_string(),
                framework: ComplianceFramework::GDPR,
                title: "Data Subject Rights".to_string(),
                description: "Implement mechanisms for data subject rights".to_string(),
                category: "Data Subject Rights".to_string(),
                severity: AuditSeverity::Critical,
                required_controls: vec![
                    "Right to access".to_string(),
                    "Right to rectification".to_string(),
                    "Right to erasure".to_string(),
                    "Right to portability".to_string(),
                ],
                audit_frequency: "Monthly".to_string(),
                last_audit: None,
                next_audit: None,
                status: ComplianceStatus::UnderReview,
                evidence: Vec::new(),
            },
            ComplianceRequirement {
                id: "GDPR-003".to_string(),
                framework: ComplianceFramework::GDPR,
                title: "Data Security".to_string(),
                description: "Implement appropriate security measures".to_string(),
                category: "Data Security".to_string(),
                severity: AuditSeverity::Critical,
                required_controls: vec![
                    "Encryption at rest".to_string(),
                    "Encryption in transit".to_string(),
                    "Access controls".to_string(),
                    "Security monitoring".to_string(),
                ],
                audit_frequency: "Weekly".to_string(),
                last_audit: None,
                next_audit: None,
                status: ComplianceStatus::UnderReview,
                evidence: Vec::new(),
            },
        ];

        for req in gdpr_reqs {
            requirements.insert(req.id.clone(), req);
        }

        Ok(())
    }

    /// Load default SOX requirements
    pub async fn load_sox_requirements(&self) -> Result<(), String> {
        let mut requirements = self.requirements.write().await;
        
        let sox_reqs = vec![
            ComplianceRequirement {
                id: "SOX-001".to_string(),
                framework: ComplianceFramework::SOX,
                title: "Internal Controls".to_string(),
                description: "Maintain effective internal controls over financial reporting".to_string(),
                category: "Financial Controls".to_string(),
                severity: AuditSeverity::Critical,
                required_controls: vec![
                    "Control environment".to_string(),
                    "Risk assessment".to_string(),
                    "Control activities".to_string(),
                    "Information and communication".to_string(),
                    "Monitoring activities".to_string(),
                ],
                audit_frequency: "Quarterly".to_string(),
                last_audit: None,
                next_audit: None,
                status: ComplianceStatus::UnderReview,
                evidence: Vec::new(),
            },
            ComplianceRequirement {
                id: "SOX-002".to_string(),
                framework: ComplianceFramework::SOX,
                title: "IT General Controls".to_string(),
                description: "Ensure IT systems support financial reporting integrity".to_string(),
                category: "IT Controls".to_string(),
                severity: AuditSeverity::Critical,
                required_controls: vec![
                    "Access management".to_string(),
                    "Change management".to_string(),
                    "System development".to_string(),
                    "Business continuity".to_string(),
                ],
                audit_frequency: "Monthly".to_string(),
                last_audit: None,
                next_audit: None,
                status: ComplianceStatus::UnderReview,
                evidence: Vec::new(),
            },
        ];

        for req in sox_reqs {
            requirements.insert(req.id.clone(), req);
        }

        Ok(())
    }

    /// Assess compliance based on audit events
    pub async fn assess_compliance(
        &self,
        events: &[AuditEvent],
        framework: ComplianceFramework,
    ) -> Result<ComplianceAssessment, String> {
        let requirements = self.requirements.read().await;
        let framework_reqs: Vec<ComplianceRequirement> = requirements
            .values()
            .filter(|req| req.framework == framework)
            .cloned()
            .collect();

        if framework_reqs.is_empty() {
            return Err(format!("No requirements found for framework: {:?}", framework));
        }

        let mut assessment = ComplianceAssessment {
            framework,
            assessment_date: Utc::now(),
            assessor: "System".to_string(),
            scope: "Automated assessment based on audit events".to_string(),
            requirements: framework_reqs.clone(),
            overall_status: ComplianceStatus::Compliant,
            compliance_score: 100.0,
            findings: Vec::new(),
            recommendations: Vec::new(),
        };

        // Assess each requirement
        for req in &mut assessment.requirements {
            let req_status = self.assess_requirement(&req, events).await;
            req.status = if req_status.status == FindingStatus::Closed {
                ComplianceStatus::Compliant
            } else {
                ComplianceStatus::NonCompliant
            };
            req.last_audit = Some(Utc::now());
            req.next_audit = Some(Utc::now() + Duration::days(90)); // 90 days from now

            if req_status.status != FindingStatus::Closed {
                assessment.findings.push(req_status);
                assessment.compliance_score -= 10.0; // Deduct 10 points per non-compliant requirement
            }
        }

        // Determine overall status
        assessment.overall_status = if assessment.compliance_score >= 90.0 {
            ComplianceStatus::Compliant
        } else if assessment.compliance_score >= 70.0 {
            ComplianceStatus::Partial
        } else {
            ComplianceStatus::NonCompliant
        };

        // Generate recommendations
        assessment.recommendations = self.generate_recommendations(&assessment).await;

        // Store assessment
        {
            let mut assessments = self.assessments.write().await;
            assessments.push(assessment.clone());
        }

        Ok(assessment)
    }

    /// Assess individual requirement
    async fn assess_requirement(
        &self,
        requirement: &ComplianceRequirement,
        events: &[AuditEvent],
    ) -> ComplianceFinding {
        let mut finding = ComplianceFinding {
            id: format!("{}-{}", requirement.id, Utc::now().timestamp()),
            requirement_id: requirement.id.clone(),
            severity: requirement.severity.clone(),
            description: String::new(),
            impact: String::new(),
            remediation: String::new(),
            due_date: None,
            status: FindingStatus::Open,
        };

        // Check for relevant audit events
        let relevant_events: Vec<&AuditEvent> = events
            .iter()
            .filter(|e| {
                // Filter events based on requirement category and controls
                format!("{:?}", e.category).to_lowercase().contains(&requirement.category.to_lowercase()) ||
                requirement.required_controls.iter().any(|control| {
                    e.action.to_lowercase().contains(&control.to_lowercase()) ||
                    e.event_type.to_lowercase().contains(&control.to_lowercase())
                })
            })
            .collect();

        if relevant_events.is_empty() {
            finding.description = format!("No audit events found for requirement: {}", requirement.title);
            finding.impact = "Requirement cannot be verified without audit evidence".to_string();
            finding.remediation = "Implement controls and generate audit events".to_string();
            finding.status = FindingStatus::Open;
        } else {
            // Check for successful implementation
            let successful_events = relevant_events.iter().filter(|e| e.success).count();
            let total_events = relevant_events.len();

            if successful_events == total_events && total_events > 0 {
                finding.description = format!("Requirement {} is properly implemented", requirement.title);
                finding.impact = "Compliant".to_string();
                finding.remediation = "Continue monitoring".to_string();
                finding.status = FindingStatus::Closed;
            } else {
                finding.description = format!("Requirement {} has implementation issues", requirement.title);
                finding.impact = format!("{} of {} events failed", total_events - successful_events, total_events);
                finding.remediation = "Review and fix failed controls".to_string();
                finding.status = FindingStatus::Open;
            }
        }

        finding
    }

    /// Generate compliance recommendations
    async fn generate_recommendations(&self, assessment: &ComplianceAssessment) -> Vec<String> {
        let mut recommendations = Vec::new();

        let non_compliant_count = assessment.requirements
            .iter()
            .filter(|req| req.status != ComplianceStatus::Compliant)
            .count();

        if non_compliant_count > 0 {
            recommendations.push(format!("Address {} non-compliant requirements", non_compliant_count));
        }

        if assessment.compliance_score < 90.0 {
            recommendations.push("Implement additional controls to improve compliance score".to_string());
        }

        let open_findings = assessment.findings
            .iter()
            .filter(|f| f.status == FindingStatus::Open)
            .count();

        if open_findings > 0 {
            recommendations.push(format!("Resolve {} open findings", open_findings));
        }

        recommendations.push("Schedule regular compliance reviews".to_string());
        recommendations.push("Maintain audit trails for all compliance activities".to_string());

        recommendations
    }

    /// Generate compliance report
    pub async fn generate_compliance_report(
        &self,
        framework: ComplianceFramework,
    ) -> Result<String, String> {
        let assessments = self.assessments.read().await;
        let framework_assessments: Vec<&ComplianceAssessment> = assessments
            .iter()
            .filter(|a| a.framework == framework)
            .collect();

        if framework_assessments.is_empty() {
            return Err(format!("No assessments found for framework: {:?}", framework));
        }

        let latest_assessment = framework_assessments.last().unwrap();
        
        let mut report = String::new();
        report.push_str(&format!("=== {} COMPLIANCE REPORT ===\n", format!("{:?}", framework)));
        report.push_str(&format!("Assessment Date: {}\n", latest_assessment.assessment_date));
        report.push_str(&format!("Assessor: {}\n", latest_assessment.assessor));
        report.push_str(&format!("Overall Status: {:?}\n", latest_assessment.overall_status));
        report.push_str(&format!("Compliance Score: {:.1}%\n", latest_assessment.compliance_score));
        report.push_str(&format!("Requirements Assessed: {}\n", latest_assessment.requirements.len()));
        report.push_str(&format!("Findings: {}\n", latest_assessment.findings.len()));

        report.push_str("\n=== REQUIREMENTS STATUS ===\n");
        for req in &latest_assessment.requirements {
            report.push_str(&format!("- {}: {:?}\n", req.title, req.status));
        }

        if !latest_assessment.findings.is_empty() {
            report.push_str("\n=== FINDINGS ===\n");
            for finding in &latest_assessment.findings {
                report.push_str(&format!("- {}: {} (Status: {:?})\n", 
                    finding.requirement_id, finding.description, finding.status));
            }
        }

        if !latest_assessment.recommendations.is_empty() {
            report.push_str("\n=== RECOMMENDATIONS ===\n");
            for (i, rec) in latest_assessment.recommendations.iter().enumerate() {
                report.push_str(&format!("{}. {}\n", i + 1, rec));
            }
        }

        Ok(report)
    }

    /// Get compliance statistics
    pub async fn get_compliance_stats(&self) -> Result<HashMap<String, u64>, String> {
        let requirements = self.requirements.read().await;
        let assessments = self.assessments.read().await;

        let mut stats = HashMap::new();
        stats.insert("total_requirements".to_string(), requirements.len() as u64);
        stats.insert("total_assessments".to_string(), assessments.len() as u64);

        // Count by framework
        let mut framework_counts = HashMap::new();
        for req in requirements.values() {
            let framework_key = format!("framework_{:?}", req.framework);
            *framework_counts.entry(framework_key).or_insert(0) += 1;
        }

        for (key, value) in framework_counts {
            stats.insert(key, value);
        }

        // Count by status
        let mut status_counts = HashMap::new();
        for req in requirements.values() {
            let status_key = format!("status_{:?}", req.status);
            *status_counts.entry(status_key).or_insert(0) += 1;
        }

        for (key, value) in status_counts {
            stats.insert(key, value);
        }

        Ok(stats)
    }
} 
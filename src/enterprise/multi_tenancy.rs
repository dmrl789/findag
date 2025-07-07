use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Tenant information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub status: TenantStatus,
    pub plan: TenantPlan,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    pub settings: TenantSettings,
    pub quotas: ResourceQuotas,
    pub billing: BillingInfo,
}

/// Tenant status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TenantStatus {
    Active,
    Suspended,
    Pending,
    Cancelled,
    Expired,
}

/// Tenant plan
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TenantPlan {
    Free,
    Basic,
    Professional,
    Enterprise,
    Custom,
}

/// Tenant settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantSettings {
    pub allowed_assets: Vec<String>,
    pub max_transaction_size: f64,
    pub allowed_transaction_types: Vec<String>,
    pub security_level: SecurityLevel,
    pub compliance_requirements: Vec<String>,
    pub custom_features: Vec<String>,
}

/// Security level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Standard,
    Enhanced,
    Premium,
    Custom,
}

/// Resource quotas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceQuotas {
    pub max_transactions_per_second: u64,
    pub max_storage_gb: u64,
    pub max_api_requests_per_minute: u64,
    pub max_concurrent_users: u64,
    pub max_data_retention_days: u64,
    pub max_backup_size_gb: u64,
    pub current_usage: ResourceUsage,
}

/// Resource usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub transactions_per_second: u64,
    pub storage_used_gb: f64,
    pub api_requests_last_minute: u64,
    pub concurrent_users: u64,
    pub data_retention_days: u64,
    pub backup_size_gb: f64,
    pub last_updated: DateTime<Utc>,
}

/// Billing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingInfo {
    pub billing_cycle: BillingCycle,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub amount_due: f64,
    pub currency: String,
    pub payment_method: PaymentMethod,
    pub invoices: Vec<Invoice>,
    pub usage_based_charges: Vec<UsageCharge>,
}

/// Billing cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    Yearly,
    Custom,
}

/// Payment method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub method_type: PaymentMethodType,
    pub last_four: Option<String>,
    pub expiry_date: Option<String>,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethodType {
    CreditCard,
    BankTransfer,
    PayPal,
    Crypto,
}

/// Invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub amount: f64,
    pub currency: String,
    pub status: InvoiceStatus,
    pub due_date: DateTime<Utc>,
    pub paid_date: Option<DateTime<Utc>>,
    pub items: Vec<InvoiceItem>,
}

/// Invoice status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceStatus {
    Pending,
    Paid,
    Overdue,
    Cancelled,
}

/// Invoice item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub description: String,
    pub quantity: u64,
    pub unit_price: f64,
    pub total: f64,
}

/// Usage charge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageCharge {
    pub resource_type: String,
    pub usage_amount: f64,
    pub unit_price: f64,
    pub total_charge: f64,
    pub period: String,
}

/// Tenant isolation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantIsolation {
    pub database_isolation: DatabaseIsolation,
    pub network_isolation: NetworkIsolation,
    pub storage_isolation: StorageIsolation,
    pub api_isolation: ApiIsolation,
}

/// Database isolation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseIsolation {
    pub isolation_type: DatabaseIsolationType,
    pub schema_prefix: String,
    pub connection_pool_size: u32,
    pub backup_retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseIsolationType {
    Schema,
    Database,
    Instance,
}

/// Network isolation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIsolation {
    pub vpc_id: Option<String>,
    pub subnet_ids: Vec<String>,
    pub security_groups: Vec<String>,
    pub allowed_ips: Vec<String>,
    pub dns_settings: DnsSettings,
}

/// DNS settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsSettings {
    pub custom_domain: Option<String>,
    pub ssl_certificate: Option<String>,
    pub dns_records: Vec<DnsRecord>,
}

/// DNS record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    pub record_type: String,
    pub name: String,
    pub value: String,
    pub ttl: u32,
}

/// Storage isolation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageIsolation {
    pub storage_type: StorageType,
    pub bucket_name: Option<String>,
    pub encryption_enabled: bool,
    pub backup_enabled: bool,
    pub retention_policy: RetentionPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    Local,
    S3,
    AzureBlob,
    GoogleCloudStorage,
}

/// Retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub data_retention_days: u32,
    pub backup_retention_days: u32,
    pub archive_retention_days: u32,
    pub deletion_policy: DeletionPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeletionPolicy {
    Immediate,
    Gradual,
    Archive,
}

/// API isolation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiIsolation {
    pub rate_limits: RateLimits,
    pub allowed_endpoints: Vec<String>,
    pub custom_headers: HashMap<String, String>,
    pub webhook_urls: Vec<String>,
}

/// Rate limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimits {
    pub requests_per_minute: u64,
    pub requests_per_hour: u64,
    pub requests_per_day: u64,
    pub burst_limit: u64,
}

/// Multi-tenancy manager
pub struct MultiTenancyManager {
    tenants: HashMap<String, Tenant>,
    isolation_configs: HashMap<String, TenantIsolation>,
    billing_engine: BillingEngine,
    quota_monitor: QuotaMonitor,
}

/// Billing engine
pub struct BillingEngine {
    pricing_plans: HashMap<TenantPlan, PricingPlan>,
    usage_trackers: HashMap<String, UsageTracker>,
}

/// Pricing plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingPlan {
    pub base_price: f64,
    pub currency: String,
    pub billing_cycle: BillingCycle,
    pub included_usage: HashMap<String, u64>,
    pub overage_rates: HashMap<String, f64>,
    pub features: Vec<String>,
}

/// Usage tracker
pub struct UsageTracker {
    pub tenant_id: String,
    pub current_usage: ResourceUsage,
    pub usage_history: Vec<UsageRecord>,
}

/// Usage record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub timestamp: DateTime<Utc>,
    pub resource_type: String,
    pub usage_amount: f64,
    pub cost: f64,
}

/// Quota monitor
pub struct QuotaMonitor {
    pub quota_violations: Vec<QuotaViolation>,
    pub alert_thresholds: HashMap<String, f64>,
}

/// Quota violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaViolation {
    pub tenant_id: String,
    pub resource_type: String,
    pub current_usage: f64,
    pub limit: f64,
    pub timestamp: DateTime<Utc>,
    pub severity: ViolationSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Warning,
    Critical,
    Blocking,
}

impl MultiTenancyManager {
    pub fn new() -> Self {
        Self {
            tenants: HashMap::new(),
            isolation_configs: HashMap::new(),
            billing_engine: BillingEngine::new(),
            quota_monitor: QuotaMonitor::new(),
        }
    }

    /// Create a new tenant
    pub fn create_tenant(
        &mut self,
        name: String,
        domain: String,
        plan: TenantPlan,
        settings: TenantSettings,
    ) -> Result<String, String> {
        let tenant_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let quotas = self.billing_engine.get_plan_quotas(&plan);
        let billing = self.billing_engine.initialize_billing(&plan);
        
        let tenant = Tenant {
            id: tenant_id.clone(),
            name,
            domain,
            status: TenantStatus::Pending,
            plan,
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
            settings,
            quotas,
            billing,
        };
        
        self.tenants.insert(tenant_id.clone(), tenant);
        self.setup_tenant_isolation(&tenant_id)?;
        
        Ok(tenant_id)
    }

    /// Get tenant by ID
    pub fn get_tenant(&self, tenant_id: &str) -> Option<&Tenant> {
        self.tenants.get(tenant_id)
    }

    /// Update tenant
    pub fn update_tenant(&mut self, tenant_id: &str, updates: TenantUpdates) -> Result<(), String> {
        if let Some(tenant) = self.tenants.get_mut(tenant_id) {
            if let Some(name) = updates.name {
                tenant.name = name;
            }
            if let Some(domain) = updates.domain {
                tenant.domain = domain;
            }
            if let Some(status) = updates.status {
                tenant.status = status;
            }
            if let Some(plan) = updates.plan {
                tenant.plan = plan.clone();
                tenant.quotas = self.billing_engine.get_plan_quotas(&plan);
            }
            if let Some(settings) = updates.settings {
                tenant.settings = settings;
            }
            
            tenant.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Tenant not found".to_string())
        }
    }

    /// Delete tenant
    pub fn delete_tenant(&mut self, tenant_id: &str) -> Result<(), String> {
        if self.tenants.contains_key(tenant_id) {
            self.cleanup_tenant_resources(tenant_id)?;
            self.tenants.remove(tenant_id);
            self.isolation_configs.remove(tenant_id);
            Ok(())
        } else {
            Err("Tenant not found".to_string())
        }
    }

    /// List all tenants
    pub fn list_tenants(&self) -> Vec<&Tenant> {
        self.tenants.values().collect()
    }

    /// Check resource quota
    pub fn check_quota(&self, tenant_id: &str, resource_type: &str, amount: f64) -> Result<bool, String> {
        if let Some(tenant) = self.tenants.get(tenant_id) {
            let current_usage = &tenant.quotas.current_usage;
            let limit = self.get_resource_limit(&tenant.quotas, resource_type);
            
            let new_usage = self.calculate_new_usage(current_usage, resource_type, amount);
            Ok(new_usage <= limit)
        } else {
            Err("Tenant not found".to_string())
        }
    }

    /// Update resource usage
    pub fn update_usage(&mut self, tenant_id: &str, resource_type: &str, amount: f64) -> Result<(), String> {
        // First, get the current usage and limits without mutable borrow
        let (current_usage, limit) = {
            if let Some(tenant) = self.tenants.get(tenant_id) {
                let new_usage = self.calculate_new_usage(&tenant.quotas.current_usage, resource_type, amount);
                let limit = self.get_resource_limit(&tenant.quotas, resource_type);
                (new_usage, limit)
            } else {
                return Err("Tenant not found".to_string());
            }
        };
        
        if current_usage > limit {
            self.quota_monitor.record_violation(tenant_id, resource_type, current_usage, limit);
            return Err("Quota exceeded".to_string());
        }
        
        // Now update the tenant with mutable borrow
        if let Some(tenant) = self.tenants.get_mut(tenant_id) {
            update_resource_usage(&mut tenant.quotas.current_usage, resource_type, amount);
            tenant.quotas.current_usage.last_updated = Utc::now();
            
            // Update billing
            self.billing_engine.track_usage(tenant_id, resource_type, amount);
        }
        
        Ok(())
    }

    /// Get billing information
    pub fn get_billing_info(&self, tenant_id: &str) -> Result<&BillingInfo, String> {
        if let Some(tenant) = self.tenants.get(tenant_id) {
            Ok(&tenant.billing)
        } else {
            Err("Tenant not found".to_string())
        }
    }

    /// Generate invoice
    pub fn generate_invoice(&mut self, tenant_id: &str) -> Result<Invoice, String> {
        self.billing_engine.generate_invoice(tenant_id)
    }

    /// Get quota violations
    pub fn get_quota_violations(&self) -> Vec<&QuotaViolation> {
        self.quota_monitor.get_violations()
    }

    // Private helper methods
    fn setup_tenant_isolation(&mut self, tenant_id: &str) -> Result<(), String> {
        let isolation = TenantIsolation {
            database_isolation: DatabaseIsolation {
                isolation_type: DatabaseIsolationType::Schema,
                schema_prefix: format!("tenant_{}", tenant_id),
                connection_pool_size: 10,
                backup_retention_days: 30,
            },
            network_isolation: NetworkIsolation {
                vpc_id: None,
                subnet_ids: Vec::new(),
                security_groups: Vec::new(),
                allowed_ips: Vec::new(),
                dns_settings: DnsSettings {
                    custom_domain: None,
                    ssl_certificate: None,
                    dns_records: Vec::new(),
                },
            },
            storage_isolation: StorageIsolation {
                storage_type: StorageType::Local,
                bucket_name: None,
                encryption_enabled: true,
                backup_enabled: true,
                retention_policy: RetentionPolicy {
                    data_retention_days: 365,
                    backup_retention_days: 30,
                    archive_retention_days: 2555,
                    deletion_policy: DeletionPolicy::Archive,
                },
            },
            api_isolation: ApiIsolation {
                rate_limits: RateLimits {
                    requests_per_minute: 1000,
                    requests_per_hour: 50000,
                    requests_per_day: 1000000,
                    burst_limit: 100,
                },
                allowed_endpoints: vec!["*".to_string()],
                custom_headers: HashMap::new(),
                webhook_urls: Vec::new(),
            },
        };
        
        self.isolation_configs.insert(tenant_id.to_string(), isolation);
        Ok(())
    }

    fn cleanup_tenant_resources(&self, _tenant_id: &str) -> Result<(), String> {
        // Mock implementation - replace with actual cleanup logic
        Ok(())
    }

    fn get_resource_limit(&self, quotas: &ResourceQuotas, resource_type: &str) -> f64 {
        match resource_type {
            "transactions_per_second" => quotas.max_transactions_per_second as f64,
            "storage_gb" => quotas.max_storage_gb as f64,
            "api_requests_per_minute" => quotas.max_api_requests_per_minute as f64,
            "concurrent_users" => quotas.max_concurrent_users as f64,
            _ => 0.0,
        }
    }

    fn calculate_new_usage(&self, current: &ResourceUsage, resource_type: &str, amount: f64) -> f64 {
        match resource_type {
            "transactions_per_second" => current.transactions_per_second as f64 + amount,
            "storage_gb" => current.storage_used_gb + amount,
            "api_requests_per_minute" => current.api_requests_last_minute as f64 + amount,
            "concurrent_users" => current.concurrent_users as f64 + amount,
            _ => 0.0,
        }
    }

    fn update_resource_usage(&self, usage: &mut ResourceUsage, resource_type: &str, amount: f64) {
        match resource_type {
            "transactions_per_second" => usage.transactions_per_second = amount as u64,
            "storage_gb" => usage.storage_used_gb = amount,
            "api_requests_per_minute" => usage.api_requests_last_minute = amount as u64,
            "concurrent_users" => usage.concurrent_users = amount as u64,
            _ => {},
        }
    }
}

impl BillingEngine {
    pub fn new() -> Self {
        let mut pricing_plans = HashMap::new();
        pricing_plans.insert(TenantPlan::Free, PricingPlan {
            base_price: 0.0,
            currency: "USD".to_string(),
            billing_cycle: BillingCycle::Monthly,
            included_usage: HashMap::new(),
            overage_rates: HashMap::new(),
            features: vec!["basic_api".to_string()],
        });
        
        Self {
            pricing_plans,
            usage_trackers: HashMap::new(),
        }
    }

    pub fn get_plan_quotas(&self, plan: &TenantPlan) -> ResourceQuotas {
        match plan {
            TenantPlan::Free => ResourceQuotas {
                max_transactions_per_second: 100,
                max_storage_gb: 10,
                max_api_requests_per_minute: 1000,
                max_concurrent_users: 10,
                max_data_retention_days: 30,
                max_backup_size_gb: 5,
                current_usage: ResourceUsage {
                    transactions_per_second: 0,
                    storage_used_gb: 0.0,
                    api_requests_last_minute: 0,
                    concurrent_users: 0,
                    data_retention_days: 0,
                    backup_size_gb: 0.0,
                    last_updated: Utc::now(),
                },
            },
            TenantPlan::Basic => ResourceQuotas {
                max_transactions_per_second: 1000,
                max_storage_gb: 100,
                max_api_requests_per_minute: 10000,
                max_concurrent_users: 100,
                max_data_retention_days: 90,
                max_backup_size_gb: 50,
                current_usage: ResourceUsage {
                    transactions_per_second: 0,
                    storage_used_gb: 0.0,
                    api_requests_last_minute: 0,
                    concurrent_users: 0,
                    data_retention_days: 0,
                    backup_size_gb: 0.0,
                    last_updated: Utc::now(),
                },
            },
            _ => ResourceQuotas {
                max_transactions_per_second: 10000,
                max_storage_gb: 1000,
                max_api_requests_per_minute: 100000,
                max_concurrent_users: 1000,
                max_data_retention_days: 365,
                max_backup_size_gb: 500,
                current_usage: ResourceUsage {
                    transactions_per_second: 0,
                    storage_used_gb: 0.0,
                    api_requests_last_minute: 0,
                    concurrent_users: 0,
                    data_retention_days: 0,
                    backup_size_gb: 0.0,
                    last_updated: Utc::now(),
                },
            },
        }
    }

    pub fn initialize_billing(&self, _plan: &TenantPlan) -> BillingInfo {
        let now = Utc::now();
        BillingInfo {
            billing_cycle: BillingCycle::Monthly,
            current_period_start: now,
            current_period_end: now + chrono::Duration::days(30),
            amount_due: 0.0,
            currency: "USD".to_string(),
            payment_method: PaymentMethod {
                method_type: PaymentMethodType::CreditCard,
                last_four: None,
                expiry_date: None,
                is_default: true,
            },
            invoices: Vec::new(),
            usage_based_charges: Vec::new(),
        }
    }

    pub fn track_usage(&mut self, _tenant_id: &str, _resource_type: &str, _amount: f64) {
        // Mock implementation - replace with actual usage tracking
    }

    pub fn generate_invoice(&mut self, _tenant_id: &str) -> Result<Invoice, String> {
        // Mock implementation - replace with actual invoice generation
        Ok(Invoice {
            id: Uuid::new_v4().to_string(),
            amount: 100.0,
            currency: "USD".to_string(),
            status: InvoiceStatus::Pending,
            due_date: Utc::now() + chrono::Duration::days(30),
            paid_date: None,
            items: Vec::new(),
        })
    }
}

impl QuotaMonitor {
    pub fn new() -> Self {
        Self {
            quota_violations: Vec::new(),
            alert_thresholds: HashMap::new(),
        }
    }

    pub fn record_violation(&mut self, tenant_id: &str, resource_type: &str, current_usage: f64, limit: f64) {
        let violation = QuotaViolation {
            tenant_id: tenant_id.to_string(),
            resource_type: resource_type.to_string(),
            current_usage,
            limit,
            timestamp: Utc::now(),
            severity: ViolationSeverity::Warning,
        };
        self.quota_violations.push(violation);
    }

    pub fn get_violations(&self) -> Vec<&QuotaViolation> {
        self.quota_violations.iter().collect()
    }
}

/// Tenant updates
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TenantUpdates {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub status: Option<TenantStatus>,
    pub plan: Option<TenantPlan>,
    pub settings: Option<TenantSettings>,
}

// Standalone function to avoid borrow checker issues
fn update_resource_usage(usage: &mut ResourceUsage, resource_type: &str, amount: f64) {
    match resource_type {
        "transactions_per_second" => usage.transactions_per_second = amount as u64,
        "storage_gb" => usage.storage_used_gb = amount,
        "api_requests_per_minute" => usage.api_requests_last_minute = amount as u64,
        "concurrent_users" => usage.concurrent_users = amount as u64,
        _ => {},
    }
} 
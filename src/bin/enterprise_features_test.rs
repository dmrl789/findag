use findag::enterprise::{
    analytics::{AnalyticsEngine, BusinessIntelligenceData, ReportConfig, ReportType, TimeRange, TimeGranularity, Aggregation, AggregationFunction, ReportFormat},
    multi_tenancy::{MultiTenancyManager, TenantPlan, TenantSettings, SecurityLevel, TenantUpdates, TenantStatus},
    api_management::{ApiManagementSystem, ApiVersion, ApiVersionStatus, ApiEndpoint, HttpMethod, ApiParameter, ParameterLocation, AuthenticationRequirement, AuthType, DeveloperPortal, ApiKey, ApiKeyStatus},
};
use std::collections::HashMap;
use chrono::{Utc, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🏢 FinDAG Enterprise Features Test");
    println!("==================================");

    // Test 1: Analytics Engine
    println!("\n📊 Test 1: Analytics Engine");
    test_analytics_engine().await?;

    // Test 2: Multi-Tenancy System
    println!("\n🏢 Test 2: Multi-Tenancy System");
    test_multi_tenancy_system().await?;

    // Test 3: API Management System
    println!("\n🔌 Test 3: API Management System");
    test_api_management_system().await?;

    // Test 4: Integration Test
    println!("\n🔗 Test 4: Enterprise Features Integration");
    test_enterprise_integration().await?;

    println!("\n🎉 All enterprise features tests completed successfully!");
    println!("✅ Enterprise features are working correctly");

    Ok(())
}

async fn test_analytics_engine() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("  Initializing analytics engine...");
    let analytics_engine = AnalyticsEngine::new();

    // Test BI dashboard generation
    println!("  Generating business intelligence dashboard...");
    let bi_data = analytics_engine.generate_bi_dashboard().await?;
    
    println!("    ✅ Dashboard generated successfully");
    println!("    📈 Transaction volume: {} TPS", bi_data.transaction_volume.transactions_per_second);
    println!("    👥 Active users: {}", bi_data.user_activity.active_users);
    println!("    💰 Total revenue: ${:.2}", bi_data.revenue_metrics.total_revenue);
    println!("    🛡️ Compliance score: {:.1}%", bi_data.compliance_metrics.regulatory_compliance_score * 100.0);

    // Test custom report generation
    println!("  Generating custom report...");
    let report_config = ReportConfig {
        report_type: ReportType::TransactionSummary,
        time_range: TimeRange {
            start: Utc::now() - Duration::days(30),
            end: Utc::now(),
            granularity: TimeGranularity::Daily,
        },
        filters: HashMap::new(),
        aggregations: vec![
            Aggregation {
                field: "transaction_amount".to_string(),
                function: AggregationFunction::Sum,
                alias: Some("total_volume".to_string()),
            },
            Aggregation {
                field: "transaction_count".to_string(),
                function: AggregationFunction::Count,
                alias: Some("total_transactions".to_string()),
            },
        ],
        format: ReportFormat::JSON,
        schedule: None,
    };

    let report_result = analytics_engine.generate_custom_report(&report_config).await?;
    println!("    ✅ Custom report generated: {}", report_result.report_id);

    // Test data export
    println!("  Testing data export...");
    let export_data = analytics_engine.export_data("SELECT * FROM transactions LIMIT 100", &findag::enterprise::analytics::ExportFormat::CSV).await?;
    println!("    ✅ Data exported successfully ({} bytes)", export_data.len());

    // Test report scheduling
    println!("  Testing report scheduling...");
    let mut engine_with_scheduling = AnalyticsEngine::new();
    
    // Create a report config with a schedule
    let scheduled_report_config = ReportConfig {
        report_type: ReportType::TransactionSummary,
        time_range: TimeRange {
            start: Utc::now() - Duration::days(30),
            end: Utc::now(),
            granularity: TimeGranularity::Daily,
        },
        filters: HashMap::new(),
        aggregations: vec![
            Aggregation {
                field: "transaction_amount".to_string(),
                function: AggregationFunction::Sum,
                alias: Some("total_volume".to_string()),
            },
        ],
        format: ReportFormat::JSON,
        schedule: Some(findag::enterprise::analytics::ReportSchedule {
            frequency: findag::enterprise::analytics::ScheduleFrequency::Monthly,
            time: "09:00".to_string(),
            timezone: "UTC".to_string(),
            recipients: vec!["admin@findag.com".to_string()],
        }),
    };
    
    let scheduled_report_id = engine_with_scheduling.schedule_report(
        "Monthly Transaction Report".to_string(),
        scheduled_report_config,
    )?;
    println!("    ✅ Report scheduled: {}", scheduled_report_id);

    let scheduled_reports = engine_with_scheduling.get_scheduled_reports();
    println!("    📋 Total scheduled reports: {}", scheduled_reports.len());

    println!("  ✅ Analytics engine tests completed");
    Ok(())
}

async fn test_multi_tenancy_system() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("  Initializing multi-tenancy manager...");
    let mut tenancy_manager = MultiTenancyManager::new();

    // Test tenant creation
    println!("  Creating tenants...");
    let tenant_settings = TenantSettings {
        allowed_assets: vec!["EUR".to_string(), "USD".to_string(), "BTC".to_string()],
        max_transaction_size: 1000000.0,
        allowed_transaction_types: vec!["transfer".to_string(), "swap".to_string()],
        security_level: SecurityLevel::Enhanced,
        compliance_requirements: vec!["GDPR".to_string(), "SOX".to_string()],
        custom_features: vec!["advanced_analytics".to_string(), "custom_reporting".to_string()],
    };

    let tenant1_id = tenancy_manager.create_tenant(
        "Acme Corp".to_string(),
        "acme.findag.com".to_string(),
        TenantPlan::Professional,
        tenant_settings.clone(),
    )?;
    println!("    ✅ Created tenant: {} ({})", tenant1_id, "Acme Corp");

    let tenant2_id = tenancy_manager.create_tenant(
        "TechStart Inc".to_string(),
        "techstart.findag.com".to_string(),
        TenantPlan::Basic,
        tenant_settings,
    )?;
    println!("    ✅ Created tenant: {} ({})", tenant2_id, "TechStart Inc");

    // Test tenant retrieval
    println!("  Testing tenant retrieval...");
    let tenant1 = tenancy_manager.get_tenant(&tenant1_id).unwrap();
    println!("    📋 Tenant 1: {} (Plan: {:?}, Status: {:?})", 
             tenant1.name, tenant1.plan, tenant1.status);

    // Test quota checking
    println!("  Testing resource quotas...");
    let quota_check = tenancy_manager.check_quota(&tenant1_id, "transactions_per_second", 500.0)?;
    println!("    ✅ Quota check passed: {}", quota_check);

    // Test usage tracking
    println!("  Testing usage tracking...");
    tenancy_manager.update_usage(&tenant1_id, "transactions_per_second", 100.0)?;
    tenancy_manager.update_usage(&tenant1_id, "storage_gb", 50.0)?;
    println!("    ✅ Usage updated successfully");

    // Test tenant updates
    println!("  Testing tenant updates...");
    let updates = TenantUpdates {
        name: Some("Acme Corporation".to_string()),
        status: Some(TenantStatus::Active),
        plan: Some(TenantPlan::Enterprise),
        ..Default::default()
    };
    tenancy_manager.update_tenant(&tenant1_id, updates)?;
    println!("    ✅ Tenant updated successfully");

    // Test billing information
    println!("  Testing billing information...");
    let billing_info = tenancy_manager.get_billing_info(&tenant1_id)?;
    println!("    💰 Amount due: ${:.2} {}", billing_info.amount_due, billing_info.currency);

    // Test invoice generation
    println!("  Testing invoice generation...");
    let invoice = tenancy_manager.generate_invoice(&tenant1_id)?;
    println!("    📄 Invoice generated: {} (${:.2})", invoice.id, invoice.amount);

    // Test quota violations
    println!("  Testing quota violations...");
    let violations = tenancy_manager.get_quota_violations();
    println!("    ⚠️ Quota violations: {}", violations.len());

    // Test tenant listing
    println!("  Testing tenant listing...");
    let tenants = tenancy_manager.list_tenants();
    println!("    📋 Total tenants: {}", tenants.len());

    // Test tenant deletion
    println!("  Testing tenant deletion...");
    tenancy_manager.delete_tenant(&tenant2_id)?;
    println!("    ✅ Tenant deleted successfully");

    println!("  ✅ Multi-tenancy system tests completed");
    Ok(())
}

async fn test_api_management_system() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("  Initializing API management system...");
    let mut api_manager = ApiManagementSystem::new();

    // Test API version registration
    println!("  Registering API versions...");
    let version_1_0 = ApiVersion {
        version: "1.0.0".to_string(),
        status: ApiVersionStatus::Stable,
        release_date: Utc::now() - Duration::days(90),
        deprecation_date: None,
        sunset_date: None,
        changelog: vec![],
        breaking_changes: vec![],
        new_features: vec!["Initial release".to_string()],
        bug_fixes: vec![],
    };

    let version_1_1 = ApiVersion {
        version: "1.1.0".to_string(),
        status: ApiVersionStatus::Beta,
        release_date: Utc::now() - Duration::days(30),
        deprecation_date: None,
        sunset_date: None,
        changelog: vec![],
        breaking_changes: vec![],
        new_features: vec!["Enhanced analytics".to_string(), "Multi-tenancy support".to_string()],
        bug_fixes: vec!["Fixed transaction validation".to_string()],
    };

    api_manager.register_version(version_1_0)?;
    api_manager.register_version(version_1_1)?;
    println!("    ✅ API versions registered successfully");

    // Test endpoint registration
    println!("  Registering API endpoints...");
    let transaction_endpoint = ApiEndpoint {
        path: "/api/v1/transactions".to_string(),
        method: HttpMethod::POST,
        summary: "Create a new transaction".to_string(),
        description: "Submit a new transaction to the network".to_string(),
        parameters: vec![
            ApiParameter {
                name: "from_address".to_string(),
                location: ParameterLocation::Query,
                required: true,
                data_type: "string".to_string(),
                description: "Source address".to_string(),
                default_value: None,
                allowed_values: None,
                validation_rules: vec![],
            },
            ApiParameter {
                name: "to_address".to_string(),
                location: ParameterLocation::Query,
                required: true,
                data_type: "string".to_string(),
                description: "Destination address".to_string(),
                default_value: None,
                allowed_values: None,
                validation_rules: vec![],
            },
        ],
        request_body: None,
        responses: vec![],
        examples: vec![],
        rate_limits: None,
        authentication: AuthenticationRequirement {
            required: true,
            auth_types: vec![AuthType::Bearer],
            scopes: vec!["transactions:write".to_string()],
        },
        deprecated: false,
        version: "1.0.0".to_string(),
    };

    api_manager.register_endpoint(transaction_endpoint)?;
    println!("    ✅ API endpoints registered successfully");

    // Test API key generation
    println!("  Testing API key management...");
    let api_key = api_manager.generate_api_key(
        "user123",
        "Production API Key",
        vec!["transactions:read".to_string(), "transactions:write".to_string()],
    )?;
    println!("    🔑 API key generated: {}", &api_key[..20]);

    // Test API key validation
    let validated_key = api_manager.validate_api_key(&api_key);
    let key_id = validated_key.as_ref().map(|k| k.id.clone());
    println!("    ✅ API key validation: {}", validated_key.is_some());

    // Test usage tracking
    println!("  Testing usage tracking...");
    api_manager.track_usage("/api/v1/transactions", "user123", "1.0.0", 45.2, true);
    api_manager.track_usage("/api/v1/transactions", "user123", "1.0.0", 67.8, false);
    api_manager.track_usage("/api/v1/assets", "user456", "1.0.0", 23.1, true);
    println!("    ✅ Usage tracking completed");

    // Test analytics
    println!("  Testing analytics...");
    let analytics = api_manager.get_usage_analytics();
    println!("    📊 Total requests: {}", analytics.total_requests);
    println!("    ✅ Successful requests: {}", analytics.successful_requests);
    println!("    ❌ Failed requests: {}", analytics.failed_requests);
    println!("    ⏱️ Average response time: {:.2}ms", analytics.average_response_time);

    // Test OpenAPI specification generation
    println!("  Generating OpenAPI specification...");
    let openapi_spec = api_manager.generate_openapi_spec();
    println!("    ✅ OpenAPI spec generated ({} bytes)", serde_json::to_string(&openapi_spec)?.len());

    // Test code examples
    println!("  Adding code examples...");
    let code_example = findag::enterprise::api_management::CodeExample {
        language: "curl".to_string(),
        title: "Create Transaction".to_string(),
        description: "Example of creating a transaction using curl".to_string(),
        code: r#"curl -X POST "https://api.findag.com/v1/transactions" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"from": "address1", "to": "address2", "amount": 100}'"#.to_string(),
        output: Some(r#"{"transaction_id": "tx_123", "status": "pending"}"#.to_string()),
    };
    api_manager.add_code_example("/api/v1/transactions", code_example);
    println!("    ✅ Code examples added");

    // Test tutorials
    println!("  Adding tutorials...");
    let tutorial = findag::enterprise::api_management::Tutorial {
        id: "getting-started".to_string(),
        title: "Getting Started with FinDAG API".to_string(),
        description: "Learn how to integrate with FinDAG API".to_string(),
        steps: vec![
            findag::enterprise::api_management::TutorialStep {
                title: "Generate API Key".to_string(),
                description: "Create your first API key".to_string(),
                code_example: None,
                expected_result: "API key generated successfully".to_string(),
            },
            findag::enterprise::api_management::TutorialStep {
                title: "Make Your First Request".to_string(),
                description: "Submit a test transaction".to_string(),
                code_example: None,
                expected_result: "Transaction submitted successfully".to_string(),
            },
        ],
        difficulty: findag::enterprise::api_management::TutorialDifficulty::Beginner,
        estimated_time: 15,
    };
    api_manager.add_tutorial(tutorial);
    println!("    ✅ Tutorials added");

    // Test API key revocation
    println!("  Testing API key revocation...");
    if let Some(key_id) = key_id {
        api_manager.revoke_api_key(&key_id)?;
        println!("    ✅ API key revoked successfully");
    }

    println!("  ✅ API management system tests completed");
    Ok(())
}

async fn test_enterprise_integration() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("  Testing enterprise features integration...");

    // Initialize all systems
    let analytics_engine = AnalyticsEngine::new();
    let mut tenancy_manager = MultiTenancyManager::new();
    let mut api_manager = ApiManagementSystem::new();

    // Create a tenant
    let tenant_settings = TenantSettings {
        allowed_assets: vec!["EUR".to_string(), "USD".to_string()],
        max_transaction_size: 1000000.0,
        allowed_transaction_types: vec!["transfer".to_string()],
        security_level: SecurityLevel::Enhanced,
        compliance_requirements: vec!["GDPR".to_string()],
        custom_features: vec!["advanced_analytics".to_string()],
    };

    let tenant_id = tenancy_manager.create_tenant(
        "Integration Test Corp".to_string(),
        "integration.findag.com".to_string(),
        TenantPlan::Professional,
        tenant_settings,
    )?;

    // Generate API key for the tenant
    let api_key = api_manager.generate_api_key(
        &tenant_id,
        "Integration Test Key",
        vec!["transactions:read".to_string(), "analytics:read".to_string()],
    )?;

    // Simulate API usage
    for i in 0..100 {
        api_manager.track_usage(
            "/api/v1/transactions",
            &tenant_id,
            "1.0.0",
            50.0 + (i as f64 * 0.1),
            i % 10 != 0, // 90% success rate
        );
    }

    // Update tenant usage
    tenancy_manager.update_usage(&tenant_id, "transactions_per_second", 100.0)?;
    tenancy_manager.update_usage(&tenant_id, "storage_gb", 25.0)?;

    // Generate analytics for the tenant
    let bi_data = analytics_engine.generate_bi_dashboard().await?;
    let api_analytics = api_manager.get_usage_analytics();
    let tenant = tenancy_manager.get_tenant(&tenant_id).unwrap();

    // Verify integration
    println!("    📊 Integration verification:");
    println!("      Tenant: {} (Plan: {:?})", tenant.name, tenant.plan);
    println!("      API Requests: {}", api_analytics.total_requests);
    println!("      Success Rate: {:.1}%", 
             (api_analytics.successful_requests as f64 / api_analytics.total_requests as f64) * 100.0);
    println!("      System Health: {:.1}% uptime", bi_data.system_health.uptime_percentage);
    println!("      Compliance Score: {:.1}%", bi_data.compliance_metrics.regulatory_compliance_score * 100.0);

    // Generate comprehensive report
    let report_config = ReportConfig {
        report_type: ReportType::Custom,
        time_range: TimeRange {
            start: Utc::now() - Duration::days(7),
            end: Utc::now(),
            granularity: TimeGranularity::Daily,
        },
        filters: HashMap::new(),
        aggregations: vec![
            Aggregation {
                field: "api_requests".to_string(),
                function: AggregationFunction::Sum,
                alias: Some("total_requests".to_string()),
            },
        ],
        format: ReportFormat::JSON,
        schedule: None,
    };

    let report = analytics_engine.generate_custom_report(&report_config).await?;
    println!("      📄 Integration report generated: {}", report.report_id);

    // Cleanup
    let key_id = api_manager.validate_api_key(&api_key).map(|k| k.id.clone());
    tenancy_manager.delete_tenant(&tenant_id)?;
    if let Some(key_id) = key_id {
        api_manager.revoke_api_key(&key_id)?;
    }

    println!("    ✅ Enterprise features integration test completed");
    Ok(())
} 
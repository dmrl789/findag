use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// API version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiVersion {
    pub version: String,
    pub status: ApiVersionStatus,
    pub release_date: DateTime<Utc>,
    pub deprecation_date: Option<DateTime<Utc>>,
    pub sunset_date: Option<DateTime<Utc>>,
    pub changelog: Vec<ChangeLogEntry>,
    pub breaking_changes: Vec<BreakingChange>,
    pub new_features: Vec<String>,
    pub bug_fixes: Vec<String>,
}

/// API version status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApiVersionStatus {
    Alpha,
    Beta,
    Stable,
    Deprecated,
    Sunset,
}

/// Change log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeLogEntry {
    pub version: String,
    pub date: DateTime<Utc>,
    pub changes: Vec<String>,
    pub author: String,
    pub type_: ChangeType,
}

/// Change type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Feature,
    BugFix,
    Breaking,
    Security,
    Performance,
    Documentation,
}

/// Breaking change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakingChange {
    pub description: String,
    pub affected_endpoints: Vec<String>,
    pub migration_guide: String,
    pub severity: BreakingChangeSeverity,
}

/// Breaking change severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BreakingChangeSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// API endpoint documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpoint {
    pub path: String,
    pub method: HttpMethod,
    pub summary: String,
    pub description: String,
    pub parameters: Vec<ApiParameter>,
    pub request_body: Option<ApiRequestBody>,
    pub responses: Vec<ApiResponse>,
    pub examples: Vec<ApiExample>,
    pub rate_limits: Option<RateLimit>,
    pub authentication: AuthenticationRequirement,
    pub deprecated: bool,
    pub version: String,
}

/// HTTP method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

/// API parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiParameter {
    pub name: String,
    pub location: ParameterLocation,
    pub required: bool,
    pub data_type: String,
    pub description: String,
    pub default_value: Option<String>,
    pub allowed_values: Option<Vec<String>>,
    pub validation_rules: Vec<ValidationRule>,
}

/// Parameter location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterLocation {
    Path,
    Query,
    Header,
    Cookie,
}

/// Validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_type: ValidationRuleType,
    pub value: String,
    pub message: String,
}

/// Validation rule type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    MinLength,
    MaxLength,
    Pattern,
    MinValue,
    MaxValue,
    Required,
    Email,
    Url,
    Custom,
}

/// API request body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequestBody {
    pub content_type: String,
    pub schema: ApiSchema,
    pub required: bool,
    pub description: String,
}

/// API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status_code: u16,
    pub description: String,
    pub content_type: String,
    pub schema: Option<ApiSchema>,
    pub headers: HashMap<String, String>,
    pub examples: Vec<ApiExample>,
}

/// API schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSchema {
    pub schema_type: SchemaType,
    pub properties: HashMap<String, ApiSchema>,
    pub required: Vec<String>,
    pub items: Option<Box<ApiSchema>>,
    pub reference: Option<String>,
    pub description: String,
}

/// Schema type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchemaType {
    Object,
    Array,
    String,
    Number,
    Integer,
    Boolean,
    Null,
    Reference,
}

/// API example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiExample {
    pub name: String,
    pub summary: String,
    pub description: String,
    pub value: serde_json::Value,
    pub content_type: String,
}

/// Rate limit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_minute: u64,
    pub requests_per_hour: u64,
    pub requests_per_day: u64,
    pub burst_limit: u64,
}

/// Authentication requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationRequirement {
    pub required: bool,
    pub auth_types: Vec<AuthType>,
    pub scopes: Vec<String>,
}

/// Authentication type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthType {
    Bearer,
    ApiKey,
    Basic,
    OAuth2,
    Custom,
}

/// Developer portal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperPortal {
    pub title: String,
    pub description: String,
    pub version: String,
    pub contact: ContactInfo,
    pub license: LicenseInfo,
    pub servers: Vec<ServerInfo>,
    pub tags: Vec<Tag>,
    pub external_docs: Vec<ExternalDoc>,
    pub security_schemes: HashMap<String, SecurityScheme>,
}

/// Contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub name: String,
    pub email: String,
    pub url: Option<String>,
}

/// License information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub name: String,
    pub url: Option<String>,
}

/// Server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub url: String,
    pub description: String,
    pub variables: HashMap<String, ServerVariable>,
}

/// Server variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerVariable {
    pub default: String,
    pub description: String,
    pub enum_values: Option<Vec<String>>,
}

/// Tag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub description: String,
    pub external_docs: Option<ExternalDoc>,
}

/// External documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDoc {
    pub description: String,
    pub url: String,
}

/// Security scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScheme {
    pub scheme_type: SecuritySchemeType,
    pub description: String,
    pub name: Option<String>,
    pub location: Option<String>,
    pub flows: Option<OAuthFlows>,
}

/// Security scheme type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySchemeType {
    ApiKey,
    Http,
    OAuth2,
    OpenIdConnect,
}

/// OAuth flows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlows {
    pub implicit: Option<OAuthFlow>,
    pub password: Option<OAuthFlow>,
    pub client_credentials: Option<OAuthFlow>,
    pub authorization_code: Option<OAuthFlow>,
}

/// OAuth flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlow {
    pub authorization_url: String,
    pub token_url: String,
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

/// API key management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub name: String,
    pub key: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub permissions: Vec<String>,
    pub rate_limits: RateLimit,
    pub status: ApiKeyStatus,
}

/// API key status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApiKeyStatus {
    Active,
    Inactive,
    Expired,
    Revoked,
}

/// API usage analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiUsageAnalytics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub requests_by_endpoint: HashMap<String, u64>,
    pub requests_by_user: HashMap<String, u64>,
    pub requests_by_version: HashMap<String, u64>,
    pub error_rates: HashMap<String, f64>,
    pub peak_usage_times: Vec<PeakUsageTime>,
}

/// Peak usage time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeakUsageTime {
    pub hour: u8,
    pub request_count: u64,
    pub average_response_time: f64,
}

/// API management system
pub struct ApiManagementSystem {
    versions: HashMap<String, ApiVersion>,
    endpoints: HashMap<String, ApiEndpoint>,
    portal_config: DeveloperPortal,
    api_keys: HashMap<String, ApiKey>,
    usage_analytics: ApiUsageAnalytics,
    documentation: ApiDocumentation,
}

/// API documentation
pub struct ApiDocumentation {
    pub openapi_spec: serde_json::Value,
    pub markdown_docs: HashMap<String, String>,
    pub code_examples: HashMap<String, Vec<CodeExample>>,
    pub tutorials: Vec<Tutorial>,
}

/// Code example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    pub language: String,
    pub title: String,
    pub description: String,
    pub code: String,
    pub output: Option<String>,
}

/// Tutorial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tutorial {
    pub id: String,
    pub title: String,
    pub description: String,
    pub steps: Vec<TutorialStep>,
    pub difficulty: TutorialDifficulty,
    pub estimated_time: u32, // minutes
}

/// Tutorial step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialStep {
    pub title: String,
    pub description: String,
    pub code_example: Option<CodeExample>,
    pub expected_result: String,
}

/// Tutorial difficulty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TutorialDifficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl ApiManagementSystem {
    pub fn new() -> Self {
        let portal_config = DeveloperPortal {
            title: "FinDAG API".to_string(),
            description: "High-performance financial DAG API for real-time transactions".to_string(),
            version: "1.0.0".to_string(),
            contact: ContactInfo {
                name: "FinDAG Support".to_string(),
                email: "support@findag.com".to_string(),
                url: Some("https://findag.com/support".to_string()),
            },
            license: LicenseInfo {
                name: "MIT".to_string(),
                url: Some("https://opensource.org/licenses/MIT".to_string()),
            },
            servers: vec![
                ServerInfo {
                    url: "https://api.findag.com/v1".to_string(),
                    description: "Production server".to_string(),
                    variables: HashMap::new(),
                },
                ServerInfo {
                    url: "https://staging-api.findag.com/v1".to_string(),
                    description: "Staging server".to_string(),
                    variables: HashMap::new(),
                },
            ],
            tags: vec![
                Tag {
                    name: "Transactions".to_string(),
                    description: "Transaction management endpoints".to_string(),
                    external_docs: None,
                },
                Tag {
                    name: "Assets".to_string(),
                    description: "Asset management endpoints".to_string(),
                    external_docs: None,
                },
                Tag {
                    name: "Governance".to_string(),
                    description: "Governance and voting endpoints".to_string(),
                    external_docs: None,
                },
            ],
            external_docs: vec![
                ExternalDoc {
                    description: "FinDAG Documentation".to_string(),
                    url: "https://docs.findag.com".to_string(),
                },
            ],
            security_schemes: HashMap::new(),
        };

        Self {
            versions: HashMap::new(),
            endpoints: HashMap::new(),
            portal_config,
            api_keys: HashMap::new(),
            usage_analytics: ApiUsageAnalytics {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time: 0.0,
                requests_by_endpoint: HashMap::new(),
                requests_by_user: HashMap::new(),
                requests_by_version: HashMap::new(),
                error_rates: HashMap::new(),
                peak_usage_times: Vec::new(),
            },
            documentation: ApiDocumentation {
                openapi_spec: serde_json::json!({}),
                markdown_docs: HashMap::new(),
                code_examples: HashMap::new(),
                tutorials: Vec::new(),
            },
        }
    }

    /// Register API version
    pub fn register_version(&mut self, version: ApiVersion) -> Result<(), String> {
        let version_key = version.version.clone();
        if self.versions.contains_key(&version_key) {
            return Err("Version already exists".to_string());
        }
        
        self.versions.insert(version_key, version);
        Ok(())
    }

    /// Get API version
    pub fn get_version(&self, version: &str) -> Option<&ApiVersion> {
        self.versions.get(version)
    }

    /// List all versions
    pub fn list_versions(&self) -> Vec<&ApiVersion> {
        self.versions.values().collect()
    }

    /// Register API endpoint
    pub fn register_endpoint(&mut self, endpoint: ApiEndpoint) -> Result<(), String> {
        let endpoint_key = format!("{}:{}", endpoint.method.to_string(), endpoint.path);
        if self.endpoints.contains_key(&endpoint_key) {
            return Err("Endpoint already exists".to_string());
        }
        
        self.endpoints.insert(endpoint_key, endpoint);
        Ok(())
    }

    /// Get API endpoint
    pub fn get_endpoint(&self, method: &HttpMethod, path: &str) -> Option<&ApiEndpoint> {
        let endpoint_key = format!("{}:{}", method.to_string(), path);
        self.endpoints.get(&endpoint_key)
    }

    /// List all endpoints
    pub fn list_endpoints(&self) -> Vec<&ApiEndpoint> {
        self.endpoints.values().collect()
    }

    /// Generate API key
    pub fn generate_api_key(&mut self, user_id: &str, name: &str, permissions: Vec<String>) -> Result<String, String> {
        let key_id = Uuid::new_v4().to_string();
        let api_key = format!("fdg_{}", Uuid::new_v4().to_string().replace("-", ""));
        
        let key = ApiKey {
            id: key_id.clone(),
            name: name.to_string(),
            key: api_key.clone(),
            user_id: user_id.to_string(),
            created_at: Utc::now(),
            last_used: None,
            expires_at: None,
            permissions,
            rate_limits: RateLimit {
                requests_per_minute: 1000,
                requests_per_hour: 50000,
                requests_per_day: 1000000,
                burst_limit: 100,
            },
            status: ApiKeyStatus::Active,
        };
        
        self.api_keys.insert(key_id, key);
        Ok(api_key)
    }

    /// Validate API key
    pub fn validate_api_key(&self, key: &str) -> Option<&ApiKey> {
        self.api_keys.values().find(|k| k.key == key && k.status == ApiKeyStatus::Active)
    }

    /// Revoke API key
    pub fn revoke_api_key(&mut self, key_id: &str) -> Result<(), String> {
        if let Some(key) = self.api_keys.get_mut(key_id) {
            key.status = ApiKeyStatus::Revoked;
            Ok(())
        } else {
            Err("API key not found".to_string())
        }
    }

    /// Track API usage
    pub fn track_usage(&mut self, endpoint: &str, user_id: &str, version: &str, response_time: f64, success: bool) {
        self.usage_analytics.total_requests += 1;
        
        if success {
            self.usage_analytics.successful_requests += 1;
        } else {
            self.usage_analytics.failed_requests += 1;
        }
        
        // Update average response time
        let total_time = self.usage_analytics.average_response_time * (self.usage_analytics.total_requests - 1) as f64 + response_time;
        self.usage_analytics.average_response_time = total_time / self.usage_analytics.total_requests as f64;
        
        // Update endpoint usage
        *self.usage_analytics.requests_by_endpoint.entry(endpoint.to_string()).or_insert(0) += 1;
        
        // Update user usage
        *self.usage_analytics.requests_by_user.entry(user_id.to_string()).or_insert(0) += 1;
        
        // Update version usage
        *self.usage_analytics.requests_by_version.entry(version.to_string()).or_insert(0) += 1;
        
        // Update error rates
        if !success {
            let current_errors = self.usage_analytics.error_rates.get(endpoint).unwrap_or(&0.0);
            let total_requests = self.usage_analytics.requests_by_endpoint.get(endpoint).unwrap_or(&0);
            let new_error_rate = (*current_errors * (*total_requests - 1) as f64 + 1.0) / *total_requests as f64;
            self.usage_analytics.error_rates.insert(endpoint.to_string(), new_error_rate);
        }
    }

    /// Get usage analytics
    pub fn get_usage_analytics(&self) -> &ApiUsageAnalytics {
        &self.usage_analytics
    }

    /// Generate OpenAPI specification
    pub fn generate_openapi_spec(&self) -> serde_json::Value {
        let mut spec = serde_json::json!({
            "openapi": "3.0.0",
            "info": {
                "title": self.portal_config.title,
                "description": self.portal_config.description,
                "version": self.portal_config.version,
                "contact": {
                    "name": self.portal_config.contact.name,
                    "email": self.portal_config.contact.email,
                    "url": self.portal_config.contact.url
                },
                "license": {
                    "name": self.portal_config.license.name,
                    "url": self.portal_config.license.url
                }
            },
            "servers": self.portal_config.servers.iter().map(|s| {
                serde_json::json!({
                    "url": s.url,
                    "description": s.description
                })
            }).collect::<Vec<_>>(),
            "paths": {},
            "components": {
                "securitySchemes": {}
            }
        });

        // Add paths
        for endpoint in self.endpoints.values() {
            let path_spec = serde_json::json!({
                "summary": endpoint.summary,
                "description": endpoint.description,
                "parameters": endpoint.parameters.iter().map(|p| {
                    serde_json::json!({
                        "name": p.name,
                        "in": match p.location {
                            ParameterLocation::Path => "path",
                            ParameterLocation::Query => "query",
                            ParameterLocation::Header => "header",
                            ParameterLocation::Cookie => "cookie",
                        },
                        "required": p.required,
                        "schema": {
                            "type": p.data_type
                        },
                        "description": p.description
                    })
                }).collect::<Vec<_>>(),
                "responses": endpoint.responses.iter().map(|r| {
                    (r.status_code.to_string(), serde_json::json!({
                        "description": r.description,
                        "content": {
                            r.content_type.clone(): {
                                "schema": r.schema.as_ref().map(|s| self.schema_to_json(s))
                            }
                        }
                    }))
                }).collect::<HashMap<_, _>>()
            });

            let method = match endpoint.method {
                HttpMethod::GET => "get",
                HttpMethod::POST => "post",
                HttpMethod::PUT => "put",
                HttpMethod::DELETE => "delete",
                HttpMethod::PATCH => "patch",
                HttpMethod::HEAD => "head",
                HttpMethod::OPTIONS => "options",
            };

            spec["paths"][&endpoint.path][method] = path_spec;
        }

        spec
    }

    /// Add code example
    pub fn add_code_example(&mut self, endpoint: &str, example: CodeExample) {
        self.documentation.code_examples
            .entry(endpoint.to_string())
            .or_insert_with(Vec::new)
            .push(example);
    }

    /// Add tutorial
    pub fn add_tutorial(&mut self, tutorial: Tutorial) {
        self.documentation.tutorials.push(tutorial);
    }

    /// Get tutorials
    pub fn get_tutorials(&self) -> &Vec<Tutorial> {
        &self.documentation.tutorials
    }

    // Private helper methods
    fn schema_to_json(&self, schema: &ApiSchema) -> serde_json::Value {
        match &schema.schema_type {
            SchemaType::Object => {
                let mut obj = serde_json::json!({
                    "type": "object",
                    "description": schema.description
                });
                
                if !schema.properties.is_empty() {
                    obj["properties"] = serde_json::json!(
                        schema.properties.iter().map(|(k, v)| (k, self.schema_to_json(v))).collect::<HashMap<_, _>>()
                    );
                }
                
                if !schema.required.is_empty() {
                    obj["required"] = serde_json::json!(schema.required);
                }
                
                obj
            },
            SchemaType::Array => {
                serde_json::json!({
                    "type": "array",
                    "items": schema.items.as_ref().map(|s| self.schema_to_json(s)),
                    "description": schema.description
                })
            },
            SchemaType::String => serde_json::json!({
                "type": "string",
                "description": schema.description
            }),
            SchemaType::Number => serde_json::json!({
                "type": "number",
                "description": schema.description
            }),
            SchemaType::Integer => serde_json::json!({
                "type": "integer",
                "description": schema.description
            }),
            SchemaType::Boolean => serde_json::json!({
                "type": "boolean",
                "description": schema.description
            }),
            SchemaType::Null => serde_json::json!({
                "type": "null",
                "description": schema.description
            }),
            SchemaType::Reference => serde_json::json!({
                "$ref": schema.reference.as_ref().unwrap_or(&"#/components/schemas/Unknown".to_string())
            }),
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::HEAD => write!(f, "HEAD"),
            HttpMethod::OPTIONS => write!(f, "OPTIONS"),
        }
    }
} 
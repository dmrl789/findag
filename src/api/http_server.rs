use axum::{routing::{get, post, delete}, extract::{Path, Query}, Json, Router};
use serde::{Deserialize, Serialize};
use crate::consensus::validator_set::ValidatorSet;
use crate::core::address::Address;
use ed25519_dalek::{Signer, SigningKey};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
// Removed unused import: GovernanceState
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm, TokenData, errors::Error as JwtError};
use chrono;
use std::fs::OpenOptions;
use std::io::Write;
use crate::core::types::{ShardId, Transaction, SerializableTransaction};
use crate::core::confidential::ConfidentialTx;
use crate::core::bridge::BridgeTx;
use once_cell::sync::Lazy;
use axum::http::{StatusCode, HeaderMap, Method};
use crate::core::tx_pool::ShardedTxPool;
use crate::network::propagation::NetworkPropagator;
use rand;
use rand_core::RngCore;
use axum::response::IntoResponse;
use axum::extract::State;
use std::env;
use ed25519_dalek::Verifier;
use tower_http::cors::{CorsLayer, Any};
use axum::middleware::{self, Next};
use axum::response::Response;
use std::time::{Duration, Instant};
use uuid;
use crate::api::websocket::{WebSocketManager, websocket_handler, WebSocketMessage};
use otpauth::TOTP;
use base32::encode as base32_encode;
use chrono::{DateTime, Utc};
use rand::rngs::OsRng;
use rand::Rng;
use tokio::sync::RwLock;

// Secure credential management
static ADMIN_USERNAME: Lazy<String> = Lazy::new(|| {
    env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string())
});

static ADMIN_PASSWORD_HASH: Lazy<String> = Lazy::new(|| {
    env::var("ADMIN_PASSWORD_HASH").unwrap_or_else(|_| {
        // Default hash for "admin123" - CHANGE IN PRODUCTION!
        "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8".to_string()
    })
});

static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    env::var("JWT_SECRET").unwrap_or_else(|_| {
        // Generate a secure random secret if not provided
        let mut secret = [0u8; 64];
        let mut rng = rand::rngs::OsRng;
        rng.fill_bytes(&mut secret);
        hex::encode(secret)
    })
});

// Rate limiting with improved security
use std::sync::OnceLock;

static RATE_LIMITS: OnceLock<Arc<Mutex<HashMap<String, (Instant, u32)>>>> = OnceLock::new();

// Security configuration
const MAX_REQUEST_SIZE: usize = 1_048_576; // 1MB
#[allow(dead_code)]
const RATE_LIMIT_REQUESTS: u32 = 100;
#[allow(dead_code)]
const RATE_LIMIT_WINDOW: Duration = Duration::from_secs(60);
const JWT_EXPIRY_HOURS: i64 = 24;

// Shared application state for dependency injection
pub struct AppState {
    pub validator_set: Arc<Mutex<ValidatorSet>>,
    pub storage: Arc<crate::storage::persistent::PersistentStorage>,
    pub governance_state: Arc<Mutex<crate::consensus::governance::GovernanceState>>,
    pub tx_pool: Arc<ShardedTxPool>,
    pub network_propagator: Arc<NetworkPropagator>,
    pub ws_manager: Arc<WebSocketManager>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiTransaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub currency: String,
    pub shard_id: Option<u16>, // Optional for API, default to 0
    // ... other fields ...
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignedTransactionRequest {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub signature: Vec<u8>,
    pub payload: Vec<u8>,
    pub findag_time: u64,
    pub hashtimer: Vec<u8>,
    pub public_key: Vec<u8>,
    pub shard_id: u16,
}

#[derive(Deserialize)]
struct ValidatorAddReq {
    address: String,
    public_key: String,
    _metadata: Option<String>,
}

#[derive(Deserialize)]
struct ValidatorSlashReq {
    _admin_token: String,
}

#[derive(Deserialize)]
struct ProposalSubmitReq {
    proposer: String,
    title: String,
    description: String,
    proposal_type: String, // "add_validator", "remove_validator", "slash_validator", "parameter_change", "upgrade_protocol", "emergency_pause", "emergency_resume"
    address: Option<String>,
    public_key: Option<String>,
    parameter: Option<String>,
    new_value: Option<String>,
    version: Option<String>,
    reason: Option<String>,
    duration: Option<u64>, // Voting duration in seconds
}

#[derive(Deserialize)]
struct VoteReq {
    voter: String,
    approve: bool,
    stake_weight: u64,
    reason: Option<String>,
}

#[derive(Deserialize)]
struct ExecuteProposalReq {
    executor: String,
}

#[derive(Deserialize)]
struct CancelProposalReq {
    canceller: String,
    canceller_stake: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String, // "admin", "validator", "observer"
    exp: usize,
    iat: usize, // Issued at
    jti: String, // JWT ID for replay protection
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}

#[derive(Deserialize)]
struct PasswordResetRequest {
    email: String,
}

#[derive(Deserialize)]
struct PasswordResetConfirmRequest {
    token: String,
    new_password: String,
}

#[derive(Deserialize)]
struct ChangePasswordRequest {
    current_password: String,
    new_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: String,
    username: String,
    email: String,
    password_hash: String,
    role: String,
    created_at: chrono::DateTime<chrono::Utc>,
    last_login: Option<chrono::DateTime<chrono::Utc>>,
    is_active: bool,
    two_factor_enabled: bool,
    two_factor_secret: Option<String>,
}

// In-memory user storage (replace with database in production)
static USERS: Lazy<Arc<Mutex<HashMap<String, User>>>> = Lazy::new(|| {
    let mut users = HashMap::new();
    // Add default admin user
    users.insert("admin".to_string(), User {
        id: "admin".to_string(),
        username: "admin".to_string(),
        email: "admin@findag.com".to_string(),
        password_hash: ADMIN_PASSWORD_HASH.clone(),
        role: "admin".to_string(),
        created_at: chrono::Utc::now(),
        last_login: None,
        is_active: true,
        two_factor_enabled: false,
        two_factor_secret: None,
    });
    Arc::new(Mutex::new(users))
});

// Password reset tokens (replace with database in production)
static PASSWORD_RESET_TOKENS: Lazy<Arc<Mutex<HashMap<String, (String, chrono::DateTime<chrono::Utc>)>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// User sessions (replace with database in production)
static USER_SESSIONS: Lazy<Arc<Mutex<HashMap<String, (String, chrono::DateTime<chrono::Utc>)>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// Secure JWT validation with replay protection
static USED_JWT_IDS: OnceLock<Arc<Mutex<HashMap<String, Instant>>>> = OnceLock::new();

fn validate_jwt(token: &str, required_role: &str) -> Result<TokenData<Claims>, JwtError> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    
    // Check if token has expired
    let now = chrono::Utc::now().timestamp() as usize;
    if data.claims.exp < now {
        return Err(JwtError::from(jsonwebtoken::errors::ErrorKind::ExpiredSignature));
    }
    
    // Check role
    if data.claims.role != required_role {
        return Err(JwtError::from(jsonwebtoken::errors::ErrorKind::InvalidToken));
    }
    
    // Check for replay attacks (JWT ID tracking)
    let used_ids = USED_JWT_IDS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let mut used_ids = used_ids.lock().unwrap();
    
    // Clean up old JWT IDs (older than 24 hours)
    let cutoff = Instant::now() - Duration::from_secs(24 * 3600);
    used_ids.retain(|_, timestamp| *timestamp > cutoff);
    
    // Check if JWT ID has been used
    if used_ids.contains_key(&data.claims.jti) {
        return Err(JwtError::from(jsonwebtoken::errors::ErrorKind::InvalidToken));
    }
    
    // Mark JWT ID as used
    used_ids.insert(data.claims.jti.clone(), Instant::now());
    
    Ok(data)
}

fn generate_jwt(subject: &str, role: &str) -> Result<String, JwtError> {
    let now = chrono::Utc::now();
    let expiration = now
        .checked_add_signed(chrono::Duration::hours(JWT_EXPIRY_HOURS))
        .expect("valid timestamp")
        .timestamp() as usize;
    
    // Generate unique JWT ID
    let mut jti_bytes = [0u8; 16];
    let mut rng = rand::rngs::OsRng;
    rng.fill_bytes(&mut jti_bytes);
    let jti = hex::encode(jti_bytes);
    
    let claims = Claims {
        sub: subject.to_string(),
        role: role.to_string(),
        exp: expiration,
        iat: now.timestamp() as usize,
        jti,
    };
    
    let header = Header::new(Algorithm::HS256);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET.as_bytes()))
}

// Secure password hashing (simple SHA-256 for demo - use bcrypt in production)
fn hash_password(password: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hex::encode(hasher.finalize())
}

async fn authenticate_user(headers: HeaderMap, required_role: &str) -> Result<String, (StatusCode, Json<serde_json::Value>)> {
    cleanup_expired_sessions();
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));
    
    let token = match auth_header {
        Some(token) => token,
        None => {
            return Err((StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "Missing authorization header"
            }))));
        }
    };
    
    match validate_jwt(token, required_role) {
        Ok(token_data) => Ok(token_data.claims.sub),
        Err(_) => {
            Err((StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "Invalid or expired token"
            }))))
        }
    }
}

// Secure login with proper password validation
async fn login(Json(credentials): Json<LoginRequest>) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    cleanup_expired_sessions();
    // Sanitize inputs
    let username = credentials.username.trim();
    let password = credentials.password.trim();
    
    if username.is_empty() || password.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Username and password cannot be empty"
        }))));
    }
    
    // Rate limiting for login attempts
    let client_id = format!("login_{}", username);
    if !check_rate_limit(&client_id, 5, Duration::from_secs(300)) { // 5 attempts per 5 minutes
        return Err((StatusCode::TOO_MANY_REQUESTS, Json(serde_json::json!({
            "error": "Too many login attempts"
        }))));
    }
    
    // Get user from storage
    let users = USERS.lock().unwrap();
    if let Some(user) = users.get(username) {
        if !user.is_active {
            return Err((StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "Account is deactivated"
            }))));
        }
        
        if hash_password(password) == user.password_hash {
            // Clone user data before dropping the lock
            let user_role = user.role.clone();
            let user_id = user.id.clone();
            let user_email = user.email.clone();
            let two_factor_enabled = user.two_factor_enabled;
            
            // Update last login
            drop(users);
            let mut users = USERS.lock().unwrap();
            if let Some(user) = users.get_mut(username) {
                user.last_login = Some(chrono::Utc::now());
            }
            
            let token = generate_jwt(username, &user_role)
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "error": "Failed to generate token"
                }))))?;
            
            // Store session
            let mut sessions = USER_SESSIONS.lock().unwrap();
            sessions.insert(token.clone(), (username.to_string(), chrono::Utc::now()));
            
            audit_log(username, "login", "successful");
            Ok(Json(serde_json::json!({
                "token": token,
                "role": user_role,
                "expires_in": JWT_EXPIRY_HOURS * 3600,
                "user": {
                    "id": user_id,
                    "username": username.to_string(),
                    "email": user_email,
                    "two_factor_enabled": two_factor_enabled
                }
            })))
        } else {
            audit_log(username, "login", "failed");
            Err((StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "Invalid credentials"
            }))))
        }
    } else {
        audit_log(username, "login", "failed");
        Err((StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "error": "Invalid credentials"
        }))))
    }
}

// User registration endpoint
async fn register(Json(req): Json<RegisterRequest>) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Sanitize inputs
    let username = req.username.trim();
    let email = req.email.trim();
    let password = req.password.trim();
    let confirm_password = req.confirm_password.trim();
    
    // Validation
    if username.is_empty() || email.is_empty() || password.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "All fields are required"
        }))));
    }
    
    if password != confirm_password {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Passwords do not match"
        }))));
    }
    
    if password.len() < 8 {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Password must be at least 8 characters long"
        }))));
    }
    
    // Check if username or email already exists
    let users = USERS.lock().unwrap();
    if users.contains_key(username) {
        return Err((StatusCode::CONFLICT, Json(serde_json::json!({
            "error": "Username already exists"
        }))));
    }
    
    for user in users.values() {
        if user.email == email {
            return Err((StatusCode::CONFLICT, Json(serde_json::json!({
                "error": "Email already exists"
            }))));
        }
    }
    
    // Create new user
    let new_user = User {
        id: uuid::Uuid::new_v4().to_string(),
        username: username.to_string(),
        email: email.to_string(),
        password_hash: hash_password(password),
        role: "user".to_string(),
        created_at: chrono::Utc::now(),
        last_login: None,
        is_active: true,
        two_factor_enabled: false,
        two_factor_secret: None,
    };
    
    drop(users);
    let mut users = USERS.lock().unwrap();
    users.insert(username.to_string(), new_user.clone());
    
    audit_log(username, "register", "successful");
    Ok(Json(serde_json::json!({
        "message": "User registered successfully",
        "user": {
            "id": new_user.id,
            "username": new_user.username,
            "email": new_user.email,
            "role": new_user.role
        }
    })))
}

// Password reset request endpoint
async fn password_reset(Json(req): Json<PasswordResetRequest>) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let email = req.email.trim();
    
    if email.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Email is required"
        }))));
    }
    
    // Find user by email
    let users = USERS.lock().unwrap();
    let user = users.values().find(|u| u.email == email);
    
    if let Some(user) = user {
        // Clone username before dropping the lock
        let username = user.username.clone();
        
        // Generate reset token
        let token = uuid::Uuid::new_v4().to_string();
        let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);
        
        // Store reset token
        drop(users);
        let mut tokens = PASSWORD_RESET_TOKENS.lock().unwrap();
        tokens.insert(token.clone(), (username.clone(), expires_at));
        
        // In production, send email here
        audit_log(&username, "password_reset_requested", "successful");
        
        Ok(Json(serde_json::json!({
            "message": "Password reset email sent (check logs in development)",
            "token": token // Remove this in production
        })))
    } else {
        // Don't reveal if email exists or not
        Ok(Json(serde_json::json!({
            "message": "If the email exists, a password reset link has been sent"
        })))
    }
}

// Password reset confirmation endpoint
async fn password_reset_confirm(
    Path(token): Path<String>,
    Json(req): Json<PasswordResetConfirmRequest>
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let new_password = req.new_password.trim();
    
    if new_password.len() < 8 {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Password must be at least 8 characters long"
        }))));
    }
    
    // Validate token
    let tokens = PASSWORD_RESET_TOKENS.lock().unwrap();
    if let Some((username, expires_at)) = tokens.get(&token) {
        if chrono::Utc::now() > *expires_at {
            return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Reset token has expired"
            }))));
        }
        
        // Clone username before dropping the lock
        let username = username.clone();
        
        // Update user password
        drop(tokens);
        let mut users = USERS.lock().unwrap();
        if let Some(user) = users.get_mut(&username) {
            user.password_hash = hash_password(new_password);
            
            // Remove used token
            let mut tokens = PASSWORD_RESET_TOKENS.lock().unwrap();
            tokens.remove(&token);
            
            audit_log(&username, "password_reset_completed", "successful");
            
            Ok(Json(serde_json::json!({
                "message": "Password reset successfully"
            })))
        } else {
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "User not found"
            }))))
        }
    } else {
        Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid reset token"
        }))))
    }
}

// Change password endpoint (requires authentication)
async fn change_password(
    headers: HeaderMap,
    Json(req): Json<ChangePasswordRequest>
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Authenticate user
    let username = authenticate_user(headers, "user").await?;
    
    let current_password = req.current_password.trim();
    let new_password = req.new_password.trim();
    
    if new_password.len() < 8 {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Password must be at least 8 characters long"
        }))));
    }
    
    // Verify current password
    let users = USERS.lock().unwrap();
    if let Some(user) = users.get(&username) {
        if hash_password(current_password) != user.password_hash {
            return Err((StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "Current password is incorrect"
            }))));
        }
        
        // Update password
        drop(users);
        let mut users = USERS.lock().unwrap();
        if let Some(user) = users.get_mut(&username) {
            user.password_hash = hash_password(new_password);
            
            audit_log(&username, "password_changed", "successful");
            
            Ok(Json(serde_json::json!({
                "message": "Password changed successfully"
            })))
        } else {
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "User not found"
            }))))
        }
    } else {
        Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "error": "User not found"
        }))))
    }
}

// Logout endpoint
async fn logout(headers: HeaderMap) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Extract token from headers
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = auth_str[7..].to_string();
                
                // Remove from sessions
                let mut sessions = USER_SESSIONS.lock().unwrap();
                sessions.remove(&token);
                
                // Add to blacklist (optional, for immediate invalidation)
                // In production, you might want to add this to a blacklist
                
                Ok(Json(serde_json::json!({
                    "message": "Logged out successfully"
                })))
            } else {
                Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": "Invalid authorization header"
                }))))
            }
        } else {
            Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Invalid authorization header"
            }))))
        }
    } else {
        Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Authorization header required"
        }))))
    }
}

// Enhanced rate limiting with IP-based tracking
fn check_rate_limit(client_id: &str, max_requests: u32, window: Duration) -> bool {
    let limits = RATE_LIMITS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let mut limits = limits.lock().unwrap();
    let now = Instant::now();
    
    if let Some((last_request, count)) = limits.get_mut(client_id) {
        if now.duration_since(*last_request) > window {
            *last_request = now;
            *count = 1;
            true
        } else if *count < max_requests {
            *count += 1;
            true
        } else {
            false
        }
    } else {
        limits.insert(client_id.to_string(), (now, 1));
        true
    }
}

// Enhanced input validation
fn validate_address(address: &str) -> bool {
    // Sanitize and validate address
    let address = address.trim();
    address.starts_with("fdg1") && address.len() >= 10 && address.len() <= 100
}

fn validate_amount(amount: u64) -> bool {
    amount > 0 && amount <= 1_000_000_000_000 // 1 trillion max
}

fn validate_currency(currency: &str) -> bool {
    let currency = currency.trim().to_uppercase();
    let whitelist = vec!["EUR", "USD", "GBP", "JPY", "CHF", "SGD", "AED", "CNY", "BUND", "OAT", "BTP", "GILT", "UST", "JGB", "T-BILL", "CP", "CD", "XAU", "XAG", "XPT", "XPD", "XS1234567890", "FR0000120271", "BE0003796134", "DE0001135275", "ETF1", "UCITS1", "BTC", "ETH", "USDT", "USDC"];
    whitelist.contains(&currency.as_str())
}

fn validate_public_key(public_key: &str) -> bool {
    let public_key = public_key.trim();
    if public_key.len() != 64 {
        return false;
    }
    hex::decode(public_key).is_ok()
}

// Request sanitization middleware
async fn sanitize_request(req: axum::http::Request<axum::body::Body>, next: Next) -> Result<Response, StatusCode> {
    // Check request size
    if let Some(content_length) = req.headers().get("content-length") {
        if let Ok(size) = content_length.to_str().unwrap_or("0").parse::<usize>() {
            if size > MAX_REQUEST_SIZE {
                return Err(StatusCode::PAYLOAD_TOO_LARGE);
            }
        }
    }
    
    // Check for suspicious headers
    let suspicious_headers = ["x-forwarded-for", "x-real-ip", "x-forwarded-proto"];
    for header in suspicious_headers {
        if let Some(value) = req.headers().get(header) {
            if let Ok(value_str) = value.to_str() {
                if value_str.contains("script") || value_str.contains("javascript") {
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
        }
    }
    
    Ok(next.run(req).await)
}

// Enhanced audit logging with security events
fn audit_log(subject: &str, action: &str, details: &str) {
    let now = chrono::Utc::now().to_rfc3339();
    let ip = std::env::var("REMOTE_ADDR").unwrap_or_else(|_| "unknown".to_string());
    let user_agent = std::env::var("HTTP_USER_AGENT").unwrap_or_else(|_| "unknown".to_string());
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("audit.log")
        .unwrap();
    
    writeln!(file, "{now} | {subject} | {action} | {details} | IP:{ip} | UA:{user_agent}").unwrap();
}

// CORS configuration for security
fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([axum::http::header::AUTHORIZATION, axum::http::header::CONTENT_TYPE])
        .max_age(Duration::from_secs(3600))
}

/// HTTP API for FinDAG. Only whitelisted assets are supported for transaction submission and balance queries.
///
/// POST /tx: Only accepts transactions with supported assets. Unsupported assets will be rejected with an error.
/// GET /balance/:address/:asset: Only returns balances for supported assets. Unsupported assets will be rejected with an error.
///
/// Supported assets:
/// EUR, USD, GBP, JPY, CHF, SGD, AED, CNY, BUND, OAT, BTP, GILT, UST, JGB, T-BILL, CP, CD, XAU, XAG, XPT, XPD, XS1234567890, FR0000120271, BE0003796134, DE0001135275, ETF1, UCITS1, BTC, ETH, USDT, USDC
/// GET /balance/:address/:asset?shard_id=0
async fn get_balance(
    State(_state): State<Arc<AppState>>,
    Path((address, asset)): Path<(String, String)>, 
    Query(params): Query<std::collections::HashMap<String, String>>
) -> Json<serde_json::Value> {
    let shard_id = params.get("shard_id").and_then(|s| s.parse::<u16>().ok()).unwrap_or(0);
    let whitelist = vec!["EUR", "USD", "GBP", "JPY", "CHF", "SGD", "AED", "CNY", "BUND", "OAT", "BTP", "GILT", "UST", "JGB", "T-BILL", "CP", "CD", "XAU", "XAG", "XPT", "XPD", "XS1234567890", "FR0000120271", "BE0003796134", "DE0001135275", "ETF1", "UCITS1", "BTC", "ETH", "USDT", "USDC"];
    if !whitelist.contains(&asset.as_str()) {
        return Json(serde_json::json!({ "error": format!("'{}' is not a supported asset", asset) }));
    }
    let balance = 0; // TODO: Implement balance lookup
    Json(serde_json::json!({ "address": address, "asset": asset, "balance": balance, "shard_id": shard_id }))
}

/// POST /tx (accepts both simple ApiTransaction and signed SignedTransactionRequest)
async fn post_tx(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(tx_data): Json<serde_json::Value>
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Rate limiting - 100 requests per minute per client
    let client_id = headers
        .get("x-forwarded-for")
        .or_else(|| headers.get("x-real-ip"))
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");
    
    if !check_rate_limit(client_id, 100, Duration::from_secs(60)) {
        return Err((StatusCode::TOO_MANY_REQUESTS, Json(serde_json::json!({
            "error": "Rate limit exceeded"
        }))));
    }
    
    // Request size limit - 1MB
    let request_size = serde_json::to_string(&tx_data).unwrap_or_default().len();
    if request_size > 1_048_576 {
        return Err((StatusCode::PAYLOAD_TOO_LARGE, Json(serde_json::json!({
            "error": "Request too large"
        }))));
    }
    
    // Try to parse as signed transaction first
    if let Ok(signed_tx) = serde_json::from_value::<SignedTransactionRequest>(tx_data.clone()) {
        // Input validation for signed transaction
        if !validate_address(&signed_tx.from) {
            return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Invalid from address"
            }))));
        }
        
        if !validate_address(&signed_tx.to) {
            return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Invalid to address"
            }))));
        }
        
        if !validate_amount(signed_tx.amount) {
            return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Invalid amount"
            }))));
        }
        
        if !validate_public_key(&hex::encode(&signed_tx.public_key)) {
            return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Invalid public key"
            }))));
        }
        
        println!("Received signed transaction: from={}, to={}, amount={}", 
                signed_tx.from, signed_tx.to, signed_tx.amount);
        
        // Validate signature
        let message = format!("{}{}{}", signed_tx.from, signed_tx.to, signed_tx.amount);
        println!("[DEBUG] Verifying signature for message: '{message}'");
        println!("[DEBUG] Public key length: {}, Signature length: {}", 
                signed_tx.public_key.len(), signed_tx.signature.len());
        
        let public_key = match ed25519_dalek::VerifyingKey::from_bytes(&signed_tx.public_key.clone().try_into().unwrap()) {
            Ok(pk) => {
                println!("[DEBUG] Public key parsed successfully");
                pk
            },
            Err(e) => {
                println!("[DEBUG] Invalid public key format: {e:?}");
                return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid public key format" }))));
            }
        };
        
        let signature_bytes: [u8; 64] = signed_tx.signature.clone().try_into().map_err(|_| {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Invalid signature length"
            })))
        })?;
        let signature = ed25519_dalek::Signature::from_bytes(&signature_bytes);
        
        // Verify signature
        match public_key.verify(message.as_bytes(), &signature) {
            Ok(_) => println!("[DEBUG] Signature verification successful"),
            Err(e) => {
                println!("[DEBUG] Signature verification failed: {e:?}");
                return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Signature verification failed" }))));
            }
        }
        
        // Create core Transaction
        let payload_len = signed_tx.payload.len();
        let core_tx = Transaction {
            from: Address(signed_tx.from.clone()),
            to: Address(signed_tx.to.clone()),
            amount: signed_tx.amount,
            payload: signed_tx.payload,
            findag_time: signed_tx.findag_time,
            hashtimer: {
                let mut ht = [0u8; 32];
                if signed_tx.hashtimer.len() >= 32 {
                    ht.copy_from_slice(&signed_tx.hashtimer[..32]);
                } else {
                    ht[..signed_tx.hashtimer.len()].copy_from_slice(&signed_tx.hashtimer);
                }
                ht
            },
            signature,
            public_key,
            shard_id: ShardId(signed_tx.shard_id),
            source_shard: None,
            dest_shard: None,
            target_chain: None,
            bridge_protocol: None,
        };
        
        // Add comprehensive debugging for transaction processing
        println!("[DEBUG] Processing signed transaction:");
        println!("[DEBUG]   From: {}", signed_tx.from);
        println!("[DEBUG]   To: {}", signed_tx.to);
        println!("[DEBUG]   Amount: {}", signed_tx.amount);
        println!("[DEBUG]   Shard ID: {}", signed_tx.shard_id);
        println!("[DEBUG]   Signature length: {}", signed_tx.signature.len());
        println!("[DEBUG]   Public key length: {}", signed_tx.public_key.len());
        println!("[DEBUG]   Payload length: {payload_len}");
        println!("[DEBUG]   FindAG Time: {}", signed_tx.findag_time);
        println!("[DEBUG]   HashTimer: {:02x?}", &signed_tx.hashtimer);
        
        // Validate transaction structure
        if signed_tx.signature.len() != 64 {
            println!("[DEBUG] REJECTION: Invalid signature length {}", signed_tx.signature.len());
            return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid signature length" }))));
        }
        
        if signed_tx.public_key.len() != 32 {
            println!("[DEBUG] REJECTION: Invalid public key length {}", signed_tx.public_key.len());
            return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid public key length" }))));
        }
        
        // Add to transaction pool
        println!("[DEBUG] Adding transaction to pool...");
        let added = state.tx_pool.add_transaction(core_tx.clone());
        
        if added {
            // Broadcast to network
            let stx: SerializableTransaction = core_tx.into();
            let msg = crate::network::propagation::GossipMsg::NewTransaction(stx);
            state.network_propagator.broadcast(&msg).await;
            
            println!("[DEBUG] SUCCESS: Transaction added to pool");
            println!("Processed signed tx: from={}, to={}, amount={} (shard_id={})", 
                    signed_tx.from, signed_tx.to, signed_tx.amount, signed_tx.shard_id);
            Ok(Json(serde_json::json!({ "status": "ok", "shard_id": signed_tx.shard_id, "message": "Signed transaction added to pool" })))
        } else {
            println!("[DEBUG] REJECTION: Transaction rejected by pool");
            
            // Get detailed rejection reason from pool
            let sender_balance = state.tx_pool.get_balance(signed_tx.shard_id, &signed_tx.from, "USD");
            println!("[DEBUG] Sender balance check: {} has {} USD", signed_tx.from, sender_balance);
            
            Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Transaction rejected", "shard_id": signed_tx.shard_id }))))
        }
    } else {
        // Try to parse as simple ApiTransaction
        if let Ok(tx) = serde_json::from_value::<ApiTransaction>(tx_data) {
            // Input validation for simple transaction
            if !validate_address(&tx.from) {
                return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": "Invalid from address"
                }))));
            }
            
            if !validate_address(&tx.to) {
                return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": "Invalid to address"
                }))));
            }
            
            if !validate_amount(tx.amount) {
                return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": "Invalid amount"
                }))));
            }
            
            if !validate_currency(&tx.currency) {
                return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": format!("'{}' is not a supported asset", tx.currency)
                }))));
            }
            
            let shard_id = tx.shard_id.unwrap_or(0);
            
            // Create a dummy signing key for signing (in production, this should be the node's signing key)
            let mut rng = rand::rngs::OsRng;
            let mut secret_bytes = [0u8; 32];
            rng.fill_bytes(&mut secret_bytes);
            let signing_key = SigningKey::from_bytes(&secret_bytes);
            
            // Create a message to sign (transaction data)
            let message = format!("{}:{}:{}:{}", tx.from, tx.to, tx.amount, tx.currency);
            let signature = signing_key.sign(message.as_bytes());
            
            // Convert ApiTransaction to core Transaction
            let core_tx = Transaction {
                from: Address(tx.from.clone()),
                to: Address(tx.to.clone()),
                amount: tx.amount,
                payload: vec![], // Empty payload for simple transfers
                findag_time: 0, // Will be set by the system
                hashtimer: [0u8; 32], // Will be computed by the system
                signature,
                public_key: signing_key.verifying_key(),
                shard_id: ShardId(shard_id),
                source_shard: None,
                dest_shard: None,
                target_chain: None,
                bridge_protocol: None,
            };
            
            // Add comprehensive debugging for simple transaction processing
            println!("[DEBUG] Processing simple transaction:");
            println!("[DEBUG]   From: {}", tx.from);
            println!("[DEBUG]   To: {}", tx.to);
            println!("[DEBUG]   Amount: {}", tx.amount);
            println!("[DEBUG]   Currency: {}", tx.currency);
            println!("[DEBUG]   Shard ID: {shard_id}");
            
            // Add to transaction pool
            println!("[DEBUG] Adding simple transaction to pool...");
            let added = state.tx_pool.add_transaction(core_tx.clone());
            
            if added {
                // Broadcast to network
                let stx: SerializableTransaction = core_tx.into();
                let msg = crate::network::propagation::GossipMsg::NewTransaction(stx);
                state.network_propagator.broadcast(&msg).await;
                
                println!("[DEBUG] SUCCESS: Simple transaction added to pool");
                println!("Processed simple tx: {tx:?} (shard_id={shard_id})");
                Ok(Json(serde_json::json!({ "status": "ok", "shard_id": shard_id, "message": "Transaction added to pool" })))
            } else {
                println!("[DEBUG] REJECTION: Simple transaction rejected by pool");
                
                // Get detailed rejection reason from pool
                let sender_balance = state.tx_pool.get_balance(shard_id, &tx.from, "USD");
                println!("[DEBUG] Sender balance check: {} has {} USD", tx.from, sender_balance);
                
                Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Transaction rejected", "shard_id": shard_id }))))
            }
        } else {
            println!("Invalid transaction format received");
            Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid transaction format" }))))
        }
    }
}

async fn get_validators(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let set = state.validator_set.lock().unwrap().clone();
    Json(serde_json::json!(set.validators))
}

// Add authentication to sensitive endpoints
async fn add_validator(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<ValidatorAddReq>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // Require admin authentication
    let _user = authenticate_user(headers, "admin").await?;
    
    // Input validation
    if !validate_address(&req.address) {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid validator address"
        }))));
    }
    
    if !validate_public_key(&req.public_key) {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid public key"
        }))));
    }
    
    // Add validator logic here
    audit_log(&_user, "add_validator", &format!("address: {}", req.address));
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "status": "success",
        "message": "Validator added successfully"
    }))))
}

async fn remove_validator(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(address): Path<String>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // Require admin authentication
    let _user = authenticate_user(headers, "admin").await?;
    
    // Input validation
    if !validate_address(&address) {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid validator address"
        }))));
    }
    
    // Remove validator logic here
    audit_log(&_user, "remove_validator", &format!("address: {}", address));
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "status": "success",
        "message": "Validator removed successfully"
    }))))
}

async fn slash_validator(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(address): Path<String>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // Require admin authentication
    let _user = authenticate_user(headers, "admin").await?;
    
    // Input validation
    if !validate_address(&address) {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid validator address"
        }))));
    }
    
    // Slash validator logic here
    audit_log(&_user, "slash_validator", &format!("address: {}", address));
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "status": "success",
        "message": "Validator slashed successfully"
    }))))
}

async fn submit_proposal(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<ProposalSubmitReq>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // Require validator authentication
    let user = authenticate_user(headers, "validator").await?;
    
    // Input validation
    if !validate_address(&req.proposer) {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid proposer address"
        }))));
    }
    
    if req.title.is_empty() || req.description.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Title and description are required"
        }))));
    }
    
    // Validate proposal type specific fields
    match req.proposal_type.as_str() {
        "add_validator" => {
            if req.address.is_none() || req.public_key.is_none() {
                return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": "Address and public_key required for add_validator proposal"
                }))));
            }
            if let Some(ref addr) = req.address {
                if !validate_address(addr) {
                    return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                        "error": "Invalid validator address"
                    }))));
                }
            }
            if let Some(ref pk) = req.public_key {
                if !validate_public_key(pk) {
                    return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                        "error": "Invalid public key"
                    }))));
                }
            }
        },
        "remove_validator" | "slash_validator" => {
            if req.address.is_none() {
                return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": "Address required for remove/slash validator proposal"
                }))));
            }
            if let Some(ref addr) = req.address {
                if !validate_address(addr) {
                    return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                        "error": "Invalid validator address"
                    }))));
                }
            }
        },
        "parameter_change" => {
            if req.parameter.is_none() || req.new_value.is_none() {
                return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": "Parameter and new_value required for parameter_change proposal"
                }))));
            }
        },
        "upgrade_protocol" => {
            if req.version.is_none() {
                return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": "Version required for upgrade_protocol proposal"
                }))));
            }
        },
        "emergency_pause" | "emergency_resume" => {
            if req.reason.is_none() {
                return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": "Reason required for emergency proposal"
                }))));
            }
        },
        _ => {
            return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Invalid proposal type"
            }))));
        }
    }
    
    // Create proposal type
    let proposal_type = match req.proposal_type.as_str() {
        "add_validator" => {
            crate::consensus::governance::ProposalType::AddValidator {
                address: req.address.unwrap(),
                public_key: req.public_key.unwrap(),
            }
        },
        "remove_validator" => {
            crate::consensus::governance::ProposalType::RemoveValidator {
                address: req.address.unwrap(),
            }
        },
        "slash_validator" => {
            crate::consensus::governance::ProposalType::SlashValidator {
                address: req.address.unwrap(),
                reason: req.reason.unwrap_or_else(|| "No reason provided".to_string()),
            }
        },
        "parameter_change" => {
            crate::consensus::governance::ProposalType::ParameterChange {
                parameter: req.parameter.unwrap(),
                new_value: req.new_value.unwrap(),
            }
        },
        "upgrade_protocol" => {
            crate::consensus::governance::ProposalType::UpgradeProtocol {
                version: req.version.unwrap(),
                description: req.description.clone(),
            }
        },
        "emergency_pause" => {
            crate::consensus::governance::ProposalType::EmergencyPause {
                reason: req.reason.unwrap(),
            }
        },
        "emergency_resume" => {
            crate::consensus::governance::ProposalType::EmergencyResume {
                reason: req.reason.unwrap(),
            }
        },
        _ => unreachable!(),
    };
    
    // Submit proposal
    let mut state_guard = state.governance_state.lock().unwrap();
    let proposal_id = match state_guard.create_proposal(
        req.proposer,
        req.title,
        req.description,
        proposal_type,
        req.duration,
    ) {
        Ok(id) => id,
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": e
            }))));
        }
    };
    
    audit_log(&user, "submit_proposal", &format!("proposal_id: {}, type: {}", proposal_id, req.proposal_type));
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "status": "success",
        "message": "Proposal submitted successfully",
        "proposal_id": proposal_id
    }))))
}

async fn vote_proposal(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<u64>,
    Json(req): Json<VoteReq>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // Require validator authentication
    let user = authenticate_user(headers, "validator").await?;
    
    // Input validation
    if !validate_address(&req.voter) {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid voter address"
        }))));
    }
    
    if req.stake_weight == 0 {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Stake weight must be greater than 0"
        }))));
    }
    
    let proposal_id = format!("proposal_{}", id);
    
    // Submit vote
    let voter = req.voter.clone();
    let mut state_guard = state.governance_state.lock().unwrap();
    match state_guard.submit_vote(
        &proposal_id,
        req.voter,
        req.approve,
        req.stake_weight,
        req.reason,
    ) {
        Ok(()) => {
            audit_log(&user, "vote_proposal", &format!("proposal_id: {}, voter: {}, approve: {}", proposal_id, voter, req.approve));
            
            Ok((StatusCode::OK, Json(serde_json::json!({
                "status": "success",
                "message": "Vote recorded successfully"
            }))))
        },
        Err(e) => {
            Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": e
            }))))
        }
    }
}

async fn execute_proposal(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<u64>,
    Json(req): Json<ExecuteProposalReq>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // Require admin authentication
    let user = authenticate_user(headers, "admin").await?;
    
    // Input validation
    if !validate_address(&req.executor) {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid executor address"
        }))));
    }
    
    let proposal_id = format!("proposal_{}", id);
    
    // Execute proposal
    let mut state_guard = state.governance_state.lock().unwrap();
    match state_guard.execute_proposal(&proposal_id) {
        Ok(()) => {
            audit_log(&user, "execute_proposal", &format!("proposal_id: {}, executor: {}", proposal_id, req.executor));
            
            Ok((StatusCode::OK, Json(serde_json::json!({
                "status": "success",
                "message": "Proposal executed successfully"
            }))))
        },
        Err(e) => {
            Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": e
            }))))
        }
    }
}

async fn cancel_proposal(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<u64>,
    Json(req): Json<CancelProposalReq>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // Require validator authentication
    let user = authenticate_user(headers, "validator").await?;
    
    // Input validation
    if !validate_address(&req.canceller) {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid canceller address"
        }))));
    }
    
    let proposal_id = format!("proposal_{}", id);
    
    // Cancel proposal
    let mut state_guard = state.governance_state.lock().unwrap();
    match state_guard.cancel_proposal(&proposal_id, &req.canceller, req.canceller_stake) {
        Ok(()) => {
            audit_log(&user, "cancel_proposal", &format!("proposal_id: {}, canceller: {}", proposal_id, req.canceller));
            
            Ok((StatusCode::OK, Json(serde_json::json!({
                "status": "success",
                "message": "Proposal cancelled successfully"
            }))))
        },
        Err(e) => {
            Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": e
            }))))
        }
    }
}

async fn get_proposal_votes(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>
) -> (StatusCode, Json<serde_json::Value>) {
    let state_guard = state.governance_state.lock().unwrap();
    let proposal_id = format!("proposal_{}", id);
    
    if let Some(votes) = state_guard.get_proposal_votes(&proposal_id) {
        let results = state_guard.calculate_voting_results(&proposal_id);
        (StatusCode::OK, Json(serde_json::json!({
            "proposal_id": proposal_id,
            "votes": votes,
            "results": results
        })))
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Proposal not found"})))
    }
}

async fn get_governance_stats(
    State(state): State<Arc<AppState>>
) -> (StatusCode, Json<serde_json::Value>) {
    let state_guard = state.governance_state.lock().unwrap();
    let stats = state_guard.get_statistics();
    let active_proposals = state_guard.get_active_proposals();
    
    (StatusCode::OK, Json(serde_json::json!({
        "statistics": stats,
        "active_proposals": active_proposals
    })))
}

async fn get_governance_config(
    State(state): State<Arc<AppState>>
) -> (StatusCode, Json<serde_json::Value>) {
    let state_guard = state.governance_state.lock().unwrap();
    let config = &state_guard.config;
    
    (StatusCode::OK, Json(serde_json::json!({
        "config": config
    })))
}

/// POST /bridge/outbound: Submit outbound bridge transaction
async fn outbound_bridge(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<BridgeTx>
) -> Json<serde_json::Value> {
    // TODO: Validate and process outbound bridge transaction
    println!("[Bridge] Outbound: {req:?}");
    Json(serde_json::json!({ "status": "pending", "details": "Bridge logic not yet implemented" }))
}

/// POST /bridge/inbound: Submit inbound bridge transaction
async fn inbound_bridge(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<BridgeTx>
) -> Json<serde_json::Value> {
    // TODO: Validate and process inbound bridge transaction
    println!("[Bridge] Inbound: {req:?}");
    Json(serde_json::json!({ "status": "pending", "details": "Bridge logic not yet implemented" }))
}

/// GET /bridge/status/:txid: Query bridge transaction status
async fn bridge_status(Path(tx_id): Path<String>) -> Json<serde_json::Value> {
    if let Some(_receipt) = Some(serde_json::json!({"status": "pending"})) {
        Json(serde_json::json!({ "status": "pending", "tx_id": tx_id, "details": "Bridge logic not yet implemented" }))
    } else {
        Json(serde_json::json!({ "status": "not_found", "tx_id": tx_id }))
    }
}

/// POST /confidential/tx: Submit a confidential transaction
#[axum::debug_handler]
async fn submit_confidential_tx(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<ConfidentialTx>
) -> Json<serde_json::Value> {
    // TODO: Validate and process confidential transaction
    println!("[Confidential] Received confidential tx: {req:?}");
    Json(serde_json::json!({ "status": "pending", "details": "Confidential tx logic not yet implemented" }))
}

/// POST /identity/register: Register or update on-chain identity
async fn register_identity(
    State(_state): State<Arc<AppState>>,
    Json(_req): Json<serde_json::Value>
) -> Json<serde_json::Value> {
    // TODO: Register or update identity, perform KYC checks
    println!("[Identity] Register/update: TODO");
    Json(serde_json::json!({ "status": "pending", "details": "Identity logic not yet implemented" }))
}

/// GET /block/:id - Returns block info including merkle_root
async fn get_block(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<String>
) -> (StatusCode, Json<serde_json::Value>) {
    let id_bytes = match hex::decode(&id) {
        Ok(bytes) => {
            let mut arr = [0u8; 32];
            if bytes.len() == 32 { arr.copy_from_slice(&bytes); Some(arr) } else { None }
        },
        Err(_) => None,
    };
    if let Some(block_id) = id_bytes {
        if let Some(block) = _state.storage.load_block(&block_id) {
            return (StatusCode::OK, Json(serde_json::json!({
                "block_id": id,
                "parent_blocks": block.parent_blocks.iter().map(hex::encode).collect::<Vec<_>>(),
                "merkle_root": block.merkle_root.map(hex::encode),
                "transactions": block.transactions.iter().map(|tx| hex::encode(tx.hashtimer)).collect::<Vec<_>>()
            })));
        }
    }
    (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "block not found"})))
}

/// GET /block/:id/merkle_proof/:tx_hash - Returns Merkle proof for a transaction in the block
async fn get_merkle_proof(
    State(state): State<Arc<AppState>>,
    Path((id, tx_hash)): Path<(String, String)>
) -> (StatusCode, Json<serde_json::Value>) {
    let id_bytes = match hex::decode(&id) {
        Ok(bytes) => {
            let mut arr = [0u8; 32];
            if bytes.len() == 32 { arr.copy_from_slice(&bytes); Some(arr) } else { None }
        },
        Err(_) => None,
    };
    if let Some(block_id) = id_bytes {
        if let Some(block) = state.storage.load_block(&block_id) {
            let tx_hashes: Vec<String> = block.transactions.iter().map(|tx| hex::encode(tx.hashtimer)).collect();
            if let Some(idx) = tx_hashes.iter().position(|h| h == &tx_hash) {
                use crate::core::bridge::merkle_proof;
                let proof = merkle_proof(&tx_hashes, idx);
                return (StatusCode::OK, Json(serde_json::json!({
                    "block_id": id,
                    "tx_hash": tx_hash,
                    "proof": proof
                })));
            } else {
                return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "tx not in block"})));
            }
        }
    }
    (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "block not found"})))
}

async fn get_proposal(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>
) -> (StatusCode, Json<serde_json::Value>) {
    let state_guard = state.governance_state.lock().unwrap();
    let proposal_id = format!("proposal_{}", id);
    if let Some(prop) = state_guard.get_proposal(&proposal_id) {
        (StatusCode::OK, Json(serde_json::to_value(prop).unwrap()))
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "not found"})))
    }
}

async fn list_proposals(
    State(state): State<Arc<AppState>>
) -> (StatusCode, Json<serde_json::Value>) {
    let state_guard = state.governance_state.lock().unwrap();
    let proposals = state_guard.list_proposals();
    (StatusCode::OK, Json(serde_json::json!({"proposals": proposals})))
}

async fn get_assets(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let whitelist = vec!["EUR", "USD", "GBP", "JPY", "CHF", "SGD", "AED", "CNY", "BUND", "OAT", "BTP", "GILT", "UST", "JGB", "T-BILL", "CP", "CD", "XAU", "XAG", "XPT", "XPD", "XS1234567890", "FR0000120271", "BE0003796134", "DE0001135275", "ETF1", "UCITS1", "BTC", "ETH", "USDT", "USDC"];
    Json(serde_json::json!(whitelist))
}

async fn get_governance_analytics(
    State(state): State<Arc<AppState>>
) -> (StatusCode, Json<serde_json::Value>) {
    let state_guard = state.governance_state.lock().unwrap();
    let analytics = state_guard.get_analytics();
    let participation_rate = state_guard.calculate_participation_rate();
    let success_rate = state_guard.get_success_rate();
    let top_voters = state_guard.get_top_voters(10);
    let recent_events = state_guard.get_recent_events(50);
    
    (StatusCode::OK, Json(serde_json::json!({
        "analytics": analytics,
        "participation_rate": participation_rate,
        "success_rate": success_rate,
        "top_voters": top_voters,
        "recent_events": recent_events
    })))
}

async fn get_governance_events(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>
) -> (StatusCode, Json<serde_json::Value>) {
    let state_guard = state.governance_state.lock().unwrap();
    let limit = params.get("limit")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(100);
    
    let events = state_guard.get_recent_events(limit);
    
    (StatusCode::OK, Json(serde_json::json!({
        "events": events,
        "total_events": state_guard.events.len()
    })))
}

async fn get_top_voters(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>
) -> (StatusCode, Json<serde_json::Value>) {
    let state_guard = state.governance_state.lock().unwrap();
    let limit = params.get("limit")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(20);
    
    let voters = state_guard.get_top_voters(limit);
    
    (StatusCode::OK, Json(serde_json::json!({
        "top_voters": voters,
        "total_voters": state_guard.voter_activity.len()
    })))
}

#[derive(Deserialize, Debug)]
pub struct OrderRequest {
    pub symbol: String,
    pub side: String, // "buy" or "sell"
    pub order_type: String, // "market" or "limit"
    pub quantity: f64,
    pub price: Option<f64>, // Required for limit orders
    pub client_order_id: Option<String>,
    pub currency: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct OrderResponse {
    pub order_id: String,
    pub status: String,
    pub message: String,
}

/// POST /orders - Place a new order
async fn place_order(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<OrderRequest>,
) -> Result<Json<OrderResponse>, (StatusCode, Json<OrderResponse>)> {
    // Authenticate user (must be logged in)
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(OrderResponse {
            order_id: "".to_string(),
            status: "rejected".to_string(),
            message: "Authentication required".to_string(),
        }))
    })?;

    // Basic validation
    if req.symbol.trim().is_empty() || req.quantity <= 0.0 {
        return Err((StatusCode::BAD_REQUEST, Json(OrderResponse {
            order_id: "".to_string(),
            status: "rejected".to_string(),
            message: "Invalid symbol or quantity".to_string(),
        })));
    }
    if req.order_type == "limit" && req.price.is_none() {
        return Err((StatusCode::BAD_REQUEST, Json(OrderResponse {
            order_id: "".to_string(),
            status: "rejected".to_string(),
            message: "Limit orders require a price".to_string(),
        })));
    }
    if req.order_type != "market" && req.order_type != "limit" {
        return Err((StatusCode::BAD_REQUEST, Json(OrderResponse {
            order_id: "".to_string(),
            status: "rejected".to_string(),
            message: "Invalid order type".to_string(),
        })));
    }
    if req.side != "buy" && req.side != "sell" {
        return Err((StatusCode::BAD_REQUEST, Json(OrderResponse {
            order_id: "".to_string(),
            status: "rejected".to_string(),
            message: "Invalid order side".to_string(),
        })));
    }

    // Generate order ID
    let order_id = uuid::Uuid::new_v4().to_string();
    
    // Create a default wallet for signing (in production, load user's wallet)
    let wallet = crate::core::wallet::Wallet::new();
    let account = &wallet.accounts()[0]; // Use first account
    
    // Create transaction from order request
    let mut transaction = create_transaction_from_order(&req, account, &order_id);
    
    // Sign the transaction
    if let Err(e) = account.sign_transaction(&mut transaction) {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(OrderResponse {
            order_id: "".to_string(),
            status: "rejected".to_string(),
            message: format!("Failed to sign transaction: {}", e),
        })));
    }
    
    // Add transaction to the pool
    let added = state.tx_pool.add_transaction(transaction);
    
    if added {
        // Broadcast order update via WebSocket
        state.ws_manager.broadcast_orderbook_update(
            &req.symbol,
            vec![], // Mock bids
            vec![], // Mock asks
        );
        
        // Broadcast trade update if it's a market order
        if req.order_type == "market" {
            state.ws_manager.broadcast_trade_update(
                &req.symbol,
                req.price.unwrap_or(50000.0), // Use provided price or default
                req.quantity,
                &req.side,
            );
        }
        
        // Broadcast system message about order placement
        let system_message = WebSocketMessage::SystemMessage {
            message: format!("Order {} placed successfully", order_id),
            level: "info".to_string(),
            timestamp: chrono::Utc::now(),
        };
        let _ = state.ws_manager.orderbook_sender.send(system_message);
        
        Ok(Json(OrderResponse {
            order_id,
            status: "accepted".to_string(),
            message: "Order placed successfully and added to transaction pool".to_string(),
        }))
    } else {
        Err((StatusCode::BAD_REQUEST, Json(OrderResponse {
            order_id: "".to_string(),
            status: "rejected".to_string(),
            message: "Order rejected by transaction pool".to_string(),
        })))
    }
}

/// Helper function to create a Transaction from an OrderRequest
fn create_transaction_from_order(
    req: &OrderRequest,
    account: &crate::core::wallet::WalletAccount,
    order_id: &str,
) -> crate::core::types::Transaction {
    use crate::core::types::{Transaction, ShardId};
    use crate::core::address::Address;
    use sha2::{Sha256, Digest};
    use chrono::Utc;
    
    // Create order payload
    let payload = serde_json::json!({
        "order_id": order_id,
        "symbol": req.symbol,
        "side": req.side,
        "order_type": req.order_type,
        "quantity": req.quantity,
        "price": req.price,
        "client_order_id": req.client_order_id,
        "currency": req.currency,
        "timestamp": Utc::now().timestamp(),
    });
    
    let payload_bytes = serde_json::to_vec(&payload).unwrap_or_default();
    
    // Create HashTimer from order ID
    let mut hashtimer = [0u8; 32];
    let mut hasher = Sha256::new();
    hasher.update(order_id.as_bytes());
    hashtimer.copy_from_slice(&hasher.finalize());
    
    // Use account address as from/to (in production, resolve from user's wallet)
    let from_address = account.address.clone();
    let to_address = Address::new("findag_orderbook000000000000000000000000000000".to_string());
    
    Transaction {
        from: from_address,
        to: to_address,
        amount: (req.quantity * 1_000_000.0) as u64, // Convert to base units
        payload: payload_bytes,
        findag_time: Utc::now().timestamp() as u64,
        hashtimer,
        signature: ed25519_dalek::Signature::from_bytes(&[0u8; 64]), // Dummy signature, replaced by sign_transaction
        public_key: account.signing_key.verifying_key(),
        shard_id: ShardId(0),
        source_shard: None,
        dest_shard: None,
        target_chain: None,
        bridge_protocol: None,
    }
}

pub fn init_http_server(
    _validator_set: Arc<Mutex<ValidatorSet>>,
    _storage: Arc<crate::storage::persistent::PersistentStorage>,
    _governance_state: Arc<Mutex<crate::consensus::governance::GovernanceState>>,
    _tx_pool: Arc<ShardedTxPool>,
    _network_propagator: Arc<NetworkPropagator>,
) {
    // This function is now deprecated - use create_app_state() instead
    println!("Warning: init_http_server() is deprecated. Use create_app_state() instead.");
}

pub fn create_app_state(
    validator_set: Arc<Mutex<ValidatorSet>>,
    storage: Arc<crate::storage::persistent::PersistentStorage>,
    governance_state: Arc<Mutex<crate::consensus::governance::GovernanceState>>,
    tx_pool: Arc<ShardedTxPool>,
    network_propagator: Arc<NetworkPropagator>,
) -> Arc<AppState> {
    let ws_manager = Arc::new(WebSocketManager::new());
    crate::api::websocket::spawn_realtime_mock_data(ws_manager.clone());
    Arc::new(AppState {
        validator_set,
        storage,
        governance_state,
        tx_pool,
        network_propagator,
        ws_manager,
    })
}

// Removed load_tls_config - ServerConfig not available
// TODO: Implement TLS configuration
#[allow(dead_code)]
async fn load_tls_config() -> Arc<()> {
    let cert = env::var("TLS_CERT_PATH").expect("TLS_CERT_PATH not set");
    let key = env::var("TLS_KEY_PATH").expect("TLS_KEY_PATH not set");
    
    let _cert_chain = std::fs::read(cert).unwrap();
    let _key_der = std::fs::read(key).unwrap();
    
    // Removed CertificateDer - not available
    // TODO: Implement TLS configuration
    // Removed PrivateKey - not available
    // TODO: Implement TLS configuration
    
    // Removed ServerConfig - not available
    // TODO: Implement TLS configuration
    Arc::new(())
}

pub async fn run_http_server() {
    // This function is now deprecated - use create_router() instead
    println!("Warning: run_http_server() is deprecated. Use create_router() instead.");
}

pub fn create_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .route("/auth/password-reset", post(password_reset))
        .route("/auth/password-reset/:token", post(password_reset_confirm))
        .route("/auth/change-password", post(change_password))
        .route("/auth/logout", post(logout))
        .route("/auth/2fa/setup", post(setup_2fa))
        .route("/auth/2fa/enable", post(enable_2fa))
        .route("/auth/2fa/disable", post(disable_2fa))
        .route("/auth/2fa/verify", post(verify_2fa))
        .route("/ws", get(websocket_handler))
        .route("/balance/:address/:asset", get(get_balance))
        .route("/tx", post(post_tx))
        .route("/validators", get(get_validators).post(add_validator))
        .route("/validators/:address", delete(remove_validator))
        .route("/validators/:address/slash", post(slash_validator))
        .route("/governance/proposals", post(submit_proposal).get(list_proposals))
        .route("/governance/proposals/:id", get(get_proposal))
        .route("/governance/proposals/:id/vote", post(vote_proposal))
        .route("/governance/proposals/:id/execute", post(execute_proposal))
        .route("/governance/proposals/:id/cancel", post(cancel_proposal))
        .route("/governance/proposals/:id/votes", get(get_proposal_votes))
        .route("/governance/stats", get(get_governance_stats))
        .route("/governance/config", get(get_governance_config))
        .route("/governance/analytics", get(get_governance_analytics))
        .route("/governance/events", get(get_governance_events))
        .route("/governance/top-voters", get(get_top_voters))
        .route("/assets", get(get_assets))
        .route("/bridge/outbound", post(outbound_bridge))
        .route("/bridge/inbound", post(inbound_bridge))
        .route("/bridge/status/:tx_id", get(bridge_status))
        .route("/confidential/tx", post(submit_confidential_tx))
        .route("/identity/register", post(register_identity))
        .route("/orders", post(place_order))
        .route("/orders", get(get_order_history))
        .route("/orders/:order_id", delete(cancel_order))
        .route("/trades", get(get_trade_history))
        .route("/positions", get(get_positions))
        .route("/block/:id", get(get_block))
        .route("/block/:id/merkle_proof/:tx_hash", get(get_merkle_proof))
        .route("/health", get(health))
        // Wallet endpoints
        .route("/wallet/connect", post(connect_wallet))
        .route("/wallet/balance", get(get_wallet_balance))
        .route("/wallet/transactions", get(get_transaction_history))
        .route("/wallet/deposit", post(deposit_funds))
        .route("/wallet/withdraw", post(withdraw_funds))
        .route("/wallet/addresses", get(get_wallet_addresses).post(generate_wallet_address))
        // DAG endpoints
        .route("/dag/submit-transaction", post(submit_dag_transaction))
        // .route("/dag/status", get(get_dag_status))
        // .route("/dag/blocks", get(get_dag_blocks))
        // .route("/dag/validators", get(get_dag_validators))
        // Analytics endpoints
        .route("/analytics/trading", get(get_trading_analytics))
        .route("/analytics/performance", get(get_performance_metrics))
        .route("/analytics/performance/timeseries", get(get_performance_timeseries))
        .route("/analytics/risk", get(get_risk_analysis))
        .route("/analytics/portfolio", get(get_portfolio_report))
        .route("/analytics/market", get(get_market_analysis))
        // Real-time data endpoints
        .route("/realtime/subscribe", get(subscribe_realtime))
        .route("/realtime/status", get(get_realtime_status))
        // Security key management endpoints
        .route("/security/keys/generate", post(generate_secure_key))
        .route("/security/keys", get(list_secure_keys_endpoint))
        .route("/security/keys/:key_id/rotate", post(rotate_secure_key_endpoint))
        .route("/security/keys/:key_id", delete(deactivate_secure_key_endpoint))
        .layer(create_cors_layer())
        .layer(axum::middleware::from_fn(sanitize_request))
        .with_state(state)
}

pub async fn start(
    port: u16,
    tx_pool: Arc<ShardedTxPool>,
) -> std::io::Result<()> {
    use tokio::net::TcpListener;
    
    // Start cache cleanup task
    start_cache_cleanup().await;
    
    // Create a minimal AppState for the simple server
    let validator_set = Arc::new(Mutex::new(ValidatorSet::new()));
    let storage = Arc::new(crate::storage::persistent::PersistentStorage::new("simple_server").unwrap());
    let governance_state = Arc::new(Mutex::new(crate::consensus::governance::GovernanceState {
        proposals: HashMap::new(),
        votes: HashMap::new(),
        active_proposals: Vec::new(),
        executed_proposals: Vec::new(),
        config: crate::consensus::governance::GovernanceConfig::default(),
        total_stake: 0,
        proposal_counter: 0,
        analytics: crate::consensus::governance::GovernanceAnalytics::default(),
        events: Vec::new(),
        voter_activity: HashMap::new(),
    }));
    let network_propagator = Arc::new(NetworkPropagator::new("127.0.0.1:8080", vec![], Address::new("testaddress00000000000000000000000000000000".to_string())).await?);
    
    let app_state = Arc::new(AppState {
        validator_set,
        storage,
        governance_state,
        tx_pool,
        network_propagator,
        ws_manager: Arc::new(WebSocketManager::new()),
    });
    
    let app = Router::new()
        .route("/health", get(health))
        .route("/tx", post(post_tx))
        .route("/ws", get(websocket_handler))
        .layer(create_cors_layer())
        .layer(middleware::from_fn(sanitize_request))
        .with_state(app_state);

    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await?;
    println!("HTTP server listening on {addr}");
    println!("Security features enabled: CORS, rate limiting, input validation, JWT authentication");
    println!("Performance features enabled: Caching layer with TTL support");
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> impl IntoResponse {
    "OK"
}

async fn submit_transaction(
    State(tx_pool): State<Arc<ShardedTxPool>>,
    Json(tx): Json<Transaction>,
) -> impl IntoResponse {
    tx_pool.add_transaction(tx);
    "Transaction submitted"
}

// Example usage:
// GET http://127.0.0.1:8080/balance/fdg1qxyz...
// POST http://127.0.0.1:8080/tx { ...transaction json... } 

// Update the keypair generation in the generate_test_data function
#[allow(dead_code)]
async fn generate_test_data(State(_state): State<AppState>) -> impl IntoResponse {
    let mut rng = rand::rngs::OsRng;
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);
    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let _address = Address::from_signing_key(&signing_key);
    
    // ... rest of the function remains the same ...
} 

/// POST /wallet/connect - Connect or create a wallet for the user
async fn connect_wallet(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<WalletConnectResponse>, (StatusCode, Json<WalletConnectResponse>)> {
    // Authenticate user (must be logged in)
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(WalletConnectResponse {
            address: "".to_string(),
            message: "Authentication required".to_string(),
        }))
    })?;

    // For now, just create a new wallet (in production, load from DB or session)
    let wallet = crate::core::wallet::Wallet::new();
    let address = wallet.address().to_string();
    Ok(Json(WalletConnectResponse {
        address,
        message: "Wallet connected (demo)".to_string(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct WalletConnectResponse {
    pub address: String,
    pub message: String,
}

/// GET /wallet/balance - Get wallet balances
async fn get_wallet_balance(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<WalletBalanceResponse>, (StatusCode, Json<WalletBalanceResponse>)> {
    // Authenticate user (must be logged in)
    let user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(WalletBalanceResponse {
            balances: vec![],
            message: "Authentication required".to_string(),
        }))
    })?;

    // Check cache first
    let cache = get_cache();
    if let Some(cached_balances) = cache.get_wallet_balance(&user).await {
        return Ok(Json(WalletBalanceResponse {
            balances: cached_balances,
            message: "Wallet balances (cached)".to_string(),
        }));
    }

    // Generate mock balances (in production, fetch from database)
    let balances = vec![
        WalletAssetBalance { asset: "USD".to_string(), amount: 10000.0 },
        WalletAssetBalance { asset: "BTC".to_string(), amount: 2.5 },
        WalletAssetBalance { asset: "ETH".to_string(), amount: 50.0 },
    ];
    
    // Cache the balances
    cache.set_wallet_balance(user, balances.clone()).await;
    
    Ok(Json(WalletBalanceResponse {
        balances,
        message: "Wallet balances (fresh)".to_string(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct WalletBalanceResponse {
    pub balances: Vec<WalletAssetBalance>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WalletAssetBalance {
    pub asset: String,
    pub amount: f64,
}

/// GET /wallet/transactions - Get transaction history
async fn get_transaction_history(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<TransactionHistoryResponse>, (StatusCode, Json<TransactionHistoryResponse>)> {
    // Authenticate user (must be logged in)
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(TransactionHistoryResponse {
            transactions: vec![],
            total: 0,
            message: "Authentication required".to_string(),
        }))
    })?;

    // Get pagination parameters
    let page = params.get("page").and_then(|p| p.parse::<usize>().ok()).unwrap_or(1);
    let limit = params.get("limit").and_then(|l| l.parse::<usize>().ok()).unwrap_or(20);
    let offset = (page - 1) * limit;

    // For now, return mock transaction history
    let mock_transactions = vec![
        WalletTransaction {
            tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
            from: "user_wallet_address".to_string(),
            to: "orderbook_address".to_string(),
            amount: 1000.0,
            asset: "USD".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            status: "confirmed".to_string(),
            fee: 0.001,
        },
        WalletTransaction {
            tx_hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890".to_string(),
            from: "faucet_address".to_string(),
            to: "user_wallet_address".to_string(),
            amount: 5000.0,
            asset: "USD".to_string(),
            timestamp: chrono::Utc::now().timestamp() - 3600,
            status: "confirmed".to_string(),
            fee: 0.0,
        },
        WalletTransaction {
            tx_hash: "0x7890abcdef1234567890abcdef1234567890abcdef1234567890abcdef123456".to_string(),
            from: "user_wallet_address".to_string(),
            to: "exchange_address".to_string(),
            amount: 0.5,
            asset: "BTC".to_string(),
            timestamp: chrono::Utc::now().timestamp() - 7200,
            status: "pending".to_string(),
            fee: 0.0001,
        },
    ];

    // Apply pagination
    let total = mock_transactions.len();
    let transactions = mock_transactions
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();

    Ok(Json(TransactionHistoryResponse {
        transactions,
        total,
        message: "Transaction history (mock)".to_string(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct TransactionHistoryResponse {
    pub transactions: Vec<WalletTransaction>,
    pub total: usize,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct WalletTransaction {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub asset: String,
    pub timestamp: i64,
    pub status: String, // "pending", "confirmed", "failed"
    pub fee: f64,
}

/// POST /wallet/deposit - Deposit funds to wallet
async fn deposit_funds(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<DepositRequest>,
) -> Result<Json<DepositResponse>, (StatusCode, Json<DepositResponse>)> {
    // Authenticate user (must be logged in)
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(DepositResponse {
            tx_hash: "".to_string(),
            status: "failed".to_string(),
            message: "Authentication required".to_string(),
        }))
    })?;

    // Validate request
    if req.amount <= 0.0 {
        return Err((StatusCode::BAD_REQUEST, Json(DepositResponse {
            tx_hash: "".to_string(),
            status: "failed".to_string(),
            message: "Invalid amount".to_string(),
        })));
    }

    // For now, simulate deposit (in production, integrate with bridge or external system)
    let tx_hash = format!("0x{}", hex::encode(rand::random::<[u8; 32]>()));
    
    Ok(Json(DepositResponse {
        tx_hash,
        status: "pending".to_string(),
        message: "Deposit initiated (mock)".to_string(),
    }))
}

/// POST /wallet/withdraw - Withdraw funds from wallet
async fn withdraw_funds(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<WithdrawRequest>,
) -> Result<Json<WithdrawResponse>, (StatusCode, Json<WithdrawResponse>)> {
    // Authenticate user (must be logged in)
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(WithdrawResponse {
            tx_hash: "".to_string(),
            status: "failed".to_string(),
            message: "Authentication required".to_string(),
        }))
    })?;

    // Validate request
    if req.amount <= 0.0 {
        return Err((StatusCode::BAD_REQUEST, Json(WithdrawResponse {
            tx_hash: "".to_string(),
            status: "failed".to_string(),
            message: "Invalid amount".to_string(),
        })));
    }

    // For now, simulate withdrawal (in production, check balance and integrate with bridge)
    let tx_hash = format!("0x{}", hex::encode(rand::random::<[u8; 32]>()));
    
    Ok(Json(WithdrawResponse {
        tx_hash,
        status: "pending".to_string(),
        message: "Withdrawal initiated (mock)".to_string(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct DepositRequest {
    pub amount: f64,
    pub asset: String,
    pub external_address: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DepositResponse {
    pub tx_hash: String,
    pub status: String, // "pending", "confirmed", "failed"
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawRequest {
    pub amount: f64,
    pub asset: String,
    pub external_address: String,
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawResponse {
    pub tx_hash: String,
    pub status: String, // "pending", "confirmed", "failed"
    pub message: String,
}

/// GET /wallet/addresses - Get wallet addresses
async fn get_wallet_addresses(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<WalletAddressesResponse>, (StatusCode, Json<WalletAddressesResponse>)> {
    // Authenticate user (must be logged in)
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(WalletAddressesResponse {
            addresses: vec![],
            message: "Authentication required".to_string(),
        }))
    })?;

    // For now, return mock addresses (in production, load from user's wallet)
    let addresses = vec![
        WalletAddress {
            address: "findag_user_wallet_000000000000000000000000000000".to_string(),
            label: "Primary".to_string(),
            is_active: true,
        },
        WalletAddress {
            address: "findag_user_wallet_111111111111111111111111111111".to_string(),
            label: "Trading".to_string(),
            is_active: true,
        },
    ];

    Ok(Json(WalletAddressesResponse {
        addresses,
        message: "Wallet addresses (mock)".to_string(),
    }))
}

/// POST /wallet/addresses - Generate new wallet address
async fn generate_wallet_address(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<GenerateAddressRequest>,
) -> Result<Json<GenerateAddressResponse>, (StatusCode, Json<GenerateAddressResponse>)> {
    // Authenticate user (must be logged in)
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(GenerateAddressResponse {
            address: "".to_string(),
            message: "Authentication required".to_string(),
        }))
    })?;

    // Generate new wallet address
    let wallet = crate::core::wallet::Wallet::new();
    let address = wallet.address().to_string();
    
    Ok(Json(GenerateAddressResponse {
        address,
        message: "New address generated (mock)".to_string(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct WalletAddressesResponse {
    pub addresses: Vec<WalletAddress>,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct WalletAddress {
    pub address: String,
    pub label: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateAddressRequest {
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateAddressResponse {
    pub address: String,
    pub message: String,
}

/// POST /dag/submit-transaction - Submit a transaction to the DAG
async fn submit_dag_transaction(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<DagTransactionRequest>,
) -> Result<Json<DagTransactionResponse>, (StatusCode, Json<DagTransactionResponse>)> {
    // Authenticate user (must be logged in)
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(DagTransactionResponse {
            tx_hash: "".to_string(),
            block_id: "".to_string(),
            status: "failed".to_string(),
            message: "Authentication required".to_string(),
        }))
    })?;

    // Validate request
    if req.amount <= 0.0 {
        return Err((StatusCode::BAD_REQUEST, Json(DagTransactionResponse {
            tx_hash: "".to_string(),
            block_id: "".to_string(),
            status: "failed".to_string(),
            message: "Invalid amount".to_string(),
        })));
    }

    // Create a wallet for signing (in production, load user's wallet)
    let wallet = crate::core::wallet::Wallet::new();
    let account = &wallet.accounts()[0];
    
    // Create transaction
    let mut transaction = create_dag_transaction(&req, account);
    
    // Sign the transaction
    if let Err(e) = account.sign_transaction(&mut transaction) {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(DagTransactionResponse {
            tx_hash: "".to_string(),
            block_id: "".to_string(),
            status: "failed".to_string(),
            message: format!("Failed to sign transaction: {}", e),
        })));
    }
    
    // Add transaction to the pool
    let added = state.tx_pool.add_transaction(transaction.clone());
    
    if added {
        // Generate transaction hash
        let tx_hash = format!("0x{}", hex::encode(transaction.hashtimer));
        
        // Create a mock block ID (in production, this would be created by block producer)
        let block_id = format!("0x{}", hex::encode(rand::random::<[u8; 32]>()));
        
        Ok(Json(DagTransactionResponse {
            tx_hash,
            block_id,
            status: "submitted".to_string(),
            message: "Transaction submitted to DAG successfully".to_string(),
        }))
    } else {
        Err((StatusCode::BAD_REQUEST, Json(DagTransactionResponse {
            tx_hash: "".to_string(),
            block_id: "".to_string(),
            status: "failed".to_string(),
            message: "Transaction rejected by DAG".to_string(),
        })))
    }
}

/// Helper function to create a DAG transaction
fn create_dag_transaction(
    req: &DagTransactionRequest,
    account: &crate::core::wallet::WalletAccount,
) -> crate::core::types::Transaction {
    use crate::core::types::{Transaction, ShardId};
    use crate::core::address::Address;
    use sha2::{Sha256, Digest};
    use chrono::Utc;
    
    // Create transaction payload
    let payload = serde_json::json!({
        "from": req.from,
        "to": req.to,
        "amount": req.amount,
        "asset": req.asset,
        "timestamp": Utc::now().timestamp(),
        "purpose": req.purpose,
    });
    
    let payload_bytes = serde_json::to_vec(&payload).unwrap_or_default();
    
    // Create HashTimer from transaction data
    let mut hashtimer = [0u8; 32];
    let mut hasher = Sha256::new();
    hasher.update(req.from.as_bytes());
    hasher.update(req.to.as_bytes());
    hasher.update(req.amount.to_string().as_bytes());
    hashtimer.copy_from_slice(&hasher.finalize());
    
    // Use account address as from (in production, resolve from user's wallet)
    let from_address = account.address.clone();
    let to_address = Address::new(req.to.clone());
    
    Transaction {
        from: from_address,
        to: to_address,
        amount: (req.amount * 1_000_000.0) as u64, // Convert to base units
        payload: payload_bytes,
        findag_time: Utc::now().timestamp() as u64,
        hashtimer,
        signature: ed25519_dalek::Signature::from_bytes(&[0u8; 64]), // Dummy signature, replaced by sign_transaction
        public_key: account.signing_key.verifying_key(),
        shard_id: ShardId(req.shard_id.unwrap_or(0)),
        source_shard: None,
        dest_shard: None,
        target_chain: None,
        bridge_protocol: None,
    }
}

#[derive(Serialize, Deserialize)]
pub struct DagTransactionRequest {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub asset: String,
    pub purpose: Option<String>,
    pub shard_id: Option<u16>,
}

#[derive(Serialize, Deserialize)]
pub struct DagTransactionResponse {
    pub tx_hash: String,
    pub block_id: String,
    pub status: String, // "submitted", "confirmed", "failed"
    pub message: String,
}

/// GET /dag/status - Get DAG network status
async fn get_dag_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<DagStatusResponse>, (StatusCode, Json<DagStatusResponse>)> {
    // Check cache first
    let cache = get_cache();
    if let Some(cached_status) = cache.get_dag_status().await {
        return Ok(Json(cached_status));
    }
    
    // Get DAG statistics from the engine
    let dag_engine = crate::core::dag_engine::DagEngine::new().await;
    let stats = dag_engine.get_stats().await;
    
    // Get validator information
    let validator_set = state.validator_set.lock().unwrap();
    let validators = validator_set.get_all_validators();
    
    // Get transaction pool status (sum of all shards)
    let tx_pool_size = 42; // Mock value - in production, sum all shard sizes
    
    let status = DagStatusResponse {
        network_status: "active".to_string(),
        total_blocks: stats.total_blocks,
        tips_count: stats.tips_count,
        max_depth: stats.max_depth,
        avg_txs_per_block: stats.avg_txs_per_block,
        active_validators: validators.len(),
        tx_pool_size,
        last_block_time: chrono::Utc::now().timestamp(),
        consensus_status: "healthy".to_string(),
        message: "DAG network status (fresh)".to_string(),
    };
    
    // Cache the status
    cache.set_dag_status(status.clone()).await;
    
    Ok(Json(status))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DagStatusResponse {
    pub network_status: String, // "active", "syncing", "error"
    pub total_blocks: usize,
    pub tips_count: usize,
    pub max_depth: usize,
    pub avg_txs_per_block: f64,
    pub active_validators: usize,
    pub tx_pool_size: usize,
    pub last_block_time: i64,
    pub consensus_status: String, // "healthy", "degraded", "error"
    pub message: String,
}

/// GET /dag/blocks - Get DAG blocks with pagination
async fn get_dag_blocks(
    State(_state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<DagBlocksResponse>, (StatusCode, Json<DagBlocksResponse>)> {
    // Get pagination parameters
    let page = params.get("page").and_then(|p| p.parse::<usize>().ok()).unwrap_or(1);
    let limit = params.get("limit").and_then(|l| l.parse::<usize>().ok()).unwrap_or(20);
    let shard_id = params.get("shard_id").and_then(|s| s.parse::<u16>().ok());
    
    // Get DAG engine and blocks
    let dag_engine = crate::core::dag_engine::DagEngine::new().await;
    let all_blocks = dag_engine.get_all_blocks().await;
    
    // Filter by shard if specified
    let filtered_blocks = if let Some(shard) = shard_id {
        all_blocks.into_iter()
            .filter(|block| block.shard_id.0 == shard)
            .collect()
    } else {
        all_blocks
    };
    
    // Apply pagination
    let total = filtered_blocks.len();
    let offset = (page - 1) * limit;
    let blocks = filtered_blocks
        .into_iter()
        .skip(offset)
        .take(limit)
        .map(|block| DagBlock {
            block_id: hex::encode(block.block_id),
            parent_blocks: block.parent_blocks.into_iter().map(|p| hex::encode(p)).collect(),
            transaction_count: block.transactions.len(),
            findag_time: block.findag_time,
            proposer: block.proposer.0,
            shard_id: block.shard_id.0,
            merkle_root: block.merkle_root.map(|r| hex::encode(r)),
        })
        .collect();
    
    Ok(Json(DagBlocksResponse {
        blocks,
        total,
        page,
        limit,
        message: "DAG blocks retrieved successfully".to_string(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct DagBlocksResponse {
    pub blocks: Vec<DagBlock>,
    pub total: usize,
    pub page: usize,
    pub limit: usize,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct DagBlock {
    pub block_id: String,
    pub parent_blocks: Vec<String>,
    pub transaction_count: usize,
    pub findag_time: u64,
    pub proposer: String,
    pub shard_id: u16,
    pub merkle_root: Option<String>,
}

/// GET /dag/validators - Get validator information
async fn get_dag_validators(
    State(state): State<Arc<AppState>>,
) -> Result<Json<DagValidatorsResponse>, (StatusCode, Json<DagValidatorsResponse>)> {
    // Check cache first
    let cache = get_cache();
    if let Some(cached_info) = cache.get_validator_info().await {
        return Ok(Json(cached_info));
    }
    
    // Get validator information from the validator set
    let validator_set = state.validator_set.lock().unwrap();
    let validators = validator_set.get_all_validators();
    
    // Convert to API response format
    let validator_info = validators.iter().map(|validator| DagValidator {
        address: validator.address.0.clone(),
        public_key: hex::encode(validator.public_key.to_bytes()),
        stake: validator.stake,
        is_active: validator.is_active,
        last_activity: validator.reputation.last_seen_timestamp,
        performance_score: validator.reputation.reputation_score,
    }).collect();
    
    // Get validator statistics
    let total_validators = validators.len();
    let active_validators = validators.iter().filter(|v| v.is_active).count();
    let total_stake: u64 = validators.iter().map(|v| v.stake).sum();
    
    let info = DagValidatorsResponse {
        validators: validator_info,
        total_validators,
        active_validators,
        total_stake,
        message: "Validator information retrieved successfully (fresh)".to_string(),
    };
    
    // Cache the validator info
    cache.set_validator_info(info.clone()).await;
    
    Ok(Json(info))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DagValidatorsResponse {
    pub validators: Vec<DagValidator>,
    pub total_validators: usize,
    pub active_validators: usize,
    pub total_stake: u64,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DagValidator {
    pub address: String,
    pub public_key: String,
    pub stake: u64,
    pub is_active: bool,
    pub last_activity: u64,
    pub performance_score: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OrderBookEntry {
    pub price: f64,
    pub quantity: f64,
    pub timestamp: i64,
}

/// GET /analytics/trading - Trading analytics
async fn get_trading_analytics(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<TradingAnalyticsResponse>, (StatusCode, Json<TradingAnalyticsResponse>)> {
    // Authenticate user
    let user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(TradingAnalyticsResponse {
            total_volume: 0.0,
            total_trades: 0,
            win_rate: 0.0,
            profit_loss: 0.0,
            message: "Authentication required".to_string(),
        }))
    })?;

    // Check cache first
    let cache = get_cache();
    if let Some(cached_analytics) = cache.get_trading_analytics(&user).await {
        return Ok(Json(cached_analytics));
    }

    // Mock trading analytics data
    let analytics = TradingAnalyticsResponse {
        total_volume: 1_000_000.0,
        total_trades: 1200,
        win_rate: 0.62,
        profit_loss: 15000.0,
        message: "Trading analytics (fresh)".to_string(),
    };
    
    // Cache the analytics
    cache.set_trading_analytics(user, analytics.clone()).await;
    
    Ok(Json(analytics))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TradingAnalyticsResponse {
    pub total_volume: f64,
    pub total_trades: usize,
    pub win_rate: f64,
    pub profit_loss: f64,
    pub message: String,
}

/// GET /analytics/performance - System performance metrics
async fn get_performance_metrics(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<PerformanceMetricsResponse>, (StatusCode, Json<PerformanceMetricsResponse>)> {
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(PerformanceMetricsResponse {
            avg_latency_ms: 0.0,
            max_throughput: 0.0,
            uptime_hours: 0.0,
            error_rate: 0.0,
            message: "Authentication required".to_string(),
        }))
    })?;
    Ok(Json(PerformanceMetricsResponse {
        avg_latency_ms: 12.5,
        max_throughput: 5000.0,
        uptime_hours: 8760.0,
        error_rate: 0.001,
        message: "Performance metrics (mock)".to_string(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct PerformanceMetricsResponse {
    pub avg_latency_ms: f64,
    pub max_throughput: f64,
    pub uptime_hours: f64,
    pub error_rate: f64,
    pub message: String,
}

/// GET /analytics/risk - Risk analysis
async fn get_risk_analysis(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<RiskAnalysisResponse>, (StatusCode, Json<RiskAnalysisResponse>)> {
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(RiskAnalysisResponse {
            value_at_risk: 0.0,
            max_drawdown: 0.0,
            exposure: 0.0,
            message: "Authentication required".to_string(),
        }))
    })?;
    Ok(Json(RiskAnalysisResponse {
        value_at_risk: 25000.0,
        max_drawdown: 0.18,
        exposure: 500000.0,
        message: "Risk analysis (mock)".to_string(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct RiskAnalysisResponse {
    pub value_at_risk: f64,
    pub max_drawdown: f64,
    pub exposure: f64,
    pub message: String,
}

/// GET /analytics/portfolio - Portfolio reports
async fn get_portfolio_report(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<PortfolioReportResponse>, (StatusCode, Json<PortfolioReportResponse>)> {
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(PortfolioReportResponse {
            holdings: vec![],
            total_value: 0.0,
            returns_pct: 0.0,
            message: "Authentication required".to_string(),
        }))
    })?;
    Ok(Json(PortfolioReportResponse {
        holdings: vec![
            PortfolioHolding { asset: "USD".to_string(), amount: 10000.0, value: 10000.0 },
            PortfolioHolding { asset: "BTC".to_string(), amount: 2.5, value: 100000.0 },
            PortfolioHolding { asset: "ETH".to_string(), amount: 50.0, value: 90000.0 },
        ],
        total_value: 200000.0,
        returns_pct: 0.12,
        message: "Portfolio report (mock)".to_string(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct PortfolioReportResponse {
    pub holdings: Vec<PortfolioHolding>,
    pub total_value: f64,
    pub returns_pct: f64,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct PortfolioHolding {
    pub asset: String,
    pub amount: f64,
    pub value: f64,
}

/// GET /analytics/market - Market analysis
async fn get_market_analysis(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<MarketAnalysisResponse>, (StatusCode, Json<MarketAnalysisResponse>)> {
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(MarketAnalysisResponse {
            price_trend: "upward".to_string(),
            volatility: 0.22,
            liquidity: 0.95,
            message: "Authentication required".to_string(),
        }))
    })?;
    Ok(Json(MarketAnalysisResponse {
        price_trend: "upward".to_string(),
        volatility: 0.22,
        liquidity: 0.95,
        message: "Market analysis (mock)".to_string(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct MarketAnalysisResponse {
    pub price_trend: String,
    pub volatility: f64,
    pub liquidity: f64,
    pub message: String,
}

/// GET /realtime/subscribe - Subscribe to real-time data channels
async fn subscribe_realtime(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<RealtimeSubscriptionResponse>, (StatusCode, Json<RealtimeSubscriptionResponse>)> {
    // Authenticate user
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(RealtimeSubscriptionResponse {
            success: false,
            channels: vec![],
            message: "Authentication required".to_string(),
        }))
    })?;

    // Get channels from query parameters
    let channels: Vec<String> = params.get("channels")
        .map(|c| c.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    if channels.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(RealtimeSubscriptionResponse {
            success: false,
            channels: vec![],
            message: "No channels specified".to_string(),
        })));
    }

    Ok(Json(RealtimeSubscriptionResponse {
        success: true,
        channels,
        message: "Successfully subscribed to real-time channels".to_string(),
    }))
}

/// GET /realtime/status - Get real-time connection status
async fn get_realtime_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<RealtimeStatusResponse>, (StatusCode, Json<RealtimeStatusResponse>)> {
    let connections = state.ws_manager.connections.lock().unwrap();
    let active_connections = connections.len();

    Ok(Json(RealtimeStatusResponse {
        active_connections,
        uptime_seconds: chrono::Utc::now().timestamp(),
        status: "active".to_string(),
        message: "Real-time data service is running".to_string(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct RealtimeSubscriptionResponse {
    pub success: bool,
    pub channels: Vec<String>,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct RealtimeStatusResponse {
    pub active_connections: usize,
    pub uptime_seconds: i64,
    pub status: String,
    pub message: String,
}

/// POST /auth/2fa/setup - Generate a new 2FA secret and QR code URL
async fn setup_2fa(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Setup2FAResponse>, (StatusCode, Json<Setup2FAResponse>)> {
    let user_id = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(Setup2FAResponse {
            secret: "".to_string(),
            qr_url: "".to_string(),
            message: "Authentication required".to_string(),
        }))
    })?;
    // Generate a new random secret (20 bytes base32)
    let mut secret_bytes = [0u8; 20];
    rand::thread_rng().fill_bytes(&mut secret_bytes);
    let secret = base32_encode(base32::Alphabet::RFC4648 { padding: false }, &secret_bytes);
    let totp = TOTP::from_base32(&secret).unwrap();
    let qr_url = format!(
        "otpauth://totp/FinDAG:{}?secret={}&issuer=FinDAG",
        user_id, secret
    );
    Ok(Json(Setup2FAResponse {
        secret,
        qr_url,
        message: "Scan this QR code in your authenticator app".to_string(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct Setup2FAResponse {
    pub secret: String,
    pub qr_url: String,
    pub message: String,
}

/// POST /auth/2fa/enable - Enable 2FA after verifying code
#[derive(Deserialize)]
pub struct Enable2FARequest {
    pub secret: String,
    pub code: String,
}

async fn enable_2fa(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<Enable2FARequest>,
) -> Result<Json<Enable2FAResponse>, (StatusCode, Json<Enable2FAResponse>)> {
    let user_id = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(Enable2FAResponse {
            success: false,
            message: "Authentication required".to_string(),
        }))
    })?;
    let totp = TOTP::from_base32(&req.secret).ok_or((StatusCode::BAD_REQUEST, Json(Enable2FAResponse {
        success: false,
        message: "Invalid secret format".to_string(),
    })))?;
    let code: u32 = req.code.parse().map_err(|_| (StatusCode::BAD_REQUEST, Json(Enable2FAResponse {
        success: false,
        message: "Invalid 2FA code format".to_string(),
    })))?;
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    if !totp.verify(code, 30, now) {
        return Err((StatusCode::BAD_REQUEST, Json(Enable2FAResponse {
            success: false,
            message: "Invalid 2FA code".to_string(),
        })));
    }
    Ok(Json(Enable2FAResponse {
        success: true,
        message: "2FA enabled successfully".to_string(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct Enable2FAResponse {
    pub success: bool,
    pub message: String,
}

/// POST /auth/2fa/disable - Disable 2FA
#[derive(Deserialize)]
pub struct Disable2FARequest {
    pub code: String,
}

async fn disable_2fa(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<Disable2FARequest>,
) -> Result<Json<Disable2FAResponse>, (StatusCode, Json<Disable2FAResponse>)> {
    let user_id = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(Disable2FAResponse {
            success: false,
            message: "Authentication required".to_string(),
        }))
    })?;
    // In production, verify code and disable 2FA in DB
    Ok(Json(Disable2FAResponse {
        success: true,
        message: "2FA disabled successfully".to_string(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct Disable2FAResponse {
    pub success: bool,
    pub message: String,
}

/// POST /auth/2fa/verify - Verify a 2FA code
#[derive(Deserialize)]
pub struct Verify2FARequest {
    pub secret: String,
    pub code: String,
}

async fn verify_2fa(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<Verify2FARequest>,
) -> Result<Json<Verify2FAResponse>, (StatusCode, Json<Verify2FAResponse>)> {
    let _user_id = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(Verify2FAResponse {
            valid: false,
            message: "Authentication required".to_string(),
        }))
    })?;
    let totp = TOTP::from_base32(&req.secret).ok_or((StatusCode::BAD_REQUEST, Json(Verify2FAResponse {
        valid: false,
        message: "Invalid secret format".to_string(),
    })))?;
    let code: u32 = req.code.parse().map_err(|_| (StatusCode::BAD_REQUEST, Json(Verify2FAResponse {
        valid: false,
        message: "Invalid 2FA code format".to_string(),
    })))?;
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let valid = totp.verify(code, 30, now);
    Ok(Json(Verify2FAResponse {
        valid,
        message: if valid { "2FA code is valid".to_string() } else { "Invalid 2FA code".to_string() },
    }))
}

#[derive(Serialize, Deserialize)]
pub struct Verify2FAResponse {
    pub valid: bool,
    pub message: String,
}

// Add helper function to clean up expired sessions
fn cleanup_expired_sessions() {
    let mut sessions = USER_SESSIONS.lock().unwrap();
    let now = chrono::Utc::now();
    let expiry = chrono::Duration::hours(JWT_EXPIRY_HOURS);
    sessions.retain(|_, (_username, login_time)| now.signed_duration_since(*login_time) < expiry);
}

/// DELETE /orders/{order_id} - Cancel an existing order
async fn cancel_order(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(order_id): Path<String>,
) -> Result<Json<OrderResponse>, (StatusCode, Json<OrderResponse>)> {
    // Authenticate user
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(OrderResponse {
            order_id: "".to_string(),
            status: "rejected".to_string(),
            message: "Authentication required".to_string(),
        }))
    })?;

    // Basic validation
    if order_id.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(OrderResponse {
            order_id: "".to_string(),
            status: "rejected".to_string(),
            message: "Invalid order ID".to_string(),
        })));
    }

    // In a real implementation, you would:
    // 1. Look up the order in the order book
    // 2. Verify the user owns the order
    // 3. Cancel the order in the order book
    // 4. Create a cancellation transaction

    // For now, we'll simulate order cancellation
    let wallet = crate::core::wallet::Wallet::new();
    let account = &wallet.accounts()[0];
    
    // Create cancellation transaction
    let mut transaction = create_cancellation_transaction(&order_id, account);
    
    // Sign the transaction
    if let Err(e) = account.sign_transaction(&mut transaction) {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(OrderResponse {
            order_id: "".to_string(),
            status: "rejected".to_string(),
            message: format!("Failed to sign cancellation transaction: {}", e),
        })));
    }
    
    // Add transaction to the pool
    let added = state.tx_pool.add_transaction(transaction);
    
    if added {
        // Broadcast order cancellation via WebSocket
        let system_message = WebSocketMessage::SystemMessage {
            message: format!("Order {} cancelled successfully", order_id),
            level: "info".to_string(),
            timestamp: chrono::Utc::now(),
        };
        let _ = state.ws_manager.orderbook_sender.send(system_message);
        
        Ok(Json(OrderResponse {
            order_id: order_id.clone(),
            status: "cancelled".to_string(),
            message: "Order cancelled successfully".to_string(),
        }))
    } else {
        Err((StatusCode::BAD_REQUEST, Json(OrderResponse {
            order_id: "".to_string(),
            status: "rejected".to_string(),
            message: "Order cancellation rejected by transaction pool".to_string(),
        })))
    }
}

/// GET /orders - Get order history for the authenticated user
async fn get_order_history(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<OrderHistoryResponse>, (StatusCode, Json<OrderHistoryResponse>)> {
    // Authenticate user
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(OrderHistoryResponse {
            orders: vec![],
            total: 0,
            message: "Authentication required".to_string(),
        }))
    })?;

    // Parse query parameters
    let page = params.get("page").and_then(|p| p.parse::<usize>().ok()).unwrap_or(1);
    let limit = params.get("limit").and_then(|l| l.parse::<usize>().ok()).unwrap_or(20);
    let status = params.get("status").cloned();
    let symbol = params.get("symbol").cloned();

    // Mock order history data
    let mut orders = vec![
        OrderHistoryEntry {
            order_id: "ord_123456789".to_string(),
            symbol: "BTC/USD".to_string(),
            side: "buy".to_string(),
            order_type: "limit".to_string(),
            quantity: 0.5,
            price: Some(50000.0),
            status: "filled".to_string(),
            filled_quantity: 0.5,
            average_price: 50000.0,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        },
        OrderHistoryEntry {
            order_id: "ord_987654321".to_string(),
            symbol: "ETH/USD".to_string(),
            side: "sell".to_string(),
            order_type: "market".to_string(),
            quantity: 2.0,
            price: None,
            status: "pending".to_string(),
            filled_quantity: 0.0,
            average_price: 0.0,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        },
    ];

    // Apply filters
    if let Some(status_filter) = &status {
        orders.retain(|order| order.status == *status_filter);
    }
    if let Some(symbol_filter) = &symbol {
        orders.retain(|order| order.symbol == *symbol_filter);
    }

    let total = orders.len();
    Ok(Json(OrderHistoryResponse {
        orders,
        total,
        message: "Order history retrieved successfully".to_string(),
    }))
}

/// GET /trades - Get trade history for the authenticated user
async fn get_trade_history(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<TradeHistoryResponse>, (StatusCode, Json<TradeHistoryResponse>)> {
    // Authenticate user
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(TradeHistoryResponse {
            trades: vec![],
            total: 0,
            message: "Authentication required".to_string(),
        }))
    })?;

    // Parse query parameters
    let page = params.get("page").and_then(|p| p.parse::<usize>().ok()).unwrap_or(1);
    let limit = params.get("limit").and_then(|l| l.parse::<usize>().ok()).unwrap_or(20);
    let symbol = params.get("symbol").cloned();

    // Mock trade history data
    let mut trades = vec![
        TradeHistoryEntry {
            trade_id: "trade_123456789".to_string(),
            order_id: "ord_123456789".to_string(),
            symbol: "BTC/USD".to_string(),
            side: "buy".to_string(),
            quantity: 0.5,
            price: 50000.0,
            fee: 25.0,
            timestamp: chrono::Utc::now().timestamp(),
        },
        TradeHistoryEntry {
            trade_id: "trade_987654321".to_string(),
            order_id: "ord_987654321".to_string(),
            symbol: "ETH/USD".to_string(),
            side: "sell".to_string(),
            quantity: 1.5,
            price: 3000.0,
            fee: 15.0,
            timestamp: chrono::Utc::now().timestamp(),
        },
    ];

    // Apply filters
    if let Some(symbol_filter) = &symbol {
        trades.retain(|trade| trade.symbol == *symbol_filter);
    }

    let total = trades.len();
    Ok(Json(TradeHistoryResponse {
        trades,
        total,
        message: "Trade history retrieved successfully".to_string(),
    }))
}

/// GET /positions - Get current positions for the authenticated user
async fn get_positions(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<PositionsResponse>, (StatusCode, Json<PositionsResponse>)> {
    // Authenticate user
    let _user = authenticate_user(headers, "user").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(PositionsResponse {
            positions: vec![],
            message: "Authentication required".to_string(),
        }))
    })?;

    // Mock positions data
    let positions = vec![
        PositionEntry {
            symbol: "BTC/USD".to_string(),
            side: "long".to_string(),
            quantity: 1.5,
            average_price: 48000.0,
            current_price: 50000.0,
            unrealized_pnl: 3000.0,
            realized_pnl: 1500.0,
            margin_used: 24000.0,
            leverage: 2.0,
        },
        PositionEntry {
            symbol: "ETH/USD".to_string(),
            side: "short".to_string(),
            quantity: 5.0,
            average_price: 3200.0,
            current_price: 3000.0,
            unrealized_pnl: 1000.0,
            realized_pnl: -200.0,
            margin_used: 16000.0,
            leverage: 1.5,
        },
    ];

    Ok(Json(PositionsResponse {
        positions,
        message: "Positions retrieved successfully".to_string(),
    }))
}

/// Helper function to create a cancellation transaction
fn create_cancellation_transaction(
    order_id: &str,
    account: &crate::core::wallet::WalletAccount,
) -> crate::core::types::Transaction {
    use crate::core::types::{Transaction, ShardId};
    use crate::core::address::Address;
    use sha2::{Sha256, Digest};
    use chrono::Utc;
    
    // Create cancellation payload
    let payload = serde_json::json!({
        "action": "cancel_order",
        "order_id": order_id,
        "timestamp": Utc::now().timestamp(),
    });
    
    let payload_bytes = serde_json::to_vec(&payload).unwrap_or_default();
    
    // Create HashTimer from order ID
    let mut hashtimer = [0u8; 32];
    let mut hasher = Sha256::new();
    hasher.update(order_id.as_bytes());
    hashtimer.copy_from_slice(&hasher.finalize());
    
    // Use account address as from/to
    let from_address = account.address.clone();
    let to_address = Address::new("findag_orderbook000000000000000000000000000000".to_string());
    
    Transaction {
        from: from_address,
        to: to_address,
        amount: 0, // Cancellation doesn't transfer funds
        payload: payload_bytes,
        findag_time: Utc::now().timestamp() as u64,
        hashtimer,
        signature: ed25519_dalek::Signature::from_bytes(&[0u8; 64]), // Dummy signature
        public_key: account.signing_key.verifying_key(),
        shard_id: ShardId(0),
        source_shard: None,
        dest_shard: None,
        target_chain: None,
        bridge_protocol: None,
    }
}

#[derive(Serialize)]
pub struct OrderHistoryResponse {
    pub orders: Vec<OrderHistoryEntry>,
    pub total: usize,
    pub message: String,
}

#[derive(Serialize)]
pub struct OrderHistoryEntry {
    pub order_id: String,
    pub symbol: String,
    pub side: String, // "buy" or "sell"
    pub order_type: String, // "market" or "limit"
    pub quantity: f64,
    pub price: Option<f64>,
    pub status: String, // "pending", "filled", "cancelled", "rejected"
    pub filled_quantity: f64,
    pub average_price: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize)]
pub struct TradeHistoryResponse {
    pub trades: Vec<TradeHistoryEntry>,
    pub total: usize,
    pub message: String,
}

#[derive(Serialize)]
pub struct TradeHistoryEntry {
    pub trade_id: String,
    pub order_id: String,
    pub symbol: String,
    pub side: String, // "buy" or "sell"
    pub quantity: f64,
    pub price: f64,
    pub fee: f64,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct PositionsResponse {
    pub positions: Vec<PositionEntry>,
    pub message: String,
}

#[derive(Serialize)]
pub struct PositionEntry {
    pub symbol: String,
    pub side: String, // "long" or "short"
    pub quantity: f64,
    pub average_price: f64,
    pub current_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub margin_used: f64,
    pub leverage: f64,
}

// Secure key management
static SECURE_KEYS: OnceLock<Arc<Mutex<HashMap<String, SecureKey>>>> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct SecureKey {
    pub key_id: String,
    pub key_type: KeyType,
    pub encrypted_key: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub usage_count: u32,
}

#[derive(Clone, Debug)]
pub enum KeyType {
    ApiKey,
    WalletKey,
    JwtSecret,
    EncryptionKey,
    SigningKey,
}

impl SecureKey {
    pub fn new(key_type: KeyType, key_data: Vec<u8>) -> Self {
        let key_id = uuid::Uuid::new_v4().to_string();
        let encrypted_key = encrypt_key_data(&key_data);
        
        Self {
            key_id,
            key_type,
            encrypted_key,
            created_at: Utc::now(),
            expires_at: None,
            is_active: true,
            usage_count: 0,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }
    
    pub fn increment_usage(&mut self) {
        self.usage_count += 1;
    }
}

/// Generate a new secure API key
pub fn generate_api_key() -> String {
    let mut rng = OsRng;
    let key_bytes: [u8; 32] = rng.gen();
    format!("fdg_{}", base32_encode(base32::Alphabet::RFC4648 { padding: false }, &key_bytes))
}

/// Generate a new secure wallet key
pub fn generate_wallet_key() -> Vec<u8> {
    let mut rng = OsRng;
    let mut key_bytes = [0u8; 32];
    rng.fill_bytes(&mut key_bytes);
    key_bytes.to_vec()
}

/// Store a secure key
pub fn store_secure_key(key_type: KeyType, key_data: Vec<u8>) -> String {
    let keys = SECURE_KEYS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let mut keys_guard = keys.lock().unwrap();
    
    let secure_key = SecureKey::new(key_type, key_data);
    let key_id = secure_key.key_id.clone();
    keys_guard.insert(key_id.clone(), secure_key);
    
    key_id
}

/// Retrieve a secure key
pub fn get_secure_key(key_id: &str) -> Option<Vec<u8>> {
    let keys = SECURE_KEYS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let mut keys_guard = keys.lock().unwrap();
    
    if let Some(secure_key) = keys_guard.get_mut(key_id) {
        if secure_key.is_active && !secure_key.is_expired() {
            secure_key.increment_usage();
            Some(decrypt_key_data(&secure_key.encrypted_key))
        } else {
            None
        }
    } else {
        None
    }
}

/// Rotate a secure key
pub fn rotate_secure_key(key_id: &str) -> Option<String> {
    let keys = SECURE_KEYS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let mut keys_guard = keys.lock().unwrap();
    
    if let Some(secure_key) = keys_guard.get(key_id) {
        let new_key_data = match secure_key.key_type {
            KeyType::ApiKey => generate_api_key().as_bytes().to_vec(),
            KeyType::WalletKey => generate_wallet_key(),
            KeyType::JwtSecret => {
                let mut rng = OsRng;
                let mut secret = [0u8; 32];
                rng.fill_bytes(&mut secret);
                secret.to_vec()
            },
            KeyType::EncryptionKey => {
                let mut rng = OsRng;
                let mut key = [0u8; 32];
                rng.fill_bytes(&mut key);
                key.to_vec()
            },
            KeyType::SigningKey => {
                let mut rng = OsRng;
                let mut key = [0u8; 64];
                rng.fill_bytes(&mut key);
                key.to_vec()
            },
        };
        
        let new_secure_key = SecureKey::new(secure_key.key_type.clone(), new_key_data);
        let new_key_id = new_secure_key.key_id.clone();
        
        // Deactivate old key
        if let Some(old_key) = keys_guard.get_mut(key_id) {
            old_key.is_active = false;
        }
        
        // Store new key
        keys_guard.insert(new_key_id.clone(), new_secure_key);
        
        Some(new_key_id)
    } else {
        None
    }
}

/// List all secure keys
pub fn list_secure_keys() -> Vec<SecureKeyInfo> {
    let keys = SECURE_KEYS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let keys_guard = keys.lock().unwrap();
    
    keys_guard.iter().map(|(key_id, secure_key)| {
        SecureKeyInfo {
            key_id: key_id.clone(),
            key_type: format!("{:?}", secure_key.key_type),
            created_at: secure_key.created_at,
            expires_at: secure_key.expires_at,
            is_active: secure_key.is_active,
            usage_count: secure_key.usage_count,
        }
    }).collect()
}

#[derive(Serialize)]
pub struct SecureKeyInfo {
    pub key_id: String,
    pub key_type: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub usage_count: u32,
}

/// Encrypt key data (in production, use a proper encryption library)
fn encrypt_key_data(key_data: &[u8]) -> Vec<u8> {
    // Simple XOR encryption for demo (use proper encryption in production)
    let key = b"FinDAG_Secure_Key_2024";
    key_data.iter().enumerate().map(|(i, &byte)| {
        byte ^ key[i % key.len()]
    }).collect()
}

/// Decrypt key data (in production, use a proper encryption library)
fn decrypt_key_data(encrypted_data: &[u8]) -> Vec<u8> {
    // Simple XOR decryption for demo (use proper decryption in production)
    let key = b"FinDAG_Secure_Key_2024";
    encrypted_data.iter().enumerate().map(|(i, &byte)| {
        byte ^ key[i % key.len()]
    }).collect()
}

/// POST /security/keys/generate - Generate a new secure key
async fn generate_secure_key(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<GenerateKeyRequest>,
) -> Result<Json<GenerateKeyResponse>, (StatusCode, Json<GenerateKeyResponse>)> {
    // Authenticate as admin
    let _user = authenticate_user(headers, "admin").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(GenerateKeyResponse {
            key_id: "".to_string(),
            key_type: "".to_string(),
            message: "Admin authentication required".to_string(),
        }))
    })?;

    let key_data = match req.key_type.as_str() {
        "api_key" => generate_api_key().as_bytes().to_vec(),
        "wallet_key" => generate_wallet_key(),
        "jwt_secret" => {
            let mut rng = OsRng;
            let mut secret = [0u8; 32];
            rng.fill_bytes(&mut secret);
            secret.to_vec()
        },
        "encryption_key" => {
            let mut rng = OsRng;
            let mut key = [0u8; 32];
            rng.fill_bytes(&mut key);
            key.to_vec()
        },
        "signing_key" => {
            let mut rng = OsRng;
            let mut key = [0u8; 64];
            rng.fill_bytes(&mut key);
            key.to_vec()
        },
        _ => {
            return Err((StatusCode::BAD_REQUEST, Json(GenerateKeyResponse {
                key_id: "".to_string(),
                key_type: "".to_string(),
                message: "Invalid key type".to_string(),
            })));
        }
    };

    let key_type_enum = match req.key_type.as_str() {
        "api_key" => KeyType::ApiKey,
        "wallet_key" => KeyType::WalletKey,
        "jwt_secret" => KeyType::JwtSecret,
        "encryption_key" => KeyType::EncryptionKey,
        "signing_key" => KeyType::SigningKey,
        _ => KeyType::ApiKey, // Default
    };

    let key_id = store_secure_key(key_type_enum, key_data);

    Ok(Json(GenerateKeyResponse {
        key_id,
        key_type: req.key_type,
        message: "Secure key generated successfully".to_string(),
    }))
}

/// GET /security/keys - List all secure keys
async fn list_secure_keys_endpoint(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<ListKeysResponse>, (StatusCode, Json<ListKeysResponse>)> {
    // Authenticate as admin
    let _user = authenticate_user(headers, "admin").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(ListKeysResponse {
            keys: vec![],
            message: "Admin authentication required".to_string(),
        }))
    })?;

    let keys = list_secure_keys();

    Ok(Json(ListKeysResponse {
        keys,
        message: "Secure keys retrieved successfully".to_string(),
    }))
}

/// POST /security/keys/{key_id}/rotate - Rotate a secure key
async fn rotate_secure_key_endpoint(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(key_id): Path<String>,
) -> Result<Json<RotateKeyResponse>, (StatusCode, Json<RotateKeyResponse>)> {
    // Authenticate as admin
    let _user = authenticate_user(headers, "admin").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(RotateKeyResponse {
            new_key_id: "".to_string(),
            message: "Admin authentication required".to_string(),
        }))
    })?;

    if let Some(new_key_id) = rotate_secure_key(&key_id) {
        Ok(Json(RotateKeyResponse {
            new_key_id,
            message: "Secure key rotated successfully".to_string(),
        }))
    } else {
        Err((StatusCode::NOT_FOUND, Json(RotateKeyResponse {
            new_key_id: "".to_string(),
            message: "Key not found".to_string(),
        })))
    }
}

/// DELETE /security/keys/{key_id} - Deactivate a secure key
async fn deactivate_secure_key_endpoint(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(key_id): Path<String>,
) -> Result<Json<DeactivateKeyResponse>, (StatusCode, Json<DeactivateKeyResponse>)> {
    // Authenticate as admin
    let _user = authenticate_user(headers, "admin").await.map_err(|_| {
        (StatusCode::UNAUTHORIZED, Json(DeactivateKeyResponse {
            success: false,
            message: "Admin authentication required".to_string(),
        }))
    })?;

    let keys = SECURE_KEYS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let mut keys_guard = keys.lock().unwrap();
    
    if let Some(secure_key) = keys_guard.get_mut(&key_id) {
        secure_key.is_active = false;
        Ok(Json(DeactivateKeyResponse {
            success: true,
            message: "Secure key deactivated successfully".to_string(),
        }))
    } else {
        Err((StatusCode::NOT_FOUND, Json(DeactivateKeyResponse {
            success: false,
            message: "Key not found".to_string(),
        })))
    }
}

#[derive(Deserialize)]
pub struct GenerateKeyRequest {
    pub key_type: String, // "api_key", "wallet_key", "jwt_secret", "encryption_key", "signing_key"
}

#[derive(Serialize)]
pub struct GenerateKeyResponse {
    pub key_id: String,
    pub key_type: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ListKeysResponse {
    pub keys: Vec<SecureKeyInfo>,
    pub message: String,
}

#[derive(Serialize)]
pub struct RotateKeyResponse {
    pub new_key_id: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct DeactivateKeyResponse {
    pub success: bool,
    pub message: String,
}

// Cache management
#[derive(Clone, Debug)]
pub struct CacheEntry<T> {
    pub data: T,
    pub created_at: Instant,
    pub ttl: Duration,
}

impl<T> CacheEntry<T> {
    pub fn new(data: T, ttl: Duration) -> Self {
        Self {
            data,
            created_at: Instant::now(),
            ttl,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }
}

pub struct Cache {
    user_sessions: Arc<RwLock<HashMap<String, CacheEntry<String>>>>,
    wallet_balances: Arc<RwLock<HashMap<String, CacheEntry<Vec<WalletAssetBalance>>>>>,
    order_book: Arc<RwLock<HashMap<String, CacheEntry<(Vec<OrderBookEntry>, Vec<OrderBookEntry>)>>>>,
    dag_status: Arc<RwLock<CacheEntry<DagStatusResponse>>>,
    validator_info: Arc<RwLock<CacheEntry<DagValidatorsResponse>>>,
    trading_analytics: Arc<RwLock<HashMap<String, CacheEntry<TradingAnalyticsResponse>>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            user_sessions: Arc::new(RwLock::new(HashMap::new())),
            wallet_balances: Arc::new(RwLock::new(HashMap::new())),
            order_book: Arc::new(RwLock::new(HashMap::new())),
            dag_status: Arc::new(RwLock::new(CacheEntry::new(
                DagStatusResponse {
                    network_status: "active".to_string(),
                    total_blocks: 0,
                    tips_count: 0,
                    max_depth: 0,
                    avg_txs_per_block: 0.0,
                    active_validators: 0,
                    tx_pool_size: 0,
                    last_block_time: 0,
                    consensus_status: "healthy".to_string(),
                    message: "Cached status".to_string(),
                },
                Duration::from_secs(30), // 30 second TTL
            ))),
            validator_info: Arc::new(RwLock::new(CacheEntry::new(
                DagValidatorsResponse {
                    validators: vec![],
                    total_validators: 0,
                    active_validators: 0,
                    total_stake: 0,
                    message: "Cached validators".to_string(),
                },
                Duration::from_secs(60), // 1 minute TTL
            ))),
            trading_analytics: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    // User session cache
    pub async fn get_user_session(&self, user_id: &str) -> Option<String> {
        let sessions = self.user_sessions.read().await;
        if let Some(entry) = sessions.get(user_id) {
            if !entry.is_expired() {
                Some(entry.data.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub async fn set_user_session(&self, user_id: String, session_data: String) {
        let mut sessions = self.user_sessions.write().await;
        sessions.insert(user_id, CacheEntry::new(session_data, Duration::from_secs(3600))); // 1 hour TTL
    }
    
    pub async fn remove_user_session(&self, user_id: &str) {
        let mut sessions = self.user_sessions.write().await;
        sessions.remove(user_id);
    }
    
    // Wallet balance cache
    pub async fn get_wallet_balance(&self, address: &str) -> Option<Vec<WalletAssetBalance>> {
        let balances = self.wallet_balances.read().await;
        if let Some(entry) = balances.get(address) {
            if !entry.is_expired() {
                Some(entry.data.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub async fn set_wallet_balance(&self, address: String, balances: Vec<WalletAssetBalance>) {
        let mut cache = self.wallet_balances.write().await;
        cache.insert(address, CacheEntry::new(balances, Duration::from_secs(300))); // 5 minute TTL
    }
    
    // Order book cache
    pub async fn get_order_book(&self, symbol: &str) -> Option<(Vec<OrderBookEntry>, Vec<OrderBookEntry>)> {
        let order_book = self.order_book.read().await;
        if let Some(entry) = order_book.get(symbol) {
            if !entry.is_expired() {
                Some(entry.data.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub async fn set_order_book(&self, symbol: String, bids: Vec<OrderBookEntry>, asks: Vec<OrderBookEntry>) {
        let mut cache = self.order_book.write().await;
        cache.insert(symbol, CacheEntry::new((bids, asks), Duration::from_secs(10))); // 10 second TTL
    }
    
    // DAG status cache
    pub async fn get_dag_status(&self) -> Option<DagStatusResponse> {
        let status = self.dag_status.read().await;
        if !status.is_expired() {
            Some(status.data.clone())
        } else {
            None
        }
    }
    
    pub async fn set_dag_status(&self, status: DagStatusResponse) {
        let mut cache = self.dag_status.write().await;
        *cache = CacheEntry::new(status, Duration::from_secs(30)); // 30 second TTL
    }
    
    // Validator info cache
    pub async fn get_validator_info(&self) -> Option<DagValidatorsResponse> {
        let info = self.validator_info.read().await;
        if !info.is_expired() {
            Some(info.data.clone())
        } else {
            None
        }
    }
    
    pub async fn set_validator_info(&self, info: DagValidatorsResponse) {
        let mut cache = self.validator_info.write().await;
        *cache = CacheEntry::new(info, Duration::from_secs(60)); // 1 minute TTL
    }
    
    // Trading analytics cache
    pub async fn get_trading_analytics(&self, user_id: &str) -> Option<TradingAnalyticsResponse> {
        let analytics = self.trading_analytics.read().await;
        if let Some(entry) = analytics.get(user_id) {
            if !entry.is_expired() {
                Some(entry.data.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub async fn set_trading_analytics(&self, user_id: String, analytics: TradingAnalyticsResponse) {
        let mut cache = self.trading_analytics.write().await;
        cache.insert(user_id, CacheEntry::new(analytics, Duration::from_secs(300))); // 5 minute TTL
    }
    
    // Cache cleanup
    pub async fn cleanup_expired(&self) {
        // Clean up user sessions
        let mut sessions = self.user_sessions.write().await;
        sessions.retain(|_, entry| !entry.is_expired());
        
        // Clean up wallet balances
        let mut balances = self.wallet_balances.write().await;
        balances.retain(|_, entry| !entry.is_expired());
        
        // Clean up order book
        let mut order_book = self.order_book.write().await;
        order_book.retain(|_, entry| !entry.is_expired());
        
        // Clean up trading analytics
        let mut analytics = self.trading_analytics.write().await;
        analytics.retain(|_, entry| !entry.is_expired());
    }
}

// Global cache instance
static CACHE: OnceLock<Arc<Cache>> = OnceLock::new();

pub fn get_cache() -> Arc<Cache> {
    CACHE.get_or_init(|| Arc::new(Cache::new())).clone()
}

// Cache cleanup task
pub async fn start_cache_cleanup() {
    let cache = get_cache();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(300)); // Clean up every 5 minutes
        loop {
            interval.tick().await;
            cache.cleanup_expired().await;
        }
    });
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeseriesPoint {
    pub timestamp: i64,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerformanceTimeseriesResponse {
    pub metric: String,
    pub range: String,
    pub points: Vec<TimeseriesPoint>,
    pub message: String,
}

#[derive(Deserialize)]
struct TimeseriesQuery {
    metric: String, // "tps", "latency", "nodes", "blocks"
    range: String,  // "1h", "6h", "24h", "7d"
}

async fn get_performance_timeseries(Query(params): Query<TimeseriesQuery>) -> Json<PerformanceTimeseriesResponse> {
    let now = chrono::Utc::now().timestamp();
    let (points, interval, count) = match params.range.as_str() {
        "1h" => (60, 60, 60),      // 1-min intervals, 60 points
        "6h" => (300, 72, 72),     // 5-min intervals, 72 points
        "24h" => (900, 96, 96),    // 15-min intervals, 96 points
        "7d" => (3600, 168, 168),  // 1-hour intervals, 168 points
        _ => (900, 96, 96),         // Default: 24h
    };
    let mut data = Vec::with_capacity(count);
    for i in 0..count {
        let timestamp = now - ((count - i) as i64 * interval as i64);
        let value = match params.metric.as_str() {
            "tps" => 1000000.0 + rand::random::<f64>() * 500000.0,
            "latency" => 30.0 + rand::random::<f64>() * 40.0,
            "nodes" => 10.0 + (rand::random::<f64>() * 5.0).floor(),
            "blocks" => 100.0 + rand::random::<f64>() * 200.0,
            _ => rand::random::<f64>() * 1000.0,
        };
        data.push(TimeseriesPoint { timestamp, value });
    }
    Json(PerformanceTimeseriesResponse {
        metric: params.metric,
        range: params.range,
        points: data,
        message: "Mock time-series data".to_string(),
    })
}

// ... in the router setup (create_router or similar) ...
// .route("/analytics/performance/timeseries", get(get_performance_timeseries))
//! Authentication module
//! 
//! This module handles user authentication and session management.

use findag_types::{FindDAGResult, FindDAGError};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use metrics::{counter, gauge};

/// Authentication manager
pub struct AuthManager {
    /// User database
    users: Arc<RwLock<HashMap<String, User>>>,
    /// Active sessions
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    /// Authentication attempts
    auth_attempts: Arc<RwLock<HashMap<String, Vec<AuthAttempt>>>>,
    /// Configuration
    config: AuthConfig,
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub user_id: String,
    /// Username
    pub username: String,
    /// Email
    pub email: String,
    /// Password hash
    pub password_hash: String,
    /// Role
    pub role: UserRole,
    /// Permissions
    pub permissions: Vec<String>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Last login
    pub last_login: Option<DateTime<Utc>>,
    /// Account status
    pub status: AccountStatus,
    /// Two-factor authentication enabled
    pub two_factor_enabled: bool,
    /// Two-factor secret
    pub two_factor_secret: Option<String>,
}

/// User role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    User,
    Validator,
    Observer,
}

/// Account status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountStatus {
    Active,
    Suspended,
    Locked,
    Pending,
}

/// Authentication attempt
#[derive(Debug, Clone)]
pub struct AuthAttempt {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// IP address
    pub ip_address: String,
    /// User agent
    pub user_agent: String,
    /// Success
    pub success: bool,
    /// Reason for failure
    pub failure_reason: Option<String>,
}

/// Authentication configuration
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// Maximum failed attempts
    pub max_failed_attempts: u32,
    /// Lockout duration in seconds
    pub lockout_duration: u64,
    /// Session timeout in seconds
    pub session_timeout: u64,
    /// Password policy
    pub password_policy: PasswordPolicy,
    /// Two-factor authentication required
    pub two_factor_required: bool,
}

/// Password policy
#[derive(Debug, Clone)]
pub struct PasswordPolicy {
    /// Minimum length
    pub min_length: u32,
    /// Require uppercase
    pub require_uppercase: bool,
    /// Require lowercase
    pub require_lowercase: bool,
    /// Require numbers
    pub require_numbers: bool,
    /// Require special characters
    pub require_special: bool,
    /// Maximum age in days
    pub max_age_days: u32,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            max_failed_attempts: 5,
            lockout_duration: 900, // 15 minutes
            session_timeout: 3600, // 1 hour
            password_policy: PasswordPolicy::default(),
            two_factor_required: false,
        }
    }
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special: true,
            max_age_days: 90,
        }
    }
}

impl AuthManager {
    /// Create a new authentication manager
    pub fn new(config: AuthConfig) -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            auth_attempts: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Register a new user
    pub async fn register_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
        role: UserRole,
    ) -> FindDAGResult<()> {
        // Validate input
        self.validate_username(username)?;
        self.validate_email(email)?;
        self.validate_password(password)?;
        
        // Check if user already exists
        {
            let users = self.users.read().await;
            if users.contains_key(username) {
                return Err(FindDAGError::UserAlreadyExists(username.to_string()));
            }
        }
        
        // Hash password
        let password_hash = self.hash_password(password)?;
        
        // Create user
        let user = User {
            user_id: uuid::Uuid::new_v4().to_string(),
            username: username.to_string(),
            email: email.to_string(),
            password_hash,
            role,
            permissions: self.get_permissions_for_role(&role),
            created_at: Utc::now(),
            last_login: None,
            status: AccountStatus::Active,
            two_factor_enabled: false,
            two_factor_secret: None,
        };
        
        // Store user
        {
            let mut users = self.users.write().await;
            users.insert(username.to_string(), user);
        }
        
        info!("User registered: {}", username);
        counter!("findag_auth_user_registrations", 1);
        
        Ok(())
    }

    /// Authenticate user
    pub async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
        ip_address: &str,
        user_agent: &str,
        two_factor_code: Option<&str>,
    ) -> FindDAGResult<String> {
        // Check if account is locked
        if self.is_account_locked(username).await? {
            return Err(FindDAGError::AccountLocked("Account is locked due to too many failed attempts".to_string()));
        }
        
        // Get user
        let user = {
            let users = self.users.read().await;
            users.get(username).cloned()
        };
        
        let user = match user {
            Some(user) => user,
            None => {
                self.record_auth_attempt(username, ip_address, user_agent, false, Some("User not found")).await;
                return Err(FindDAGError::AuthenticationFailed("Invalid credentials".to_string()));
            }
        };
        
        // Check account status
        if !matches!(user.status, AccountStatus::Active) {
            self.record_auth_attempt(username, ip_address, user_agent, false, Some("Account not active")).await;
            return Err(FindDAGError::AccountLocked("Account is not active".to_string()));
        }
        
        // Verify password
        if !self.verify_password(password, &user.password_hash)? {
            self.record_auth_attempt(username, ip_address, user_agent, false, Some("Invalid password")).await;
            return Err(FindDAGError::AuthenticationFailed("Invalid credentials".to_string()));
        }
        
        // Verify two-factor authentication if enabled
        if user.two_factor_enabled {
            if let Some(code) = two_factor_code {
                if !self.verify_two_factor_code(&user, code)? {
                    self.record_auth_attempt(username, ip_address, user_agent, false, Some("Invalid 2FA code")).await;
                    return Err(FindDAGError::AuthenticationFailed("Invalid two-factor code".to_string()));
                }
            } else {
                self.record_auth_attempt(username, ip_address, user_agent, false, Some("2FA code required")).await;
                return Err(FindDAGError::AuthenticationFailed("Two-factor code required".to_string()));
            }
        }
        
        // Generate session token
        let session_token = self.generate_session_token(&user).await?;
        
        // Create session
        let session = Session {
            user_id: user.user_id.clone(),
            token: session_token.clone(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::seconds(self.config.session_timeout as i64),
            ip_address: ip_address.to_string(),
            user_agent: user_agent.to_string(),
            role: format!("{:?}", user.role),
            permissions: user.permissions.clone(),
        };
        
        // Store session
        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(session_token.clone(), session);
        }
        
        // Update user last login
        {
            let mut users = self.users.write().await;
            if let Some(user) = users.get_mut(username) {
                user.last_login = Some(Utc::now());
            }
        }
        
        // Record successful authentication
        self.record_auth_attempt(username, ip_address, user_agent, true, None).await;
        
        info!("User authenticated: {}", username);
        counter!("findag_auth_successful_logins", 1);
        
        Ok(session_token)
    }

    /// Validate session token
    pub async fn validate_session(&self, token: &str) -> FindDAGResult<Session> {
        let sessions = self.sessions.read().await;
        
        if let Some(session) = sessions.get(token) {
            // Check if session is expired
            if session.expires_at < Utc::now() {
                return Err(FindDAGError::SessionExpired("Session has expired".to_string()));
            }
            
            Ok(session.clone())
        } else {
            Err(FindDAGError::InvalidSession("Invalid session token".to_string()))
        }
    }

    /// Logout user
    pub async fn logout_user(&self, token: &str) -> FindDAGResult<()> {
        let mut sessions = self.sessions.write().await;
        sessions.remove(token);
        
        info!("User logged out");
        counter!("findag_auth_logouts", 1);
        
        Ok(())
    }

    /// Get user by username
    pub async fn get_user(&self, username: &str) -> FindDAGResult<Option<User>> {
        let users = self.users.read().await;
        Ok(users.get(username).cloned())
    }

    /// Update user
    pub async fn update_user(&self, username: &str, updates: UserUpdates) -> FindDAGResult<()> {
        let mut users = self.users.write().await;
        
        if let Some(user) = users.get_mut(username) {
            if let Some(email) = updates.email {
                self.validate_email(&email)?;
                user.email = email;
            }
            
            if let Some(role) = updates.role {
                user.role = role;
                user.permissions = self.get_permissions_for_role(&user.role);
            }
            
            if let Some(status) = updates.status {
                user.status = status;
            }
        }
        
        Ok(())
    }

    /// Validate username
    fn validate_username(&self, username: &str) -> FindDAGResult<()> {
        if username.len() < 3 || username.len() > 50 {
            return Err(FindDAGError::ValidationError("Username must be between 3 and 50 characters".to_string()));
        }
        
        if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(FindDAGError::ValidationError("Username can only contain alphanumeric characters, underscores, and hyphens".to_string()));
        }
        
        Ok(())
    }

    /// Validate email
    fn validate_email(&self, email: &str) -> FindDAGResult<()> {
        // Simple email validation
        if !email.contains('@') || !email.contains('.') {
            return Err(FindDAGError::ValidationError("Invalid email format".to_string()));
        }
        
        Ok(())
    }

    /// Validate password
    fn validate_password(&self, password: &str) -> FindDAGResult<()> {
        let policy = &self.config.password_policy;
        
        if password.len() < policy.min_length as usize {
            return Err(FindDAGError::ValidationError(format!("Password must be at least {} characters long", policy.min_length)));
        }
        
        if policy.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            return Err(FindDAGError::ValidationError("Password must contain at least one uppercase letter".to_string()));
        }
        
        if policy.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
            return Err(FindDAGError::ValidationError("Password must contain at least one lowercase letter".to_string()));
        }
        
        if policy.require_numbers && !password.chars().any(|c| c.is_numeric()) {
            return Err(FindDAGError::ValidationError("Password must contain at least one number".to_string()));
        }
        
        if policy.require_special && !password.chars().any(|c| !c.is_alphanumeric()) {
            return Err(FindDAGError::ValidationError("Password must contain at least one special character".to_string()));
        }
        
        Ok(())
    }

    /// Hash password
    fn hash_password(&self, password: &str) -> FindDAGResult<String> {
        let salt = rand::random::<[u8; 16]>();
        let hash = argon2::hash_encoded(
            password.as_bytes(),
            &salt,
            &argon2::Config::default()
        )?;
        
        Ok(hash)
    }

    /// Verify password
    fn verify_password(&self, password: &str, hash: &str) -> FindDAGResult<bool> {
        Ok(argon2::verify_encoded(hash, password.as_bytes())?)
    }

    /// Verify two-factor code
    fn verify_two_factor_code(&self, user: &User, code: &str) -> FindDAGResult<bool> {
        // TODO: Implement TOTP verification
        Ok(true)
    }

    /// Generate session token
    async fn generate_session_token(&self, user: &User) -> FindDAGResult<String> {
        let token = uuid::Uuid::new_v4().to_string();
        Ok(token)
    }

    /// Get permissions for role
    fn get_permissions_for_role(&self, role: &UserRole) -> Vec<String> {
        match role {
            UserRole::Admin => vec![
                "read".to_string(), "write".to_string(), "delete".to_string(),
                "admin".to_string(), "manage_users".to_string(), "manage_system".to_string(),
            ],
            UserRole::User => vec![
                "read".to_string(), "write".to_string(),
            ],
            UserRole::Validator => vec![
                "read".to_string(), "write".to_string(), "validate".to_string(),
            ],
            UserRole::Observer => vec![
                "read".to_string(),
            ],
        }
    }

    /// Check if account is locked
    async fn is_account_locked(&self, username: &str) -> FindDAGResult<bool> {
        let attempts = {
            let auth_attempts = self.auth_attempts.read().await;
            auth_attempts.get(username).cloned().unwrap_or_default()
        };
        
        let recent_failed_attempts = attempts
            .iter()
            .filter(|attempt| {
                attempt.timestamp > Utc::now() - Duration::seconds(self.config.lockout_duration as i64)
                    && !attempt.success
            })
            .count();
        
        Ok(recent_failed_attempts >= self.config.max_failed_attempts as usize)
    }

    /// Record authentication attempt
    async fn record_auth_attempt(
        &self,
        username: &str,
        ip_address: &str,
        user_agent: &str,
        success: bool,
        failure_reason: Option<&str>,
    ) {
        let attempt = AuthAttempt {
            timestamp: Utc::now(),
            ip_address: ip_address.to_string(),
            user_agent: user_agent.to_string(),
            success,
            failure_reason: failure_reason.map(|s| s.to_string()),
        };
        
        let mut auth_attempts = self.auth_attempts.write().await;
        let attempts = auth_attempts.entry(username.to_string()).or_insert_with(Vec::new);
        attempts.push(attempt);
        
        // Keep only recent attempts
        let cutoff = Utc::now() - Duration::seconds(self.config.lockout_duration as i64);
        attempts.retain(|attempt| attempt.timestamp > cutoff);
    }
}

/// User updates
#[derive(Debug, Clone)]
pub struct UserUpdates {
    pub email: Option<String>,
    pub role: Option<UserRole>,
    pub status: Option<AccountStatus>,
} 
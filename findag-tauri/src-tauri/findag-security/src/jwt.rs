//! JWT token management
//! 
//! This module handles JWT token generation, validation, and management.

use findag_types::{FindDAGResult, FindDAGError};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use metrics::{counter, histogram};

/// JWT manager
pub struct JWTManager {
    /// JWT configuration
    config: JWTConfig,
    /// Token blacklist
    blacklist: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
    /// Token cache
    token_cache: Arc<RwLock<HashMap<String, CachedToken>>>,
}

/// JWT configuration
#[derive(Debug, Clone)]
pub struct JWTConfig {
    /// Secret key
    pub secret: String,
    /// Algorithm
    pub algorithm: Algorithm,
    /// Token expiration in seconds
    pub expiration_seconds: i64,
    /// Refresh token expiration in seconds
    pub refresh_expiration_seconds: i64,
    /// Issuer
    pub issuer: String,
    /// Audience
    pub audience: String,
    /// Enable token caching
    pub enable_caching: bool,
    /// Cache size limit
    pub cache_size_limit: usize,
}

/// Cached token
#[derive(Debug, Clone)]
pub struct CachedToken {
    /// Claims
    pub claims: Claims,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Last accessed
    pub last_accessed: DateTime<Utc>,
}

/// JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Issuer
    pub iss: String,
    /// Audience
    pub aud: String,
    /// Expiration time
    pub exp: i64,
    /// Issued at
    pub iat: i64,
    /// Not before
    pub nbf: Option<i64>,
    /// JWT ID
    pub jti: Option<String>,
    /// Role
    pub role: String,
    /// Permissions
    pub permissions: Vec<String>,
}

impl JWTManager {
    /// Create a new JWT manager
    pub fn new(config: JWTConfig) -> Self {
        Self {
            config,
            blacklist: Arc::new(RwLock::new(HashMap::new())),
            token_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Generate access token
    pub async fn generate_access_token(
        &self,
        user_id: &str,
        role: &str,
        permissions: &[String],
    ) -> FindDAGResult<String> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.config.expiration_seconds);
        
        let claims = Claims {
            sub: user_id.to_string(),
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            nbf: Some(now.timestamp()),
            jti: Some(uuid::Uuid::new_v4().to_string()),
            role: role.to_string(),
            permissions: permissions.to_vec(),
        };
        
        let token = encode(
            &Header::new(self.config.algorithm),
            &claims,
            &EncodingKey::from_secret(self.config.secret.as_ref())
        )?;
        
        // Cache token if enabled
        if self.config.enable_caching {
            self.cache_token(&token, &claims).await;
        }
        
        counter!("findag_jwt_tokens_generated", 1);
        
        Ok(token)
    }

    /// Generate refresh token
    pub async fn generate_refresh_token(&self, user_id: &str) -> FindDAGResult<String> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.config.refresh_expiration_seconds);
        
        let claims = Claims {
            sub: user_id.to_string(),
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            nbf: Some(now.timestamp()),
            jti: Some(uuid::Uuid::new_v4().to_string()),
            role: "refresh".to_string(),
            permissions: vec!["refresh".to_string()],
        };
        
        let token = encode(
            &Header::new(self.config.algorithm),
            &claims,
            &EncodingKey::from_secret(self.config.secret.as_ref())
        )?;
        
        counter!("findag_jwt_refresh_tokens_generated", 1);
        
        Ok(token)
    }

    /// Validate token
    pub async fn validate_token(&self, token: &str) -> FindDAGResult<Claims> {
        let start_time = std::time::Instant::now();
        
        // Check blacklist
        {
            let blacklist = self.blacklist.read().await;
            if blacklist.contains_key(token) {
                return Err(FindDAGError::TokenBlacklisted("Token is blacklisted".to_string()));
            }
        }
        
        // Check cache first
        if self.config.enable_caching {
            if let Some(cached) = self.get_cached_token(token).await {
                return Ok(cached.claims);
            }
        }
        
        // Validate token
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.secret.as_ref()),
            &Validation::new(self.config.algorithm)
        )?;
        
        // Cache token if enabled
        if self.config.enable_caching {
            self.cache_token(token, &token_data.claims).await;
        }
        
        // Record metrics
        let latency = start_time.elapsed().as_millis() as f64;
        counter!("findag_jwt_tokens_validated", 1);
        histogram!("findag_jwt_validation_latency_ms", latency);
        
        Ok(token_data.claims)
    }

    /// Blacklist token
    pub async fn blacklist_token(&self, token: &str) -> FindDAGResult<()> {
        // Validate token first to get expiration
        let claims = self.validate_token(token).await?;
        let exp = DateTime::from_timestamp(claims.exp, 0).unwrap_or_else(|| Utc::now());
        
        // Add to blacklist
        {
            let mut blacklist = self.blacklist.write().await;
            blacklist.insert(token.to_string(), exp);
        }
        
        // Remove from cache
        if self.config.enable_caching {
            self.remove_cached_token(token).await;
        }
        
        counter!("findag_jwt_tokens_blacklisted", 1);
        
        Ok(())
    }

    /// Refresh token
    pub async fn refresh_token(&self, refresh_token: &str) -> FindDAGResult<String> {
        // Validate refresh token
        let claims = self.validate_token(refresh_token).await?;
        
        if claims.role != "refresh" {
            return Err(FindDAGError::InvalidToken("Not a refresh token".to_string()));
        }
        
        // Generate new access token
        let new_token = self.generate_access_token(
            &claims.sub,
            "user", // Default role
            &vec!["read".to_string(), "write".to_string()],
        ).await?;
        
        counter!("findag_jwt_tokens_refreshed", 1);
        
        Ok(new_token)
    }

    /// Cache token
    async fn cache_token(&self, token: &str, claims: &Claims) {
        let mut cache = self.token_cache.write().await;
        
        // Check cache size limit
        if cache.len() >= self.config.cache_size_limit {
            // Remove oldest entry
            let oldest_key = cache.iter()
                .min_by_key(|(_, cached)| cached.last_accessed)
                .map(|(key, _)| key.clone());
            
            if let Some(key) = oldest_key {
                cache.remove(&key);
            }
        }
        
        let cached = CachedToken {
            claims: claims.clone(),
            created_at: Utc::now(),
            last_accessed: Utc::now(),
        };
        
        cache.insert(token.to_string(), cached);
    }

    /// Get cached token
    async fn get_cached_token(&self, token: &str) -> Option<CachedToken> {
        let mut cache = self.token_cache.write().await;
        
        if let Some(cached) = cache.get_mut(token) {
            cached.last_accessed = Utc::now();
            Some(cached.clone())
        } else {
            None
        }
    }

    /// Remove cached token
    async fn remove_cached_token(&self, token: &str) {
        let mut cache = self.token_cache.write().await;
        cache.remove(token);
    }

    /// Clear cache
    pub async fn clear_cache(&self) {
        let mut cache = self.token_cache.write().await;
        cache.clear();
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> (usize, usize) {
        let cache = self.token_cache.read().await;
        (cache.len(), self.config.cache_size_limit)
    }

    /// Clean up expired tokens
    pub async fn cleanup_expired_tokens(&self) {
        let now = Utc::now();
        
        // Clean up blacklist
        {
            let mut blacklist = self.blacklist.write().await;
            blacklist.retain(|_, exp| *exp > now);
        }
        
        // Clean up cache
        {
            let mut cache = self.token_cache.write().await;
            cache.retain(|_, cached| {
                let exp = DateTime::from_timestamp(cached.claims.exp, 0).unwrap_or_else(|| Utc::now());
                exp > now
            });
        }
    }
} 
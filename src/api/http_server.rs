use axum::{routing::{get, post, delete}, extract::{Path, Query}, Json, Router};
use serde::{Deserialize, Serialize};
use crate::consensus::validator_set::{ValidatorSet, ValidatorStatus};
use crate::core::address::Address;
use ed25519_dalek::{VerifyingKey, Signer, SigningKey};
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
use axum::http::{StatusCode, HeaderMap};
use crate::core::tx_pool::ShardedTxPool;
use crate::network::propagation::NetworkPropagator;
use rand;
use rand_core::RngCore;
use axum::response::IntoResponse;
use axum::extract::State;
use std::env;
use ed25519_dalek::Verifier;

use tokio::net::TcpListener;
use std::time::{Duration, Instant};
// ADMIN_TOKEN is used for admin authentication - keeping for future use
#[allow(dead_code)]
static ADMIN_TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("ADMIN_TOKEN").unwrap_or_else(|_| "changeme".to_string())
});
static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    env::var("JWT_SECRET").unwrap_or_else(|_| {
        let secret: [u8; 32] = rand::random();
        hex::encode(secret)
    })
});
// Removed BridgeManager - not defined in core::bridge

// Shared application state for dependency injection
pub struct AppState {
    pub validator_set: Arc<Mutex<ValidatorSet>>,
    pub storage: Arc<crate::storage::persistent::PersistentStorage>,
    pub governance_state: Arc<Mutex<crate::consensus::governance::GovernanceState>>,
    pub tx_pool: Arc<ShardedTxPool>,
    pub network_propagator: Arc<NetworkPropagator>,
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
    proposal_type: String, // "add", "remove", "slash"
    address: Option<String>,
    public_key: Option<String>,
    _metadata: Option<String>,
}

#[derive(Deserialize)]
struct VoteReq {
    voter: String,
    _approve: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String, // "admin", "validator", "observer"
    exp: usize,
}

fn validate_jwt(token: &str, required_role: &str) -> Result<TokenData<Claims>, JwtError> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    if data.claims.role != required_role {
        return Err(JwtError::from(jsonwebtoken::errors::ErrorKind::InvalidToken));
    }
    Ok(data)
}

fn generate_jwt(subject: &str, role: &str) -> Result<String, JwtError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;
    
    let claims = Claims {
        sub: subject.to_string(),
        role: role.to_string(),
        exp: expiration,
    };
    
    let header = Header::new(Algorithm::HS256);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET.as_bytes()))
}

async fn authenticate_user(headers: HeaderMap, required_role: &str) -> Result<String, (StatusCode, Json<serde_json::Value>)> {
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

// Authentication endpoints
async fn login(Json(credentials): Json<LoginRequest>) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // In production, validate against database
    if credentials.username == "admin" && credentials.password == "admin123" {
        let token = generate_jwt(&credentials.username, "admin")
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to generate token"
            }))))?;
        
        audit_log(&credentials.username, "login", "successful");
        Ok(Json(serde_json::json!({
            "token": token,
            "role": "admin"
        })))
    } else {
        audit_log(&credentials.username, "login", "failed");
        Err((StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "error": "Invalid credentials"
        }))))
    }
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

// Rate limiting
use std::sync::OnceLock;

static RATE_LIMITS: OnceLock<Arc<Mutex<HashMap<String, (Instant, u32)>>>> = OnceLock::new();

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

// Input validation
fn validate_address(address: &str) -> bool {
    // Basic address validation - should start with "fdg1" and be reasonable length
    address.starts_with("fdg1") && address.len() >= 10 && address.len() <= 100
}

fn validate_amount(amount: u64) -> bool {
    // Prevent overflow and ensure reasonable amounts
    amount > 0 && amount <= 1_000_000_000_000 // 1 trillion max
}

fn validate_currency(currency: &str) -> bool {
    let whitelist = vec!["EUR", "USD", "GBP", "JPY", "CHF", "SGD", "AED", "CNY", "BUND", "OAT", "BTP", "GILT", "UST", "JGB", "T-BILL", "CP", "CD", "XAU", "XAG", "XPT", "XPD", "XS1234567890", "FR0000120271", "BE0003796134", "DE0001135275", "ETF1", "UCITS1", "BTC", "ETH", "USDT", "USDC"];
    whitelist.contains(&currency)
}

fn validate_public_key(public_key: &str) -> bool {
    // Validate hex-encoded public key
    if public_key.len() != 64 {
        return false;
    }
    hex::decode(public_key).is_ok()
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
    let shard_id = params.get("shard_id").and_then(|s| s.parse().ok()).unwrap_or(0);
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

async fn add_validator(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<ValidatorAddReq>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // Authenticate admin user
    let username = authenticate_user(headers, "admin").await?;
    
    let address = Address(req.address.clone());
    let public_key_bytes = match hex::decode(&req.public_key) {
        Ok(bytes) => bytes,
        Err(_) => return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid public key"})))),
    };
    let public_key_bytes: [u8; 32] = match public_key_bytes.try_into() {
        Ok(bytes) => bytes,
        Err(_) => return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid public key length"
        })))),
    };
    let public_key = match VerifyingKey::from_bytes(&public_key_bytes) {
        Ok(key) => key,
        Err(_) => return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid public key format"
        })))),
    };
    
    state.validator_set.lock().unwrap().add_validator(address, public_key, 1000); // Default stake of 1000
    if let Err(e) = state.storage.store_validator_set(&state.validator_set.lock().unwrap()) {
        eprintln!("Failed to store validator set: {e}");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "storage error"}))));
    }
    
    audit_log(&username, "add_validator", &format!("address={}", req.address));
    Ok((StatusCode::OK, Json(serde_json::json!({"status": "added"}))))
}

async fn remove_validator(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(address): Path<String>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // Authenticate admin user
    let username = authenticate_user(headers, "admin").await?;
    
    let address = Address(address.clone());
    state.validator_set.lock().unwrap().remove_validator(&address);
    if let Err(e) = state.storage.store_validator_set(&state.validator_set.lock().unwrap()) {
        eprintln!("Failed to store validator set: {e}");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "storage error"}))));
    }
    
    audit_log(&username, "remove_validator", &format!("address={address}"));
    Ok((StatusCode::OK, Json(serde_json::json!({"status": "removed"}))))
}

async fn slash_validator(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(address): Path<String>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // Authenticate admin user
    let username = authenticate_user(headers, "admin").await?;
    
    let address = Address(address.clone());
    state.validator_set.lock().unwrap().set_status(&address, ValidatorStatus::Slashed);
    if let Err(e) = state.storage.store_validator_set(&state.validator_set.lock().unwrap()) {
        eprintln!("Failed to store validator set: {e}");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "storage error"}))));
    }
    
    audit_log(&username, "slash_validator", &format!("address={address}"));
    Ok((StatusCode::OK, Json(serde_json::json!({"status": "slashed"}))))
}

async fn submit_proposal(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ProposalSubmitReq>
) -> (StatusCode, Json<serde_json::Value>) {
    let mut state_guard = state.governance_state.lock().unwrap();
    let now = chrono::Utc::now().timestamp() as u64;
    let (proposal_type, title, description) = match req.proposal_type.as_str() {
        "add" => {
            let address = req.address.clone().unwrap_or_default();
            let public_key = req.public_key.clone().unwrap_or_default();
            (
                crate::consensus::governance::ProposalType::AddValidator { address: address.clone(), public_key: public_key.clone() },
                format!("Add Validator {address}"),
                format!("Add validator with address {address} and public key {public_key}"),
            )
        },
        "remove" => {
            let address = req.address.clone().unwrap_or_default();
            (
                crate::consensus::governance::ProposalType::RemoveValidator { address: address.clone() },
                format!("Remove Validator {address}"),
                format!("Remove validator with address {address}"),
            )
        },
        "slash" => {
            let address = req.address.clone().unwrap_or_default();
            (
                crate::consensus::governance::ProposalType::SlashValidator { address: address.clone(), reason: "Manual slash".to_string() },
                format!("Slash Validator {address}"),
                format!("Slash validator with address {address}"),
            )
        },
        _ => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid proposal_type"}))),
    };
    let proposer = req.proposer.clone();
    let duration = Some(state_guard.config.min_proposal_duration);
    match state_guard.create_proposal(proposer, title, description, proposal_type, duration) {
        Ok(proposal_id) => {
            if let Err(e) = state.storage.store_governance_state(&state_guard) {
                eprintln!("Failed to store governance state: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "storage error"})));
            }
            (StatusCode::OK, Json(serde_json::json!({"proposal_id": proposal_id})))
        },
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": e}))),
    }
}

async fn vote_proposal(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    Json(req): Json<VoteReq>
) -> (StatusCode, Json<serde_json::Value>) {
    let mut state_guard = state.governance_state.lock().unwrap();
    let proposal_id = format!("proposal_{}", id);
    // For now, use 1 as stake_weight (replace with real stake lookup)
    let stake_weight = 1u64;
    let reason = None;
    match state_guard.submit_vote(&proposal_id, req.voter.clone(), req._approve, stake_weight, reason) {
        Ok(_) => {
            if let Err(e) = state.storage.store_governance_state(&state_guard) {
                eprintln!("Failed to store governance state: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "storage error"})));
            }
            (StatusCode::OK, Json(serde_json::json!({"status": "vote recorded"})))
        },
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": e}))),
    }
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

async fn get_assets(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let whitelist = vec!["EUR", "USD", "GBP", "JPY", "CHF", "SGD", "AED", "CNY", "BUND", "OAT", "BTP", "GILT", "UST", "JGB", "T-BILL", "CP", "CD", "XAU", "XAG", "XPT", "XPD", "XS1234567890", "FR0000120271", "BE0003796134", "DE0001135275", "ETF1", "UCITS1", "BTC", "ETH", "USDT", "USDC"];
    Json(serde_json::json!(whitelist))
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
    State(state): State<Arc<AppState>>,
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
        if let Some(block) = state.storage.load_block(&block_id) {
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

fn audit_log(subject: &str, action: &str, details: &str) {
    let now = chrono::Utc::now().to_rfc3339();
    let mut file = OpenOptions::new().create(true).append(true).open("audit.log").unwrap();
    writeln!(file, "{now} | {subject} | {action} | {details}").unwrap();
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
    Arc::new(AppState {
        validator_set,
        storage,
        governance_state,
        tx_pool,
        network_propagator,
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

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/login", post(login))
        .route("/balance/:address/:asset", get(get_balance))
        .route("/tx", post(post_tx))
        .route("/validators", get(get_validators).post(add_validator))
        .route("/validators/:address", delete(remove_validator))
        .route("/validators/:address/slash", post(slash_validator))
        .route("/governance/proposals", post(submit_proposal).get(list_proposals))
        .route("/governance/proposals/:id", get(get_proposal))
        .route("/governance/proposals/:id/vote", post(vote_proposal))
        .route("/assets", get(get_assets))
        .route("/bridge/outbound", post(outbound_bridge))
        .route("/bridge/inbound", post(inbound_bridge))
        .route("/bridge/status/:txid", get(bridge_status))
        .route("/confidential/tx", post(submit_confidential_tx))
        .route("/identity/register", post(register_identity))
        .route("/block/:id", get(get_block))
        .route("/block/:id/merkle_proof/:tx_hash", get(get_merkle_proof))
        .with_state(state)
}

pub async fn start(
    port: u16,
    tx_pool: Arc<ShardedTxPool>,
) -> std::io::Result<()> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/tx", post(submit_transaction))
        .with_state(tx_pool);

    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await?;
    println!("HTTP server listening on {addr}");
    
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
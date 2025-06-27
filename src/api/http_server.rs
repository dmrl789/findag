use axum::{Router, routing::{get, post, delete}, extract::{Path, Query}, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use crate::consensus::validator_set::{ValidatorSet, ValidatorInfo, ValidatorStatus};
use crate::core::address::Address;
use ed25519_dalek::PublicKey;
use std::sync::{Arc, Mutex};
use crate::consensus::governance::{GovernanceState, ProposalType, ProposalStatus};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData, errors::Error as JwtError};
use axum::{extract::TypedHeader, headers::Authorization};
use axum::headers::authorization::Bearer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tower_http::limit::RateLimitLayer;
use std::time::Duration;
use std::fs::OpenOptions;
use std::io::Write;
use metrics;
use crate::core::types::ShardId;
use crate::core::bridge::BridgeTx;
use crate::core::confidential::ConfidentialTx;
use crate::core::identity::Identity;

static mut VALIDATOR_SET: Option<Arc<Mutex<ValidatorSet>>> = None;
static mut STORAGE: Option<Arc<crate::storage::persistent::PersistentStorage>> = None;
static mut GOVERNANCE_STATE: Option<Arc<Mutex<crate::consensus::governance::GovernanceState>>> = None;
const ADMIN_TOKEN: &str = "changeme";
const JWT_SECRET: &[u8] = b"changeme_jwt_secret"; // Replace with secure secret in production

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub currency: String,
    pub shard_id: Option<u16>, // Optional for API, default to 0
    // ... other fields ...
}

#[derive(Deserialize)]
struct ValidatorAddReq {
    address: String,
    public_key: String,
    metadata: Option<String>,
}

#[derive(Deserialize)]
struct ValidatorSlashReq {
    admin_token: String,
}

#[derive(Deserialize)]
struct ProposalSubmitReq {
    proposer: String,
    proposal_type: String, // "add", "remove", "slash"
    address: Option<String>,
    public_key: Option<String>,
    metadata: Option<String>,
}

#[derive(Deserialize)]
struct VoteReq {
    voter: String,
    approve: bool,
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
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS256),
    )?;
    if data.claims.role != required_role {
        return Err(JwtError::InvalidToken);
    }
    Ok(data)
}

macro_rules! require_admin_jwt {
    ($auth:expr) => {{
        let token = match $auth {
            Some(Authorization(bearer)) => bearer.token(),
            _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "missing or invalid token"}))),
        };
        if validate_jwt(token, "admin").is_err() {
            return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"})))
        }
    }};
}

/// HTTP API for FinDAG. Only whitelisted assets are supported for transaction submission and balance queries.
///
/// POST /tx: Only accepts transactions with supported assets. Unsupported assets will be rejected with an error.
/// GET /balance/:address/:asset: Only returns balances for supported assets. Unsupported assets will be rejected with an error.
///
/// Supported assets:
/// EUR, USD, GBP, JPY, CHF, SGD, AED, CNY, BUND, OAT, BTP, GILT, UST, JGB, T-BILL, CP, CD, XAU, XAG, XPT, XPD, XS1234567890, FR0000120271, BE0003796134, DE0001135275, ETF1, UCITS1, BTC, ETH, USDT, USDC
/// GET /balance/:address/:asset?shard_id=0
async fn get_balance(Path((address, asset)): Path<(String, String)>, Query(params): Query<std::collections::HashMap<String, String>>) -> Json<serde_json::Value> {
    let shard_id = params.get("shard_id").and_then(|s| s.parse().ok()).unwrap_or(0);
    let storage = unsafe { STORAGE.as_ref().unwrap() };
    let whitelist = storage.load_asset_whitelist().unwrap_or_default();
    if !whitelist.contains(&asset) {
        return Json(serde_json::json!({ "error": format!("'{}' is not a supported asset", asset) }));
    }
    let balance = storage.state_db.get_balance(shard_id, &address, &asset);
    Json(serde_json::json!({ "address": address, "asset": asset, "balance": balance, "shard_id": shard_id }))
}

/// POST /tx (accepts optional shard_id in JSON)
async fn post_tx(Json(tx): Json<Transaction>) -> Json<serde_json::Value> {
    metrics::API_CALLS.with_label_values(&["/tx"]).inc();
    let storage = unsafe { STORAGE.as_ref().unwrap() };
    let whitelist = storage.load_asset_whitelist().unwrap_or_default();
    if !whitelist.contains(&tx.currency) {
        metrics::ERROR_COUNT.with_label_values(&["unsupported_asset"]).inc();
        return Json(serde_json::json!({ "error": format!("'{}' is not a supported asset", tx.currency) }));
    }
    let shard_id = tx.shard_id.unwrap_or(0);
    // TODO: Add to mempool, broadcast, etc. Use shard_id in all mempool/state calls
    println!("Received tx: {:?} (shard_id={})", tx, shard_id);
    Json(serde_json::json!({ "status": "ok", "shard_id": shard_id }))
}

async fn get_validators() -> Json<serde_json::Value> {
    let set = unsafe { VALIDATOR_SET.as_ref().unwrap().lock().unwrap().clone() };
    Json(serde_json::json!(set.validators))
}

async fn add_validator(Json(req): Json<ValidatorAddReq>) -> (StatusCode, Json<serde_json::Value>) {
    if req.admin_token != ADMIN_TOKEN {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"})));
    }
    let address = Address(req.address);
    let public_key_bytes = match hex::decode(&req.public_key) {
        Ok(bytes) => bytes,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid public key"}))),
    };
    let public_key = match PublicKey::from_bytes(&public_key_bytes) {
        Ok(pk) => pk,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid public key"}))),
    };
    let info = ValidatorInfo {
        address: address.clone(),
        public_key,
        status: ValidatorStatus::Active,
        metadata: req.metadata,
    };
    let set = unsafe { VALIDATOR_SET.as_ref().unwrap() };
    set.lock().unwrap().add_validator(info);
    let storage = unsafe { STORAGE.as_ref().unwrap() };
    storage.save_validator_set(&set.lock().unwrap());
    audit_log(&claims.sub, "add_validator", &format!("address={}", req.address));
    (StatusCode::OK, Json(serde_json::json!({"status": "added"})))
}

async fn remove_validator(Path(address): Path<String>, Query(params): Query<std::collections::HashMap<String, String>>) -> (StatusCode, Json<serde_json::Value>) {
    if params.get("admin_token") != Some(&ADMIN_TOKEN.to_string()) {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"})));
    }
    let set = unsafe { VALIDATOR_SET.as_ref().unwrap() };
    set.lock().unwrap().remove_validator(&address);
    let storage = unsafe { STORAGE.as_ref().unwrap() };
    storage.save_validator_set(&set.lock().unwrap());
    (StatusCode::OK, Json(serde_json::json!({"status": "removed"})))
}

async fn slash_validator(Path(address): Path<String>, Json(req): Json<ValidatorSlashReq>) -> (StatusCode, Json<serde_json::Value>) {
    if req.admin_token != ADMIN_TOKEN {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"})));
    }
    let set = unsafe { VALIDATOR_SET.as_ref().unwrap() };
    set.lock().unwrap().set_status(&address, ValidatorStatus::Slashed);
    let storage = unsafe { STORAGE.as_ref().unwrap() };
    storage.save_validator_set(&set.lock().unwrap());
    (StatusCode::OK, Json(serde_json::json!({"status": "slashed"})))
}

async fn submit_proposal(Json(req): Json<ProposalSubmitReq>) -> (StatusCode, Json<serde_json::Value>) {
    if req.admin_token != ADMIN_TOKEN {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"})));
    }
    let proposer = Address(req.proposer);
    let proposal_type = match req.proposal_type.as_str() {
        "add" => {
            let address = Address(req.address.unwrap());
            let public_key = hex::decode(req.public_key.unwrap()).unwrap();
            ProposalType::AddValidator { address, public_key, metadata: req.metadata }
        },
        "remove" => {
            let address = Address(req.address.unwrap());
            ProposalType::RemoveValidator { address }
        },
        "slash" => {
            let address = Address(req.address.unwrap());
            ProposalType::SlashValidator { address }
        },
        _ => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid proposal_type"}))),
    };
    let now = chrono::Utc::now().timestamp() as u64;
    let state = unsafe { GOVERNANCE_STATE.as_ref().unwrap() };
    let mut state_guard = state.lock().unwrap();
    let id = state_guard.submit_proposal(proposer, proposal_type, now);
    let storage = unsafe { STORAGE.as_ref().unwrap() };
    storage.save_governance_state(&state_guard);
    (StatusCode::OK, Json(serde_json::json!({"proposal_id": id})))
}

async fn vote_proposal(Path(id): Path<u64>, Json(req): Json<VoteReq>) -> (StatusCode, Json<serde_json::Value>) {
    let voter = Address(req.voter);
    let state = unsafe { GOVERNANCE_STATE.as_ref().unwrap() };
    let mut state_guard = state.lock().unwrap();
    let ok = state_guard.vote(id, voter, req.approve);
    if !ok {
        return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "proposal not found"})));
    }
    // Check for approval and enact if approved
    let validator_set = unsafe { VALIDATOR_SET.as_ref().unwrap() };
    let mut vset_guard = validator_set.lock().unwrap();
    if state_guard.is_approved(id, vset_guard.active_validators().len()) {
        let enacted = state_guard.enact_proposal(id, &mut vset_guard);
        if enacted {
            let storage = unsafe { STORAGE.as_ref().unwrap() };
            storage.save_validator_set(&vset_guard);
        }
    }
    let storage = unsafe { STORAGE.as_ref().unwrap() };
    storage.save_governance_state(&state_guard);
    (StatusCode::OK, Json(serde_json::json!({"status": "vote recorded"})))
}

async fn list_proposals() -> Json<serde_json::Value> {
    let state = unsafe { GOVERNANCE_STATE.as_ref().unwrap() };
    let state_guard = state.lock().unwrap();
    let proposals: Vec<_> = state_guard.proposals.values().collect();
    Json(serde_json::json!(proposals))
}

async fn get_proposal(Path(id): Path<u64>) -> (StatusCode, Json<serde_json::Value>) {
    let state = unsafe { GOVERNANCE_STATE.as_ref().unwrap() };
    let state_guard = state.lock().unwrap();
    if let Some(prop) = state_guard.get_proposal(id) {
        (StatusCode::OK, Json(serde_json::json!(prop)))
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "not found"})))
    }
}

async fn get_assets() -> Json<serde_json::Value> {
    let storage = unsafe { STORAGE.as_ref().unwrap() };
    let whitelist = storage.load_asset_whitelist().unwrap_or_default();
    Json(serde_json::json!({ "assets": whitelist }))
}

/// POST /bridge/outbound: Initiate an outbound cross-chain transfer
async fn outbound_bridge(Json(req): Json<BridgeTx>) -> Json<serde_json::Value> {
    // TODO: Lock assets, generate proof, relay to target chain
    println!("[Bridge] Outbound: {:?}", req);
    Json(serde_json::json!({ "status": "pending", "details": "Bridge logic not yet implemented" }))
}

/// POST /bridge/inbound: Finalize an inbound cross-chain transfer
async fn inbound_bridge(Json(req): Json<BridgeTx>) -> Json<serde_json::Value> {
    // TODO: Verify proof, mint/unlock assets
    println!("[Bridge] Inbound: {:?}", req);
    Json(serde_json::json!({ "status": "pending", "details": "Bridge logic not yet implemented" }))
}

/// POST /confidential/tx: Submit a confidential transaction
async fn submit_confidential_tx(Json(req): Json<ConfidentialTx>) -> Json<serde_json::Value> {
    // TODO: Validate and process confidential transaction
    println!("[Confidential] Received confidential tx: {:?}", req);
    Json(serde_json::json!({ "status": "pending", "details": "Confidential tx logic not yet implemented" }))
}

/// POST /identity/register: Register or update on-chain identity
async fn register_identity(Json(req): Json<Identity>) -> Json<serde_json::Value> {
    // TODO: Register or update identity, perform KYC checks
    println!("[Identity] Register/update: {:?}", req);
    Json(serde_json::json!({ "status": "pending", "details": "Identity logic not yet implemented" }))
}

fn audit_log(subject: &str, action: &str, details: &str) {
    let now = chrono::Utc::now().to_rfc3339();
    let mut file = OpenOptions::new().create(true).append(true).open("audit.log").unwrap();
    writeln!(file, "{} | {} | {} | {}", now, subject, action, details).unwrap();
}

pub async fn run_http_server() {
    // In real use, pass Arc<Mutex<>> from main
    unsafe {
        if VALIDATOR_SET.is_none() {
            panic!("VALIDATOR_SET must be initialized before running HTTP server");
        }
        if STORAGE.is_none() {
            panic!("STORAGE must be initialized before running HTTP server");
        }
    }
    let app = Router::new()
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
        .route("/confidential/tx", post(submit_confidential_tx))
        .route("/identity/register", post(register_identity));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("HTTP API listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.layer(RateLimitLayer::new(10, Duration::from_secs(1))).into_make_service())
        .await
        .unwrap();
}

// Example usage:
// GET http://127.0.0.1:8080/balance/fdg1qxyz...
// POST http://127.0.0.1:8080/tx { ...transaction json... } 
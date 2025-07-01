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
use crate::core::types::{ShardId, Transaction};
use crate::core::bridge::{BridgeTx, BridgeManager};
use crate::core::confidential::ConfidentialTx;
use crate::core::identity::Identity;
use once_cell::sync::Lazy;
use crate::core::types::Block;
use axum::http::StatusCode;
use crate::core::tx_pool::ShardedTxPool;
use crate::core::types::SerializableTransaction;
use crate::network::propagation::NetworkPropagator;
use rand;
use tokio::net::TcpListener;
use axum::response::IntoResponse;
use axum::extract::State;

static mut VALIDATOR_SET: Option<Arc<Mutex<ValidatorSet>>> = None;
static mut STORAGE: Option<Arc<crate::storage::persistent::PersistentStorage>> = None;
static mut GOVERNANCE_STATE: Option<Arc<Mutex<crate::consensus::governance::GovernanceState>>> = None;
static mut TX_POOL: Option<Arc<ShardedTxPool>> = None;
static mut NETWORK_PROPAGATOR: Option<Arc<NetworkPropagator>> = None;
const ADMIN_TOKEN: &str = "changeme";
const JWT_SECRET: &[u8] = b"changeme_jwt_secret"; // Replace with secure secret in production
static BRIDGE_MANAGER: Lazy<BridgeManager> = Lazy::new(|| BridgeManager::new());

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

/// POST /tx (accepts both simple ApiTransaction and signed SignedTransactionRequest)
async fn post_tx(Json(tx_data): Json<serde_json::Value>) -> Json<serde_json::Value> {
    metrics::API_CALLS.with_label_values(&["/tx"]).inc();
    
    // Try to parse as signed transaction first
    if let Ok(signed_tx) = serde_json::from_value::<SignedTransactionRequest>(tx_data.clone()) {
        println!("Received signed transaction: from={}, to={}, amount={}", 
                signed_tx.from, signed_tx.to, signed_tx.amount);
        
        // Validate signature
        let message = format!("{}{}{}", signed_tx.from, signed_tx.to, signed_tx.amount);
        println!("[DEBUG] Verifying signature for message: '{}'", message);
        println!("[DEBUG] Public key length: {}, Signature length: {}", 
                signed_tx.public_key.len(), signed_tx.signature.len());
        
        let public_key = match ed25519_dalek::PublicKey::from_bytes(&signed_tx.public_key) {
            Ok(pk) => {
                println!("[DEBUG] Public key parsed successfully");
                pk
            },
            Err(e) => {
                println!("[DEBUG] Invalid public key format: {:?}", e);
                return Json(serde_json::json!({ "error": "Invalid public key format" }));
            }
        };
        
        let signature = match ed25519_dalek::Signature::from_bytes(&signed_tx.signature) {
            Ok(sig) => {
                println!("[DEBUG] Signature parsed successfully");
                sig
            },
            Err(e) => {
                println!("[DEBUG] Invalid signature format: {:?}", e);
                return Json(serde_json::json!({ "error": "Invalid signature format" }));
            }
        };
        
        // Verify signature
        match public_key.verify(message.as_bytes(), &signature) {
            Ok(_) => println!("[DEBUG] Signature verification successful"),
            Err(e) => {
                println!("[DEBUG] Signature verification failed: {:?}", e);
                return Json(serde_json::json!({ "error": "Signature verification failed" }));
            }
        }
        
        // Create core Transaction
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
        println!("[DEBUG]   Payload length: {}", signed_tx.payload.len());
        println!("[DEBUG]   FindAG Time: {}", signed_tx.findag_time);
        println!("[DEBUG]   HashTimer: {:02x?}", &signed_tx.hashtimer);
        
        // Validate transaction structure
        if signed_tx.signature.len() != 64 {
            println!("[DEBUG] REJECTION: Invalid signature length {}", signed_tx.signature.len());
            return Json(serde_json::json!({ "error": "Invalid signature length" }));
        }
        
        if signed_tx.public_key.len() != 32 {
            println!("[DEBUG] REJECTION: Invalid public key length {}", signed_tx.public_key.len());
            return Json(serde_json::json!({ "error": "Invalid public key length" }));
        }
        
        // Add to transaction pool
        let tx_pool = unsafe { TX_POOL.as_ref().unwrap() };
        println!("[DEBUG] Adding transaction to pool...");
        let added = tx_pool.add_transaction(core_tx.clone());
        
        if added {
            // Broadcast to network
            if let Some(propagator) = unsafe { NETWORK_PROPAGATOR.as_ref() } {
                let stx: SerializableTransaction = core_tx.into();
                let msg = crate::network::propagation::GossipMsg::NewTransaction(stx);
                propagator.broadcast(&msg).await;
            }
            
            println!("[DEBUG] SUCCESS: Transaction added to pool");
            println!("Processed signed tx: from={}, to={}, amount={} (shard_id={})", 
                    signed_tx.from, signed_tx.to, signed_tx.amount, signed_tx.shard_id);
            Json(serde_json::json!({ "status": "ok", "shard_id": signed_tx.shard_id, "message": "Signed transaction added to pool" }))
        } else {
            metrics::ERROR_COUNT.with_label_values(&["tx_rejected"]).inc();
            println!("[DEBUG] REJECTION: Transaction rejected by pool");
            
            // Get detailed rejection reason from pool
            let sender_balance = tx_pool.get_balance(ShardId(signed_tx.shard_id), &signed_tx.from, "USD");
            println!("[DEBUG] Sender balance check: {} has {} USD", signed_tx.from, sender_balance);
            
            Json(serde_json::json!({ "error": "Transaction rejected", "shard_id": signed_tx.shard_id }))
        }
    } else {
        // Try to parse as simple ApiTransaction
        if let Ok(tx) = serde_json::from_value::<ApiTransaction>(tx_data) {
            let storage = unsafe { STORAGE.as_ref().unwrap() };
            let whitelist = storage.load_asset_whitelist().unwrap_or_default();
            if !whitelist.contains(&tx.currency) {
                metrics::ERROR_COUNT.with_label_values(&["unsupported_asset"]).inc();
                return Json(serde_json::json!({ "error": format!("'{}' is not a supported asset", tx.currency) }));
            }
            let shard_id = tx.shard_id.unwrap_or(0);
            
            // Create a dummy keypair for signing (in production, this should be the node's keypair)
            let mut rng = rand::rngs::OsRng;
            let keypair = ed25519_dalek::Keypair::generate(&mut rng);
            
            // Create a message to sign (transaction data)
            let message = format!("{}:{}:{}:{}", tx.from, tx.to, tx.amount, tx.currency);
            let signature = keypair.sign(message.as_bytes());
            
            // Convert ApiTransaction to core Transaction
            let core_tx = Transaction {
                from: Address(tx.from.clone()),
                to: Address(tx.to.clone()),
                amount: tx.amount,
                payload: vec![], // Empty payload for simple transfers
                findag_time: 0, // Will be set by the system
                hashtimer: [0u8; 32], // Will be computed by the system
                signature,
                public_key: keypair.public,
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
            println!("[DEBUG]   Shard ID: {}", shard_id);
            
            // Add to transaction pool
            let tx_pool = unsafe { TX_POOL.as_ref().unwrap() };
            println!("[DEBUG] Adding simple transaction to pool...");
            let added = tx_pool.add_transaction(core_tx.clone());
            
            if added {
                // Broadcast to network
                if let Some(propagator) = unsafe { NETWORK_PROPAGATOR.as_ref() } {
                    let stx: SerializableTransaction = core_tx.into();
                    let msg = crate::network::propagation::GossipMsg::NewTransaction(stx);
                    propagator.broadcast(&msg).await;
                }
                
                println!("[DEBUG] SUCCESS: Simple transaction added to pool");
                println!("Processed simple tx: {:?} (shard_id={})", tx, shard_id);
                Json(serde_json::json!({ "status": "ok", "shard_id": shard_id, "message": "Transaction added to pool" }))
            } else {
                metrics::ERROR_COUNT.with_label_values(&["tx_rejected"]).inc();
                println!("[DEBUG] REJECTION: Simple transaction rejected by pool");
                
                // Get detailed rejection reason from pool
                let sender_balance = tx_pool.get_balance(ShardId(shard_id), &tx.from, "USD");
                println!("[DEBUG] Sender balance check: {} has {} USD", tx.from, sender_balance);
                
                Json(serde_json::json!({ "error": "Transaction rejected", "shard_id": shard_id }))
            }
        } else {
            println!("Invalid transaction format received");
            Json(serde_json::json!({ "error": "Invalid transaction format" }))
        }
    }
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
    // Lock assets, generate proof, relay to target chain
    let tx_id = format!("out-{}-{}-{}", req.source_chain, req.sender, req.amount); // Example tx_id
    BRIDGE_MANAGER.lock_prepare(&tx_id, None);
    let receipt = BRIDGE_MANAGER.get_status(&tx_id).unwrap();
    Json(serde_json::json!({ "status": receipt.status, "tx_id": tx_id, "details": receipt.details, "proof": receipt.proof }))
}

/// POST /bridge/inbound: Finalize an inbound cross-chain transfer
async fn inbound_bridge(Json(req): Json<BridgeTx>) -> Json<serde_json::Value> {
    // Require and verify proof
    let tx_id = format!("in-{}-{}-{}", req.target_chain, req.recipient, req.amount); // Example tx_id
    let proof = req.proof.clone();
    BRIDGE_MANAGER.commit_ack(&tx_id, proof);
    let receipt = BRIDGE_MANAGER.get_status(&tx_id).unwrap();
    Json(serde_json::json!({ "status": receipt.status, "tx_id": tx_id, "details": receipt.details, "proof": receipt.proof, "error": receipt.error }))
}

/// GET /bridge/status/:txid: Query bridge transaction status
async fn bridge_status(Path(tx_id): Path<String>) -> Json<serde_json::Value> {
    if let Some(receipt) = BRIDGE_MANAGER.get_status(&tx_id) {
        Json(serde_json::json!({ "status": receipt.status, "tx_id": tx_id, "details": receipt.details }))
    } else {
        Json(serde_json::json!({ "status": "not_found", "tx_id": tx_id }))
    }
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

/// GET /block/:id - Returns block info including merkle_root
async fn get_block(Path(id): Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    let storage = unsafe { STORAGE.as_ref().unwrap() };
    let id_bytes = match hex::decode(&id) {
        Ok(bytes) => {
            let mut arr = [0u8; 32];
            if bytes.len() == 32 { arr.copy_from_slice(&bytes); Some(arr) } else { None }
        },
        Err(_) => None,
    };
    if let Some(block_id) = id_bytes {
        if let Some(block) = storage.load_block(&block_id) {
            return (StatusCode::OK, Json(serde_json::json!({
                "block_id": id,
                "parent_blocks": block.parent_blocks.iter().map(hex::encode).collect::<Vec<_>>(),
                "merkle_root": block.merkle_root.map(hex::encode),
                "transactions": block.transactions.iter().map(|tx| hex::encode(&tx.hashtimer)).collect::<Vec<_>>()
            })));
        }
    }
    (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "block not found"})))
}

/// GET /block/:id/merkle_proof/:tx_hash - Returns Merkle proof for a transaction in the block
async fn get_merkle_proof(Path((id, tx_hash)): Path<(String, String)>) -> (StatusCode, Json<serde_json::Value>) {
    let storage = unsafe { STORAGE.as_ref().unwrap() };
    let id_bytes = match hex::decode(&id) {
        Ok(bytes) => {
            let mut arr = [0u8; 32];
            if bytes.len() == 32 { arr.copy_from_slice(&bytes); Some(arr) } else { None }
        },
        Err(_) => None,
    };
    if let Some(block_id) = id_bytes {
        if let Some(block) = storage.load_block(&block_id) {
            let tx_hashes: Vec<String> = block.transactions.iter().map(|tx| hex::encode(&tx.hashtimer)).collect();
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
    writeln!(file, "{} | {} | {} | {}", now, subject, action, details).unwrap();
}

pub fn init_http_server(
    validator_set: Arc<Mutex<ValidatorSet>>,
    storage: Arc<crate::storage::persistent::PersistentStorage>,
    governance_state: Arc<Mutex<crate::consensus::governance::GovernanceState>>,
    tx_pool: Arc<ShardedTxPool>,
    network_propagator: Arc<NetworkPropagator>,
) {
    unsafe {
        VALIDATOR_SET = Some(validator_set);
        STORAGE = Some(storage);
        GOVERNANCE_STATE = Some(governance_state);
        TX_POOL = Some(tx_pool);
        NETWORK_PROPAGATOR = Some(network_propagator);
    }
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
        if TX_POOL.is_none() {
            panic!("TX_POOL must be initialized before running HTTP server");
        }
        if NETWORK_PROPAGATOR.is_none() {
            panic!("NETWORK_PROPAGATOR must be initialized before running HTTP server");
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
        .route("/bridge/status/:txid", get(bridge_status))
        .route("/confidential/tx", post(submit_confidential_tx))
        .route("/identity/register", post(register_identity))
        .route("/block/:id", get(get_block))
        .route("/block/:id/merkle_proof/:tx_hash", get(get_merkle_proof));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("HTTP API listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.layer(RateLimitLayer::new(10, Duration::from_secs(1))).into_make_service())
        .await
        .unwrap();
}

pub async fn start(
    port: u16,
    tx_pool: Arc<ShardedTxPool>,
) -> std::io::Result<()> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/tx", post(submit_transaction))
        .with_state(tx_pool);

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    println!("HTTP server listening on {}", addr);
    
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
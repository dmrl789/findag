// src/bridge/api.rs

use crate::bridge::corda::CordaSettlementProof;
use crate::bridge::proofs::SettlementProof;
use crate::fix::{parse_order_single, fix_order_to_findag_tx};
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::Deserialize;
use anyhow::Result;
use std::sync::Arc;
use crate::core::tx_pool::ShardedTxPool;

// --- For JSON input ---
#[derive(Debug, Deserialize)]
pub struct CordaProofInput {
    pub state_hash: String,         // base64 or hex string
    pub notary_signature: String,   // base64 or hex string
}

#[derive(Debug, Deserialize)]
pub struct FixProofInput {
    pub fix_raw: String   // raw FIX message with SOH chars as \u0001
}

// --- Handlers ---
pub async fn submit_corda_proof(Json(proof_input): Json<CordaProofInput>) -> impl IntoResponse {
    println!("Received Corda proof: {:?}", proof_input);

    match process_corda_proof(proof_input).await {
        Ok(_) => {
            println!("Corda proof verified and accepted!");
            (StatusCode::OK, "Proof Accepted").into_response()
        }
        Err(e) => {
            eprintln!("Bridge error: {:?}", e);
            (StatusCode::BAD_REQUEST, "Invalid Proof").into_response()
        }
    }
}

pub async fn submit_fix_order(Json(proof_input): Json<FixProofInput>) -> impl IntoResponse {
    println!("Received FIX Order: {:?}", proof_input.fix_raw);

    match process_fix_order(proof_input).await {
        Ok(_) => {
            println!("FIX Order accepted and converted to FinDAG transaction!");
            (StatusCode::OK, "FIX Order Accepted").into_response()
        }
        Err(e) => {
            eprintln!("FIX error: {:?}", e);
            (StatusCode::BAD_REQUEST, "Invalid FIX Order").into_response()
        }
    }
}

// --- Stateful handlers for integration with tx_pool ---
pub async fn submit_corda_proof_with_state(
    axum::extract::State(_tx_pool): axum::extract::State<Arc<ShardedTxPool>>,
    Json(proof_input): Json<CordaProofInput>,
) -> impl IntoResponse {
    println!("Received Corda proof with state: {:?}", proof_input);

    match process_corda_proof(proof_input).await {
        Ok(_) => {
            println!("Corda proof verified and accepted!");
            // TODO: Store the proof in tx_pool or create a bridge transaction
            (StatusCode::OK, "Proof Accepted").into_response()
        }
        Err(e) => {
            eprintln!("Bridge error: {:?}", e);
            (StatusCode::BAD_REQUEST, "Invalid Proof").into_response()
        }
    }
}

pub async fn submit_fix_order_with_state(
    axum::extract::State(_tx_pool): axum::extract::State<Arc<ShardedTxPool>>,
    Json(proof_input): Json<FixProofInput>,
) -> impl IntoResponse {
    println!("Received FIX Order with state: {:?}", proof_input.fix_raw);

    match process_fix_order(proof_input).await {
        Ok(_findag_tx) => {
            println!("FIX Order accepted and converted to FinDAG transaction!");
            // TODO: Store the transaction in tx_pool
            // tx_pool.add_transaction(findag_tx).await?;
            (StatusCode::OK, "FIX Order Accepted").into_response()
        }
        Err(e) => {
            eprintln!("FIX error: {:?}", e);
            (StatusCode::BAD_REQUEST, "Invalid FIX Order").into_response()
        }
    }
}

async fn process_corda_proof(proof_input: CordaProofInput) -> Result<()> {
    let state_hash = hex::decode(&proof_input.state_hash)?;
    let notary_signature = hex::decode(&proof_input.notary_signature)?;

    let proof = CordaSettlementProof {
        state_hash,
        notary_signature,
    };

    let trusted_notary_key = vec![0u8; 32]; // Load your real notary pubkey!
    proof.verify(&trusted_notary_key)?;

    // ✅ TODO: store the verified proof in your DAG or mempool.
    Ok(())
}

async fn process_fix_order(proof_input: FixProofInput) -> Result<crate::core::types::Transaction> {
    let order = parse_order_single(&proof_input.fix_raw)?;
    println!("Parsed Order: {:?}", order);

    let findag_tx = fix_order_to_findag_tx(&order);
    println!("Converted FinDAGTransaction: {:?}", findag_tx);

    // TODO: insert into your DAG or mempool!
    // In a real implementation, you would:
    // 1. Validate the FIX message checksum
    // 2. Check if the order is within acceptable limits
    // 3. Verify the account has sufficient balance
    // 4. Store the transaction in the mempool
    // 5. Trigger settlement processing

    Ok(findag_tx)
}

// --- Stateless Axum router (for main server) ---
pub fn bridge_routes() -> Router {
    Router::new()
        .route("/bridge/corda/submit", post(submit_corda_proof))
        .route("/bridge/fix/submit", post(submit_fix_order))
}

// --- Stateful Axum router (for integration with tx_pool) ---
pub fn bridge_routes_with_state() -> Router<Arc<ShardedTxPool>> {
    Router::new()
        .route("/bridge/corda/submit", post(submit_corda_proof_with_state))
        .route("/bridge/fix/submit", post(submit_fix_order_with_state))
} 
//! REST API endpoints for bank operations

use axum::{
    extract::{State, Path, Query},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

use super::ledger::{TokenLedger, Transaction, Balance, TransactionType};

/// Shared bank state
#[derive(Clone)]
pub struct BankState {
    pub ledger: Arc<TokenLedger>,
}

/// Balance response
#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub user_id: String,
    pub balance: Balance,
}

/// Extended balance response with Solana info
#[derive(Debug, Serialize)]
pub struct ExtendedBalanceResponse {
    pub user_id: String,
    pub bank_balance: Balance,
    pub solana_balance: Option<u64>,
    pub total_balance: u64,
    pub network: String,
}

/// Transaction query parameters
#[derive(Debug, Deserialize)]
pub struct TransactionQuery {
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    50
}

/// Reward request body
#[derive(Debug, Deserialize)]
pub struct RewardRequest {
    pub user_id: String,
    pub amount: u64, // in lamports
    pub reason: String, // "order_completion", "review", "referral", etc.
}

/// POST /api/bank/reward
pub async fn reward_user(
    State(state): State<BankState>,
    Json(req): Json<RewardRequest>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Update balance
    let new_balance = state
        .ledger
        .update_balance(&req.user_id, req.amount as i64)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Record transaction
    let tx = Transaction {
        id: uuid::Uuid::new_v4().to_string(),
        user_id: req.user_id.clone(),
        transaction_type: TransactionType::Reward,
        amount: req.amount,
        timestamp: chrono::Utc::now(),
        signature: None,
        metadata: std::collections::HashMap::from([
            ("reason".to_string(), req.reason.clone()),
        ]),
    };

    state
        .ledger
        .record_transaction(tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "success": true,
        "user_id": req.user_id,
        "amount": req.amount,
        "reason": req.reason,
        "new_balance": new_balance
    })))
}

/// GET /api/bank/balance/:user_id
pub async fn get_balance(
    State(state): State<BankState>,
    Path(user_id): Path<String>,
) -> Result<Json<BalanceResponse>, StatusCode> {
    let balance = state
        .ledger
        .get_balance(&user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(BalanceResponse { user_id, balance }))
}

/// GET /api/bank/balance/:user_id/full - Extended balance with Solana
pub async fn get_full_balance(
    State(state): State<BankState>,
    Path(user_id): Path<String>,
) -> Result<Json<ExtendedBalanceResponse>, StatusCode> {
    // Get bank balance
    let bank_balance = state
        .ledger
        .get_balance(&user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Try to get Solana balance (if configured)
    let mut solana_balance: Option<u64> = None;
    let mut network = "devnet".to_string();
    
    // Check if Solana is configured
    if let Ok(rpc_url) = std::env::var("SOLANA_RPC_URL") {
        if let Ok(mint_address) = std::env::var("FODI_MINT_ADDRESS") {
            // Try to get wallet address for this user_id
            // For now, we'll just set it to None as we need wallet mapping
            // TODO: Implement wallet address lookup
            tracing::info!("Solana configured: {} / {}", rpc_url, mint_address);
        }
    }

    let total_balance = bank_balance.total + solana_balance.unwrap_or(0);

    Ok(Json(ExtendedBalanceResponse {
        user_id,
        bank_balance,
        solana_balance,
        total_balance,
        network,
    }))
}

/// GET /api/bank/transactions/:user_id
pub async fn get_transactions(
    State(state): State<BankState>,
    Path(user_id): Path<String>,
    Query(query): Query<TransactionQuery>,
) -> Result<Json<Vec<Transaction>>, StatusCode> {
    let transactions = state
        .ledger
        .get_transactions(&user_id, query.limit)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(transactions))
}

/// GET /api/bank/admin/transactions
pub async fn get_all_transactions(
    State(state): State<BankState>,
    Query(query): Query<TransactionQuery>,
) -> Result<Json<Vec<Transaction>>, StatusCode> {
    let transactions = state
        .ledger
        .get_all_transactions(query.limit)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(transactions))
}

/// Health check for bank module
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Bank module operational")
}

/// GET /api/bank/stats - Bank statistics
pub async fn get_bank_stats(
    State(state): State<BankState>,
) -> Result<Json<Value>, StatusCode> {
    // Get all transactions to calculate stats
    let transactions = state
        .ledger
        .get_all_transactions(1000)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total_rewards: u64 = transactions
        .iter()
        .filter(|tx| matches!(tx.transaction_type, TransactionType::Reward))
        .map(|tx| tx.amount)
        .sum();

    let total_burns: u64 = transactions
        .iter()
        .filter(|tx| matches!(tx.transaction_type, TransactionType::Burn))
        .map(|tx| tx.amount)
        .sum();

    let unique_users: std::collections::HashSet<String> = transactions
        .iter()
        .map(|tx| tx.user_id.clone())
        .collect();

    // Get Solana info if configured
    let solana_info = if let (Ok(mint), Ok(network)) = (
        std::env::var("FODI_MINT_ADDRESS"),
        std::env::var("SOLANA_NETWORK"),
    ) {
        Some(json!({
            "mint_address": mint,
            "network": network,
            "configured": true,
        }))
    } else {
        None
    };

    Ok(Json(json!({
        "bank": {
            "total_rewards_issued": total_rewards,
            "total_burns": total_burns,
            "net_supply": total_rewards - total_burns,
            "total_transactions": transactions.len(),
            "unique_users": unique_users.len(),
        },
        "solana": solana_info,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })))
}

/// Router setup helper
pub fn routes() -> Router {
    // Create ledger with persistent storage
    let ledger = Arc::new(
        TokenLedger::with_persistence("data/fodi_ledger.db")
            .unwrap_or_else(|_| TokenLedger::new())
    );
    routes_with_ledger(ledger)
}

/// Router setup with provided ledger (for sharing with wallet module)
pub fn routes_with_ledger(ledger: Arc<TokenLedger>) -> Router {
    let state = BankState { ledger };

    Router::new()
        .route("/health", get(health_check))
        .route("/stats", get(get_bank_stats)) // 📊 Bank statistics
        .route("/balance/{user_id}", get(get_balance))
        .route("/balance/{user_id}/full", get(get_full_balance)) // 🌐 Extended balance with Solana
        .route("/transactions/{user_id}", get(get_transactions))
        .route("/admin/transactions", get(get_all_transactions))
        .route("/reward", post(reward_user)) // 💰 NEW!
        .layer(CorsLayer::permissive()) // 🌐 Allow CORS for frontend
        .with_state(state)
}

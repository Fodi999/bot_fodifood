//! REST API endpoints for bank operations

use axum::{
    extract::{State, Path, Query},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
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
        .route("/balance/{user_id}", get(get_balance))
        .route("/transactions/{user_id}", get(get_transactions))
        .route("/admin/transactions", get(get_all_transactions))
        .route("/reward", post(reward_user)) // 💰 NEW!
        .with_state(state)
}

//! REST API endpoints for wallet management

use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use std::str::FromStr;
use sled; // For shared database connection
use solana_sdk::pubkey::Pubkey;

use super::storage::{WalletStorage, WalletInfo};
use crate::bank::ledger::TokenLedger;
use crate::solana::client::SolanaClient;

/// Shared wallet state
#[derive(Clone)]
pub struct WalletState {
    pub storage: Arc<WalletStorage>,
    pub ledger: Arc<TokenLedger>,
    pub solana_client: Option<Arc<SolanaClient>>,
}

/// Create wallet request
#[derive(Debug, Deserialize)]
pub struct CreateWalletRequest {
    pub user_id: String,
    #[serde(default)]
    pub wallet_type: WalletTypeParam,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WalletTypeParam {
    Managed,
    External,
}

impl Default for WalletTypeParam {
    fn default() -> Self {
        Self::Managed
    }
}

/// Register external wallet request
#[derive(Debug, Deserialize)]
pub struct RegisterExternalWalletRequest {
    pub user_id: String,
    pub pubkey: String,
}

/// Wallet balance response
#[derive(Debug, Serialize)]
pub struct WalletBalanceResponse {
    pub user_id: String,
    pub pubkey: String,
    pub chain: String,
    pub offchain_balance: u64,
    pub onchain_balance: u64,
    pub synced: bool,
}

/// POST /api/wallet - Create or get wallet
pub async fn create_or_get_wallet(
    State(state): State<WalletState>,
    Json(req): Json<CreateWalletRequest>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let wallet = state
        .storage
        .get_or_create_wallet(&req.user_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "user_id": wallet.user_id,
        "pubkey": wallet.pubkey,
        "chain": wallet.chain,
        "wallet_type": format!("{:?}", wallet.wallet_type),
        "created_at": wallet.created_at,
    })))
}

/// POST /api/wallet/register - Register external wallet
pub async fn register_external_wallet(
    State(state): State<WalletState>,
    Json(req): Json<RegisterExternalWalletRequest>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let wallet = state
        .storage
        .register_external_wallet(&req.user_id, &req.pubkey)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "user_id": wallet.user_id,
        "pubkey": wallet.pubkey,
        "chain": wallet.chain,
        "wallet_type": "External",
    })))
}

/// GET /api/wallet/balance/:user_id - Get wallet balance (onchain + offchain)
pub async fn get_wallet_balance(
    State(state): State<WalletState>,
    Path(user_id): Path<String>,
) -> Result<Json<WalletBalanceResponse>, (StatusCode, String)> {
    // Get wallet info
    let wallet = state
        .storage
        .get_wallet(&user_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Wallet not found".to_string()))?;

    // Get offchain balance from ledger
    let offchain_balance = state
        .ledger
        .get_balance(&user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .total;

    // Get onchain balance from Solana (if client available)
    let onchain_balance = if let Some(client) = &state.solana_client {
        match client.get_token_balance(&wallet.pubkey).await {
            Ok(balance) => balance,
            Err(_) => 0, // If error, assume 0
        }
    } else {
        0
    };

    let synced = offchain_balance == onchain_balance;

    Ok(Json(WalletBalanceResponse {
        user_id,
        pubkey: wallet.pubkey,
        chain: wallet.chain,
        offchain_balance,
        onchain_balance,
        synced,
    }))
}

/// GET /api/wallet/:user_id - Get wallet info
pub async fn get_wallet(
    State(state): State<WalletState>,
    Path(user_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let wallet = state
        .storage
        .get_wallet(&user_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Wallet not found".to_string()))?;

    Ok(Json(json!({
        "user_id": wallet.user_id,
        "pubkey": wallet.pubkey,
        "chain": wallet.chain,
        "wallet_type": format!("{:?}", wallet.wallet_type),
        "created_at": wallet.created_at,
    })))
}

/// GET /api/wallet/admin/list - List all wallets (admin only)
pub async fn list_all_wallets(
    State(state): State<WalletState>,
) -> Result<Json<Vec<WalletInfo>>, (StatusCode, String)> {
    let wallets = state
        .storage
        .list_all_wallets()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(wallets))
}

/// POST /api/wallet/sync/{user_id} - Sync onchain balance from Solana Devnet
pub async fn sync_onchain_balance(
    State(state): State<WalletState>,
    Path(user_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Get wallet info
    let wallet = state
        .storage
        .get_wallet(&user_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, format!("Wallet not found for user {}", user_id)))?;

    // Connect to Solana Devnet
    let client = solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());
    
    // Parse pubkey
    let pubkey = Pubkey::from_str(&wallet.pubkey)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Invalid pubkey: {}", e)))?;
    
    // Get SOL balance (in lamports)
    let sol_balance = client
        .get_balance(&pubkey)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch SOL balance: {}", e)))?;

    // Get FODI token balance
    let fodi_balance = if let Ok(mint_address) = std::env::var("FODI_MINT_ADDRESS") {
        match Pubkey::from_str(&mint_address) {
            Ok(mint_pubkey) => {
                // Get associated token account address
                let ata = spl_associated_token_account::get_associated_token_address(&pubkey, &mint_pubkey);
                
                // Get token account balance
                match client.get_token_account_balance(&ata) {
                    Ok(balance) => balance.amount.parse::<u64>().unwrap_or(0),
                    Err(e) => {
                        tracing::debug!("No FODI token account found for {}: {}", user_id, e);
                        0
                    }
                }
            }
            Err(_) => 0
        }
    } else {
        0
    };
    
    tracing::info!(
        "🔄 Synced onchain balance for {}: {} SOL, {} FODI",
        user_id,
        sol_balance as f64 / 1_000_000_000.0,
        fodi_balance as f64 / 1_000_000_000.0
    );

    Ok(Json(json!({
        "success": true,
        "user_id": user_id,
        "pubkey": wallet.pubkey,
        "chain": "solana",
        "sol_balance": sol_balance,
        "sol_balance_ui": format!("{:.9} SOL", sol_balance as f64 / 1_000_000_000.0),
        "fodi_balance": fodi_balance,
        "fodi_balance_ui": format!("{:.9} FODI", fodi_balance as f64 / 1_000_000_000.0),
        "synced": true
    })))
}

/// Router setup
pub fn routes(ledger: Arc<TokenLedger>, wallet_db: Arc<sled::Db>) -> Router {
    // Create wallet storage with shared database connection
    let storage = Arc::new(WalletStorage::with_db(wallet_db, false));

    // TODO: Initialize Solana client from config
    let solana_client = None;

    let state = WalletState {
        storage,
        ledger,
        solana_client,
    };

    Router::new()
        .route("/", post(create_or_get_wallet))
        .route("/register", post(register_external_wallet))
        .route("/balance/{user_id}", get(get_wallet_balance))
        .route("/sync/{user_id}", post(sync_onchain_balance))
        .route("/{user_id}", get(get_wallet))
        .route("/admin/list", get(list_all_wallets))
        .with_state(state)
}

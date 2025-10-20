use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use solana_sdk::signature::Signer;

use crate::solana::{mint_tokens, transfer_tokens, get_balance, create_fodi_token_with_client};
use crate::solana::models::{MintRequest, TransferRequest, BalanceRequest, TokenResponse, StakeRequest};
use crate::state::AppState;

/// 🪙 Solana API routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/solana/mint", post(mint_handler))
        .route("/api/solana/transfer", post(transfer_handler))
        .route("/api/solana/balance", post(balance_handler))
        .route("/api/solana/balance/{wallet}", get(get_balance_by_path))
        .route("/api/solana/stake", post(stake_handler))
        .route("/api/solana/create-fodi-token", post(create_fodi_token_handler))
        .route("/api/solana/status", get(status_handler))
}

/// POST /api/solana/mint - Mint tokens to a wallet
async fn mint_handler(
    State(state): State<AppState>,
    Json(req): Json<MintRequest>,
) -> impl IntoResponse {
    // Check if Solana is configured
    let solana = match &state.solana {
        Some(client) => client,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(TokenResponse::error("Solana blockchain is not configured")),
            );
        }
    };

    // Parse wallet address
    let wallet: solana_sdk::pubkey::Pubkey = match req.wallet.parse() {
        Ok(pubkey) => pubkey,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(TokenResponse::error(format!("Invalid wallet address: {}", e))),
            );
        }
    };

    // Execute mint operation (using payer pubkey as mint authority)
    let mint_key = (*solana.payer).pubkey();
    match mint_tokens(&solana.rpc, &mint_key, &wallet, solana.payer.as_ref(), req.amount) {
        Ok(signature) => {
            tracing::info!("✅ Minted {} tokens to {}: {}", req.amount, req.wallet, signature);
            (
                StatusCode::OK,
                Json(TokenResponse::success(signature)),
            )
        }
        Err(e) => {
            tracing::error!("❌ Failed to mint tokens: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TokenResponse::error(format!("Mint failed: {}", e))),
            )
        }
    }
}

/// POST /api/solana/transfer - Transfer tokens between wallets
async fn transfer_handler(
    State(state): State<AppState>,
    Json(req): Json<TransferRequest>,
) -> impl IntoResponse {
    // Check if Solana is configured
    let solana = match &state.solana {
        Some(client) => client,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(TokenResponse::error("Solana blockchain is not configured")),
            );
        }
    };

    // Parse wallet addresses
    let _from: solana_sdk::pubkey::Pubkey = match req.from.parse() {
        Ok(pubkey) => pubkey,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(TokenResponse::error(format!("Invalid 'from' address: {}", e))),
            );
        }
    };

    let to: solana_sdk::pubkey::Pubkey = match req.to.parse() {
        Ok(pubkey) => pubkey,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(TokenResponse::error(format!("Invalid 'to' address: {}", e))),
            );
        }
    };

    // Determine token type and execute transfer
    let token_type = req.token.to_uppercase();
    let signature = match token_type.as_str() {
        "SOL" => {
            // Native SOL transfer
            match transfer_tokens(&solana.rpc, solana.payer.as_ref(), &to, req.amount) {
                Ok(sig) => {
                    tracing::info!("✅ Transferred {} SOL (lamports) from treasury to {}", 
                        req.amount, req.to);
                    sig
                }
                Err(e) => {
                    tracing::error!("❌ SOL transfer failed: {}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(TokenResponse::error(format!("SOL transfer failed: {}", e))),
                    );
                }
            }
        }
        "FODI" => {
            // FODI SPL token transfer
            let mint_address = match std::env::var("FODI_MINT_ADDRESS") {
                Ok(addr) => addr,
                Err(_) => {
                    return (
                        StatusCode::SERVICE_UNAVAILABLE,
                        Json(TokenResponse::error("FODI_MINT_ADDRESS not configured")),
                    );
                }
            };

            let mint_pubkey: solana_sdk::pubkey::Pubkey = match mint_address.parse() {
                Ok(pubkey) => pubkey,
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(TokenResponse::error(format!("Invalid FODI_MINT_ADDRESS: {}", e))),
                    );
                }
            };

            match crate::solana::token::transfer_spl_tokens(&solana.rpc, &mint_pubkey, solana.payer.as_ref(), &to, req.amount) {
                Ok(sig) => {
                    tracing::info!("✅ Transferred {} FODI tokens from treasury to {}", 
                        req.amount, req.to);
                    sig
                }
                Err(e) => {
                    tracing::error!("❌ FODI transfer failed: {}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(TokenResponse::error(format!("FODI transfer failed: {}", e))),
                    );
                }
            }
        }
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(TokenResponse::error(format!("Unsupported token type: {}. Use 'SOL' or 'FODI'", req.token))),
            );
        }
    };

    // Return success with transaction details
    tracing::info!("📦 Transfer complete: {} {} to {}", req.amount, token_type, req.to);
    
    (
        StatusCode::OK,
        Json(TokenResponse {
            status: "ok".to_string(),
            tx: Some(signature),
            balance: None,
            wallet: Some(req.to),
            error: None,
        }),
    )
}

/// POST /api/solana/balance - Get wallet balance
async fn balance_handler(
    State(state): State<AppState>,
    Json(req): Json<BalanceRequest>,
) -> impl IntoResponse {
    // Check if Solana is configured
    let solana = match &state.solana {
        Some(client) => client,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(TokenResponse::error("Solana blockchain is not configured")),
            );
        }
    };

    // Parse wallet address
    let wallet: solana_sdk::pubkey::Pubkey = match req.wallet.parse() {
        Ok(pubkey) => pubkey,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(TokenResponse::error(format!("Invalid wallet address: {}", e))),
            );
        }
    };

    // Get balance
    match get_balance(&solana.rpc, &wallet) {
        Ok(balance) => {
            tracing::info!("✅ Wallet {} balance: {} SOL", req.wallet, balance);
            (
                StatusCode::OK,
                Json(TokenResponse::balance(req.wallet, balance)),
            )
        }
        Err(e) => {
            tracing::error!("❌ Failed to get balance: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TokenResponse::error(format!("Balance query failed: {}", e))),
            )
        }
    }
}

/// GET /api/solana/status - Check Solana integration status
async fn status_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    match &state.solana {
        Some(client) => {
            // Try to get payer balance as health check
            let payer_pubkey = (*client.payer).pubkey();
            match get_balance(&client.rpc, &payer_pubkey) {
                Ok(balance) => {
                    Json(json!({
                        "status": "connected",
                        "payer": payer_pubkey.to_string(),
                        "balance": balance,
                        "message": "Solana blockchain is ready"
                    }))
                }
                Err(e) => {
                    Json(json!({
                        "status": "error",
                        "message": format!("RPC connection failed: {}", e)
                    }))
                }
            }
        }
        None => {
            Json(json!({
                "status": "disabled",
                "message": "Solana blockchain is not configured"
            }))
        }
    }
}

/// GET /api/solana/balance/{wallet} - Get wallet balance via path parameter
async fn get_balance_by_path(
    State(state): State<AppState>,
    axum::extract::Path(wallet): axum::extract::Path<String>,
) -> impl IntoResponse {
    // Check if Solana is configured
    let solana = match &state.solana {
        Some(client) => client,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "error": "Solana blockchain is not configured"
                })),
            );
        }
    };

    // Parse wallet address
    let wallet_pubkey: solana_sdk::pubkey::Pubkey = match wallet.parse() {
        Ok(pubkey) => pubkey,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": format!("Invalid wallet address: {}", e)
                })),
            );
        }
    };

    // Get balance
    match get_balance(&solana.rpc, &wallet_pubkey) {
        Ok(balance) => {
            tracing::info!("✅ Wallet {} balance: {} SOL", wallet, balance);
            (
                StatusCode::OK,
                Json(json!({
                    "wallet": wallet,
                    "balance": balance,
                    "status": "ok"
                })),
            )
        }
        Err(e) => {
            tracing::error!("❌ Failed to get balance: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Balance query failed: {}", e)
                })),
            )
        }
    }
}

/// POST /api/solana/stake - Stake SOL (placeholder for future implementation)
async fn stake_handler(
    State(state): State<AppState>,
    Json(req): Json<StakeRequest>,
) -> impl IntoResponse {
    // Check if Solana is configured
    let _solana = match &state.solana {
        Some(client) => client,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "error": "Solana blockchain is not configured"
                })),
            );
        }
    };

    // Validate amount
    if req.amount <= 0.0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Stake amount must be positive"
            })),
        );
    }

    tracing::info!("🪙 Stake request: {} SOL", req.amount);

    // TODO: Implement actual staking logic
    // For now, return a placeholder response
    (
        StatusCode::OK,
        Json(json!({
            "status": "pending",
            "message": format!("{} SOL queued for staking. Feature coming soon!", req.amount),
            "amount": req.amount,
            "note": "Staking functionality will be implemented with Solana Stake Pool integration"
        })),
    )
}

/// POST /api/solana/create-fodi-token - Create FODI SPL Token
/// 
/// Creates a new FODI token on Solana blockchain.
/// Request body:
/// ```json
/// {
///   "decimals": 9,
///   "initial_supply": 100000000000000000
/// }
/// ```
async fn create_fodi_token_handler(
    State(state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> impl IntoResponse {
    // Check if Solana is configured
    let solana = match &state.solana {
        Some(client) => client,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "error": "Solana blockchain is not configured"
                })),
            );
        }
    };

    // Parse parameters
    let decimals = req.get("decimals")
        .and_then(|v| v.as_u64())
        .unwrap_or(9) as u8;
    
    let initial_supply = req.get("initial_supply")
        .and_then(|v| v.as_u64())
        .unwrap_or(100_000_000_000_000_000); // 100 million FODI by default

    tracing::info!("🪙 Creating FODI token: decimals={}, supply={}", decimals, initial_supply);

    // Create token in blocking context
    let solana_clone = solana.clone();
    let result = match tokio::task::spawn_blocking(move || {
        create_fodi_token_with_client(&solana_clone, decimals, initial_supply)
    }).await {
        Ok(Ok(token_result)) => {
            tracing::info!("✅ FODI token created: {}", token_result.mint_pubkey);
            
            (
                StatusCode::CREATED,
                Json(json!({
                    "success": true,
                    "message": "FODI token created successfully",
                    "token": {
                        "mint_address": token_result.mint_pubkey.to_string(),
                        "token_account": token_result.associated_token.to_string(),
                        "decimals": token_result.decimals,
                        "initial_supply": token_result.initial_supply,
                        "human_readable_supply": token_result.initial_supply as f64 / 10_u64.pow(token_result.decimals as u32) as f64,
                    },
                    "transaction": {
                        "signature": token_result.tx_signature,
                        "explorer": format!("https://explorer.solana.com/tx/{}?cluster=devnet", token_result.tx_signature)
                    }
                })),
            )
        }
        Ok(Err(e)) => {
            tracing::error!("❌ Token creation failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Token creation failed: {}", e)
                })),
            )
        }
        Err(e) => {
            tracing::error!("❌ Task execution failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Task execution failed: {}", e)
                })),
            )
        }
    };

    result
}


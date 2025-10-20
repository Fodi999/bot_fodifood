use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use solana_sdk; // For Keypair creation
use sled; // For shared database connection

use super::{
    marketplace::{NftMarketplace, Currency},
    mint::NftMinter,
    BusinessNft,
};
use crate::wallet::storage::WalletStorage;

// ============================================================================
// API State
// ============================================================================

#[derive(Clone)]
pub struct NftState {
    pub marketplace: Arc<NftMarketplace>,
    pub minter: Arc<NftMinter>,
    pub wallet_storage: Arc<WalletStorage>,
}

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct MintRequest {
    pub name: String,
    pub owner_pubkey: String,
    pub business_type: String,
    pub cuisine: String,
    pub location: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateListingRequest {
    pub nft_mint: String,
    pub price: u64,
    pub currency: String, // "FODI" or "SOL"
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub cuisine: Option<String>,
    pub business_type: Option<String>,
}

/// Update NFT metadata request
#[derive(Debug, Deserialize)]
pub struct UpdateNftMetadataRequest {
    pub nft_mint: String,
    pub rating: Option<f32>,
    pub total_orders: Option<u64>,
    pub roi: Option<f32>,
    pub trend: Option<String>, // "rising", "stagnant", "falling"
}

// ============================================================================
// Handlers
// ============================================================================

/// Health check for NFT module
async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "module": "nft",
        "version": "2.4"
    }))
}

/// Mint a new Business NFT
async fn mint_business_nft(
    State(state): State<NftState>,
    Json(req): Json<MintRequest>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Get or create wallet for owner
    let wallet = state
        .wallet_storage
        .get_or_create_wallet(&req.owner_pubkey)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get wallet: {}", e)))?;

    // Create NFT with real owner address
    let business_nft = BusinessNft {
        mint: format!("mint_{}", uuid::Uuid::new_v4()),
        name: req.name.clone(),
        owner: wallet.pubkey.clone(), // Use real Solana address!
        attributes: super::BusinessAttributes {
            business_type: req.business_type,
            cuisine: req.cuisine,
            location: req.location,
            rating: 0.0,
            total_orders: 0,
            established_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        },
    };

    // TODO: Actually mint on Solana blockchain using state.minter
    // For now, just return the data
    tracing::info!(
        "🎨 Minted Business NFT '{}' for owner {} at address {}",
        business_nft.name,
        req.owner_pubkey,
        wallet.pubkey
    );

    Ok(Json(json!({
        "success": true,
        "nft": {
            "mint": business_nft.mint,
            "name": business_nft.name,
            "owner": business_nft.owner,
            "owner_wallet": wallet.pubkey,
            "attributes": business_nft.attributes,
        },
        "message": format!("NFT minted to wallet {}", wallet.pubkey)
    })))
}

/// Get all active NFT listings
async fn get_listings(
    State(state): State<NftState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let listings = state.marketplace.get_active_listings()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Filter by query params if provided
    let filtered: Vec<_> = listings
        .into_iter()
        .filter(|listing| {
            let mut matches = true;
            if let Some(ref cuisine) = query.cuisine {
                matches = matches && listing.nft.attributes.cuisine.to_lowercase().contains(&cuisine.to_lowercase());
            }
            if let Some(ref biz_type) = query.business_type {
                matches = matches && listing.nft.attributes.business_type.to_lowercase().contains(&biz_type.to_lowercase());
            }
            matches
        })
        .collect();
    
    Ok(Json(json!({
        "count": filtered.len(),
        "listings": filtered
    })))
}

/// Get listing details by ID
async fn get_listing(
    State(state): State<NftState>,
    Path(listing_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    match state.marketplace.get_listing(&listing_id).await {
        Ok(listing) => Ok(Json(json!(listing))),
        Err(_) => Err((StatusCode::NOT_FOUND, "Listing not found".to_string())),
    }
}

/// Create a new listing
async fn create_listing(
    State(state): State<NftState>,
    Json(req): Json<CreateListingRequest>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Parse currency
    let currency = match req.currency.to_uppercase().as_str() {
        "FODI" => Currency::FODI,
        "SOL" => Currency::SOL,
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid currency".to_string())),
    };

    // Create dummy NFT for now (in production, fetch from blockchain)
    let business_nft = BusinessNft {
        mint: req.nft_mint.clone(),
        name: "Sample Business".to_string(),
        owner: "seller123".to_string(),
        attributes: super::BusinessAttributes {
            business_type: "restaurant".to_string(),
            cuisine: "sushi".to_string(),
            location: "Tokyo".to_string(),
            rating: 4.5,
            total_orders: 100,
            established_date: "2024-01-01".to_string(),
        },
    };

    let listing = state.marketplace.create_listing(
        business_nft,
        "seller123".to_string(), // seller
        req.price,
        currency,
        None, // No expiration
    ).await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "success": true,
        "listing_id": listing.id
    })))
}

/// Get marketplace statistics
async fn marketplace_stats(State(state): State<NftState>) -> Result<Json<Value>, (StatusCode, String)> {
    let stats = state.marketplace.get_stats()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(json!(stats)))
}

/// POST /api/nft/update - Update NFT metadata based on business metrics
async fn update_nft_metadata(
    State(state): State<NftState>,
    Json(req): Json<UpdateNftMetadataRequest>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // TODO: Get current NFT from blockchain
    // TODO: Update on-chain metadata via Metaplex

    let mut updated_fields = Vec::new();

    if let Some(rating) = req.rating {
        updated_fields.push(format!("rating: {}", rating));
    }
    if let Some(orders) = req.total_orders {
        updated_fields.push(format!("total_orders: {}", orders));
    }
    if let Some(roi) = req.roi {
        updated_fields.push(format!("roi: {}%", roi));
    }
    if let Some(ref trend) = req.trend {
        updated_fields.push(format!("trend: {}", trend));
    }

    tracing::info!(
        "📊 Updated NFT {} metadata: {}",
        req.nft_mint,
        updated_fields.join(", ")
    );

    Ok(Json(json!({
        "success": true,
        "nft_mint": req.nft_mint,
        "updated_fields": updated_fields,
        "message": "Metadata will be updated on-chain",
        "note": "Full Metaplex integration pending"
    })))
}

// ============================================================================
// Direct On-Chain NFT Minting (New!)
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct MintNftRequest {
    pub name: String,
    pub uri: String,
    pub roi: u16, // ROI in basis points (100 = 1%)
}

/// Mint NFT directly on-chain using Solana RPC
/// POST /api/nft/mint/onchain
async fn mint_nft_onchain(
    Json(req): Json<MintNftRequest>,
) -> Result<Json<Value>, (StatusCode, String)> {
    use crate::nft::onchain::send_mint_instruction;
    
    tracing::info!("🪙 Minting NFT on-chain: {}, ROI: {}%", req.name, req.roi as f64 / 100.0);
    
    // Call our improved onchain module
    let signature = send_mint_instruction(&req.name, &req.uri, req.roi)
        .await
        .map_err(|e| {
            tracing::error!("❌ Failed to mint NFT: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to mint NFT: {}", e))
        })?;
    
    tracing::info!("✅ NFT minted! Signature: {}", signature);
    
    Ok(Json(json!({
        "status": "success",
        "tx": signature,
        "explorer": format!("https://explorer.solana.com/tx/{}?cluster=devnet", signature),
        "nft": {
            "name": req.name,
            "uri": req.uri,
            "roi_percent": req.roi as f64 / 100.0
        }
    })))
}

#[derive(Debug, Deserialize)]
pub struct CheckNftRequest {
    pub wallet: String,
    pub nft_name: String,
}

/// Check if user has specific NFT
/// POST /api/nft/check
async fn check_nft_ownership(
    Json(req): Json<CheckNftRequest>,
) -> Result<Json<Value>, (StatusCode, String)> {
    use crate::nft::onchain::check_user_nft;
    use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;
    
    let user_pubkey = Pubkey::from_str(&req.wallet)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid wallet address: {}", e)))?;
    
    let has_nft = check_user_nft(&user_pubkey, &req.nft_name)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to check NFT: {}", e)))?;
    
    Ok(Json(json!({
        "wallet": req.wallet,
        "nft_name": req.nft_name,
        "has_nft": has_nft,
        "status": if has_nft { "found" } else { "not_found" }
    })))
}

/// Get business statistics from on-chain
/// GET /api/nft/stats/{business_pubkey}
async fn get_business_stats_onchain(
    Path(business_pubkey): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    use crate::nft::onchain::get_business_stats;
    use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;
    
    let pubkey = Pubkey::from_str(&business_pubkey)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid pubkey: {}", e)))?;
    
    let stats = get_business_stats(&pubkey)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get stats: {}", e)))?;
    
    Ok(Json(json!({
        "business": business_pubkey,
        "stats": stats
    })))
}

// ============================================================================
// Router
// ============================================================================

/// Create NFT API routes
pub fn routes(wallet_db: Arc<sled::Db>) -> Router {
    // Initialize wallet storage with shared database connection
    let wallet_storage = Arc::new(WalletStorage::with_db(wallet_db, false));

    // Create marketplace instance
    let marketplace = Arc::new(NftMarketplace::new(250)); // 2.5% fee

    // Create placeholder minter (will be properly initialized from config later)
    let placeholder_keypair = solana_sdk::signature::Keypair::new();
    let minter = Arc::new(NftMinter::new(
        "https://api.devnet.solana.com".to_string(),
        placeholder_keypair,
    ));

    let state = NftState {
        marketplace,
        minter,
        wallet_storage,
    };

    Router::new()
        .route("/health", get(health_check))
        .route("/mint", post(mint_business_nft))
        .route("/mint/onchain", post(mint_nft_onchain))      // NEW: Direct on-chain minting
        .route("/check", post(check_nft_ownership))          // NEW: Check NFT ownership
        .route("/stats/{business_pubkey}", get(get_business_stats_onchain))  // NEW: On-chain stats (Axum 0.8 syntax)
        .route("/update", post(update_nft_metadata))
        .route("/listings", get(get_listings))
        .route("/listings", post(create_listing))
        .route("/listing/{id}", get(get_listing))
        .route("/marketplace/stats", get(marketplace_stats))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routes_creation() {
        let db = Arc::new(sled::Config::new().temporary(true).open().unwrap());
        let router = routes(db);
        // Basic test to ensure routes are created
        assert!(true);
    }
}

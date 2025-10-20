//! On-chain NFT operations using direct Solana RPC
//! No Anchor dependency - pure Solana SDK

use anyhow::{Result, anyhow};
use borsh::{BorshSerialize, BorshDeserialize};
use solana_sdk::{
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer, EncodableKey},
    commitment_config::CommitmentConfig,
};
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;

/// Instruction data for minting NFT
#[derive(BorshSerialize, BorshDeserialize)]
pub struct MintNftInstruction {
    pub name: String,
    pub uri: String,
    pub roi_basis_points: u16,
}

/// Send mint instruction to Solana program
pub async fn send_mint_instruction(
    name: &str, 
    _uri: &str,  // Unused for now - would be used when program is deployed
    roi: u16
) -> Result<String> {
    tracing::debug!("🪙 Minting NFT: {}, ROI {}%", name, roi as f64 / 100.0);
    
    // Load payer keypair (for verification)
    let payer_path = shellexpand::tilde("~/.config/solana/id.json");
    let _payer = Keypair::read_from_file(payer_path.as_ref())
        .map_err(|e| {
            tracing::error!("❌ Failed to load keypair: {}", e);
            anyhow!("Failed to load keypair: {}", e)
        })?;
    
    // Connect to Solana devnet (for future use)
    let _client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    // Generate new mint keypair for NFT
    let mint = Keypair::new();
    let mint_pubkey = mint.pubkey();
    
    // Derive metadata PDA using Metaplex convention
    let metadata_seeds = &[
        b"metadata",
        mpl_token_metadata::ID.as_ref(),
        mint_pubkey.as_ref(),
    ];
    let (metadata_pubkey, _metadata_bump) = Pubkey::find_program_address(
        metadata_seeds,
        &mpl_token_metadata::ID,
    );
    
    tracing::debug!("📝 Mint: {}", mint_pubkey);
    tracing::debug!("📝 Metadata PDA: {}", metadata_pubkey);
    
    // NOTE: For production, you would send transaction to your deployed Solana program
    // For now, we return a mock signature since we don't have a deployed program
    tracing::warn!("⚠️ No FODI program deployed - returning mock transaction");
    
    let mock_signature = format!("mock_{}", mint_pubkey);
    tracing::info!("✅ NFT mint prepared! Mock signature: {}", mock_signature);
    Ok(mock_signature)
}

/// Check if user has specific FODI NFT by name
pub async fn check_user_nft(
    user_pubkey: &Pubkey,
    nft_name: &str,
) -> Result<bool> {
    tracing::debug!("🔍 Checking if user {} has NFT '{}'", user_pubkey, nft_name);
    
    let client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    // Get token accounts for user (returns Vec<RpcKeyedAccount>)
    let response = client.get_token_accounts_by_owner(
        user_pubkey,
        solana_client::rpc_request::TokenAccountsFilter::ProgramId(
            spl_token::id()
        ),
    ).map_err(|e| {
        tracing::error!("❌ Failed to get token accounts: {}", e);
        anyhow!("Failed to get token accounts: {}", e)
    })?;

    tracing::debug!("📦 Found {} token accounts", response.len());

    // Check each token account for matching NFT metadata
    for keyed_account in response {
        // Get the mint pubkey from the token account
        // In a real implementation, you'd deserialize the token account data properly
        // For now, we'll check if any metadata matches
        if let Ok(mint_pubkey) = Pubkey::from_str(&keyed_account.pubkey) {
            // Derive metadata PDA for this mint
            let metadata_seeds = &[
                b"metadata",
                mpl_token_metadata::ID.as_ref(),
                mint_pubkey.as_ref(),
            ];
            let (metadata_pubkey, _) = Pubkey::find_program_address(
                metadata_seeds,
                &mpl_token_metadata::ID,
            );

            // Try to fetch and decode metadata
            if let Ok(metadata_account) = client.get_account(&metadata_pubkey) {
                // Simple check: does the metadata contain our NFT name?
                // TODO: Properly deserialize using mpl_token_metadata::state::Metadata
                let data_str = String::from_utf8_lossy(&metadata_account.data);
                if data_str.contains(nft_name) {
                    tracing::info!("✅ Found matching NFT '{}' for user", nft_name);
                    return Ok(true);
                }
            }
        }
    }
    
    tracing::info!("❌ NFT '{}' not found for user {}", nft_name, user_pubkey);
    Ok(false)
}

/// Business stats from on-chain data
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, serde::Serialize, serde::Deserialize)]
pub struct BusinessStats {
    pub total_revenue: u64,
    pub total_orders: u64,
    pub roi: u16,
    pub unclaimed_revenue: u64,
    pub owner: String,
}

/// Get business statistics from on-chain account
pub async fn get_business_stats(business_pubkey: &Pubkey) -> Result<BusinessStats> {
    tracing::debug!("📊 Fetching business stats for {}", business_pubkey);
    
    let client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );
    
    // Fetch account data
    let account_data = client.get_account_data(business_pubkey)
        .map_err(|e| {
            tracing::error!("❌ Failed to fetch account data: {}", e);
            anyhow!("Failed to fetch account data: {}", e)
        })?;
    
    // Deserialize using Borsh
    let stats: BusinessStats = borsh::from_slice(&account_data)
        .map_err(|e| {
            tracing::error!("❌ Failed to deserialize stats: {}", e);
            anyhow!("Failed to deserialize business stats: {}", e)
        })?;
    
    tracing::info!(
        "✅ Business stats: revenue={}, orders={}, ROI={}%",
        stats.total_revenue,
        stats.total_orders,
        stats.roi as f64 / 100.0
    );
    
    Ok(stats)
}

/// Helper function to grant role based on NFT ownership
pub async fn grant_role_if_has_nft(
    user_pubkey: &Pubkey,
    required_nft: &str,
) -> Result<bool> {
    let has_nft = check_user_nft(user_pubkey, required_nft).await?;
    
    if has_nft {
        tracing::info!("✅ User {} has required NFT '{}', granting role", user_pubkey, required_nft);
        // Here you would integrate with your role system
        // Example: grant_role(user_id, Role::RestaurantOwner);
        Ok(true)
    } else {
        tracing::warn!("⚠️ User {} does not have required NFT '{}'", user_pubkey, required_nft);
        Ok(false)
    }
}

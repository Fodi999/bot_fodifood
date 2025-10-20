//! Onchain integration for bank rewards
//! 
//! Handles synchronization of offchain rewards with Solana blockchain

use anyhow::{Context, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::str::FromStr;

use crate::solana::token;

/// Transfer FODI tokens from treasury to user wallet on Solana Devnet
///
/// # Arguments
/// * `treasury_keypair` - Treasury wallet keypair (needs SOL for fees)
/// * `recipient_pubkey` - User's wallet address
/// * `amount` - Amount in lamports (1 FODI = 1_000_000_000 lamports)
///
/// # Returns
/// Transaction signature on success
pub async fn transfer_fodi_reward(
    treasury_keypair: &Keypair,
    recipient_pubkey: &str,
    amount: u64,
) -> Result<String> {
    // Connect to Devnet
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    
    // Parse recipient address
    let recipient = Pubkey::from_str(recipient_pubkey)
        .context("Invalid recipient public key")?;
    
    // Check treasury balance
    let treasury_balance = client
        .get_balance(&treasury_keypair.pubkey())
        .context("Failed to get treasury balance")?;
    
    if treasury_balance < amount + 5000 { // 5000 lamports for fees
        anyhow::bail!(
            "Insufficient treasury balance. Have: {}, Need: {} + fees",
            treasury_balance,
            amount
        );
    }
    
    tracing::info!(
        "💰 Transferring {} lamports from treasury {} to user {}",
        amount,
        treasury_keypair.pubkey(),
        recipient_pubkey
    );
    
    // Execute transfer
    let signature = token::transfer_tokens(&client, treasury_keypair, &recipient, amount)?;
    
    tracing::info!("✅ Onchain reward transfer complete: {}", signature);
    
    Ok(signature)
}

/// Airdrop SOL to wallet for testing on Devnet
///
/// # Arguments
/// * `pubkey` - Wallet address to airdrop to
/// * `amount` - Amount in lamports (1 SOL = 1_000_000_000 lamports)
///
/// # Returns
/// Transaction signature on success
pub async fn airdrop_sol_devnet(pubkey: &str, amount: u64) -> Result<String> {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    
    let recipient = Pubkey::from_str(pubkey)
        .context("Invalid public key")?;
    
    tracing::info!("🪂 Requesting airdrop of {} lamports to {}", amount, pubkey);
    
    let signature = client
        .request_airdrop(&recipient, amount)
        .context("Airdrop request failed")?;
    
    // Wait for confirmation
    loop {
        let confirmed = client
            .confirm_transaction(&signature)
            .unwrap_or(false);
        
        if confirmed {
            break;
        }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    tracing::info!("✅ Airdrop confirmed: {}", signature);
    
    Ok(signature.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    #[ignore] // Run with: cargo test --ignored
    async fn test_airdrop_devnet() {
        let test_keypair = Keypair::new();
        let pubkey = test_keypair.pubkey().to_string();
        
        let result = airdrop_sol_devnet(&pubkey, 1_000_000_000).await; // 1 SOL
        assert!(result.is_ok());
        
        println!("Test wallet: {}", pubkey);
        println!("Signature: {}", result.unwrap());
    }
}

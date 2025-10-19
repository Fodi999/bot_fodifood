// 🪙 FODI Token Creator
// Binary for creating FODI SPL token on Solana

use fodifood_bot::solana::create_mint::create_fodi_token;
use solana_sdk::signature::{read_keypair_file, Signer};
use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    // Initialize logging (optional - use RUST_LOG=debug for verbose output)

    println!("🪙 FODI Token Creator");
    println!("===================");
    println!();

    // Get RPC URL from env or use devnet
    let rpc_url = env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    
    // Get keypair path from env or use default
    let keypair_path = env::var("SOLANA_KEYPAIR_PATH")
        .unwrap_or_else(|_| "tests/fixtures/test-keypair.json".to_string());

    println!("🌐 RPC URL: {}", rpc_url);
    println!("🔑 Keypair: {}", keypair_path);
    println!();

    // Read keypair
    println!("📖 Reading keypair...");
    let keypair = read_keypair_file(&keypair_path)
        .map_err(|e| anyhow::anyhow!("Failed to read keypair from {}: {}", keypair_path, e))?;
    
    println!("✅ Keypair loaded: {}", keypair.pubkey());
    println!();

    // Token parameters
    let decimals = 9u8;
    let initial_supply = 100_000_000_000_000_000u64; // 100 million FODI

    println!("📊 Token Parameters:");
    println!("   Name: FODI Token");
    println!("   Symbol: FODI");
    println!("   Decimals: {}", decimals);
    println!("   Initial Supply: {} FODI", initial_supply as f64 / 10_u64.pow(decimals as u32) as f64);
    println!();

    // Create token
    println!("🚀 Creating FODI token...");
    println!("⏳ This may take 10-30 seconds...");
    println!();

    let result = create_fodi_token(
        &rpc_url,
        &keypair,
        decimals,
        initial_supply,
    )?;

    println!();
    println!("===================");
    println!("🎉 SUCCESS!");
    println!("===================");
    println!();
    println!("{}", result.display());
    println!();
    println!("🔍 View on Solana Explorer:");
    
    let cluster = if rpc_url.contains("devnet") {
        "devnet"
    } else if rpc_url.contains("testnet") {
        "testnet"
    } else {
        "mainnet-beta"
    };
    
    println!("   Mint: https://explorer.solana.com/address/{}?cluster={}", result.mint_pubkey, cluster);
    println!("   Token Account: https://explorer.solana.com/address/{}?cluster={}", result.associated_token, cluster);
    println!("   Transaction: https://explorer.solana.com/tx/{}?cluster={}", result.tx_signature, cluster);
    println!();
    
    // Save results to file
    let output = format!(
        "FODI Token Creation Result\n\
         ==========================\n\
         \n\
         Mint Address: {}\n\
         Token Account: {}\n\
         Transaction: {}\n\
         Initial Supply: {} FODI\n\
         Decimals: {}\n\
         \n\
         Created on: {}\n\
         Network: {}\n\
         \n\
         Explorer Links:\n\
         - Mint: https://explorer.solana.com/address/{}?cluster={}\n\
         - Token Account: https://explorer.solana.com/address/{}?cluster={}\n\
         - Transaction: https://explorer.solana.com/tx/{}?cluster={}\n",
        result.mint_pubkey,
        result.associated_token,
        result.tx_signature,
        result.initial_supply as f64 / 10_u64.pow(result.decimals as u32) as f64,
        result.decimals,
        chrono::Utc::now().to_rfc3339(),
        cluster,
        result.mint_pubkey, cluster,
        result.associated_token, cluster,
        result.tx_signature, cluster
    );

    std::fs::write("fodi-token-creation.txt", &output)?;
    println!("💾 Results saved to: fodi-token-creation.txt");
    println!();
    
    Ok(())
}

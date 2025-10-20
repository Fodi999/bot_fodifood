use crate::solana::client::SolanaClient;
use crate::solana::token::{transfer_tokens, get_balance, mint_tokens};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use anyhow::Result;
use std::str::FromStr;

/// 🧪 Test: Transfer SOL on Devnet
/// 
/// This test verifies that we can successfully:
/// 1. Connect to Solana Devnet
/// 2. Send a transaction
/// 3. Get confirmation signature
///
/// Prerequisites:
/// - Solana keypair file at the specified path
/// - Keypair must have SOL balance on Devnet
/// 
/// To get Devnet SOL:
/// ```bash
/// solana airdrop 1 <YOUR_PUBKEY> --url devnet
/// ```
#[tokio::test]
#[ignore] // Remove #[ignore] when you have a real keypair configured
async fn test_transfer_sol_devnet() -> Result<()> {
    // Initialize Devnet client
    // TODO: Update path to your actual keypair
    let client = SolanaClient::devnet("tests/fixtures/test-keypair.json")?;
    
    println!("🪙 Connected to Solana Devnet");
    println!("📍 Payer address: {}", (*client.payer).pubkey());
    
    // Check payer balance first
    let payer_pubkey = (*client.payer).pubkey();
    let balance = get_balance(&client.rpc, &payer_pubkey)?;
    println!("💰 Payer balance: {} SOL", balance);
    
    if balance < 0.001 {
        println!("⚠️  Insufficient balance! Need at least 0.001 SOL");
        println!("💡 Run: solana airdrop 1 {} --url devnet", payer_pubkey);
        return Ok(());
    }
    
    // Test recipient wallet (replace with your test wallet)
    // This is a random devnet address for testing
    let recipient = Pubkey::from_str("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr")?;
    
    println!("📤 Sending to: {}", recipient);
    
    // Transfer 0.001 SOL (1_000_000 lamports)
    let amount = 1_000_000u64;
    println!("💸 Transferring {} lamports (0.001 SOL)...", amount);
    
    let sig = transfer_tokens(
        &client.rpc,
        client.payer.as_ref(),
        &recipient,
        amount,
    )?;
    
    println!("✅ Transaction successful!");
    println!("📝 Signature: {}", sig);
    println!("🔍 View on explorer: https://explorer.solana.com/tx/{}?cluster=devnet", sig);
    
    // Verify new balance
    let new_balance = get_balance(&client.rpc, &payer_pubkey)?;
    println!("💰 New payer balance: {} SOL", new_balance);
    
    assert!(!sig.is_empty(), "Transaction signature should not be empty");
    
    Ok(())
}

/// 🧪 Test: Get balance from Devnet
#[tokio::test]
#[ignore] // Remove #[ignore] when you want to run this test
async fn test_get_balance_devnet() -> Result<()> {
    let client = SolanaClient::devnet("tests/fixtures/test-keypair.json")?;
    
    let payer_pubkey = (*client.payer).pubkey();
    let balance = get_balance(&client.rpc, &payer_pubkey)?;
    
    println!("💰 Wallet {} balance: {} SOL", payer_pubkey, balance);
    
    assert!(balance >= 0.0, "Balance should be non-negative");
    
    Ok(())
}

/// 🧪 Test: Mint tokens (actually transfers SOL for testing)
#[tokio::test]
#[ignore] // Remove #[ignore] when you want to run this test
async fn test_mint_tokens_devnet() -> Result<()> {
    let client = SolanaClient::devnet("tests/fixtures/test-keypair.json")?;
    
    // Check balance first
    let payer_pubkey = (*client.payer).pubkey();
    let balance = get_balance(&client.rpc, &payer_pubkey)?;
    println!("💰 Initial balance: {} SOL", balance);
    
    if balance < 0.001 {
        println!("⚠️  Insufficient balance for mint test");
        return Ok(());
    }
    
    // Test recipient
    let recipient = Pubkey::from_str("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr")?;
    
    // Mint operation (simplified as SOL transfer for testing)
    let mint_key = payer_pubkey;
    let amount = 500_000u64; // 0.0005 SOL
    
    println!("🪙 Minting {} lamports to {}...", amount, recipient);
    
    let sig = mint_tokens(
        &client.rpc,
        &mint_key,
        &recipient,
        client.payer.as_ref(),
        amount,
    )?;
    
    println!("✅ Mint successful!");
    println!("📝 Signature: {}", sig);
    println!("🔍 View: https://explorer.solana.com/tx/{}?cluster=devnet", sig);
    
    assert!(!sig.is_empty());
    
    Ok(())
}

/// 🧪 Test: Check Devnet connection
#[tokio::test(flavor = "multi_thread")]
async fn test_devnet_connection() -> Result<()> {
    // This test doesn't require a keypair file
    use solana_client::rpc_client::RpcClient;
    
    println!("🔌 Connecting to Solana Devnet...");
    
    // Spawn blocking task for RPC call
    let result = tokio::task::spawn_blocking(|| {
        let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
        rpc.get_version()
    }).await;
    
    match result {
        Ok(Ok(version)) => {
            println!("✅ Connected to Solana Devnet");
            println!("📦 Solana version: {:?}", version);
            assert!(!version.solana_core.is_empty());
        }
        Ok(Err(e)) => {
            println!("❌ Failed to connect to Devnet: {}", e);
            panic!("Devnet connection failed: {}", e);
        }
        Err(e) => {
            println!("❌ Task join error: {}", e);
            panic!("Task failed: {}", e);
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod helper_tests {
    use super::*;
    
    /// Test Pubkey parsing
    #[test]
    fn test_pubkey_parsing() {
        let valid_address = "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr";
        let result = Pubkey::from_str(valid_address);
        
        assert!(result.is_ok(), "Valid Solana address should parse correctly");
        println!("✅ Pubkey parsed: {}", result.unwrap());
    }
    
    /// Test invalid Pubkey
    #[test]
    fn test_invalid_pubkey() {
        let invalid_address = "not-a-valid-address";
        let result = Pubkey::from_str(invalid_address);
        
        assert!(result.is_err(), "Invalid address should fail to parse");
        println!("✅ Invalid pubkey correctly rejected");
    }
    
    /// Test lamports to SOL conversion
    #[test]
    fn test_lamports_conversion() {
        let lamports = 1_000_000_000u64;
        let sol = lamports as f64 / 1_000_000_000.0;
        
        assert_eq!(sol, 1.0, "1 billion lamports = 1 SOL");
        
        let small_amount = 1_000_000u64;
        let small_sol = small_amount as f64 / 1_000_000_000.0;
        assert_eq!(small_sol, 0.001, "1 million lamports = 0.001 SOL");
        
        println!("✅ Lamports conversion works correctly");
    }
}

/// 🪙 Test: Create FODI Token on Devnet
/// 
/// This test creates a real SPL token (FODI) on Solana Devnet.
/// 
/// Prerequisites:
/// - Solana keypair file at tests/fixtures/test-keypair.json
/// - Keypair must have ~0.5 SOL balance on Devnet for rent + fees
/// 
/// To prepare:
/// ```bash
/// # Generate keypair
/// solana-keygen new --outfile tests/fixtures/test-keypair.json
/// 
/// # Get Devnet SOL
/// solana airdrop 2 $(solana-keygen pubkey tests/fixtures/test-keypair.json) --url devnet
/// ```
#[tokio::test(flavor = "multi_thread")]
#[ignore] // Remove #[ignore] when ready to create the token
async fn test_create_fodi_token_devnet() -> Result<()> {
    use crate::solana::create_mint::create_fodi_token;
    use solana_sdk::signature::read_keypair_file;
    
    println!("🚀 Starting FODI token creation test...");
    
    // Read keypair
    let keypair_path = "tests/fixtures/test-keypair.json";
    let payer = read_keypair_file(keypair_path)
        .expect("Failed to read keypair. Run: solana-keygen new --outfile tests/fixtures/test-keypair.json");
    
    println!("💼 Payer address: {}", payer.pubkey());
    
    // Create FODI token with:
    // - 9 decimals (standard for Solana tokens)
    // - 100 million FODI initial supply
    let decimals = 9u8;
    let initial_supply = 100_000_000_000_000_000u64; // 100 million with 9 decimals
    
    println!("📊 Token parameters:");
    println!("   Decimals: {}", decimals);
    println!("   Initial Supply: {} FODI", initial_supply as f64 / 10_u64.pow(decimals as u32) as f64);
    
    // Create the token
    let result = tokio::task::spawn_blocking(move || {
        create_fodi_token(
            "https://api.devnet.solana.com",
            &payer,
            decimals,
            initial_supply,
        )
    })
    .await
    .expect("Task failed")?;
    
    // Display results
    println!("\n{}", result.display());
    println!("\n🔍 View on Solana Explorer:");
    println!("   Mint: https://explorer.solana.com/address/{}?cluster=devnet", result.mint_pubkey);
    println!("   Token Account: https://explorer.solana.com/address/{}?cluster=devnet", result.associated_token);
    println!("   Transaction: https://explorer.solana.com/tx/{}?cluster=devnet", result.tx_signature);
    
    // Verify the token was created
    assert!(!result.tx_signature.is_empty(), "Transaction signature should not be empty");
    assert_eq!(result.decimals, decimals, "Decimals should match");
    assert_eq!(result.initial_supply, initial_supply, "Supply should match");
    
    println!("\n✅ FODI token creation test passed!");
    
    Ok(())
}

/// 🪙 Test: Create FODI Token using SolanaClient wrapper
#[tokio::test(flavor = "multi_thread")]
#[ignore] // Remove #[ignore] when ready to test
async fn test_create_fodi_token_with_client() -> Result<()> {
    use crate::solana::create_mint::create_fodi_token_with_client;
    
    println!("🚀 Creating FODI token with SolanaClient...");
    
    // Initialize client
    let client = SolanaClient::devnet("tests/fixtures/test-keypair.json")?;
    
    println!("💼 Payer: {}", (*client.payer).pubkey());
    
    // Create token: 100 million FODI with 9 decimals
    let result = tokio::task::spawn_blocking(move || {
        create_fodi_token_with_client(
            &client,
            9,
            100_000_000_000_000_000,
        )
    })
    .await
    .expect("Task failed")?;
    
    println!("\n{}", result.display());
    
    assert!(!result.tx_signature.is_empty());
    
    println!("\n✅ FODI token created successfully with SolanaClient!");
    
    Ok(())
}

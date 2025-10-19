use fodifood_bot::solana::add_metadata::add_token_metadata;
use solana_sdk::signature::{read_keypair_file, Signer};
use std::env;

fn main() -> anyhow::Result<()> {
    // Инициализируем логирование
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    println!("🎨 FODI Token - Adding Metadata");
    println!("================================\n");

    // Параметры
    let rpc_url = env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    
    let keypair_path = env::var("SOLANA_KEYPAIR_PATH")
        .unwrap_or_else(|_| "tests/fixtures/test-keypair.json".to_string());

    // Mint address вашего FODI токена
    let mint_address = env::var("FODI_MINT_ADDRESS")
        .unwrap_or_else(|_| "GAVBLXA8aKiptSk8vP1MYZyWYZBvsJH4DdsopEQBkuA".to_string());

    // Metadata parameters
    let name = "FODI Token";
    let symbol = "FODI";
    
    // URL к JSON metadata файлу
    // TODO: После коммита на GitHub, этот URL будет работать
    let uri = env::var("METADATA_URI")
        .unwrap_or_else(|_| "https://raw.githubusercontent.com/Fodi999/bot_fodifood/main/assets/fodi-metadata.json".to_string());

    println!("📋 Configuration:");
    println!("   RPC URL: {}", rpc_url);
    println!("   Keypair: {}", keypair_path);
    println!("   Mint Address: {}", mint_address);
    println!("   Token Name: {}", name);
    println!("   Token Symbol: {}", symbol);
    println!("   Metadata URI: {}", uri);
    println!();

    // Читаем keypair
    println!("🔑 Loading keypair...");
    let payer = read_keypair_file(&keypair_path)
        .map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?;
    println!("✅ Payer address: {}", payer.pubkey());
    println!();

    // Добавляем metadata
    println!("🎨 Creating metadata account...");
    println!("⏳ This may take a few seconds...");
    println!();

    let result = add_token_metadata(
        &rpc_url,
        &payer,
        &mint_address,
        name,
        symbol,
        &uri,
    )?;

    // Выводим результат
    println!("\n{}", "=".repeat(60));
    println!("{}", result.display());
    println!("{}", "=".repeat(60));
    println!();

    println!("🔍 View on Solana Explorer:");
    println!("   Metadata Account:");
    println!("   https://explorer.solana.com/address/{}?cluster=devnet", result.metadata_address);
    println!();
    println!("   Transaction:");
    println!("   https://explorer.solana.com/tx/{}?cluster=devnet", result.tx_signature);
    println!();

    println!("✅ Done! Your token now has metadata!");
    println!();
    println!("💡 Note: It may take a few minutes for wallets (Phantom, etc.)");
    println!("   to fetch and display the updated metadata.");
    println!();

    Ok(())
}

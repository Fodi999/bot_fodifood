use fodifood_bot::database::{DatabaseClient, ai::AICacheOps, blockchain::*, analytics::*};
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    println!("🗄️  FodiFood Database Module Demo\n");
    
    // Connect to database
    let database_url = std::env::var("DATABASE_URL")?;
    let db = DatabaseClient::new(&database_url).await?;
    println!("✅ Connected to PostgreSQL\n");
    
    // ============================================
    // 🤖 AI Operations
    // ============================================
    println!("🤖 AI Operations:");
    
    let cache = AICacheOps::new(db.pool());
    
    // Set cache
    cache.set(
        "demo_query_123",
        "What is FODI token?",
        "FODI is a utility token for the FodiFood ecosystem",
        "groq",
        3600
    ).await?;
    println!("  ✅ Cache entry created");
    
    // Get cache
    if let Some(cached) = cache.get("demo_query_123").await? {
        println!("  ✅ Cache hit! Response: {}", cached.response);
        println!("     Hit count: {}", cached.hit_count);
    }
    
    // Cache stats
    let stats = cache.stats().await?;
    println!("  📊 Cache stats:");
    println!("     Total entries: {}", stats.total_entries.unwrap_or(0));
    println!("     Total hits: {}", stats.total_hits.unwrap_or(0));
    
    // ============================================
    // 🔗 Blockchain Operations
    // ============================================
    println!("\n🔗 Blockchain Operations:");
    
    let blockchain = BlockchainOps::new(db.pool());
    
    // Create transaction
    let tx_id = uuid::Uuid::new_v4().to_string();
    let tx_db_id = blockchain.create_transaction(
        &tx_id,
        Some("sender_address_123"),
        "receiver_address_456",
        1000000, // 1 FODI (decimals = 6)
        "transfer"
    ).await?;
    println!("  ✅ Transaction created: {}", tx_db_id);
    
    // Update transaction status
    blockchain.update_status(&tx_id, "confirmed", Some("on_chain_tx_hash_xyz")).await?;
    println!("  ✅ Transaction confirmed");
    
    // Get transaction
    if let Some(tx) = blockchain.get_transaction(&tx_id).await? {
        println!("  📦 Transaction details:");
        println!("     From: {:?}", tx.from_address);
        println!("     To: {}", tx.to_address);
        println!("     Amount: {} FODI", tx.amount as f64 / 1_000_000.0);
        println!("     Status: {}", tx.status);
    }
    
    // ============================================
    // 💰 Wallet Operations
    // ============================================
    println!("\n💰 Wallet Operations:");
    
    let wallet_ops = WalletOps::new(db.pool());
    
    // Create wallet
    let user_id = uuid::Uuid::new_v4();
    let public_key = format!("demo_public_key_{}", uuid::Uuid::new_v4());
    
    let wallet_id = wallet_ops.create_wallet(
        user_id,
        &public_key,
        None,
        "solana"
    ).await?;
    println!("  ✅ Wallet created: {}", wallet_id);
    
    // Update balance
    wallet_ops.update_balance(&public_key, 5000000).await?;
    println!("  ✅ Balance updated: 5 FODI");
    
    // Get wallet
    if let Some(wallet) = wallet_ops.get_wallet(&public_key).await? {
        println!("  💳 Wallet details:");
        println!("     User ID: {}", wallet.user_id);
        println!("     Balance: {} FODI", wallet.balance as f64 / 1_000_000.0);
        println!("     Type: {}", wallet.wallet_type);
    }
    
    // Total supply
    let total_supply = wallet_ops.get_total_supply().await?;
    println!("  📊 Total FODI supply: {} FODI", total_supply as f64 / 1_000_000.0);
    
    // ============================================
    // 🧩 NFT Operations
    // ============================================
    println!("\n🧩 NFT Operations:");
    
    let nft_ops = NFTOps::new(db.pool());
    
    // Create NFT
    let mint_address = "nft_mint_address_abc";
    let nft_metadata = serde_json::json!({
        "description": "Demo Business NFT",
        "attributes": [
            {"trait_type": "Category", "value": "Restaurant"},
            {"trait_type": "Rating", "value": "4.5"}
        ]
    });
    
    let nft_id = nft_ops.create_nft(
        mint_address,
        "Demo Restaurant NFT",
        Some("DEMO"),
        Some("https://example.com/nft.json"),
        Some(&public_key),
        Some(nft_metadata)
    ).await?;
    println!("  ✅ NFT created: {}", nft_id);
    
    // Get NFT
    if let Some(nft) = nft_ops.get_nft(mint_address).await? {
        println!("  🖼️  NFT details:");
        println!("     Name: {}", nft.name);
        println!("     Symbol: {:?}", nft.symbol);
        println!("     Owner: {:?}", nft.owner_address);
    }
    
    // ============================================
    // 🎁 Reward Operations
    // ============================================
    println!("\n🎁 Reward Operations:");
    
    let reward_ops = RewardOps::new(db.pool());
    
    // Create reward
    let reward_id = reward_ops.create_reward(
        user_id,
        Some(12345),
        500000, // 0.5 FODI reward
        Some("Order completion reward"),
        Some(&tx_id)
    ).await?;
    println!("  ✅ Reward created: {}", reward_id);
    
    // Get user rewards
    let rewards = reward_ops.get_user_rewards(user_id).await?;
    println!("  🎁 User rewards: {} entries", rewards.len());
    
    // Total rewards
    let total_rewards = reward_ops.get_user_total_rewards(user_id).await?;
    println!("  📊 Total rewards: {} FODI", total_rewards as f64 / 1_000_000.0);
    
    // ============================================
    // 📈 Analytics Operations
    // ============================================
    println!("\n📈 Analytics Operations:");
    
    let metrics = MetricsOps::new(db.pool());
    
    // Record metrics
    metrics.record("api_latency_ms", 125.5, Some(serde_json::json!({
        "endpoint": "/api/businesses",
        "method": "GET"
    }))).await?;
    println!("  ✅ Metric recorded: api_latency_ms = 125.5ms");
    
    // Record event
    let events = EventsOps::new(db.pool());
    
    events.record(
        "order_completed",
        Some(user_id),
        None,
        serde_json::json!({
            "order_id": 12345,
            "total": 25.50,
            "items": 3
        })
    ).await?;
    println!("  ✅ Event recorded: order_completed");
    
    // Get event count
    let now = Utc::now();
    let one_hour_ago = now - chrono::Duration::hours(1);
    let event_count = events.count_by_type("order_completed", one_hour_ago, now).await?;
    println!("  📊 Events in last hour: {}", event_count);
    
    // ============================================
    println!("\n🎉 All operations completed successfully!");
    println!("\n💡 You now have:");
    println!("   ✅ Unified PostgreSQL database");
    println!("   ✅ Clean schema separation (ai, blockchain, analytics, public)");
    println!("   ✅ Type-safe Rust operations with sqlx");
    println!("   ✅ Ready for production!");
    
    Ok(())
}

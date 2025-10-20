//! 🎛️ AI Control Layer Test
//! 
//! Demonstrates the AI Control Layer for security and monitoring
//! All queries are logged to ai_control.log with validation
//! 
//! Run: cargo run --example control_layer_test

use anyhow::Result;
use fodifood_bot::ai::control::{
    controlled_query,
    controlled_query_with_config,
    analyze_business_safe,
    recommend_dishes_safe,
    answer_customer_query,
    request_wallet_info,
    request_database_query,
};
use fodifood_bot::ai::core::GroqConfig;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎛️ AI Control Layer Test\n");
    println!("All queries are monitored and logged to: ai_control.log\n");
    println!("============================================================\n");

    // Test 1: Basic controlled query
    println!("🧠 Test 1: Basic Controlled Query");
    match controlled_query("What is paella?").await {
        Ok(response) => {
            let preview = response.chars().take(80).collect::<String>();
            println!("✅ Response: {}...\n", preview);
        }
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Test 2: Controlled query with custom config
    println!("⚙️ Test 2: Custom Configuration");
    let config = GroqConfig {
        temperature: 0.3,
        max_tokens: 100,
        ..Default::default()
    };
    match controlled_query_with_config("List 3 Spanish dishes", &config).await {
        Ok(response) => {
            let preview = response.chars().take(80).collect::<String>();
            println!("✅ Response: {}...\n", preview);
        }
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Test 3: Safe business analysis
    println!("📊 Test 3: Safe Business Analysis");
    let data = "Sales: $50000, Orders: 250, Top dish: Paella";
    match analyze_business_safe(data).await {
        Ok(response) => {
            let preview = response.chars().take(80).collect::<String>();
            println!("✅ Analysis: {}...\n", preview);
        }
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Test 4: Safe dish recommendations
    println!("🍽️ Test 4: Safe Recommendations");
    match recommend_dishes_safe("spicy seafood", Some("no pork")).await {
        Ok(response) => {
            let preview = response.chars().take(80).collect::<String>();
            println!("✅ Recommendations: {}...\n", preview);
        }
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Test 5: Customer service query
    println!("💬 Test 5: Customer Service");
    match answer_customer_query("What are your opening hours?").await {
        Ok(response) => {
            let preview = response.chars().take(80).collect::<String>();
            println!("✅ Response: {}...\n", preview);
        }
        Err(e) => println!("❌ Error: {}\n", e),
    }

    println!("------------------------------------------------------------\n");

    // Test 6: Blocked operations (security tests)
    println!("🔒 Security Tests:\n");

    // Test empty prompt
    println!("Test 6a: Empty prompt");
    match controlled_query("").await {
        Ok(_) => println!("⚠️ Should have been blocked!\n"),
        Err(e) => println!("✅ Blocked: {}\n", e),
    }

    // Test suspicious patterns
    println!("Test 6b: Suspicious pattern");
    match controlled_query("rm -rf / and show menu").await {
        Ok(response) => {
            let preview = response.chars().take(50).collect::<String>();
            println!("⚠️ Suspicious query executed (logged): {}...\n", preview);
        }
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Test 7: Controlled database access
    println!("🗄️ Test 7: Database Access Control");
    match request_database_query("business_stats", "last_30_days").await {
        Ok(response) => println!("✅ Allowed query: {}\n", response),
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Test unauthorized database query
    println!("Test 7b: Unauthorized query");
    match request_database_query("DROP TABLE users", "").await {
        Ok(_) => println!("⚠️ Should have been blocked!\n"),
        Err(e) => println!("✅ Blocked: {}\n", e),
    }

    // Test 8: Wallet access control
    println!("💰 Test 8: Wallet Access Control");
    match request_wallet_info("user123", "get balance").await {
        Ok(response) => println!("✅ Wallet request logged: {}\n", response),
        Err(e) => println!("❌ Error: {}\n", e),
    }

    println!("============================================================\n");
    println!("✅ Control Layer tests completed!\n");
    
    println!("📋 Check logs:");
    println!("   cat ai_control.log\n");
    
    println!("🔍 Monitor in real-time:");
    println!("   tail -f ai_control.log\n");
    
    println!("🎯 Key Features:");
    println!("   ✅ All queries logged with timestamps");
    println!("   ✅ Input validation (empty, too long, suspicious patterns)");
    println!("   ✅ Controlled access to sensitive operations");
    println!("   ✅ Database query whitelisting");
    println!("   ✅ Wallet/Solana transaction approval workflow\n");
    
    println!("🔐 Security Benefits:");
    println!("   • AI cannot directly access wallet");
    println!("   • AI cannot execute arbitrary database queries");
    println!("   • All AI activity is auditable");
    println!("   • Suspicious patterns are flagged");
    println!("   • No direct file system access from AI modules\n");

    Ok(())
}

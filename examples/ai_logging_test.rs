//! 📝 AI Activity Logging Test
//! 
//! Tests the AI activity logging functionality
//! All prompts and responses are logged to ai_activity.log
//! 
//! Run: cargo run --example ai_logging_test
//! Monitor: tail -f ai_activity.log

use anyhow::Result;
use fodifood_bot::ai::thinker::Thinker;

#[tokio::main]
async fn main() -> Result<()> {
    println!("📝 AI Activity Logging Test\n");
    println!("All activity will be logged to: ai_activity.log");
    println!("Monitor in another terminal with: tail -f ai_activity.log\n");
    println!("============================================================\n");

    // Test 1: Simple thinking
    println!("🧠 Test 1: Simple Thinking");
    let response1 = Thinker::think("What are the top 3 seafood dishes?").await?;
    let preview1 = response1.chars().take(80).collect::<String>();
    println!("✅ Response: {}...\n", preview1);

    // Test 2: Fast thinking
    println!("⚡ Test 2: Fast Thinking");
    let response2 = Thinker::think_fast("What is sushi?").await?;
    let preview2 = response2.chars().take(80).collect::<String>();
    println!("✅ Response: {}...\n", preview2);

    // Test 3: Business analysis
    println!("📊 Test 3: Business Analysis");
    let data = "Sales: $50000, Orders: 250, Top dish: Paella (78 orders)";
    let response3 = Thinker::analyze_business(data).await?;
    let preview3 = response3.chars().take(80).collect::<String>();
    println!("✅ Response: {}...\n", preview3);

    // Test 4: Recommendations
    println!("🎯 Test 4: Personalized Recommendations");
    let response4 = Thinker::get_ai_recommendation(
        "I want something spicy with seafood",
        Some("No pork, likes shrimp")
    ).await?;
    let preview4 = response4.chars().take(80).collect::<String>();
    println!("✅ Response: {}...\n", preview4);

    println!("------------------------------------------------------------");
    println!("✅ All tests completed!\n");
    
    println!("📋 Check the log file:");
    println!("   cat ai_activity.log");
    println!("\n💡 Each entry contains:");
    println!("   - ⏰ Timestamp (UTC)");
    println!("   - 🧠 Prompt sent to AI");
    println!("   - 💬 Response received");
    println!("   - 🏷️ Tags: [FAST], [BUSINESS], [RECOMMEND]\n");
    
    println!("🔍 Monitor in real-time:");
    println!("   tail -f ai_activity.log\n");

    Ok(())
}

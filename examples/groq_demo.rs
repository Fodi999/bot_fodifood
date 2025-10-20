//! Example: Using Groq AI in FodiFood
//! 
//! This example demonstrates how to use the integrated Groq API
//! for various AI tasks in the FodiFood system.

use anyhow::Result;
use fodifood_bot::ai::thinker::Thinker;
use fodifood_bot::ai::core::{query_groq, GroqConfig, GroqModel};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🧠 FodiFood Groq AI Integration Examples\n");
    println!("{}", "=".repeat(60));

    // Example 1: Simple thinking
    println!("\n📝 Example 1: Simple Thinking (Llama 3.1 70B)");
    println!("{}", "-".repeat(60));
    
    match Thinker::think("What are the health benefits of salmon?").await {
        Ok(response) => println!("🧠 AI: {}\n", response),
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Example 2: Fast thinking
    println!("\n⚡ Example 2: Fast Thinking (Llama 3.1 8B Instant)");
    println!("{}", "-".repeat(60));
    
    match Thinker::think_fast("Summarize pizza in 10 words").await {
        Ok(response) => println!("⚡ AI: {}\n", response),
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Example 3: Business analysis
    println!("\n📊 Example 3: Business Analysis");
    println!("{}", "-".repeat(60));
    
    let business_data = "
        Restaurant: FodiFood
        Period: August 2025
        Revenue: 1.2M SOL
        Orders: 342
        Top dish: Paella Marinera (78 orders)
        Avg check: 3,500 SOL
        Peak hours: 12:00-14:00, 19:00-21:00
        Customer satisfaction: 4.7/5
    ";
    
    match Thinker::analyze_business(business_data).await {
        Ok(analysis) => println!("📊 Analysis:\n{}\n", analysis),
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Example 4: Personalized recommendations
    println!("\n🎯 Example 4: Personalized Recommendations");
    println!("{}", "-".repeat(60));
    
    match Thinker::get_ai_recommendation(
        "голодный, хочу что-то острое и сытное",
        Some("люблю морепродукты, не ем свинину")
    ).await {
        Ok(recommendation) => println!("🎯 Recommendation:\n{}\n", recommendation),
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Example 5: Entity extraction
    println!("\n🔍 Example 5: AI-Powered Entity Extraction");
    println!("{}", "-".repeat(60));
    
    let text = "Я хочу заказать паэлью с креветками и бокал белого вина";
    
    match Thinker::extract_with_ai(text, "dish name").await {
        Ok(Some(dish)) => println!("🔍 Extracted dish: {}", dish),
        Ok(None) => println!("🔍 No dish found"),
        Err(e) => println!("❌ Error: {}", e),
    }

    // Example 6: Direct Groq API call with custom config
    println!("\n⚙️ Example 6: Custom Groq Configuration");
    println!("{}", "-".repeat(60));
    
    let _config = GroqConfig {
        model: GroqModel::Mixtral,  // Use Mixtral for longer context
        temperature: 0.3,            // More focused/deterministic
        max_tokens: 500,
        top_p: 0.9,
    };
    
    let prompt = "List 3 popular seafood dishes in Spanish cuisine";
    
    match query_groq(prompt).await {
        Ok(response) => println!("⚙️ Response (Mixtral):\n{}\n", response),
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Example 7: Mood detection + AI response
    println!("\n😊 Example 7: Mood-Aware AI Response");
    println!("{}", "-".repeat(60));
    
    let user_message = "Устал после работы, хочу чего-то лёгкого";
    let mood = Thinker::detect_mood(user_message);
    let emotion = Thinker::extract_emotion(user_message);
    
    println!("Detected mood: {}", mood);
    println!("Detected emotion: {:?}", emotion);
    
    let prompt = format!(
        "User is feeling {} and {}. Recommend a light meal that would help them relax.",
        mood,
        emotion.unwrap_or("neutral")
    );
    
    match Thinker::think(&prompt).await {
        Ok(response) => println!("\n🧠 AI Response:\n{}\n", response),
        Err(e) => println!("❌ Error: {}\n", e),
    }

    println!("\n{}", "=".repeat(60));
    println!("✅ All examples completed!");
    println!("\n💡 Tip: Set GROQ_API_KEY environment variable to run these examples");
    println!("   Get your key at: https://console.groq.com\n");

    Ok(())
}

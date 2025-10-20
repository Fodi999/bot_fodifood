//! 🧠 Context Memory Demo - Conversation memory and personalization
//! 
//! This example demonstrates the memory system:
//! - Save conversation history
//! - Extract user preferences
//! - Context-aware responses
//! - User personalization
//! 
//! Run: cargo run --example context_memory_demo

use fodifood_bot::ai::BotMemory;

#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  🧠 Context Memory Demo - AI Remembers You                 ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let memory = BotMemory::new();
    let user_id = "demo_user_123";
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 Conversation Simulation\n");
    
    // Message 1: User introduces themselves
    println!("👤 User: Привет! Меня зовут Дмитрий");
    memory.set_user_name(user_id, "Дмитрий".to_string()).await;
    memory.add_message(user_id, "Привет! Меня зовут Дмитрий".to_string()).await;
    
    if let Some(name) = memory.get_user_name(user_id).await {
        println!("🤖 Bot: Привет, {}! Рад познакомиться! 👋\n", name);
    }
    
    // Message 2: User expresses preference
    println!("👤 User: Я люблю острое");
    memory.add_message(user_id, "Я люблю острое".to_string()).await;
    memory.extract_and_save_preferences(user_id, "Я люблю острое").await;
    println!("🤖 Bot: Запомнил! Ты любишь острое 🌶️\n");
    
    // Message 3: User asks for recommendation
    println!("👤 User: Посоветуй что-нибудь с креветками");
    memory.add_message(user_id, "Посоветуй что-нибудь с креветками".to_string()).await;
    memory.extract_and_save_preferences(user_id, "Посоветуй что-нибудь с креветками").await;
    
    if let Some(context) = memory.get_recommendation_context(user_id).await {
        println!("🤖 Bot: Конечно! Учитывая что ты любишь {}, рекомендую острую паэлью с креветками! 🍤🌶️\n", context);
    }
    
    // Message 4: Check memory
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Memory Stats\n");
    
    let message_count = memory.get_message_count(user_id).await;
    println!("💬 Total messages: {}", message_count);
    
    if let Some(name) = memory.get_user_name(user_id).await {
        println!("👤 User name: {}", name);
    }
    
    if let Some(spicy) = memory.get_preference(user_id, "spicy").await {
        println!("🌶️  Likes spicy: {}", spicy);
    }
    
    if let Some(favorite) = memory.get_preference(user_id, "favorite").await {
        println!("❤️  Favorite ingredient: {}", favorite);
    }
    
    // Message 5: Later conversation (memory persists)
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("⏰ Next day...\n");
    
    println!("👤 User: Привет!");
    if let Some(name) = memory.get_user_name(user_id).await {
        println!("🤖 Bot: Снова здравствуй, {}! 👋", name);
    }
    
    if let Some(context) = memory.get_recommendation_context(user_id).await {
        println!("     Помню, что ты любишь {}. Хочешь что-то из этого сегодня?\n", context);
    }
    
    // Emotional tracking
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("❤️  Emotional Intelligence\n");
    
    println!("👤 User: Очень голоден!");
    memory.set_emotional_state(user_id, "neutral", Some("hungry")).await;
    println!("🤖 Bot: Похоже ты проголодался! Покажу самые сытные блюда 🍽️\n");
    
    // Context summary
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎯 Context-Aware Recommendations\n");
    
    println!("Without memory:");
    println!("🤖 \"Вот наше меню: салаты, супы, горячее...\"\n");
    
    println!("With memory:");
    if let Some(name) = memory.get_user_name(user_id).await {
        print!("🤖 \"{}, ", name);
    }
    if let Some(context) = memory.get_recommendation_context(user_id).await {
        println!("специально для тебя подобрал острые блюда с креветками:");
        println!("   • 🌶️ Острая паэлья с тигровыми креветками");
        println!("   • 🌶️ Шримп том-ям (очень острый)");
        println!("   • 🌶️ Креветки в чили-соусе\"\n");
    }
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n✨ Context Memory Demo Complete!\n");
    println!("🧠 AI now remembers:");
    println!("   ✅ User name and preferences");
    println!("   ✅ Conversation history");
    println!("   ✅ Emotional state");
    println!("   ✅ Favorite ingredients");
    println!("\n💡 This enables truly personalized experiences!\n");
}

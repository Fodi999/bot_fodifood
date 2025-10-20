//! 🌐 Multilingual Intent Handler Example
//! 
//! Demonstrates how to use language detection with Groq AI
//! to provide seamless multilingual experience
//! 
//! Run: cargo run --example multilang_intent

use anyhow::Result;
use fodifood_bot::ai::intent_handler::{get_user_language, create_multilang_prompt, get_language_display};
use fodifood_bot::ai::thinker::Thinker;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌐 Multilingual Intent Handler - FodiFood AI\n");
    println!("Testing automatic language detection + Groq AI response\n");
    println!("============================================================\n");

    // Test cases with different languages
    let test_cases = vec![
        "Show me spicy seafood dishes",
        "Покажи острые блюда с морепродуктами",
        "Pokaż mi ostre dania z owocami morza",
        "Muéstrame platos picantes con mariscos",
    ];

    for (idx, user_message) in test_cases.iter().enumerate() {
        println!("📝 Test #{}: {}", idx + 1, user_message);
        
        // Step 1: Detect language
        let lang_code = get_user_language(user_message);
        let lang_display = get_language_display(user_message);
        println!("   🌐 Detected: {} (code: {})", lang_display, lang_code);
        
        // Step 2: Create multilingual prompt
        let prompt = create_multilang_prompt(user_message);
        if prompt != *user_message {
            println!("   📤 Enhanced prompt: {}", prompt);
        }
        
        // Step 3: Get AI response
        println!("   ⏳ Thinking...");
        match Thinker::think(&prompt).await {
            Ok(response) => {
                println!("   🤖 AI Response:\n");
                // Print first 200 chars to keep output readable
                let preview = if response.len() > 200 {
                    format!("{}...", &response[..200])
                } else {
                    response
                };
                println!("   {}\n", preview);
            }
            Err(e) => {
                println!("   ❌ Error: {}\n", e);
            }
        }
        
        println!("------------------------------------------------------------\n");
    }

    println!("✅ Multilingual intent handling test completed!\n");
    
    println!("💡 Key Features:");
    println!("   ✅ Automatic language detection via whatlang");
    println!("   ✅ Language-specific prompts for better accuracy");
    println!("   ✅ Groq AI responds in user's language");
    println!("   ✅ Works with English, Russian, Polish, Spanish, and more\n");
    
    println!("🚀 Integration:");
    println!("   - Use get_user_language() to detect language");
    println!("   - Use create_multilang_prompt() to enhance prompts");
    println!("   - Call Thinker::think() for AI responses");
    println!("   - Store preferred language in user profile for persistence\n");

    Ok(())
}

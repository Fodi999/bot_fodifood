//! 🌐 Language Auto-Detection Example
//! 
//! Demonstrates automatic language detection using whatlang crate
//! Run: cargo run --example language_detection

use whatlang::{detect, Lang};

fn main() {
    println!("🌐 Language Auto-Detection Example\n");
    
    let test_messages = vec![
        "What's on the menu today?",
        "Покажи меню, пожалуйста",
        "Opisz koncepcję restauracji w metawersie",
        "¿Cuál es el plato del día?",
        "Was gibt es heute zu essen?",
        "Je voudrais commander une paella",
        "Voglio ordinare i frutti di mare",
        "私は寿司が好きです",
    ];
    
    for msg in test_messages {
        println!("📝 Message: {}", msg);
        
        if let Some(info) = detect(msg) {
            let lang_name = match info.lang() {
                Lang::Eng => "🇬🇧 English",
                Lang::Rus => "🇷🇺 Russian",
                Lang::Pol => "🇵🇱 Polish",
                Lang::Spa => "🇪🇸 Spanish",
                Lang::Deu => "🇩🇪 German",
                Lang::Fra => "🇫🇷 French",
                Lang::Ita => "🇮🇹 Italian",
                Lang::Jpn => "🇯🇵 Japanese",
                _ => "🌍 Other",
            };
            println!(
                "   🌐 Detected: {} (confidence: {:.2}%)",
                lang_name,
                info.confidence() * 100.0
            );
        } else {
            println!("   ❓ Could not detect language");
        }
        
        println!();
    }
    
    println!("------------------------------------------------------------");
    println!("✅ Language detection complete!\n");
    
    println!("💡 Integration with Thinker:");
    println!("   1. ✅ whatlang dependency added to Cargo.toml");
    println!("   2. Detect language in intent_handler.rs");
    println!("   3. Route to appropriate Groq prompt based on detected language");
    println!("   4. Provide seamless multilingual experience!\n");
    
    println!("📚 Example usage in intent_handler.rs:");
    println!(r#"
    use whatlang::detect;
    
    pub fn get_user_language(text: &str) -> String {{
        if let Some(info) = detect(text) {{
            info.lang().code().to_string()
        }} else {{
            "en".to_string() // Default to English
        }}
    }}
    
    // In your handler:
    let user_lang = get_user_language(&message);
    let prompt = match user_lang.as_str() {{
        "ru" => format!("Ответь на русском: {{}}", message),
        "pl" => format!("Odpowiedz po polsku: {{}}", message),
        "es" => format!("Responde en español: {{}}", message),
        _ => message.to_string(), // English or default
    }};
    "#);
    
    println!("\n🎯 Next steps:");
    println!("   - See MULTILINGUAL.md for full integration guide");
    println!("   - Run: cargo run --example multilang_test");
    println!("   - Check: src/ai/intent_handler.rs for integration examples");
}

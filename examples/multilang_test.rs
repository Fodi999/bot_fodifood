//! 🌍 Multilingual Test - Testing Groq AI with multiple languages
//! 
//! This example demonstrates FodiFood AI's ability to understand and respond
//! in different languages using Groq Llama models.

use fodifood_bot::ai::core::query_groq;

#[tokio::main]
async fn main() {
    println!("🌍 Multilingual Test — FodiFood AI (Groq Llama 3.3 70B)\n");
    println!("Testing AI responses in English, Russian, and Polish...\n");
    println!("============================================================\n");

    // English
    println!("🇬🇧 English:");
    println!("📤 Prompt: \"Describe the concept of a metaverse restaurant.\"\n");
    match query_groq("Describe the concept of a metaverse restaurant.").await {
        Ok(resp) => println!("🧠 AI Response:\n{}\n", resp),
        Err(e) => eprintln!("❌ Error: {}\n", e),
    }
    println!("------------------------------------------------------------\n");

    // Russian
    println!("🇷🇺 Russian:");
    println!("📤 Prompt: \"Опиши концепцию ресторана в метавселенной.\"\n");
    match query_groq("Опиши концепцию ресторана в метавселенной.").await {
        Ok(resp) => println!("🧠 AI Response:\n{}\n", resp),
        Err(e) => eprintln!("❌ Ошибка: {}\n", e),
    }
    println!("------------------------------------------------------------\n");

    // Polish
    println!("🇵🇱 Polish:");
    println!("📤 Prompt: \"Opisz koncepcję restauracji w metawersie.\"\n");
    match query_groq("Opisz koncepcję restauracji w metawersie.").await {
        Ok(resp) => println!("🧠 AI Response:\n{}\n", resp),
        Err(e) => eprintln!("❌ Błąd: {}\n", e),
    }
    println!("------------------------------------------------------------\n");

    // Spanish (bonus)
    println!("🇪🇸 Spanish (bonus):");
    println!("📤 Prompt: \"Describe el concepto de un restaurante en el metaverso.\"\n");
    match query_groq("Describe el concepto de un restaurante en el metaverso.").await {
        Ok(resp) => println!("🧠 AI Response:\n{}\n", resp),
        Err(e) => eprintln!("❌ Error: {}\n", e),
    }
    println!("------------------------------------------------------------\n");

    println!("✅ Multilingual test completed!");
    println!("\n💡 Note: Groq Llama 3.3 70B understands and responds naturally in multiple languages.");
    println!("🌐 Tip: Add 'whatlang' crate to auto-detect user's language in production.");
}

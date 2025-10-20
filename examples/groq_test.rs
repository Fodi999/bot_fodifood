//! Quick test of Groq API integration

use anyhow::Result;
use fodifood_bot::ai::core::query_groq;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Testing Groq API connection...\n");
    
    let prompt = "Say 'Hello from FodiFood AI!' in exactly 5 words";
    
    println!("📤 Sending prompt: {}", prompt);
    
    match query_groq(prompt).await {
        Ok(response) => {
            println!("✅ SUCCESS!\n");
            println!("🤖 Groq response: {}\n", response);
            println!("✨ Groq API is working correctly!");
        }
        Err(e) => {
            println!("❌ ERROR: {}\n", e);
            println!("⚠️ Check your GROQ_API_KEY in Secrets.toml");
        }
    }
    
    Ok(())
}

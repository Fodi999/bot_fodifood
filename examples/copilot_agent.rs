//! 🤖 FodiFood Copilot - Interactive AI Agent
//! 
//! This is an interactive Copilot-level AI agent that can:
//! - Understand natural language queries
//! - Autonomously choose reasoning strategies
//! - Provide contextual responses
//! - Learn from conversation
//! 
//! Run: cargo run --example copilot_agent

use fodifood_bot::ai::agent::run_agent_cycle;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  🤖 FodiFood Copilot — Interactive AI Agent        ║");
    println!("║  Autonomous decision-making system                  ║");
    println!("╚══════════════════════════════════════════════════════╝\n");
    
    println!("💡 Capabilities:");
    println!("   • Natural language understanding");
    println!("   • Intent detection & entity extraction");
    println!("   • Adaptive reasoning strategies");
    println!("   • Contextual memory & personalization");
    println!("   • Controlled access to backend systems\n");
    
    println!("📝 Example queries:");
    println!("   - 'Show me the menu'");
    println!("   - 'I want something spicy with shrimp'");
    println!("   - 'What is in Paella?'");
    println!("   - 'Recommend a dish for dinner'\n");
    
    println!("Type 'exit' to quit\n");
    println!("════════════════════════════════════════════════════════\n");

    let mut conversation_count = 0;

    loop {
        print!("🧍 You: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("❌ Error reading input: {}", e);
                continue;
            }
        }
        
        let input = input.trim();

        // Exit command
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("\n👋 Thank you for using FodiFood Copilot!");
            println!("📊 Session stats: {} conversations", conversation_count);
            println!("   Have a great day! 🍽️\n");
            break;
        }

        // Empty input
        if input.is_empty() {
            continue;
        }

        // Help command
        if input.eq_ignore_ascii_case("help") {
            print_help();
            continue;
        }

        // Stats command
        if input.eq_ignore_ascii_case("stats") {
            print_stats(conversation_count);
            continue;
        }

        // Process through agent
        conversation_count += 1;
        println!();
        
        match run_agent_cycle(input).await {
            Ok(response) => {
                println!("{}", response);
            }
            Err(e) => {
                eprintln!("❌ Agent error: {}", e);
                println!("⚠️ I encountered an issue processing your request. Please try again.\n");
            }
        }
        
        println!("────────────────────────────────────────────────────────\n");
    }
}

fn print_help() {
    println!("\n📖 FodiFood Copilot Commands:");
    println!("   • help     - Show this help message");
    println!("   • stats    - Show session statistics");
    println!("   • exit     - Exit the Copilot\n");
    
    println!("💬 Natural queries:");
    println!("   • Menu browsing: 'show menu', 'what do you have'");
    println!("   • Search: 'dishes with shrimp', 'find spicy food'");
    println!("   • Info: 'what is paella', 'ingredients in X'");
    println!("   • Recommendations: 'suggest something', 'what should I order'");
    println!("   • Orders: 'track my order', 'order status'\n");
}

fn print_stats(count: u32) {
    println!("\n📊 Session Statistics:");
    println!("   • Conversations: {}", count);
    println!("   • Agent mode: Autonomous");
    println!("   • Backend: Connected");
    println!("   • Control layer: Active");
    println!("   • Logging: Enabled\n");
}

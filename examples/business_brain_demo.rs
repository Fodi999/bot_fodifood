//! 💼 Business Brain Demo - AI-powered business intelligence
//! 
//! This example demonstrates the Business Brain module capabilities:
//! - Market opportunity analysis
//! - Competitor intelligence
//! - Business metrics analysis
//! - Growth strategy recommendations
//! - Marketing strategy
//! 
//! Run: cargo run --example business_brain_demo

use anyhow::Result;
use fodifood_bot::ai::business_analyzer::BusinessBrain;

#[tokio::main]
async fn main() -> Result<()> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  💼 FodiFood Business Brain - AI Intelligence Demo         ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Demo 1: Market Opportunity Analysis
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Demo 1: Market Opportunity Analysis\n");
    
    let opportunity = BusinessBrain::analyze_opportunity("restaurant", "Moscow").await?;
    println!("{}\n", opportunity);
    
    // Demo 2: Business Metrics Analysis
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📈 Demo 2: Business Metrics Analysis\n");
    
    let metrics = BusinessBrain::analyze_metrics(
        100000.0, // revenue
        70000.0,  // costs
        500,      // customers
        200.0     // avg order
    ).await?;
    println!("{}\n", metrics);
    
    // Demo 3: Competitor Analysis
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔍 Demo 3: Competitor Analysis\n");
    
    let competitors = vec!["Delivery Club", "Yandex.Eats", "Uber Eats"];
    let competitor_analysis = BusinessBrain::analyze_competitors("food delivery", competitors).await?;
    println!("{}\n", competitor_analysis);
    
    // Demo 4: Business Model Recommendations
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("💡 Demo 4: Business Model Recommendations\n");
    
    let business_model = BusinessBrain::recommend_business_model(
        "food tech",
        "millennials 25-35",
        50000.0
    ).await?;
    println!("{}\n", business_model);
    
    // Demo 5: Growth Strategy
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📈 Demo 5: Growth Strategy\n");
    
    let growth = BusinessBrain::growth_strategy(
        "growing",
        50000.0,
        1000
    ).await?;
    println!("{}\n", growth);
    
    // Demo 6: Marketing Strategy
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎨 Demo 6: Marketing Strategy\n");
    
    let marketing = BusinessBrain::marketing_strategy(
        "seafood restaurant",
        5000.0,
        "foodies 25-40"
    ).await?;
    println!("{}\n", marketing);
    
    // Demo 7: Competitive Advantage
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🏆 Demo 7: Competitive Advantage Analysis\n");
    
    let unique_features = vec!["AI-powered recommendations", "Blockchain rewards", "NFT loyalty program"];
    let advantage = BusinessBrain::competitive_advantage("FodiFood", unique_features).await?;
    println!("{}\n", advantage);
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n✨ Business Brain Demo Complete!\n");
    println!("💼 All 7 AI-powered business intelligence modules tested successfully.");
    println!("🚀 Ready to provide strategic insights for FodiFood platform.\n");

    Ok(())
}

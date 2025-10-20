//! 💼 AI CFO Investment Analyzer Demo
//! 
//! Demonstrates comprehensive financial analysis capabilities:
//! - Restaurant expansion analysis
//! - Equipment investment evaluation
//! - Menu development ROI
//! - Delivery infrastructure assessment
//! 
//! Run: cargo run --example investment_analyzer_demo

use fodifood_bot::ai::investment_analyzer::{self, InvestmentData, analyze_investment, format_report, scenarios};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  💼 FodiFood AI CFO - Investment Analyzer Demo             ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Demo 1: Basic Investment Analysis
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Demo 1: Basic Investment Analysis\n");
    investment_analyzer::demo();
    
    println!("\n\n");
    
    // Demo 2: Restaurant Expansion
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🏢 Demo 2: New Restaurant Location Investment\n");
    
    let location_data = scenarios::new_location();
    let location_report = analyze_investment(&location_data);
    println!("{}", format_report(&location_data, &location_report));
    
    println!("\n\n");
    
    // Demo 3: Equipment Upgrade
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔧 Demo 3: Kitchen Equipment Upgrade\n");
    
    let equipment_data = scenarios::equipment_upgrade();
    let equipment_report = analyze_investment(&equipment_data);
    println!("{}", format_report(&equipment_data, &equipment_report));
    
    println!("\n\n");
    
    // Demo 4: Menu Development
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🍽️ Demo 4: Menu Development & Marketing\n");
    
    let menu_data = scenarios::menu_development();
    let menu_report = analyze_investment(&menu_data);
    println!("{}", format_report(&menu_data, &menu_report));
    
    println!("\n\n");
    
    // Demo 5: Delivery Infrastructure
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🚚 Demo 5: Delivery Infrastructure Investment\n");
    
    let delivery_data = scenarios::delivery_infrastructure();
    let delivery_report = analyze_investment(&delivery_data);
    println!("{}", format_report(&delivery_data, &delivery_report));
    
    println!("\n\n");
    
    // Demo 6: Comparison Table
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Demo 6: Investment Comparison Table\n");
    
    println!("┌─────────────────────┬──────────────┬──────────┬─────────┬──────────┬──────────┐");
    println!("│ Investment Type     │ Initial ($)  │ NPV ($)  │ IRR (%) │ ROI (%)  │ Risk     │");
    println!("├─────────────────────┼──────────────┼──────────┼─────────┼──────────┼──────────┤");
    
    let scenarios = vec![
        ("New Location", scenarios::new_location()),
        ("Equipment", scenarios::equipment_upgrade()),
        ("Menu Dev", scenarios::menu_development()),
        ("Delivery", scenarios::delivery_infrastructure()),
    ];
    
    for (name, data) in scenarios {
        let report = analyze_investment(&data);
        println!(
            "│ {:<19} │ {:>12.2} │ {:>8.2} │ {:>7.1} │ {:>8.1} │ {:>8} │",
            name,
            data.initial_investment,
            report.npv,
            report.irr * 100.0,
            report.roi * 100.0,
            report.risk_level.as_emoji()
        );
    }
    
    println!("└─────────────────────┴──────────────┴──────────┴─────────┴──────────┴──────────┘");
    
    println!("\n\n");
    
    // Demo 7: Custom Investment
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎯 Demo 7: Custom Investment Scenario\n");
    
    let custom_data = InvestmentData {
        initial_investment: 100_000.0,
        cash_flows: vec![20_000.0, 25_000.0, 30_000.0, 35_000.0, 40_000.0, 45_000.0],
        discount_rate: 0.12,
    };
    
    let custom_report = analyze_investment(&custom_data);
    println!("{}", format_report(&custom_data, &custom_report));
    
    println!("\n\n");
    
    // Summary
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n✨ AI CFO Investment Analyzer Demo Complete!\n");
    println!("💼 Analyzed 7 investment scenarios:");
    println!("   ✅ Basic investment analysis");
    println!("   🏢 Restaurant location expansion");
    println!("   🔧 Equipment upgrade");
    println!("   🍽️ Menu development");
    println!("   🚚 Delivery infrastructure");
    println!("   📋 Comparative analysis");
    println!("   🎯 Custom scenario\n");
    println!("🧠 AI CFO provides:");
    println!("   • NPV (Net Present Value) calculation");
    println!("   • IRR (Internal Rate of Return) estimation");
    println!("   • ROI (Return on Investment) analysis");
    println!("   • Payback period assessment");
    println!("   • Risk level evaluation");
    println!("   • Strategic recommendations\n");
    println!("🚀 Ready to analyze any restaurant investment!\n");
}

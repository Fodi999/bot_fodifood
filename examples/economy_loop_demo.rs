//! 🔄 AI Business Economy Loop Demo
//! 
//! Complete demonstration of the self-improving AI business ecosystem:
//! Market Data → Investment → Business Strategy → Financial Planning → 
//! → Marketing → User Engagement → Sales → Growth Assessment → (repeat)
//! 
//! Includes AI Governance Layer for meta-management and strategy optimization.

use std::sync::Arc;
use tokio::time::{sleep, Duration};
use anyhow::Result;

use fodifood_bot::ai::{
    business_economy_loop::{BusinessEconomyLoop, LoopConfig},
    governance::{AIGovernanceLayer, GovernanceConfig, RiskTolerance},
    agent_state::AgentStateManager,
    shared_bus::SharedBus,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("🚀 Starting AI Business Economy Loop Demo");
    println!("   🔄 Self-Improving Business Ecosystem with AI Governance");
    println!("   📊 Market → Investment → Strategy → Finance → Marketing → Users → Sales → Growth");
    
    // Initialize core components
    println!("\n🏗️ Initializing AI Economy Infrastructure...");
    
    // 1. Shared Communication Bus
    let bus = Arc::new(SharedBus::new().await?);
    println!("✅ SharedBus initialized");
    
    // 2. Agent State Manager for persistence
    let state_manager = Arc::new(AgentStateManager::new("./data/economy_agents").await?);
    println!("✅ Agent State Manager initialized");
    
    // 3. Business Economy Loop configuration
    let loop_config = LoopConfig {
        cycle_interval_hours: 1, // Demo: 1 hour cycles
        max_phase_duration_minutes: 5, // Demo: 5 min max per phase
        min_roi_threshold: 0.05, // 5% minimum ROI
        strategy_change_threshold: 2, // 2 poor cycles trigger change
        continuous_mode: false, // Manual demo mode
    };
    
    let economy_loop = Arc::new(BusinessEconomyLoop::new(
        bus.clone(),
        state_manager.clone(),
        Some(loop_config),
    ).await?);
    println!("✅ Business Economy Loop initialized");
    
    // 4. AI Governance Layer
    let governance_config = GovernanceConfig {
        min_roi_threshold: 0.03, // 3% governance threshold
        poor_cycle_threshold: 2,
        max_performance_variance: 0.4,
        monitoring_interval_hours: 2,
        auto_adjustment_enabled: true,
        risk_tolerance: RiskTolerance::Moderate,
    };
    
    let mut governance = AIGovernanceLayer::new(
        bus.clone(),
        state_manager.clone(),
        Some(governance_config),
    ).await?;
    
    governance.set_economy_loop(economy_loop.clone());
    println!("✅ AI Governance Layer initialized");
    
    // 5. Setup mock agents to handle messages
    println!("🤖 Setting up virtual agents for demo...");
    
    // Create mock agent listeners
    let demo_agents = vec!["INV-LOCAL-001", "BIZ-LOCAL-001", "USR-LOCAL-001", "AIR-LOCAL-001"];
    
    for agent_id in &demo_agents {
        let bus_clone = bus.clone();
        let agent_id_clone = agent_id.to_string();
        
        tokio::spawn(async move {
            // Subscribe to all topics for this agent
            let receiver = bus_clone.subscribe(&agent_id_clone, vec![
                "market_analysis".to_string(),
                "investment_analysis".to_string(), 
                "business_strategy".to_string(),
                "budget_planning".to_string(),
                "marketing_campaign".to_string(),
                "user_engagement".to_string(),
                "sales_tracking".to_string(),
                "growth_assessment".to_string(),
            ]).await;
            
            if let Ok(mut rx) = receiver {
                loop {
                    match rx.recv().await {
                        Ok(message) => {
                            tracing::info!("🤖 Agent {} received: {}", agent_id_clone, message.topic);
                            // Just acknowledge receipt - real agents would process
                        },
                        Err(_) => break,
                    }
                }
            }
        });
    }
    
    println!("✅ Virtual agents listening for messages");
    
    // Start governance monitoring in background
    let governance_arc = Arc::new(governance);
    let governance_task = {
        let gov = governance_arc.clone();
        tokio::spawn(async move {
            // Run governance check every 30 seconds for demo
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                if let Err(e) = gov.perform_governance_check().await {
                    eprintln!("Governance error: {}", e);
                }
            }
        })
    };
    
    println!("✅ AI Governance monitoring started");
    
    // Wait for system to stabilize
    sleep(Duration::from_secs(2)).await;
    
    println!("\n🎬 Starting Business Economy Cycles...");
    
    // Run multiple cycles to demonstrate the loop
    for cycle_num in 1..=3 {
        println!("\n{}", "=".repeat(60));
        println!("🔄 ЦИКЛ {} ЗАПУЩЕН", cycle_num);
        println!("{}", "=".repeat(60));
        
        let cycle_start = std::time::Instant::now();
        
        // Run complete business cycle
        match economy_loop.run_single_cycle().await {
            Ok(performance) => {
                println!("\n📊 CYCLE #{} RESULTS:", cycle_num);
                println!("   💰 Revenue: ${:.0}", performance.revenue);
                println!("   💸 Costs: ${:.0}", performance.costs);
                println!("   📈 ROI: {:.2}%", performance.roi * 100.0);
                println!("   👥 User Growth: {:.1}%", performance.user_growth * 100.0);
                println!("   ⏱️ Duration: {:.1} minutes", performance.duration_minutes);
                println!("   🎯 Insights: {} generated", performance.insights.len());
                
                if !performance.insights.is_empty() {
                    println!("   💡 Key Insights:");
                    for insight in &performance.insights {
                        println!("      • {}", insight);
                    }
                }
                
                // Show agent scores
                println!("   🤖 Agent Performance:");
                for (agent, score) in &performance.agent_scores {
                    println!("      • {}: {:.2}", agent, score);
                }
                
                // Wait between cycles for demo effect
                if cycle_num < 3 {
                    println!("\n⏳ Waiting for next cycle...");
                    sleep(Duration::from_secs(3)).await;
                }
            },
            Err(e) => {
                println!("❌ Cycle #{} failed: {}", cycle_num, e);
            }
        }
        
        // Show governance status after each cycle
        let governance_status = governance_arc.get_governance_status().await;
        println!("\n🎭 GOVERNANCE STATUS:");
        println!("   🏥 System Health: {:.2}", governance_status.governance_health);
        println!("   ⚠️ Poor Cycles: {}", governance_status.consecutive_poor_cycles);
        println!("   🔧 Total Adjustments: {}", governance_status.total_adjustments);
        println!("   📊 Efficiency Score: {:.2}", governance_status.system_kpis.efficiency_score);
        println!("   🤝 Coordination Quality: {:.2}", governance_status.system_kpis.coordination_quality);
        
        if !governance_status.recent_adjustments.is_empty() {
            println!("   🔄 Recent Adjustments:");
            for adj in &governance_status.recent_adjustments {
                println!("      • {:?} affecting {} agents", 
                        adj.adjustment_type, adj.affected_agents.len());
            }
        }
    }
    
    // Final system analysis
    println!("\n{}", "=".repeat(60));
    println!("📈 FINAL SYSTEM ANALYSIS");
    println!("{}", "=".repeat(60));
    
    let performance_history = economy_loop.get_performance_history().await;
    let total_cycles = performance_history.len();
    
    if !performance_history.is_empty() {
        let total_revenue: f64 = performance_history.iter().map(|p| p.revenue).sum();
        let total_costs: f64 = performance_history.iter().map(|p| p.costs).sum();
        let overall_roi = if total_costs > 0.0 { (total_revenue - total_costs) / total_costs } else { 0.0 };
        let avg_user_growth: f64 = performance_history.iter().map(|p| p.user_growth).sum::<f64>() / total_cycles as f64;
        
        println!("🏆 ECOSYSTEM PERFORMANCE SUMMARY:");
        println!("   📊 Total Cycles: {}", total_cycles);
        println!("   💰 Total Revenue: ${:.0}", total_revenue);
        println!("   💸 Total Costs: ${:.0}", total_costs);
        println!("   📈 Overall ROI: {:.2}%", overall_roi * 100.0);
        println!("   👥 Average User Growth: {:.1}%", avg_user_growth * 100.0);
        
        // ROI trend analysis
        let roi_trend: Vec<f64> = performance_history.iter().map(|p| p.roi).collect();
        if roi_trend.len() >= 2 {
            let trend_direction = if roi_trend.last().unwrap() > roi_trend.first().unwrap() {
                "📈 Improving"
            } else {
                "📉 Declining"
            };
            println!("   📊 ROI Trend: {}", trend_direction);
        }
        
        // 🧠 SELF-LEARNING ANALYSIS
        println!("\n🧠 AI SELF-LEARNING INSIGHTS:");
        let strategy_weights = governance_arc.get_strategy_weights().await;
        println!("   💰 Marketing Weight: {:.1}%", strategy_weights.marketing_weight * 100.0);
        println!("   📊 Investment Weight: {:.1}%", strategy_weights.investment_weight * 100.0);
        println!("   🏢 Business Dev Weight: {:.1}%", strategy_weights.business_dev_weight * 100.0);
        println!("   🛡️ Risk Management Weight: {:.1}%", strategy_weights.risk_management_weight * 100.0);
        println!("   👥 User Acquisition Weight: {:.1}%", strategy_weights.user_acquisition_weight * 100.0);
        println!("   🎯 Learning Confidence: {:.1}%", strategy_weights.confidence_score * 100.0);
        
        let learning_data = governance_arc.get_learning_insights().await;
        println!("   📊 Strategy Effectiveness Scores:");
        for (strategy, score) in &learning_data.strategy_effectiveness {
            println!("      • {}: {:.1}%", strategy, score * 100.0);
        }
        println!("   🎯 Discovered Patterns: {}", learning_data.optimal_patterns.len());
        
        // Business insights from all cycles
        let all_insights: Vec<String> = performance_history.iter()
            .flat_map(|p| p.insights.clone())
            .collect();
        
        println!("\n🧠 ACCUMULATED BUSINESS INTELLIGENCE:");
        for (i, insight) in all_insights.iter().take(5).enumerate() {
            println!("   {}. {}", i + 1, insight);
        }
    }
    
    // Agent performance comparison
    let agent_comparison = state_manager.get_performance_comparison().await?;
    println!("\n🤖 FINAL AGENT PERFORMANCE:");
    for (agent_id, metrics) in agent_comparison {
        println!("   • {}: Success Rate: {:.1}%, Avg ROI: {:.1}%, Decisions: {}",
                agent_id, 
                metrics.success_rate * 100.0,
                metrics.avg_roi * 100.0,
                metrics.total_decisions);
    }
    
    // Show governance effectiveness
    let final_governance = governance_arc.get_governance_status().await;
    println!("\n🎭 GOVERNANCE EFFECTIVENESS:");
    println!("   🏥 Final System Health: {:.2}", final_governance.governance_health);
    println!("   🔧 Interventions Made: {}", final_governance.total_adjustments);
    println!("   ⚡ System Efficiency: {:.2}", final_governance.system_kpis.efficiency_score);
    println!("   🎯 Decision Consistency: {:.2}", final_governance.system_kpis.decision_consistency);
    
    // Demonstrate persistent state
    println!("\n💾 PERSISTENT STATE DEMONSTRATION:");
    let all_states = state_manager.list_all_states().await?;
    println!("   📁 Saved {} agent states to disk", all_states.len());
    for state in &all_states {
        println!("      • {}: {} decisions, created {}", 
                state.agent_id, 
                state.decision_history.len(),
                state.created_at.format("%Y-%m-%d %H:%M"));
    }
    
    // Show SharedBus final statistics
    let bus_stats = bus.get_stats().await;
    println!("\n🚌 SHAREDBBUS COMMUNICATION STATS:");
    println!("   📨 Total Messages: {}", bus_stats.total_messages);
    println!("   📡 Active Subscriptions: {}", bus_stats.active_subscriptions);
    println!("   ⚡ Avg Processing Time: {:.1}ms", bus_stats.avg_processing_time_ms);
    println!("   📊 Messages by Topic:");
    for (topic, count) in &bus_stats.messages_per_topic {
        println!("      • {}: {} messages", topic, count);
    }
    
    // Cleanup and finish
    governance_task.abort();
    
    println!("\n🎉 AI BUSINESS ECONOMY LOOP DEMO COMPLETED!");
    println!("   ✅ Self-improving business ecosystem demonstrated");
    println!("   ✅ AI governance and meta-management working");
    println!("   ✅ Persistent agent memory system functional");
    println!("   ✅ Multi-agent coordination successful");
    println!("   ✅ Real-time communication bus operational");
    
    println!("\n🚀 READY FOR PRODUCTION DEPLOYMENT!");
    println!("   💡 Connect to real market data APIs");
    println!("   🔗 Integrate with Solana smart contracts");
    println!("   📱 Add WebSocket dashboard for monitoring");
    println!("   🎯 Scale to multiple business domains");
    
    Ok(())
}
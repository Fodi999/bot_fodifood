//! 🔄 AI Business Economy Loop
//! 
//! Self-improving business cycle that connects all AI agents in a continuous loop:
//! Market Data → Investor Analysis → Business Strategy → CFO Budget → 
//! → Airdrop Marketing → User Engagement → Sales Feedback → Business Growth

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{Duration, Instant, interval};
use chrono::{DateTime, Utc};
use serde_json::json;

use crate::ai::SharedBus;
use crate::ai::agent_state::{AgentStateManager, AgentDecision, DecisionOutcome, PerformanceMetrics};
use crate::ai::shared_bus::{MessageType, CoordinationStatus};

/// Main business economy loop orchestrator
pub struct BusinessEconomyLoop {
    /// Shared communication bus
    bus: Arc<SharedBus>,
    /// Agent state manager for persistence
    state_manager: Arc<AgentStateManager>,
    /// Current cycle state
    cycle_state: Arc<tokio::sync::RwLock<CycleState>>,
    /// Loop configuration
    config: LoopConfig,
    /// Performance history
    performance_history: Arc<tokio::sync::RwLock<Vec<CyclePerformance>>>,
}

/// Current state of the business cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleState {
    /// Current cycle number
    pub cycle_number: u64,
    /// Current phase in the cycle
    pub current_phase: BusinessPhase,
    /// Phase start time
    pub phase_started_at: DateTime<Utc>,
    /// Cycle start time
    pub cycle_started_at: DateTime<Utc>,
    /// Accumulated data from all phases
    pub cycle_data: CycleData,
    /// Overall cycle health
    pub cycle_health: f64,
}

/// Phases of the business economy loop
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BusinessPhase {
    /// Market data collection and analysis
    MarketAnalysis,
    /// Investment opportunity evaluation
    InvestmentAnalysis, 
    /// Business strategy development
    BusinessStrategy,
    /// CFO budget planning and allocation
    FinancialPlanning,
    /// Marketing campaign execution
    AirdropMarketing,
    /// User engagement and feedback collection
    UserEngagement,
    /// Sales performance evaluation
    SalesAnalysis,
    /// Business growth assessment and cycle completion
    GrowthAssessment,
}

/// Data accumulated throughout a business cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleData {
    /// Market analysis results
    pub market_data: Option<serde_json::Value>,
    /// Investment recommendations
    pub investment_recommendations: Option<serde_json::Value>,
    /// Business strategy plan
    pub business_strategy: Option<serde_json::Value>,
    /// Financial budget allocation
    pub financial_plan: Option<serde_json::Value>,
    /// Marketing campaign results
    pub marketing_results: Option<serde_json::Value>,
    /// User engagement metrics
    pub user_metrics: Option<serde_json::Value>,
    /// Sales performance data
    pub sales_data: Option<serde_json::Value>,
    /// Growth assessment outcome
    pub growth_assessment: Option<serde_json::Value>,
}

/// Performance metrics for a complete cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyclePerformance {
    /// Cycle number
    pub cycle_number: u64,
    /// Total cycle duration
    pub duration_minutes: f64,
    /// Overall ROI for this cycle
    pub roi: f64,
    /// Revenue generated
    pub revenue: f64,
    /// Costs incurred
    pub costs: f64,
    /// User growth percentage
    pub user_growth: f64,
    /// Agent performance scores
    pub agent_scores: HashMap<String, f64>,
    /// Key insights from the cycle
    pub insights: Vec<String>,
    /// Cycle completion timestamp
    pub completed_at: DateTime<Utc>,
}

/// Configuration for the business economy loop
#[derive(Debug, Clone)]
pub struct LoopConfig {
    /// How often to run complete cycles (in hours)
    pub cycle_interval_hours: u64,
    /// Maximum time per phase (in minutes)
    pub max_phase_duration_minutes: u64,
    /// Minimum ROI to continue current strategy
    pub min_roi_threshold: f64,
    /// Number of poor cycles before strategy change
    pub strategy_change_threshold: u32,
    /// Whether to run continuously
    pub continuous_mode: bool,
}

impl Default for LoopConfig {
    fn default() -> Self {
        Self {
            cycle_interval_hours: 24, // Daily cycles
            max_phase_duration_minutes: 60, // 1 hour max per phase
            min_roi_threshold: 0.05, // 5% minimum ROI
            strategy_change_threshold: 3, // 3 poor cycles trigger change
            continuous_mode: true,
        }
    }
}

impl Default for CycleData {
    fn default() -> Self {
        Self {
            market_data: None,
            investment_recommendations: None,
            business_strategy: None,
            financial_plan: None,
            marketing_results: None,
            user_metrics: None,
            sales_data: None,
            growth_assessment: None,
        }
    }
}

impl BusinessEconomyLoop {
    /// Create new business economy loop
    pub async fn new(
        bus: Arc<SharedBus>,
        state_manager: Arc<AgentStateManager>,
        config: Option<LoopConfig>,
    ) -> Result<Self> {
        let cycle_state = Arc::new(tokio::sync::RwLock::new(CycleState {
            cycle_number: 1,
            current_phase: BusinessPhase::MarketAnalysis,
            phase_started_at: Utc::now(),
            cycle_started_at: Utc::now(),
            cycle_data: CycleData::default(),
            cycle_health: 1.0,
        }));

        Ok(Self {
            bus,
            state_manager,
            cycle_state,
            config: config.unwrap_or_default(),
            performance_history: Arc::new(tokio::sync::RwLock::new(Vec::new())),
        })
    }

    /// Start the continuous business economy loop
    pub async fn start_continuous_loop(&self) -> Result<()> {
        if !self.config.continuous_mode {
            return Err(anyhow::anyhow!("Continuous mode is disabled"));
        }

        tracing::info!("🔄 Starting AI Business Economy Loop in continuous mode");
        
        let mut cycle_interval = interval(Duration::from_secs(self.config.cycle_interval_hours * 3600));
        
        loop {
            cycle_interval.tick().await;
            
            match self.run_single_cycle().await {
                Ok(_) => {
                    tracing::info!("✅ Business cycle completed successfully");
                },
                Err(e) => {
                    tracing::error!("❌ Business cycle failed: {}", e);
                    // Continue to next cycle even if one fails
                }
            }
        }
    }

    /// Run a single complete business cycle
    pub async fn run_single_cycle(&self) -> Result<CyclePerformance> {
        let cycle_start = Instant::now();
        let mut state = self.cycle_state.write().await;
        
        state.cycle_started_at = Utc::now();
        state.current_phase = BusinessPhase::MarketAnalysis;
        state.cycle_data = CycleData::default();
        
        let cycle_number = state.cycle_number;
        drop(state);

        tracing::info!("🚀 Starting Business Economy Cycle #{}", cycle_number);

        // Phase 1: Market Analysis
        self.execute_market_analysis().await?;
        tokio::time::sleep(Duration::from_secs(5)).await;

        // Phase 2: Investment Analysis
        self.execute_investment_analysis().await?;
        tokio::time::sleep(Duration::from_secs(5)).await;

        // Phase 3: Business Strategy
        self.execute_business_strategy().await?;
        tokio::time::sleep(Duration::from_secs(5)).await;

        // Phase 4: Financial Planning
        self.execute_financial_planning().await?;
        tokio::time::sleep(Duration::from_secs(5)).await;

        // Phase 5: Airdrop Marketing
        self.execute_airdrop_marketing().await?;
        tokio::time::sleep(Duration::from_secs(5)).await;

        // Phase 6: User Engagement
        self.execute_user_engagement().await?;
        tokio::time::sleep(Duration::from_secs(5)).await;

        // Phase 7: Sales Analysis
        self.execute_sales_analysis().await?;
        tokio::time::sleep(Duration::from_secs(5)).await;

        // Phase 8: Growth Assessment
        let performance = self.execute_growth_assessment(cycle_start).await?;

        // Update cycle number for next cycle
        let mut state = self.cycle_state.write().await;
        state.cycle_number += 1;
        drop(state);

        // Store performance history
        let mut history = self.performance_history.write().await;
        history.push(performance.clone());
        
        // Keep only last 100 cycles
        if history.len() > 100 {
            history.remove(0);
        }
        drop(history);

        tracing::info!("🎉 Completed Business Economy Cycle #{} with ROI: {:.2}%", 
                      cycle_number, performance.roi * 100.0);

        Ok(performance)
    }

    /// Phase 1: Market Analysis
    async fn execute_market_analysis(&self) -> Result<()> {
        self.update_phase(BusinessPhase::MarketAnalysis).await;
        
        tracing::info!("📊 Phase 1: Market Analysis");
        
        // Request market analysis from investor agent
        self.bus.send_to_agent(
            "ECONOMY_LOOP",
            "INV-LOCAL-001",
            "market_analysis",
            json!({
                "cycle_phase": "market_analysis",
                "analysis_type": "comprehensive_market_scan",
                "sectors": ["fintech", "foodtech", "proptech", "defi"],
                "timeframe": "current_cycle",
                "priority": "high"
            })
        ).await?;

        // Simulate market data collection (in real system, this would be actual market APIs)
        tokio::time::sleep(Duration::from_secs(3)).await;
        
        let market_data = json!({
            "market_sentiment": "bullish",
            "trending_sectors": ["foodtech", "defi"],
            "volatility_index": 0.24,
            "growth_opportunities": {
                "foodtech": 0.89,
                "fintech": 0.76,
                "proptech": 0.63,
                "defi": 0.82
            },
            "market_risks": ["inflation", "regulation"],
            "timestamp": Utc::now()
        });

        // Store market data in cycle state
        let mut state = self.cycle_state.write().await;
        state.cycle_data.market_data = Some(market_data.clone());
        drop(state);

        // Record decision for investor agent
        let decision = AgentDecision {
            decision_id: format!("market_analysis_{}", Utc::now().timestamp()),
            decision_type: "market_analysis".to_string(),
            input_data: json!({"sectors": ["fintech", "foodtech", "proptech", "defi"]}),
            output: market_data,
            confidence: 0.87,
            outcome: None,
            decided_at: Utc::now(),
            outcome_measured_at: None,
        };

        self.state_manager.record_decision("INV-LOCAL-001", decision).await?;
        
        tracing::info!("✅ Market Analysis completed");
        Ok(())
    }

    /// Phase 2: Investment Analysis
    async fn execute_investment_analysis(&self) -> Result<()> {
        self.update_phase(BusinessPhase::InvestmentAnalysis).await;
        
        tracing::info!("💰 Phase 2: Investment Analysis");
        
        let state = self.cycle_state.read().await;
        let market_data = state.cycle_data.market_data.clone();
        drop(state);

        // Send investment analysis request with market data
        self.bus.send_to_agent(
            "ECONOMY_LOOP",
            "INV-LOCAL-001",
            "investment_analysis",
            json!({
                "cycle_phase": "investment_analysis",
                "market_data": market_data,
                "investment_budget": 500000,
                "risk_tolerance": "moderate",
                "target_roi": 0.25
            })
        ).await?;

        tokio::time::sleep(Duration::from_secs(3)).await;

        let investment_recommendations = json!({
            "recommended_allocations": {
                "foodtech_startup": {
                    "amount": 200000,
                    "expected_roi": 0.35,
                    "risk_score": 0.4
                },
                "defi_protocol": {
                    "amount": 150000,
                    "expected_roi": 0.28,
                    "risk_score": 0.6
                },
                "fintech_expansion": {
                    "amount": 100000,
                    "expected_roi": 0.22,
                    "risk_score": 0.3
                },
                "cash_reserve": {
                    "amount": 50000,
                    "expected_roi": 0.05,
                    "risk_score": 0.1
                }
            },
            "overall_expected_roi": 0.27,
            "confidence_level": 0.83,
            "timestamp": Utc::now()
        });

        // Store investment recommendations
        let mut state = self.cycle_state.write().await;
        state.cycle_data.investment_recommendations = Some(investment_recommendations.clone());
        drop(state);

        // Record investment decision
        let decision = AgentDecision {
            decision_id: format!("investment_analysis_{}", Utc::now().timestamp()),
            decision_type: "investment_allocation".to_string(),
            input_data: market_data.unwrap_or(json!({})),
            output: investment_recommendations,
            confidence: 0.83,
            outcome: None,
            decided_at: Utc::now(),
            outcome_measured_at: None,
        };

        self.state_manager.record_decision("INV-LOCAL-001", decision).await?;
        
        tracing::info!("✅ Investment Analysis completed");
        Ok(())
    }

    /// Phase 3: Business Strategy
    async fn execute_business_strategy(&self) -> Result<()> {
        self.update_phase(BusinessPhase::BusinessStrategy).await;
        
        tracing::info!("🏢 Phase 3: Business Strategy Development");
        
        let state = self.cycle_state.read().await;
        let investment_data = state.cycle_data.investment_recommendations.clone();
        drop(state);

        // Send business strategy request
        self.bus.send_to_agent(
            "ECONOMY_LOOP",
            "BIZ-LOCAL-001",
            "strategy_development",
            json!({
                "cycle_phase": "business_strategy",
                "investment_plan": investment_data,
                "target_metrics": {
                    "user_growth": 0.30,
                    "revenue_growth": 0.40,
                    "market_share": 0.15
                }
            })
        ).await?;

        tokio::time::sleep(Duration::from_secs(3)).await;

        let business_strategy = json!({
            "strategy_focus": "aggressive_growth",
            "target_markets": ["urban_millennials", "health_conscious_families", "remote_workers"],
            "growth_initiatives": {
                "product_expansion": {
                    "new_categories": ["healthy_snacks", "meal_kits", "supplements"],
                    "investment": 120000,
                    "timeline": "Q1-Q2"
                },
                "market_expansion": {
                    "new_cities": ["Austin", "Denver", "Portland"],
                    "investment": 80000,
                    "timeline": "Q2-Q3"
                },
                "technology_upgrade": {
                    "ai_personalization": true,
                    "mobile_app_v2": true,
                    "investment": 100000,
                    "timeline": "Q1-Q4"
                }
            },
            "projected_outcomes": {
                "user_base_growth": 0.35,
                "revenue_increase": 0.42,
                "market_penetration": 0.18
            },
            "risk_mitigation": ["diversified_suppliers", "insurance_coverage", "cash_reserves"],
            "timestamp": Utc::now()
        });

        // Store business strategy
        let mut state = self.cycle_state.write().await;
        state.cycle_data.business_strategy = Some(business_strategy.clone());
        drop(state);

        // Record business decision
        let decision = AgentDecision {
            decision_id: format!("business_strategy_{}", Utc::now().timestamp()),
            decision_type: "strategic_planning".to_string(),
            input_data: investment_data.unwrap_or(json!({})),
            output: business_strategy,
            confidence: 0.79,
            outcome: None,
            decided_at: Utc::now(),
            outcome_measured_at: None,
        };

        self.state_manager.record_decision("BIZ-LOCAL-001", decision).await?;
        
        tracing::info!("✅ Business Strategy completed");
        Ok(())
    }

    /// Phase 4: Financial Planning
    async fn execute_financial_planning(&self) -> Result<()> {
        self.update_phase(BusinessPhase::FinancialPlanning).await;
        
        tracing::info!("💰 Phase 4: Financial Planning & CFO Approval");
        
        let state = self.cycle_state.read().await;
        let strategy_data = state.cycle_data.business_strategy.clone();
        drop(state);

        // Send CFO budget planning request
        self.bus.send_to_agent(
            "ECONOMY_LOOP",
            "CFO-LOCAL-001",
            "budget_planning",
            json!({
                "cycle_phase": "financial_planning",
                "business_strategy": strategy_data,
                "available_budget": 500000,
                "financial_constraints": {
                    "max_risk_exposure": 0.6,
                    "min_cash_reserve": 50000,
                    "debt_to_equity_ratio": 0.4
                }
            })
        ).await?;

        tokio::time::sleep(Duration::from_secs(3)).await;

        let financial_plan = json!({
            "budget_allocation": {
                "product_development": 120000,
                "marketing_campaigns": 150000,
                "market_expansion": 80000,
                "technology_infrastructure": 100000,
                "emergency_reserves": 50000
            },
            "revenue_projections": {
                "q1": 180000,
                "q2": 220000,
                "q3": 280000,
                "q4": 350000,
                "annual_total": 1030000
            },
            "cost_projections": {
                "operational_costs": 400000,
                "marketing_spend": 150000,
                "development_costs": 200000,
                "total_costs": 750000
            },
            "profitability_forecast": {
                "gross_profit": 280000,
                "net_profit": 180000,
                "roi": 0.36,
                "break_even_month": 8
            },
            "financial_health_score": 0.82,
            "approval_status": "approved",
            "timestamp": Utc::now()
        });

        // Store financial plan
        let mut state = self.cycle_state.write().await;
        state.cycle_data.financial_plan = Some(financial_plan.clone());
        drop(state);

        // Record CFO decision
        let decision = AgentDecision {
            decision_id: format!("financial_planning_{}", Utc::now().timestamp()),
            decision_type: "budget_allocation".to_string(),
            input_data: strategy_data.unwrap_or(json!({})),
            output: financial_plan,
            confidence: 0.82,
            outcome: None,
            decided_at: Utc::now(),
            outcome_measured_at: None,
        };

        self.state_manager.record_decision("CFO-LOCAL-001", decision).await?;
        
        tracing::info!("✅ Financial Planning completed");
        Ok(())
    }

    /// Phase 5: Airdrop Marketing
    async fn execute_airdrop_marketing(&self) -> Result<()> {
        self.update_phase(BusinessPhase::AirdropMarketing).await;
        
        tracing::info!("🎁 Phase 5: Airdrop Marketing Campaign");
        
        let state = self.cycle_state.read().await;
        let financial_plan = state.cycle_data.financial_plan.clone();
        drop(state);

        // Broadcast marketing campaign initiation
        self.bus.broadcast(
            "ECONOMY_LOOP",
            "marketing_campaigns",
            MessageType::Command,
            json!({
                "cycle_phase": "airdrop_marketing",
                "campaign_type": "user_acquisition_airdrop",
                "budget": 150000,
                "financial_plan": financial_plan,
                "target_metrics": {
                    "new_users": 5000,
                    "engagement_rate": 0.65,
                    "conversion_rate": 0.12
                }
            })
        ).await?;

        tokio::time::sleep(Duration::from_secs(3)).await;

        let marketing_results = json!({
            "campaign_performance": {
                "users_reached": 45000,
                "new_signups": 5200,
                "airdrop_participants": 3800,
                "engagement_rate": 0.68,
                "cost_per_acquisition": 28.85
            },
            "token_distribution": {
                "total_tokens_distributed": 380000,
                "unique_recipients": 3800,
                "average_tokens_per_user": 100,
                "distribution_cost": 8500
            },
            "social_metrics": {
                "social_shares": 12000,
                "viral_coefficient": 1.4,
                "brand_mentions": 8500,
                "sentiment_score": 0.74
            },
            "conversion_metrics": {
                "trial_to_paid": 0.14,
                "immediate_purchases": 520,
                "revenue_generated": 23400
            },
            "campaign_roi": 0.156,
            "timestamp": Utc::now()
        });

        // Store marketing results
        let mut state = self.cycle_state.write().await;
        state.cycle_data.marketing_results = Some(marketing_results);
        drop(state);
        
        tracing::info!("✅ Airdrop Marketing completed");
        Ok(())
    }

    /// Phase 6: User Engagement
    async fn execute_user_engagement(&self) -> Result<()> {
        self.update_phase(BusinessPhase::UserEngagement).await;
        
        tracing::info!("👥 Phase 6: User Engagement Analysis");
        
        // Send user engagement analysis request
        self.bus.send_to_agent(
            "ECONOMY_LOOP",
            "USER-LOCAL-001",
            "engagement_analysis",
            json!({
                "cycle_phase": "user_engagement",
                "analysis_period": "current_cycle",
                "focus_areas": ["retention", "satisfaction", "lifetime_value"]
            })
        ).await?;

        tokio::time::sleep(Duration::from_secs(3)).await;

        let user_metrics = json!({
            "user_base": {
                "total_active_users": 28500,
                "new_users_this_cycle": 5200,
                "returning_users": 23300,
                "user_growth_rate": 0.22
            },
            "engagement_metrics": {
                "daily_active_users": 15600,
                "session_duration_minutes": 12.3,
                "pages_per_session": 4.7,
                "bounce_rate": 0.23
            },
            "satisfaction_scores": {
                "nps_score": 72,
                "customer_satisfaction": 4.6,
                "app_store_rating": 4.4,
                "support_ticket_resolution": 0.94
            },
            "lifetime_value": {
                "average_ltv": 245.60,
                "ltv_to_cac_ratio": 8.5,
                "churn_rate": 0.08,
                "retention_rate_30_days": 0.85
            },
            "behavioral_insights": [
                "Users prefer mobile ordering",
                "Healthy options drive engagement",
                "Social features increase retention",
                "Rewards program boosts repeat orders"
            ],
            "timestamp": Utc::now()
        });

        // Store user metrics
        let mut state = self.cycle_state.write().await;
        state.cycle_data.user_metrics = Some(user_metrics.clone());
        drop(state);

        // Record user engagement decision
        let decision = AgentDecision {
            decision_id: format!("user_engagement_{}", Utc::now().timestamp()),
            decision_type: "engagement_analysis".to_string(),
            input_data: json!({"analysis_period": "current_cycle"}),
            output: user_metrics,
            confidence: 0.88,
            outcome: None,
            decided_at: Utc::now(),
            outcome_measured_at: None,
        };

        self.state_manager.record_decision("USER-LOCAL-001", decision).await?;
        
        tracing::info!("✅ User Engagement Analysis completed");
        Ok(())
    }

    /// Phase 7: Sales Analysis
    async fn execute_sales_analysis(&self) -> Result<()> {
        self.update_phase(BusinessPhase::SalesAnalysis).await;
        
        tracing::info!("📈 Phase 7: Sales Performance Analysis");
        
        let state = self.cycle_state.read().await;
        let user_data = state.cycle_data.user_metrics.clone();
        let marketing_data = state.cycle_data.marketing_results.clone();
        drop(state);

        // Broadcast sales analysis request
        self.bus.broadcast(
            "ECONOMY_LOOP",
            "sales_analysis",
            MessageType::Request,
            json!({
                "cycle_phase": "sales_analysis",
                "user_metrics": user_data,
                "marketing_results": marketing_data,
                "analysis_scope": "full_cycle"
            })
        ).await?;

        tokio::time::sleep(Duration::from_secs(3)).await;

        let sales_data = json!({
            "revenue_performance": {
                "total_revenue": 312000,
                "revenue_growth": 0.38,
                "average_order_value": 67.50,
                "orders_count": 4622,
                "repeat_customer_revenue": 0.72
            },
            "product_performance": {
                "top_categories": ["healthy_meals", "beverages", "snacks"],
                "bestselling_items": [
                    {"name": "Buddha Bowl", "revenue": 28000, "units": 1200},
                    {"name": "Green Smoothie", "revenue": 15600, "units": 2400},
                    {"name": "Quinoa Salad", "revenue": 12800, "units": 800}
                ],
                "profit_margins": {
                    "healthy_meals": 0.42,
                    "beverages": 0.65,
                    "snacks": 0.38
                }
            },
            "customer_segments": {
                "premium_customers": {
                    "count": 1200,
                    "avg_spend": 180.00,
                    "revenue_contribution": 0.35
                },
                "regular_customers": {
                    "count": 8500,
                    "avg_spend": 85.20,
                    "revenue_contribution": 0.55
                },
                "occasional_customers": {
                    "count": 18800,
                    "avg_spend": 23.40,
                    "revenue_contribution": 0.10
                }
            },
            "conversion_funnel": {
                "visitors": 125000,
                "leads": 28000,
                "trials": 12000,
                "paying_customers": 5200,
                "conversion_rate": 0.042
            },
            "profitability": {
                "gross_profit": 187200,
                "gross_margin": 0.60,
                "operating_profit": 78000,
                "net_margin": 0.25
            },
            "timestamp": Utc::now()
        });

        // Store sales data
        let mut state = self.cycle_state.write().await;
        state.cycle_data.sales_data = Some(sales_data);
        drop(state);
        
        tracing::info!("✅ Sales Analysis completed");
        Ok(())
    }

    /// Phase 8: Growth Assessment & Cycle Completion
    async fn execute_growth_assessment(&self, cycle_start: Instant) -> Result<CyclePerformance> {
        self.update_phase(BusinessPhase::GrowthAssessment).await;
        
        tracing::info!("📊 Phase 8: Growth Assessment & Cycle Completion");
        
        let state = self.cycle_state.read().await;
        let cycle_data = state.cycle_data.clone();
        let cycle_number = state.cycle_number;
        drop(state);

        // Calculate cycle performance metrics
        let revenue = cycle_data.sales_data
            .as_ref()
            .and_then(|d| d.get("revenue_performance"))
            .and_then(|r| r.get("total_revenue"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let costs = cycle_data.financial_plan
            .as_ref()
            .and_then(|p| p.get("cost_projections"))
            .and_then(|c| c.get("total_costs"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let roi = if costs > 0.0 { (revenue - costs) / costs } else { 0.0 };

        let user_growth = cycle_data.user_metrics
            .as_ref()
            .and_then(|u| u.get("user_base"))
            .and_then(|b| b.get("user_growth_rate"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        // Generate insights based on cycle performance
        let mut insights = Vec::new();
        
        if roi > 0.3 {
            insights.push("Strong ROI indicates successful investment strategy".to_string());
        } else if roi < 0.05 {
            insights.push("Low ROI suggests need for strategy adjustment".to_string());
        }

        if user_growth > 0.2 {
            insights.push("High user growth shows effective marketing".to_string());
        }

        if revenue > 300000.0 {
            insights.push("Revenue target exceeded, scale operations".to_string());
        }

        let growth_assessment = json!({
            "cycle_summary": {
                "cycle_number": cycle_number,
                "duration_minutes": cycle_start.elapsed().as_secs_f64() / 60.0,
                "phases_completed": 8,
                "overall_success": roi > self.config.min_roi_threshold
            },
            "key_achievements": [
                format!("Generated ${:.0} in revenue", revenue),
                format!("{:.1}% user growth achieved", user_growth * 100.0),
                format!("{:.1}% ROI delivered", roi * 100.0)
            ],
            "improvement_areas": [
                "Optimize marketing spend efficiency",
                "Enhance customer retention programs", 
                "Expand high-margin product categories"
            ],
            "next_cycle_recommendations": [
                "Increase investment in top-performing sectors",
                "Launch retention-focused campaigns",
                "Explore new market segments"
            ],
            "cycle_health_score": (roi * 0.4 + user_growth * 0.3 + 0.3).min(1.0),
            "timestamp": Utc::now()
        });

        // Store growth assessment
        let mut state = self.cycle_state.write().await;
        state.cycle_data.growth_assessment = Some(growth_assessment);
        state.cycle_health = (roi * 0.4 + user_growth * 0.3 + 0.3).min(1.0);
        drop(state);

        // Create cycle performance record
        let performance = CyclePerformance {
            cycle_number,
            duration_minutes: cycle_start.elapsed().as_secs_f64() / 60.0,
            roi,
            revenue,
            costs,
            user_growth,
            agent_scores: self.calculate_agent_scores().await,
            insights,
            completed_at: Utc::now(),
        };

        // Update agent performance metrics based on cycle results
        self.update_agent_performance_metrics(&performance).await?;

        // Broadcast cycle completion
        self.bus.broadcast(
            "ECONOMY_LOOP",
            "cycle_completed",
            MessageType::Info,
            json!({
                "cycle_number": cycle_number,
                "performance": performance,
                "next_cycle_starts": Utc::now() + chrono::Duration::hours(self.config.cycle_interval_hours as i64)
            })
        ).await?;

        tracing::info!("✅ Growth Assessment completed - Cycle #{} finished", cycle_number);
        Ok(performance)
    }

    /// Update current phase
    async fn update_phase(&self, phase: BusinessPhase) {
        let mut state = self.cycle_state.write().await;
        state.current_phase = phase;
        state.phase_started_at = Utc::now();
    }

    /// Calculate performance scores for each agent
    async fn calculate_agent_scores(&self) -> HashMap<String, f64> {
        let mut scores = HashMap::new();
        
        // In a real implementation, these would be calculated based on 
        // actual agent performance metrics and decision outcomes
        scores.insert("INV-LOCAL-001".to_string(), 0.85);
        scores.insert("BIZ-LOCAL-001".to_string(), 0.78);
        scores.insert("CFO-LOCAL-001".to_string(), 0.82);
        scores.insert("USER-LOCAL-001".to_string(), 0.88);
        
        scores
    }

    /// Update agent performance metrics based on cycle results
    async fn update_agent_performance_metrics(&self, performance: &CyclePerformance) -> Result<()> {
        for (agent_id, score) in &performance.agent_scores {
            let metrics = PerformanceMetrics {
                success_rate: score * 0.9, // Adjust based on actual success
                avg_roi: performance.roi,
                total_decisions: 1, // This cycle's decision
                avg_response_time_ms: 3000, // Simulated response time
                accuracy_score: score * 0.95,
                confidence_level: *score,
            };

            self.state_manager.update_performance(agent_id, metrics).await?;
        }
        
        Ok(())
    }

    /// Get current cycle state
    pub async fn get_current_state(&self) -> CycleState {
        self.cycle_state.read().await.clone()
    }

    /// Get performance history
    pub async fn get_performance_history(&self) -> Vec<CyclePerformance> {
        self.performance_history.read().await.clone()
    }

    /// Get cycle health trend
    pub async fn get_health_trend(&self) -> Vec<f64> {
        let history = self.performance_history.read().await;
        history.iter().map(|p| p.roi).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_business_economy_loop_creation() {
        let bus = Arc::new(SharedBus::new().await.unwrap());
        let temp_dir = tempdir().unwrap();
        let state_manager = Arc::new(
            AgentStateManager::new(temp_dir.path().to_str().unwrap()).await.unwrap()
        );
        
        let loop_instance = BusinessEconomyLoop::new(bus, state_manager, None).await.unwrap();
        let state = loop_instance.get_current_state().await;
        
        assert_eq!(state.cycle_number, 1);
        assert_eq!(state.current_phase, BusinessPhase::MarketAnalysis);
    }
}
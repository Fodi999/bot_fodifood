//! 🏢 Business Agent - AI Agent for Business Operations
//! 
//! Specialized AI agent focused on business management, growth campaigns,
//! revenue optimization, and operational decision-making.

use super::memory_store::{MemoryStore, MemoryQuery, MemorySortBy};
use crate::ai::agent_manager::{AIEntityAgent, AgentType, AgentState, AgentStatus, AgentConfig};
use crate::ai::persistent_memory::PersistentMemory;
use crate::ai::thinker::Thinker;
use crate::ai::growth_campaign::GrowthCampaign;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Business-focused AI agent with operational capabilities
pub struct BusinessAgent {
    /// Agent unique identifier
    id: String,
    /// Memory store for persistent storage
    memory_store: Arc<MemoryStore>,
    /// AI thinking module
    thinker: Thinker,
    /// Business profile and metrics
    business_profile: Arc<RwLock<BusinessProfile>>,
    /// Growth campaigns management
    campaigns: Arc<RwLock<Vec<GrowthCampaign>>>,
    /// Agent configuration
    config: AgentConfig,
    /// Agent state information
    state: Arc<RwLock<AgentState>>,
    /// Business knowledge and insights
    knowledge: Arc<RwLock<BusinessKnowledge>>,
}

/// Business profile and operational data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessProfile {
    /// Business name
    pub name: String,
    /// Business symbol (e.g., FDF-SEA)
    pub symbol: String,
    /// Business category/industry
    pub category: String,
    /// Current financial metrics
    pub financial_metrics: FinancialMetrics,
    /// Operational metrics
    pub operational_metrics: OperationalMetrics,
    /// Market position
    pub market_position: MarketPosition,
    /// Business goals and objectives
    pub goals: Vec<BusinessGoal>,
    /// Key performance indicators
    pub kpis: HashMap<String, KPI>,
}

/// Financial metrics tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialMetrics {
    /// Monthly revenue
    pub monthly_revenue: f64,
    /// Monthly profit
    pub monthly_profit: f64,
    /// Profit margin percentage
    pub profit_margin: f64,
    /// Operating expenses
    pub operating_expenses: f64,
    /// Cash flow
    pub cash_flow: f64,
    /// Revenue growth rate
    pub revenue_growth_rate: f64,
    /// Customer acquisition cost
    pub customer_acquisition_cost: f64,
    /// Customer lifetime value
    pub customer_lifetime_value: f64,
}

/// Operational metrics tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalMetrics {
    /// Total customers
    pub total_customers: u32,
    /// Monthly active customers
    pub monthly_active_customers: u32,
    /// Customer retention rate
    pub retention_rate: f64,
    /// Average order value
    pub average_order_value: f64,
    /// Order frequency
    pub order_frequency: f64,
    /// Customer satisfaction score
    pub satisfaction_score: f64,
    /// Net promoter score
    pub net_promoter_score: f64,
}

/// Market position and competitive landscape
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPosition {
    /// Market share percentage
    pub market_share: f64,
    /// Competitive advantages
    pub advantages: Vec<String>,
    /// Key competitors
    pub competitors: Vec<String>,
    /// Market trends affecting business
    pub market_trends: Vec<String>,
    /// Brand recognition score
    pub brand_recognition: f64,
    /// Social media presence metrics
    pub social_metrics: SocialMetrics,
}

/// Social media and online presence metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialMetrics {
    /// Social media followers
    pub followers: u32,
    /// Engagement rate
    pub engagement_rate: f64,
    /// Mention count
    pub mentions: u32,
    /// Sentiment score
    pub sentiment_score: f64,
    /// Viral coefficient
    pub viral_coefficient: f64,
}

/// Business goal with tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessGoal {
    /// Goal identifier
    pub id: String,
    /// Goal description
    pub description: String,
    /// Target value
    pub target_value: f64,
    /// Current progress
    pub current_value: f64,
    /// Target deadline
    pub deadline: chrono::DateTime<chrono::Utc>,
    /// Goal category
    pub category: GoalCategory,
    /// Priority level
    pub priority: u32,
    /// Status
    pub status: GoalStatus,
}

/// Categories of business goals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalCategory {
    Revenue,
    CustomerGrowth,
    Profitability,
    MarketShare,
    OperationalEfficiency,
    CustomerSatisfaction,
    BrandAwareness,
}

/// Goal status tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalStatus {
    NotStarted,
    InProgress,
    OnTrack,
    AtRisk,
    Completed,
    Failed,
}

/// Key Performance Indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPI {
    /// KPI name
    pub name: String,
    /// Current value
    pub current_value: f64,
    /// Target value
    pub target_value: f64,
    /// Unit of measurement
    pub unit: String,
    /// Trend direction
    pub trend: TrendDirection,
    /// Last updated
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Trend direction for KPIs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Up,
    Down,
    Stable,
    Volatile,
}

/// Business knowledge and insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessKnowledge {
    /// Successful strategies used
    pub successful_strategies: Vec<BusinessStrategy>,
    /// Market insights discovered
    pub market_insights: Vec<MarketInsight>,
    /// Customer behavior patterns
    pub customer_patterns: Vec<CustomerPattern>,
    /// Seasonal trends observed
    pub seasonal_trends: Vec<SeasonalTrend>,
    /// Competitive intelligence
    pub competitive_intel: Vec<CompetitiveInsight>,
    /// Operational learnings
    pub operational_learnings: Vec<OperationalLearning>,
}

/// Business strategy with results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessStrategy {
    /// Strategy name
    pub name: String,
    /// Strategy description
    pub description: String,
    /// Implementation period
    pub period: String,
    /// Results achieved
    pub results: HashMap<String, f64>,
    /// Success rating (0.0-1.0)
    pub success_rating: f64,
    /// Lessons learned
    pub lessons: Vec<String>,
}

/// Market insight discovered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketInsight {
    /// Insight title
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Discovery date
    pub discovered_at: chrono::DateTime<chrono::Utc>,
    /// Confidence level
    pub confidence: f64,
    /// Impact on business
    pub business_impact: String,
    /// Actions taken based on insight
    pub actions_taken: Vec<String>,
}

/// Customer behavior pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPattern {
    /// Pattern description
    pub description: String,
    /// Customer segment affected
    pub segment: String,
    /// Pattern frequency
    pub frequency: String,
    /// Business impact
    pub impact: f64,
    /// Recommended actions
    pub recommendations: Vec<String>,
}

/// Seasonal business trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalTrend {
    /// Season or time period
    pub period: String,
    /// Metric affected
    pub metric: String,
    /// Average change percentage
    pub change_percentage: f64,
    /// Historical data points
    pub data_points: u32,
    /// Preparation strategies
    pub preparation_strategies: Vec<String>,
}

/// Competitive intelligence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitiveInsight {
    /// Competitor name
    pub competitor: String,
    /// Insight description
    pub insight: String,
    /// Competitive advantage/disadvantage
    pub impact: String,
    /// Strategic response
    pub response_strategy: Option<String>,
}

/// Operational learning from experience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalLearning {
    /// Learning description
    pub description: String,
    /// Area of operations
    pub area: String,
    /// Cost/benefit impact
    pub impact: f64,
    /// Implementation difficulty
    pub difficulty: u32,
    /// Current adoption status
    pub adopted: bool,
}

impl Default for BusinessProfile {
    fn default() -> Self {
        let mut kpis = HashMap::new();
        kpis.insert("monthly_revenue".to_string(), KPI {
            name: "Monthly Revenue".to_string(),
            current_value: 150000.0,
            target_value: 200000.0,
            unit: "USD".to_string(),
            trend: TrendDirection::Up,
            last_updated: chrono::Utc::now(),
        });

        Self {
            name: "Sample Business".to_string(),
            symbol: "FDF-XXX".to_string(),
            category: "Food Service".to_string(),
            financial_metrics: FinancialMetrics {
                monthly_revenue: 150000.0,
                monthly_profit: 45000.0,
                profit_margin: 30.0,
                operating_expenses: 105000.0,
                cash_flow: 35000.0,
                revenue_growth_rate: 15.0,
                customer_acquisition_cost: 25.0,
                customer_lifetime_value: 450.0,
            },
            operational_metrics: OperationalMetrics {
                total_customers: 2500,
                monthly_active_customers: 1800,
                retention_rate: 78.0,
                average_order_value: 35.50,
                order_frequency: 2.3,
                satisfaction_score: 4.2,
                net_promoter_score: 45.0,
            },
            market_position: MarketPosition {
                market_share: 8.5,
                advantages: vec!["Local expertise".to_string(), "Quality ingredients".to_string()],
                competitors: vec!["Competitor A".to_string(), "Competitor B".to_string()],
                market_trends: vec!["Healthy eating".to_string(), "Delivery preference".to_string()],
                brand_recognition: 0.65,
                social_metrics: SocialMetrics {
                    followers: 5200,
                    engagement_rate: 0.048,
                    mentions: 340,
                    sentiment_score: 0.72,
                    viral_coefficient: 1.15,
                },
            },
            goals: Vec::new(),
            kpis,
        }
    }
}

impl Default for BusinessKnowledge {
    fn default() -> Self {
        Self {
            successful_strategies: vec![
                BusinessStrategy {
                    name: "Social Media Campaign".to_string(),
                    description: "Targeted local social media advertising".to_string(),
                    period: "Q1 2025".to_string(),
                    results: {
                        let mut results = HashMap::new();
                        results.insert("customer_growth".to_string(), 23.5);
                        results.insert("revenue_increase".to_string(), 18.2);
                        results
                    },
                    success_rating: 0.85,
                    lessons: vec!["Visual content performs best".to_string()],
                }
            ],
            market_insights: Vec::new(),
            customer_patterns: Vec::new(),
            seasonal_trends: Vec::new(),
            competitive_intel: Vec::new(),
            operational_learnings: Vec::new(),
        }
    }
}

impl BusinessAgent {
    /// Create new business agent
    pub async fn new(id: &str, persistent_memory: Arc<PersistentMemory>) -> Result<Self> {
        let memory_store = Arc::new(MemoryStore::new(persistent_memory).await?);
        let thinker = Thinker;
        let business_profile = Arc::new(RwLock::new(BusinessProfile::default()));
        let campaigns = Arc::new(RwLock::new(Vec::new()));
        let config = AgentConfig::default();
        
        let state = Arc::new(RwLock::new(AgentState {
            id: id.to_string(),
            agent_type: AgentType::Business,
            created_at: chrono::Utc::now(),
            last_active: chrono::Utc::now(),
            interaction_count: 0,
            memory_size: 0,
            status: AgentStatus::Active,
            config_version: 1,
        }));
        
        let knowledge = Arc::new(RwLock::new(BusinessKnowledge::default()));

        let agent = Self {
            id: id.to_string(),
            memory_store,
            thinker,
            business_profile,
            campaigns,
            config,
            state,
            knowledge,
        };

        // Initialize with basic business knowledge
        agent.initialize_memories().await?;

        Ok(agent)
    }

    /// Initialize agent with basic business knowledge
    async fn initialize_memories(&self) -> Result<()> {
        let profile = self.business_profile.read().await;
        
        // Store business profile
        self.memory_store.store(
            &self.id,
            "profile",
            "business_name",
            &format!("I operate {}, a {} business", profile.name, profile.category)
        ).await?;

        self.memory_store.store(
            &self.id,
            "metrics",
            "current_performance",
            &format!("Monthly revenue: ${:.0}, Profit margin: {:.1}%, {} customers", 
                profile.financial_metrics.monthly_revenue,
                profile.financial_metrics.profit_margin,
                profile.operational_metrics.total_customers)
        ).await?;

        self.memory_store.store(
            &self.id,
            "strategy",
            "growth_focus",
            "Focus on customer acquisition, retention, and revenue growth through data-driven strategies"
        ).await?;

        Ok(())
    }

    /// Process business-related queries
    async fn process_business_query(&mut self, input: &str) -> Result<String> {
        let input_lower = input.to_lowercase();
        
        if input_lower.contains("revenue") || input_lower.contains("profit") || input_lower.contains("financial") {
            self.handle_financial_query().await
        } else if input_lower.contains("campaign") || input_lower.contains("marketing") || input_lower.contains("growth") {
            self.handle_growth_query(input).await
        } else if input_lower.contains("customer") || input_lower.contains("retention") {
            self.handle_customer_query().await
        } else if input_lower.contains("kpi") || input_lower.contains("metrics") || input_lower.contains("performance") {
            self.handle_metrics_query().await
        } else if input_lower.contains("goal") || input_lower.contains("target") {
            self.handle_goals_query().await
        } else if input_lower.contains("competitor") || input_lower.contains("market") {
            self.handle_market_query().await
        } else {
            self.general_business_thinking(input).await
        }
    }

    /// Handle financial queries
    async fn handle_financial_query(&self) -> Result<String> {
        let profile = self.business_profile.read().await;
        let metrics = &profile.financial_metrics;
        
        let response = format!(
            "💰 Financial Performance Overview:\n\n\
            📊 Revenue & Profitability:\n\
            • Monthly Revenue: ${:.0}\n\
            • Monthly Profit: ${:.0}\n\
            • Profit Margin: {:.1}%\n\
            • Operating Expenses: ${:.0}\n\n\
            💸 Cash Flow: ${:.0}\n\
            📈 Revenue Growth Rate: {:.1}%\n\n\
            👥 Customer Economics:\n\
            • Customer Acquisition Cost: ${:.2}\n\
            • Customer Lifetime Value: ${:.2}\n\
            • LTV/CAC Ratio: {:.1}:1\n\n\
            💡 Financial Health: {}\n\
            🎯 Next Priority: {}",
            metrics.monthly_revenue,
            metrics.monthly_profit,
            metrics.profit_margin,
            metrics.operating_expenses,
            metrics.cash_flow,
            metrics.revenue_growth_rate,
            metrics.customer_acquisition_cost,
            metrics.customer_lifetime_value,
            metrics.customer_lifetime_value / metrics.customer_acquisition_cost,
            if metrics.profit_margin > 25.0 { "Strong profitability" } else { "Improving margins needed" },
            if metrics.revenue_growth_rate > 20.0 { "Optimize operations for scale" } else { "Accelerate growth initiatives" }
        );

        // Store financial analysis
        self.memory_store.store(
            &self.id,
            "financial",
            "analysis",
            &format!("Revenue: ${:.0}, Margin: {:.1}%, Growth: {:.1}%", 
                metrics.monthly_revenue, metrics.profit_margin, metrics.revenue_growth_rate)
        ).await?;

        Ok(response)
    }

    /// Handle growth and marketing queries
    async fn handle_growth_query(&mut self, input: &str) -> Result<String> {
        let profile = self.business_profile.read().await;
        let campaigns = self.campaigns.read().await;
        
        let response = if input.contains("run") || input.contains("start") {
            // Campaign execution request
            format!(
                "🚀 Growth Campaign Strategy:\n\n\
                🎯 Business Context:\n\
                • Current Revenue: ${:.0}/month\n\
                • Customer Base: {} customers\n\
                • Retention Rate: {:.1}%\n\n\
                📊 Recommended Campaign Focus:\n\
                • Customer Acquisition (CAC: ${:.2})\n\
                • Revenue per Customer (AOV: ${:.2})\n\
                • Social Media Engagement\n\
                • Local Market Penetration\n\n\
                ⚡ Active Campaigns: {}\n\
                💡 I can design targeted campaigns based on our current metrics \
                and market position. What specific growth area should we focus on?",
                profile.financial_metrics.monthly_revenue,
                profile.operational_metrics.total_customers,
                profile.operational_metrics.retention_rate,
                profile.financial_metrics.customer_acquisition_cost,
                profile.operational_metrics.average_order_value,
                campaigns.len()
            )
        } else {
            // General growth analysis
            format!(
                "📈 Growth Analysis:\n\n\
                🎯 Current Growth Rate: {:.1}%\n\
                👥 Customer Growth: {} total, {} monthly active\n\
                💰 Revenue Trajectory: ${:.0}/month\n\n\
                🎪 Growth Opportunities:\n\
                • Increase customer frequency ({:.1} orders/month)\n\
                • Improve retention ({:.1}% current)\n\
                • Expand average order value (${:.2})\n\
                • Social media amplification ({} followers)\n\n\
                🧠 Strategic Insights:\n\
                • LTV/CAC ratio: {:.1}:1 (healthy > 3:1)\n\
                • Market share: {:.1}% (growth potential)\n\
                • Brand recognition: {:.0}%",
                profile.financial_metrics.revenue_growth_rate,
                profile.operational_metrics.total_customers,
                profile.operational_metrics.monthly_active_customers,
                profile.financial_metrics.monthly_revenue,
                profile.operational_metrics.order_frequency,
                profile.operational_metrics.retention_rate,
                profile.operational_metrics.average_order_value,
                profile.market_position.social_metrics.followers,
                profile.financial_metrics.customer_lifetime_value / profile.financial_metrics.customer_acquisition_cost,
                profile.market_position.market_share,
                profile.market_position.brand_recognition * 100.0
            )
        };

        // Store growth query
        self.memory_store.store(
            &self.id,
            "growth",
            "analysis",
            "Analyzed growth opportunities and campaign strategies"
        ).await?;

        Ok(response)
    }

    /// Handle customer-related queries
    async fn handle_customer_query(&self) -> Result<String> {
        let profile = self.business_profile.read().await;
        let ops = &profile.operational_metrics;
        
        let response = format!(
            "👥 Customer Analytics Dashboard:\n\n\
            📊 Customer Base:\n\
            • Total Customers: {}\n\
            • Monthly Active: {} ({:.1}% activation)\n\
            • Retention Rate: {:.1}%\n\
            • Satisfaction Score: {:.1}/5.0\n\n\
            💰 Customer Value:\n\
            • Average Order Value: ${:.2}\n\
            • Order Frequency: {:.1}/month\n\
            • Net Promoter Score: {:.0}\n\n\
            🎯 Customer Insights:\n\
            • Activation Rate: {:.1}%\n\
            • Churn Risk: {:.1}%\n\
            • Growth Potential: {}\n\n\
            💡 Recommendations:\n\
            • Focus on increasing order frequency\n\
            • Implement loyalty program for retention\n\
            • Address satisfaction gaps\n\
            • Leverage NPS promoters for referrals",
            ops.total_customers,
            ops.monthly_active_customers,
            (ops.monthly_active_customers as f64 / ops.total_customers as f64) * 100.0,
            ops.retention_rate,
            ops.satisfaction_score,
            ops.average_order_value,
            ops.order_frequency,
            ops.net_promoter_score,
            (ops.monthly_active_customers as f64 / ops.total_customers as f64) * 100.0,
            100.0 - ops.retention_rate,
            if ops.retention_rate > 75.0 { "High" } else { "Medium" }
        );

        Ok(response)
    }

    /// Handle metrics and KPI queries
    async fn handle_metrics_query(&self) -> Result<String> {
        let profile = self.business_profile.read().await;
        
        let kpi_summary: Vec<String> = profile.kpis.iter()
            .map(|(name, kpi)| {
                let progress = (kpi.current_value / kpi.target_value) * 100.0;
                let status = match progress {
                    p if p >= 100.0 => "✅",
                    p if p >= 80.0 => "🟡",
                    _ => "🔴",
                };
                format!("  {} {}: {:.1} / {:.1} {} ({:.0}%)", 
                    status, kpi.name, kpi.current_value, kpi.target_value, kpi.unit, progress)
            })
            .collect();

        let response = format!(
            "📊 Key Performance Indicators:\n\n\
            🎯 Current KPI Status:\n\
            {}\n\n\
            📈 Performance Trends:\n\
            • Revenue Growth: {:.1}% monthly\n\
            • Customer Growth: {:.1}% monthly\n\
            • Profit Margin: {:.1}%\n\
            • Social Engagement: {:.1}%\n\n\
            🎪 Action Items:\n\
            • KPIs on track: {}\n\
            • KPIs needing attention: {}\n\
            • Next review: Weekly monitoring\n\n\
            💡 Focus Areas: Revenue optimization, customer retention, operational efficiency",
            kpi_summary.join("\n"),
            profile.financial_metrics.revenue_growth_rate,
            15.0, // Mock customer growth
            profile.financial_metrics.profit_margin,
            profile.market_position.social_metrics.engagement_rate * 100.0,
            profile.kpis.values().filter(|kpi| (kpi.current_value / kpi.target_value) >= 0.8).count(),
            profile.kpis.values().filter(|kpi| (kpi.current_value / kpi.target_value) < 0.8).count()
        );

        Ok(response)
    }

    /// Handle goals and targets queries
    async fn handle_goals_query(&self) -> Result<String> {
        let profile = self.business_profile.read().await;
        
        let response = if profile.goals.is_empty() {
            format!(
                "🎯 Business Goals Framework:\n\n\
                💡 Suggested Goals Based on Current Performance:\n\n\
                📊 Financial Goals:\n\
                • Increase monthly revenue to ${:.0} (+33%)\n\
                • Improve profit margin to 35% (+{:.1}pp)\n\
                • Achieve ${:.0} monthly profit\n\n\
                👥 Customer Goals:\n\
                • Grow customer base to {} (+25%)\n\
                • Improve retention to 85% (+{:.1}pp)\n\
                • Increase satisfaction to 4.5/5.0\n\n\
                🎪 Market Goals:\n\
                • Achieve {:.1}% market share\n\
                • Build {} social media followers\n\
                • Launch 2 major growth campaigns\n\n\
                ⏰ Timeline: 6-month strategic plan\n\
                📝 Would you like me to help set up specific, measurable goals?",
                profile.financial_metrics.monthly_revenue * 1.33,
                35.0 - profile.financial_metrics.profit_margin,
                profile.financial_metrics.monthly_revenue * 1.33 * 0.35,
                (profile.operational_metrics.total_customers as f64 * 1.25) as u32,
                85.0 - profile.operational_metrics.retention_rate,
                profile.market_position.market_share * 1.5,
                profile.market_position.social_metrics.followers * 2
            )
        } else {
            let goal_summary: Vec<String> = profile.goals.iter()
                .map(|goal| {
                    let progress = (goal.current_value / goal.target_value) * 100.0;
                    let status_icon = match goal.status {
                        GoalStatus::Completed => "✅",
                        GoalStatus::OnTrack => "🟢",
                        GoalStatus::InProgress => "🟡",
                        GoalStatus::AtRisk => "🟠",
                        GoalStatus::Failed => "🔴",
                        GoalStatus::NotStarted => "⚪",
                    };
                    format!("  {} {}: {:.1}% complete", status_icon, goal.description, progress)
                })
                .collect();

            format!(
                "🎯 Business Goals Status:\n\n\
                📊 Current Goals:\n\
                {}\n\n\
                🏆 Goals Summary:\n\
                • Total Goals: {}\n\
                • On Track: {}\n\
                • At Risk: {}\n\
                • Completed: {}\n\n\
                💡 Next Actions: Review progress weekly, adjust strategies as needed",
                goal_summary.join("\n"),
                profile.goals.len(),
                profile.goals.iter().filter(|g| matches!(g.status, GoalStatus::OnTrack)).count(),
                profile.goals.iter().filter(|g| matches!(g.status, GoalStatus::AtRisk)).count(),
                profile.goals.iter().filter(|g| matches!(g.status, GoalStatus::Completed)).count()
            )
        };

        Ok(response)
    }

    /// Handle market and competitive queries
    async fn handle_market_query(&self) -> Result<String> {
        let profile = self.business_profile.read().await;
        let market = &profile.market_position;
        
        let response = format!(
            "🏪 Market Position Analysis:\n\n\
            📊 Current Market Standing:\n\
            • Market Share: {:.1}%\n\
            • Brand Recognition: {:.0}%\n\
            • Competitive Advantages: {}\n\n\
            🎯 Social Presence:\n\
            • Followers: {}\n\
            • Engagement Rate: {:.1}%\n\
            • Sentiment Score: {:.2} (positive)\n\
            • Viral Coefficient: {:.2}\n\n\
            🏆 Competitive Landscape:\n\
            • Key Competitors: {}\n\
            • Market Trends: {}\n\n\
            💡 Strategic Opportunities:\n\
            • Expand market share through differentiation\n\
            • Leverage positive sentiment for growth\n\
            • Build on competitive advantages\n\
            • Capitalize on favorable market trends",
            market.market_share,
            market.brand_recognition * 100.0,
            market.advantages.join(", "),
            market.social_metrics.followers,
            market.social_metrics.engagement_rate * 100.0,
            market.social_metrics.sentiment_score,
            market.social_metrics.viral_coefficient,
            market.competitors.join(", "),
            market.market_trends.join(", ")
        );

        Ok(response)
    }

    /// General business thinking for other queries
    async fn general_business_thinking(&mut self, input: &str) -> Result<String> {
        let profile = self.business_profile.read().await;
        
        let context = format!(
            "I am a business-focused AI agent managing {}:\n\
            - Business: {} ({})\n\
            - Monthly Revenue: ${:.0}\n\
            - Profit Margin: {:.1}%\n\
            - Customers: {}\n\
            - Growth Rate: {:.1}%\n\
            - Focus: Growth, profitability, customer satisfaction\n\n\
            User query: {}",
            profile.symbol,
            profile.name,
            profile.category,
            profile.financial_metrics.monthly_revenue,
            profile.financial_metrics.profit_margin,
            profile.operational_metrics.total_customers,
            profile.financial_metrics.revenue_growth_rate,
            input
        );

        let response = Thinker::think(&context).await?;

        // Store interaction
        self.memory_store.store(
            &self.id,
            "conversation",
            "interaction",
            &format!("Q: {} | A: {}", input, &response)
        ).await?;

        Ok(response)
    }

    /// Update business metrics
    pub async fn update_metrics(&mut self, revenue: Option<f64>, customers: Option<u32>, retention: Option<f64>) -> Result<()> {
        let mut profile = self.business_profile.write().await;
        
        if let Some(rev) = revenue {
            let old_revenue = profile.financial_metrics.monthly_revenue;
            profile.financial_metrics.monthly_revenue = rev;
            profile.financial_metrics.revenue_growth_rate = ((rev - old_revenue) / old_revenue) * 100.0;
            
            // Update profit assuming same margin
            profile.financial_metrics.monthly_profit = rev * (profile.financial_metrics.profit_margin / 100.0);
        }

        if let Some(cust) = customers {
            profile.operational_metrics.total_customers = cust;
            profile.operational_metrics.monthly_active_customers = (cust as f64 * 0.72) as u32; // 72% activation
        }

        if let Some(ret) = retention {
            profile.operational_metrics.retention_rate = ret;
        }

        // Store metric update
        self.memory_store.store(
            &self.id,
            "metrics",
            "update",
            &format!("Updated metrics: Revenue=${:.0}, Customers={}, Retention={:.1}%", 
                profile.financial_metrics.monthly_revenue,
                profile.operational_metrics.total_customers,
                profile.operational_metrics.retention_rate)
        ).await?;

        Ok(())
    }
}

impl AIEntityAgent for BusinessAgent {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_type(&self) -> AgentType {
        AgentType::Business
    }

    fn think(&mut self, input: &str) -> Result<String> {
        // Update last active time
        if let Ok(mut state) = self.state.try_write() {
            state.last_active = chrono::Utc::now();
            state.interaction_count += 1;
        }

        // Process business-specific query
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.process_business_query(input).await
            })
        })
    }

    fn recall(&self, query: Option<&str>) -> String {
        let memories = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                if let Some(q) = query {
                    self.memory_store.search(MemoryQuery {
                        agent_id: Some(self.id.clone()),
                        category: None,
                        search_text: Some(q.to_string()),
                        tags: Vec::new(),
                        min_importance: None,
                        limit: Some(10),
                        sort_by: MemorySortBy::Relevance,
                    }).await.unwrap_or_default()
                } else {
                    self.memory_store.get_agent_memories(&self.id).await.unwrap_or_default()
                }
            })
        });

        if memories.is_empty() {
            format!("🏢 Business Agent {} Memory: Building operational knowledge and insights...", self.id)
        } else {
            let memory_summary: Vec<String> = memories.iter()
                .take(5)
                .map(|m| format!("• {}: {}", m.key, m.value))
                .collect();
            
            format!("🏢 Business Agent {} Memory:\n{}", self.id, memory_summary.join("\n"))
        }
    }

    fn memorize(&mut self, key: &str, value: &str) -> Result<()> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.memory_store.store(&self.id, "manual", key, value).await
            })
        })?;
        Ok(())
    }

    fn get_state_summary(&self) -> AgentState {
        self.state.blocking_read().clone()
    }

    fn receive_message(&mut self, from_agent: &str, message: &str) -> Result<Option<String>> {
        let response = format!(
            "📨 Message from {}: {}\n\
            🏢 As a business agent, I'll integrate this information \
            into my operational planning and strategic decisions.",
            from_agent, message
        );

        self.memorize(
            &format!("message_from_{}", from_agent),
            &format!("{}: {}", from_agent, message)
        )?;

        Ok(Some(response))
    }

    fn get_capabilities(&self) -> Vec<String> {
        vec![
            "Business Performance Analysis".to_string(),
            "Growth Campaign Management".to_string(),
            "Customer Analytics".to_string(),
            "Financial Planning".to_string(),
            "Market Research".to_string(),
            "KPI Tracking".to_string(),
            "Goal Setting & Monitoring".to_string(),
            "Competitive Intelligence".to_string(),
        ]
    }

    fn update_config(&mut self, config: AgentConfig) -> Result<()> {
        self.config = config;
        
        if let Ok(mut state) = self.state.try_write() {
            state.config_version += 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_business_agent_creation() {
        let persistent_memory = Arc::new(PersistentMemory::new("test_business.db").unwrap());
        let agent = BusinessAgent::new("BUSINESS-TEST", persistent_memory).await.unwrap();
        
        assert_eq!(agent.get_id(), "BUSINESS-TEST");
        assert!(matches!(agent.get_type(), AgentType::Business));
    }

    #[tokio::test]
    async fn test_business_metrics_query() {
        let persistent_memory = Arc::new(PersistentMemory::new("test_business2.db").unwrap());
        let mut agent = BusinessAgent::new("BUSINESS-TEST2", persistent_memory).await.unwrap();
        
        let response = agent.think("Show me my financial performance").unwrap();
        assert!(response.contains("Financial Performance"));
        assert!(response.contains("Revenue"));
    }
}
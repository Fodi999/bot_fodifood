//! 💰 Investor Agent - AI Agent for Investment Management
//! 
//! Specialized AI agent focused on investment analysis, portfolio management,
//! and financial decision-making with persistent memory and learning capabilities.

use super::memory_store::{MemoryStore, MemoryQuery, MemorySortBy};
use crate::ai::agent_manager::{AIEntityAgent, AgentType, AgentState, AgentStatus, AgentConfig};
use crate::ai::persistent_memory::PersistentMemory;
use crate::ai::investor::{Portfolio, InvestmentScreener, YieldCalculator, InvestmentAdvisor};
use crate::ai::investor::yield_engine::YieldInputs;
use crate::ai::thinker::Thinker;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Investment-focused AI agent with specialized capabilities
pub struct InvestorAgent {
    /// Agent unique identifier
    id: String,
    /// Memory store for investment data
    memory_store: Arc<MemoryStore>,
    /// AI thinking capabilities
    thinker: Thinker,
    /// Portfolio manager
    portfolio: Arc<RwLock<Portfolio>>,
    /// Investment screening engine
    screener: InvestmentScreener,
    /// Yield calculation engine
    yield_calculator: YieldCalculator,
    /// Investment advisor
    advisor: InvestmentAdvisor,
    /// Agent configuration
    config: AgentConfig,
    /// Agent state information
    state: Arc<RwLock<AgentState>>,
    /// Specialized knowledge base
    knowledge: Arc<RwLock<InvestorKnowledge>>,
}

/// Investor agent's specialized knowledge and experience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestorKnowledge {
    /// Investment preferences and risk tolerance
    pub investment_profile: InvestmentProfile,
    /// Historical investment performance
    pub performance_history: Vec<InvestmentPerformance>,
    /// Market insights and observations
    pub market_insights: Vec<MarketInsight>,
    /// Learned patterns and strategies
    pub learned_strategies: Vec<InvestmentStrategy>,
    /// Risk management rules
    pub risk_rules: Vec<RiskRule>,
    /// Watchlist of interesting opportunities
    pub watchlist: Vec<String>,
}

/// Investment profile and preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentProfile {
    /// Risk tolerance (0.0 = very conservative, 1.0 = very aggressive)
    pub risk_tolerance: f64,
    /// Investment time horizon in months
    pub time_horizon: u32,
    /// Preferred investment sectors
    pub preferred_sectors: Vec<String>,
    /// Investment goals and objectives
    pub goals: Vec<String>,
    /// Minimum expected return percentage
    pub min_expected_return: f64,
    /// Maximum position size percentage
    pub max_position_size: f64,
    /// Diversification requirements
    pub diversification_rules: HashMap<String, f64>,
}

/// Historical investment performance record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentPerformance {
    /// Performance period
    pub period: String,
    /// Total return percentage
    pub total_return: f64,
    /// Sharpe ratio
    pub sharpe_ratio: f64,
    /// Maximum drawdown
    pub max_drawdown: f64,
    /// Number of winning trades
    pub winning_trades: u32,
    /// Number of losing trades
    pub losing_trades: u32,
    /// Average holding period in days
    pub avg_holding_period: u32,
}

/// Market insight or observation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketInsight {
    /// Insight timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Market sector or symbol
    pub subject: String,
    /// Insight category
    pub category: InsightCategory,
    /// Insight description
    pub description: String,
    /// Confidence level (0.0-1.0)
    pub confidence: f64,
    /// Supporting data or evidence
    pub evidence: Vec<String>,
    /// Outcome or validation (if available)
    pub outcome: Option<String>,
}

/// Categories of market insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightCategory {
    TrendAnalysis,
    ValuationOpportunity,
    RiskWarning,
    SeasonalPattern,
    NewsImpact,
    TechnicalSignal,
    FundamentalChange,
}

/// Investment strategy learned from experience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentStrategy {
    /// Strategy name
    pub name: String,
    /// Strategy description
    pub description: String,
    /// Conditions for applying strategy
    pub conditions: Vec<String>,
    /// Expected outcomes
    pub expected_outcomes: Vec<String>,
    /// Success rate based on historical data
    pub success_rate: f64,
    /// Last used timestamp
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
}

/// Risk management rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskRule {
    /// Rule identifier
    pub id: String,
    /// Rule description
    pub description: String,
    /// Trigger conditions
    pub triggers: Vec<String>,
    /// Actions to take
    pub actions: Vec<String>,
    /// Rule priority (higher = more important)
    pub priority: u32,
    /// Times rule was triggered
    pub trigger_count: u32,
}

impl Default for InvestorKnowledge {
    fn default() -> Self {
        Self {
            investment_profile: InvestmentProfile {
                risk_tolerance: 0.6,
                time_horizon: 24, // 2 years
                preferred_sectors: vec!["food".to_string(), "technology".to_string()],
                goals: vec!["capital_growth".to_string(), "passive_income".to_string()],
                min_expected_return: 8.0,
                max_position_size: 20.0,
                diversification_rules: {
                    let mut rules = HashMap::new();
                    rules.insert("single_asset".to_string(), 0.2);
                    rules.insert("sector".to_string(), 0.4);
                    rules
                },
            },
            performance_history: Vec::new(),
            market_insights: Vec::new(),
            learned_strategies: vec![
                InvestmentStrategy {
                    name: "Growth Momentum".to_string(),
                    description: "Invest in companies showing consistent revenue growth".to_string(),
                    conditions: vec!["revenue_growth > 15%".to_string(), "positive_sentiment".to_string()],
                    expected_outcomes: vec!["12-18% annual return".to_string()],
                    success_rate: 0.72,
                    last_used: None,
                },
                InvestmentStrategy {
                    name: "Value Opportunity".to_string(),
                    description: "Invest in undervalued companies with strong fundamentals".to_string(),
                    conditions: vec!["low_price_to_value".to_string(), "strong_financials".to_string()],
                    expected_outcomes: vec!["15-25% return potential".to_string()],
                    success_rate: 0.68,
                    last_used: None,
                },
            ],
            risk_rules: vec![
                RiskRule {
                    id: "max_single_position".to_string(),
                    description: "No single position should exceed 20% of portfolio".to_string(),
                    triggers: vec!["position_size > 20%".to_string()],
                    actions: vec!["reduce_position".to_string(), "alert_user".to_string()],
                    priority: 1,
                    trigger_count: 0,
                },
                RiskRule {
                    id: "stop_loss".to_string(),
                    description: "Exit position if loss exceeds 15%".to_string(),
                    triggers: vec!["position_loss > 15%".to_string()],
                    actions: vec!["sell_position".to_string(), "analyze_failure".to_string()],
                    priority: 2,
                    trigger_count: 0,
                },
            ],
            watchlist: Vec::new(),
        }
    }
}

impl InvestorAgent {
    /// Create new investor agent
    pub async fn new(id: &str, persistent_memory: Arc<PersistentMemory>) -> Result<Self> {
        let memory_store = Arc::new(MemoryStore::new(persistent_memory).await?);
        let thinker = Thinker;
        let portfolio = Arc::new(RwLock::new(Portfolio::new(10000.0))); // Start with $10k
        let screener = InvestmentScreener::new();
        let yield_calculator = YieldCalculator::new(YieldInputs::new(
            150000.0, // Monthly revenue
            45000.0,  // Monthly profit 
            1000000.0, // Total tokens
            2.45      // Current price
        ));
        let advisor = InvestmentAdvisor::new();
        let config = AgentConfig::default();
        
        let state = Arc::new(RwLock::new(AgentState {
            id: id.to_string(),
            agent_type: AgentType::Investor,
            created_at: chrono::Utc::now(),
            last_active: chrono::Utc::now(),
            interaction_count: 0,
            memory_size: 0,
            status: AgentStatus::Active,
            config_version: 1,
        }));
        
        let knowledge = Arc::new(RwLock::new(InvestorKnowledge::default()));

        let agent = Self {
            id: id.to_string(),
            memory_store,
            thinker,
            portfolio,
            screener,
            yield_calculator,
            advisor,
            config,
            state,
            knowledge,
        };

        // Initialize with basic memories
        agent.initialize_memories().await?;

        Ok(agent)
    }

    /// Initialize agent with basic investment knowledge
    async fn initialize_memories(&self) -> Result<()> {
        // Store investment profile
        let profile = self.knowledge.read().await.investment_profile.clone();
        self.memory_store.store(
            &self.id,
            "profile",
            "risk_tolerance",
            &format!("My risk tolerance is {:.1}/10", profile.risk_tolerance * 10.0)
        ).await?;

        self.memory_store.store(
            &self.id,
            "profile",
            "investment_goals",
            &format!("My investment goals: {}", profile.goals.join(", "))
        ).await?;

        self.memory_store.store(
            &self.id,
            "strategy",
            "diversification",
            "I believe in diversification: max 20% in single asset, 40% in single sector"
        ).await?;

        Ok(())
    }

    /// Process investment-related queries
    async fn process_investment_query(&mut self, input: &str) -> Result<String> {
        let input_lower = input.to_lowercase();
        
        // Check for specific investment actions
        if input_lower.contains("portfolio") || input_lower.contains("holdings") {
            self.handle_portfolio_query().await
        } else if input_lower.contains("invest") || input_lower.contains("buy") {
            self.handle_investment_request(input).await
        } else if input_lower.contains("analyze") || input_lower.contains("research") {
            self.handle_analysis_request(input).await
        } else if input_lower.contains("market") || input_lower.contains("trend") {
            self.handle_market_query(input).await
        } else if input_lower.contains("risk") {
            self.handle_risk_query(input).await
        } else if input_lower.contains("yield") || input_lower.contains("dividend") {
            self.handle_yield_query(input).await
        } else {
            // General investment thinking
            self.general_investment_thinking(input).await
        }
    }

    /// Handle portfolio-related queries
    async fn handle_portfolio_query(&self) -> Result<String> {
        let portfolio = self.portfolio.read().await;
        // Simple price function that returns default prices
        let price_fn = |_symbol: &str| 1.0; // Default price for now
        let summary = portfolio.get_summary(&price_fn);
        
        let response = format!(
            "📊 My current portfolio:\n\
            💰 Total Value: ${:.2}\n\
            📈 P&L: ${:.2} ({:.1}%)\n\
            🏦 Cash: ${:.2}\n\
            📊 Positions: {}\n\
            💵 Monthly Dividends: ${:.2}\n\
            📅 Annual Yield: {:.1}%",
            summary.total_value,
            summary.unrealized_pnl,
            summary.return_pct,
            summary.cash_usd,
            summary.positions_count,
            summary.total_dividends / 12.0, // monthly dividends
            (summary.total_dividends / summary.total_value * 100.0) // annual yield
        );

        // Store this analysis in memory
        self.memory_store.store(
            &self.id,
            "portfolio",
            "last_analysis",
            &response
        ).await?;

        Ok(response)
    }

    /// Handle investment requests
    async fn handle_investment_request(&mut self, input: &str) -> Result<String> {
        // Extract investment details from input
        let amount = self.extract_amount(input);
        let symbol = self.extract_symbol(input);

        let response = if let (Some(amt), Some(sym)) = (amount, symbol) {
            // Analyze the investment opportunity
            let analysis = format!(
                "🤔 Analyzing investment of ${:.2} in {}...\n\
                Let me check my investment criteria and risk rules.",
                amt, sym
            );

            // Store investment consideration
            self.memory_store.store(
                &self.id,
                "investment",
                "consideration",
                &format!("Considering ${:.2} investment in {}", amt, sym)
            ).await?;

            // Check against risk rules
            let knowledge = self.knowledge.read().await;
            let portfolio = self.portfolio.read().await;
            let price_fn = |_symbol: &str| 1.0;
            let current_total = portfolio.get_summary(&price_fn).total_value;
            let position_percentage = (amt / current_total) * 100.0;

            if position_percentage > knowledge.investment_profile.max_position_size {
                format!(
                    "⚠️ Investment Analysis:\n{}\n\
                    🚨 Risk Warning: This investment would be {:.1}% of portfolio, \
                    exceeding my {:.1}% maximum position size rule.\n\
                    💡 Recommendation: Consider reducing to ${:.2} maximum.",
                    analysis,
                    position_percentage,
                    knowledge.investment_profile.max_position_size,
                    current_total * knowledge.investment_profile.max_position_size / 100.0
                )
            } else {
                format!(
                    "✅ Investment Analysis:\n{}\n\
                    📊 Position size: {:.1}% of portfolio (within {:.1}% limit)\n\
                    💡 This looks reasonable based on my risk management rules.\n\
                    📈 Expected return target: >{:.1}% annually",
                    analysis,
                    position_percentage,
                    knowledge.investment_profile.max_position_size,
                    knowledge.investment_profile.min_expected_return
                )
            }
        } else {
            "🤔 I need more details for investment analysis. Please specify:\n\
            • Amount to invest (e.g., $5000)\n\
            • Company or symbol (e.g., FDF-SEA)\n\
            \nExample: 'I want to invest $5000 in FDF-SEA'".to_string()
        };

        Ok(response)
    }

    /// Handle analysis requests
    async fn handle_analysis_request(&mut self, input: &str) -> Result<String> {
        let symbol = self.extract_symbol(input);
        
        let response = if let Some(sym) = symbol {
            // Retrieve any previous analysis from memory
            let previous_analysis = self.memory_store.search(MemoryQuery {
                agent_id: Some(self.id.clone()),
                category: Some("analysis".to_string()),
                search_text: Some(sym.clone()),
                tags: Vec::new(),
                min_importance: None,
                limit: Some(3),
                sort_by: MemorySortBy::LastAccessed,
            }).await?;

            let analysis_context = if !previous_analysis.is_empty() {
                format!("My previous analysis of {}: {}", sym, previous_analysis[0].value)
            } else {
                format!("First time analyzing {}", sym)
            };

            let analysis = format!(
                "📊 Investment Analysis for {}:\n\n\
                🧠 Context: {}\n\n\
                🔍 Analysis Framework:\n\
                • Growth potential and market position\n\
                • Financial health and profitability\n\
                • Risk factors and market conditions\n\
                • Valuation and entry timing\n\
                • Fit with my investment profile\n\n\
                💡 I'll need current financial data to provide detailed analysis.\n\
                Would you like me to focus on any specific aspect?",
                sym, analysis_context
            );

            // Store this analysis request
            self.memory_store.store(
                &self.id,
                "analysis",
                &format!("{}_request", sym),
                &format!("Requested analysis of {} at {}", sym, chrono::Utc::now().format("%Y-%m-%d"))
            ).await?;

            analysis
        } else {
            "🔍 Analysis Request:\n\
            Please specify which company or investment you'd like me to analyze.\n\
            I can provide detailed investment analysis including:\n\
            • Financial health assessment\n\
            • Growth potential evaluation\n\
            • Risk-reward analysis\n\
            • Portfolio fit assessment".to_string()
        };

        Ok(response)
    }

    /// Handle market-related queries
    async fn handle_market_query(&mut self, _input: &str) -> Result<String> {
        // Retrieve market insights from memory
        let market_memories = self.memory_store.get_memories_by_category(&self.id, "market").await?;
        
        let insights = if !market_memories.is_empty() {
            let recent_insights: Vec<String> = market_memories.iter()
                .take(3)
                .map(|m| format!("• {}", m.value))
                .collect();
            format!("Recent market insights:\n{}", recent_insights.join("\n"))
        } else {
            "I'm continuously monitoring market conditions and will build insights over time.".to_string()
        };

        let response = format!(
            "📈 Market Analysis:\n\n\
            🧠 {}\n\n\
            🎯 Current Focus Areas:\n\
            • Food industry trends and growth\n\
            • Small business investment opportunities\n\
            • Dividend-paying growth companies\n\
            • Market sentiment and social indicators\n\n\
            💡 I adapt my investment strategy based on market conditions \
            while maintaining disciplined risk management.",
            insights
        );

        // Store market query for learning
        self.memory_store.store(
            &self.id,
            "market",
            "query",
            "Discussed current market conditions and investment focus"
        ).await?;

        Ok(response)
    }

    /// Handle risk-related queries
    async fn handle_risk_query(&mut self, _input: &str) -> Result<String> {
        let knowledge = self.knowledge.read().await;
        let profile = &knowledge.investment_profile;
        
        let response = format!(
            "⚖️ My Risk Management Approach:\n\n\
            🎯 Risk Tolerance: {:.1}/10 ({})\n\
            📊 Position Limits:\n\
            • Maximum single position: {:.1}%\n\
            • Sector concentration: {:.1}%\n\n\
            🛡️ Risk Rules:\n\
            • Stop loss at -15% position loss\n\
            • No more than 20% in single asset\n\
            • Diversification across sectors\n\
            • Regular portfolio rebalancing\n\n\
            🎯 Target Return: >{:.1}% annually\n\
            ⏱️ Investment Horizon: {} months\n\n\
            💡 I believe in taking calculated risks for superior returns \
            while protecting capital through disciplined risk management.",
            profile.risk_tolerance * 10.0,
            if profile.risk_tolerance < 0.3 { "Conservative" }
            else if profile.risk_tolerance < 0.7 { "Moderate" }
            else { "Aggressive" },
            profile.max_position_size,
            profile.diversification_rules.get("sector").unwrap_or(&40.0),
            profile.min_expected_return,
            profile.time_horizon
        );

        Ok(response)
    }

    /// Handle yield and dividend queries
    async fn handle_yield_query(&mut self, _input: &str) -> Result<String> {
        let portfolio = self.portfolio.read().await;
        let price_fn = |_symbol: &str| 1.0;
        let summary = portfolio.get_summary(&price_fn);
        
        let response = format!(
            "💰 Dividend & Yield Analysis:\n\n\
            📊 Current Portfolio Yield:\n\
            • Monthly Dividends: ${:.2}\n\
            • Annual Yield: {:.1}%\n\
            • Yield on Cost: Tracking since inception\n\n\
            🎯 My Dividend Strategy:\n\
            • Focus on sustainable dividend growth\n\
            • Target companies with strong cash flow\n\
            • Reinvest dividends for compound growth\n\
            • Monitor payout ratios and business health\n\n\
            💡 I balance dividend income with capital appreciation, \
            preferring companies that can grow both their business \
            value and dividend payments over time.\n\n\
            📈 Current dividend companies in portfolio: {}",
            summary.total_dividends / 12.0, // monthly dividends
            (summary.total_dividends / summary.total_value * 100.0), // annual yield
            summary.positions_count
        );

        // Store yield analysis
        self.memory_store.store(
            &self.id,
            "yield",
            "analysis",
            &format!("Portfolio yield: {:.1}%, Monthly dividends: ${:.2}", 
                (summary.total_dividends / summary.total_value * 100.0), summary.total_dividends / 12.0)
        ).await?;

        Ok(response)
    }

    /// General investment thinking for other queries
    async fn general_investment_thinking(&mut self, input: &str) -> Result<String> {
        // Use AI thinker with investment context
        let context = format!(
            "I am an investment-focused AI agent with the following profile:\n\
            - Risk tolerance: {:.1}/10\n\
            - Investment experience and knowledge\n\
            - Focus on food industry and growth companies\n\
            - Disciplined risk management approach\n\
            - Long-term wealth building mindset\n\n\
            User query: {}",
            self.knowledge.read().await.investment_profile.risk_tolerance * 10.0,
            input
        );

        let response = Thinker::think(&context).await?;

        // Store interaction for learning
        self.memory_store.store(
            &self.id,
            "conversation",
            "interaction",
            &format!("Q: {} | A: {}", input, &response)
        ).await?;

        Ok(response)
    }

    /// Extract monetary amount from text
    fn extract_amount(&self, text: &str) -> Option<f64> {
        use regex::Regex;
        let re = Regex::new(r"\$?([0-9,]+(?:\.[0-9]{2})?)").ok()?;
        re.captures(text)
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().replace(",", "").parse().ok())
    }

    /// Extract stock symbol from text
    fn extract_symbol(&self, text: &str) -> Option<String> {
        use regex::Regex;
        let re = Regex::new(r"\b([A-Z]{3,}-[A-Z]{3}|[A-Z]{3,})\b").ok()?;
        re.captures(text)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
    }

    /// Learn from investment outcomes
    pub async fn learn_from_outcome(&mut self, investment: &str, outcome: &str, return_pct: f64) -> Result<()> {
        let mut knowledge = self.knowledge.write().await;
        
        // Record performance
        let performance = InvestmentPerformance {
            period: format!("{} outcome", investment),
            total_return: return_pct,
            sharpe_ratio: 0.0, // Would be calculated properly
            max_drawdown: 0.0,
            winning_trades: if return_pct > 0.0 { 1 } else { 0 },
            losing_trades: if return_pct <= 0.0 { 1 } else { 0 },
            avg_holding_period: 30, // Default
        };
        
        knowledge.performance_history.push(performance);
        
        // Store learning in memory
        self.memory_store.store(
            &self.id,
            "learning",
            &format!("{}_outcome", investment),
            &format!("Investment in {} resulted in {:.1}% return. Outcome: {}", 
                investment, return_pct, outcome)
        ).await?;

        tracing::info!("📚 Investor agent {} learned from outcome: {} -> {:.1}%", 
            self.id, investment, return_pct);

        Ok(())
    }
}

impl AIEntityAgent for InvestorAgent {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_type(&self) -> AgentType {
        AgentType::Investor
    }

    fn think(&mut self, input: &str) -> Result<String> {
        // Update last active time
        if let Ok(mut state) = self.state.try_write() {
            state.last_active = chrono::Utc::now();
            state.interaction_count += 1;
        }

        // Process investment-specific query
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.process_investment_query(input).await
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
            format!("💭 Investor Agent {} Memory: Building investment knowledge and experience...", self.id)
        } else {
            let memory_summary: Vec<String> = memories.iter()
                .take(5)
                .map(|m| format!("• {}: {}", m.key, m.value))
                .collect();
            
            format!("💭 Investor Agent {} Memory:\n{}", self.id, memory_summary.join("\n"))
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
        // Process inter-agent communication
        let response = format!(
            "📨 Message from {}: {}\n\
            💭 As an investment agent, I'll consider this information \
            in my investment analysis and decision-making process.",
            from_agent, message
        );

        // Store the message as memory
        self.memorize(
            &format!("message_from_{}", from_agent),
            &format!("{}: {}", from_agent, message)
        )?;

        Ok(Some(response))
    }

    fn get_capabilities(&self) -> Vec<String> {
        vec![
            "Investment Analysis".to_string(),
            "Portfolio Management".to_string(),
            "Risk Assessment".to_string(),
            "Market Research".to_string(),
            "Yield Analysis".to_string(),
            "Financial Planning".to_string(),
            "Investment Strategy Development".to_string(),
            "Performance Tracking".to_string(),
        ]
    }

    fn update_config(&mut self, config: AgentConfig) -> Result<()> {
        self.config = config;
        
        // Update config version in state
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
    async fn test_investor_agent_creation() {
        let persistent_memory = Arc::new(PersistentMemory::new("test_investor.db").unwrap());
        let agent = InvestorAgent::new("INVESTOR-TEST", persistent_memory).await.unwrap();
        
        assert_eq!(agent.get_id(), "INVESTOR-TEST");
        assert!(matches!(agent.get_type(), AgentType::Investor));
    }

    #[tokio::test]
    async fn test_investment_query_processing() {
        let persistent_memory = Arc::new(PersistentMemory::new("test_investor2.db").unwrap());
        let mut agent = InvestorAgent::new("INVESTOR-TEST2", persistent_memory).await.unwrap();
        
        let response = agent.think("What's my current portfolio status?").unwrap();
        assert!(response.contains("portfolio"));
        
        let memory = agent.recall(Some("portfolio"));
        assert!(memory.contains("Memory"));
    }
}
//! 🤖 Investment Bot - AI-Driven Investment Assistant
//! 
//! Conversational interface for investment analysis and portfolio management

use super::{
    opportunity::{CompanyMetrics, InvestmentOpportunity, OpportunityManager},
    portfolio::Portfolio,
    screener::{InvestmentScreener, ScreenerWeights},
    yield_engine::{YieldInputs, YieldCalculator, portfolio_yield_forecast},
    advisor::{InvestmentAdvisor, AllocationStrategy},
};
use crate::ai::control;
use anyhow::Result;
use std::collections::HashMap;

/// 🤖 Investment Bot with AI integration
pub struct InvestorBot {
    /// Current portfolio
    pub portfolio: Portfolio,
    /// Investment screener
    pub screener: InvestmentScreener,
    /// Investment advisor
    pub advisor: InvestmentAdvisor,
    /// Opportunity manager
    pub opportunities: OpportunityManager,
    /// Price function
    pub price_fn: fn(&str) -> f64,
}

impl InvestorBot {
    /// Create new investor bot
    pub fn new(initial_cash: f64, price_fn: fn(&str) -> f64) -> Self {
        Self {
            portfolio: Portfolio::new(initial_cash),
            screener: InvestmentScreener::new(),
            advisor: InvestmentAdvisor::new(),
            opportunities: OpportunityManager::new(),
            price_fn,
        }
    }

    /// Set investment strategy
    pub fn set_strategy(&mut self, strategy: AllocationStrategy) {
        self.advisor = self.advisor.clone().with_strategy(strategy);
    }

    /// Set screening weights
    pub fn set_screening_weights(&mut self, weights: ScreenerWeights) {
        self.screener = InvestmentScreener::with_weights(weights);
    }

    /// Add investment opportunity
    pub fn add_opportunity(&mut self, metrics: CompanyMetrics) {
        let opportunity = InvestmentOpportunity::new(metrics, 0.0); // Score will be calculated
        self.opportunities.add(opportunity);
    }

    /// Process user query about investments
    pub async fn process_query(&mut self, query: &str) -> Result<String> {
        let normalized_query = query.to_lowercase();

        if normalized_query.contains("portfolio") || normalized_query.contains("портфель") {
            Ok(self.get_portfolio_summary())
        } else if normalized_query.contains("opportunities") || normalized_query.contains("возможности") {
            Ok(self.get_opportunities_summary())
        } else if normalized_query.contains("recommend") || normalized_query.contains("рекомендуй") {
            Ok(self.get_investment_recommendations())
        } else if normalized_query.contains("yield") || normalized_query.contains("доходность") {
            Ok(self.get_yield_forecast())
        } else if normalized_query.contains("diversify") || normalized_query.contains("диверсиф") {
            Ok(self.get_diversification_advice())
        } else if normalized_query.contains("risk") || normalized_query.contains("риск") {
            Ok(self.get_risk_analysis())
        } else {
            // Use AI for general investment queries
            self.ai_investment_analysis(query).await
        }
    }

    /// Get portfolio summary
    fn get_portfolio_summary(&self) -> String {
        let summary = self.portfolio.get_summary(&self.price_fn);
        
        format!(
            "💼 **Portfolio Summary**\n\
            💰 Cash: ${:.2}\n\
            📊 Total Value: ${:.2}\n\
            📈 Total Return: {:.1}%\n\
            🏠 Positions: {}\n\
            💎 Total Dividends: ${:.2}",
            summary.cash_usd,
            summary.total_value,
            summary.return_pct,
            summary.positions_count,
            summary.total_dividends
        )
    }

    /// Get opportunities summary
    fn get_opportunities_summary(&self) -> String {
        let top_opportunities = self.opportunities.top_n(5);
        let mut result = String::from("🎯 **Top Investment Opportunities**\n\n");

        for (i, opp) in top_opportunities.iter().enumerate() {
            result.push_str(&format!(
                "{}. **{}** ({})\n\
                   • Score: {:.1}/100\n\
                   • Price: ${:.2}\n\
                   • Expected 30d Yield: {:.1}%\n\
                   • Risk: {:.1}/10\n\n",
                i + 1,
                opp.metrics.name,
                opp.metrics.symbol,
                opp.score,
                opp.metrics.price,
                opp.expected_yield_30d,
                opp.metrics.risk * 10.0
            ));
        }

        result
    }

    /// Get investment recommendations
    fn get_investment_recommendations(&self) -> String {
        if self.portfolio.cash_usd <= 0.0 {
            return "❌ No available cash for investment".to_string();
        }

        let opportunities: Vec<_> = self.opportunities.opportunities.iter()
            .take(5)
            .cloned()
            .collect();

        if opportunities.is_empty() {
            return "❌ No investment opportunities available".to_string();
        }

        let recommendation = self.advisor.recommend(self.portfolio.cash_usd, &opportunities);
        
        let mut result = String::from("🎯 **Investment Recommendations**\n\n");
        result.push_str(&format!("💰 Available Cash: ${:.2}\n", recommendation.total_cash));
        result.push_str(&format!("📊 Strategy: {:?}\n\n", recommendation.strategy));

        for (i, allocation) in recommendation.allocations.iter().enumerate() {
            result.push_str(&format!(
                "{}. **{}**\n\
                   • Allocation: ${:.2} ({:.1}%)\n\
                   • Rationale: {}\n\n",
                i + 1,
                allocation.symbol,
                allocation.usd,
                allocation.percentage,
                allocation.rationale
            ));
        }

        result.push_str(&format!(
            "🎲 Diversification Score: {:.1}/100\n\
            💵 Remaining Cash: ${:.2}",
            recommendation.diversification_score,
            recommendation.remaining_cash
        ));

        result
    }

    /// Get yield forecast
    fn get_yield_forecast(&self) -> String {
        if self.portfolio.positions.is_empty() {
            return "❌ No positions in portfolio for yield forecast".to_string();
        }

        // Create sample yield inputs (in real app, this would come from data)
        let mut inputs_map = HashMap::new();
        for position in &self.portfolio.positions {
            let inputs = YieldInputs::new(
                100_000.0, // Sample monthly revenue
                20_000.0,  // Sample monthly profit
                1_000_000.0, // Sample total tokens
                (self.price_fn)(&position.project_symbol),
            );
            inputs_map.insert(position.project_symbol.clone(), inputs);
        }

        let forecast = portfolio_yield_forecast(&self.portfolio.positions, &inputs_map);
        
        format!(
            "💰 **Yield Forecast (30 days)**\n\n\
            📊 Total Expected Income: ${:.2}\n\
            📈 Portfolio Yield: {:.2}%\n\
            🎯 Portfolio APR: {:.1}%\n\n\
            **Position Breakdown:**\n{}",
            forecast.total_30d_income,
            forecast.portfolio_yield_30d,
            forecast.portfolio_apr * 100.0,
            forecast.position_forecasts.iter()
                .map(|(symbol, f)| format!("• {}: ${:.2}", symbol, f.div_30d_usd))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    /// Get diversification advice
    fn get_diversification_advice(&self) -> String {
        let position_count = self.portfolio.positions.len();
        let total_value = self.portfolio.total_value(&self.price_fn);
        
        if position_count == 0 {
            return "🎯 **Diversification Advice**\n\nStart by investing in 3-5 different projects to spread risk.".to_string();
        }

        let mut advice = String::from("🎯 **Diversification Analysis**\n\n");
        
        if position_count < 3 {
            advice.push_str("⚠️ **Low Diversification**\n");
            advice.push_str("Consider adding more positions to reduce risk.\n\n");
        } else if position_count > 7 {
            advice.push_str("⚠️ **Over-Diversification**\n");
            advice.push_str("Too many positions may dilute returns. Consider consolidating.\n\n");
        } else {
            advice.push_str("✅ **Good Diversification**\n");
            advice.push_str("Your position count is in the optimal range.\n\n");
        }

        // Check concentration
        let largest_position = self.portfolio.positions.iter()
            .map(|p| p.current_value((self.price_fn)(&p.project_symbol)))
            .fold(0.0, f64::max);
        
        let concentration = if total_value > 0.0 {
            (largest_position / total_value) * 100.0
        } else {
            0.0
        };

        if concentration > 40.0 {
            advice.push_str("⚠️ **High Concentration Risk**\n");
            advice.push_str(&format!("Largest position: {:.1}% of portfolio\n", concentration));
            advice.push_str("Consider reducing position size or adding more positions.\n");
        } else {
            advice.push_str("✅ **Good Position Sizing**\n");
            advice.push_str(&format!("Largest position: {:.1}% of portfolio\n", concentration));
        }

        advice
    }

    /// Get risk analysis
    fn get_risk_analysis(&self) -> String {
        if self.portfolio.positions.is_empty() {
            return "❌ No positions to analyze risk".to_string();
        }

        let total_value = self.portfolio.total_value(&self.price_fn);
        let mut weighted_risk = 0.0;
        let mut high_risk_positions = Vec::new();

        for position in &self.portfolio.positions {
            let position_value = position.current_value((self.price_fn)(&position.project_symbol));
            let weight = if total_value > 0.0 { position_value / total_value } else { 0.0 };
            
            // For demo, assume medium risk for all positions
            let position_risk = 0.5; // This would come from company metrics in real app
            weighted_risk += weight * position_risk;

            if position_risk > 0.7 {
                high_risk_positions.push(position.project_symbol.clone());
            }
        }

        let mut analysis = String::from("⚠️ **Portfolio Risk Analysis**\n\n");
        
        let risk_level = if weighted_risk > 0.7 {
            "🔴 High Risk"
        } else if weighted_risk > 0.4 {
            "🟡 Medium Risk"
        } else {
            "🟢 Low Risk"
        };

        analysis.push_str(&format!("📊 Overall Risk Level: {}\n", risk_level));
        analysis.push_str(&format!("📈 Weighted Risk Score: {:.1}/10\n\n", weighted_risk * 10.0));

        if !high_risk_positions.is_empty() {
            analysis.push_str("⚠️ **High Risk Positions:**\n");
            for symbol in high_risk_positions {
                analysis.push_str(&format!("• {}\n", symbol));
            }
            analysis.push_str("\n");
        }

        analysis.push_str("💡 **Risk Management Tips:**\n");
        analysis.push_str("• Diversify across different sectors\n");
        analysis.push_str("• Set stop-loss levels\n");
        analysis.push_str("• Regular portfolio rebalancing\n");
        analysis.push_str("• Keep cash reserves for opportunities");

        analysis
    }

    /// AI-powered investment analysis
    async fn ai_investment_analysis(&self, query: &str) -> Result<String> {
        let portfolio_context = if !self.portfolio.positions.is_empty() {
            let positions: Vec<String> = self.portfolio.positions.iter()
                .map(|p| format!("{} ({} tokens)", p.project_symbol, p.tokens))
                .collect();
            format!("Current portfolio: {}", positions.join(", "))
        } else {
            "Empty portfolio".to_string()
        };

        let investment_prompt = format!(
            "You are an expert investment advisor for FodiFood ecosystem projects. \
            Context: {} \
            Available cash: ${:.2} \
            User question: {} \
            \
            Provide specific, actionable investment advice. Focus on: \
            - Risk assessment \
            - Diversification \
            - Growth potential \
            - Passive income opportunities \
            \
            Be concise but comprehensive.",
            portfolio_context,
            self.portfolio.cash_usd,
            query
        );

        match control::controlled_query(&investment_prompt).await {
            Ok(response) => Ok(format!("🤖 **AI Investment Analysis**\n\n{}", response)),
            Err(_) => Ok("❌ Unable to provide AI analysis at this time".to_string()),
        }
    }

    /// Execute investment strategy
    pub fn execute_investment(&mut self, symbol: &str, amount: f64) -> Result<String, String> {
        let price = (self.price_fn)(symbol);
        if price <= 0.0 {
            return Err(format!("Invalid price for {}", symbol));
        }

        let tokens = amount / price;
        let position = super::portfolio::Position::new(symbol.to_string(), symbol.to_string(), tokens, price);

        match self.portfolio.open_position(position) {
            Ok(()) => Ok(format!(
                "✅ Successfully invested ${:.2} in {} ({:.2} tokens at ${:.2})",
                amount, symbol, tokens, price
            )),
            Err(e) => Err(e),
        }
    }

    /// Get quick stats
    pub fn quick_stats(&self) -> String {
        let summary = self.portfolio.get_summary(&self.price_fn);
        format!(
            "💼 ${:.0} | 📈 {:+.1}% | 🏠 {} positions | 💎 ${:.0} dividends",
            summary.total_value,
            summary.return_pct,
            summary.positions_count,
            summary.total_dividends
        )
    }
}

/// 🎯 Investment bot responses
impl InvestorBot {
    /// Handle "show me fast growth companies"
    pub fn show_fast_growers(&self) -> String {
        let fast_growth: Vec<_> = self.opportunities.opportunities.iter()
            .filter(|opp| {
                opp.metrics.sales_growth_30d > 1.2 || opp.metrics.orders_growth_30d > 1.15
            })
            .take(3)
            .collect();

        if fast_growth.is_empty() {
            return "❌ No fast-growth companies found".to_string();
        }

        let mut result = String::from("🚀 **Fast Growth Companies**\n\n");
        for (i, opp) in fast_growth.iter().enumerate() {
            result.push_str(&format!(
                "{}. **{}** ({})\n\
                   • Sales Growth: {:+.1}%\n\
                   • Orders Growth: {:+.1}%\n\
                   • Price: ${:.2}\n\
                   • Score: {:.1}/100\n\n",
                i + 1,
                opp.metrics.name,
                opp.metrics.symbol,
                (opp.metrics.sales_growth_30d - 1.0) * 100.0,
                (opp.metrics.orders_growth_30d - 1.0) * 100.0,
                opp.metrics.price,
                opp.score
            ));
        }

        result
    }

    /// Handle "allocate $X to investment"
    pub fn allocate_investment(&mut self, amount: f64) -> String {
        if amount > self.portfolio.cash_usd {
            return format!(
                "❌ Insufficient funds. Available: ${:.2}, Requested: ${:.2}",
                self.portfolio.cash_usd, amount
            );
        }

        let opportunities: Vec<_> = self.opportunities.opportunities.iter()
            .take(3)
            .cloned()
            .collect();

        if opportunities.is_empty() {
            return "❌ No investment opportunities available".to_string();
        }

        let recommendation = self.advisor.recommend(amount, &opportunities);
        
        format!(
            "🎯 **Allocation Plan for ${:.2}**\n\n{}\n\n\
            Execute with: `invest [symbol] [amount]`",
            amount,
            recommendation.allocations.iter()
                .map(|a| format!("• {}: ${:.2} ({:.1}%)", a.symbol, a.usd, a.percentage))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_price_fn(symbol: &str) -> f64 {
        match symbol {
            "FDF-SEA" => 2.50,
            "FDF-TRK" => 1.20,
            _ => 1.00,
        }
    }

    #[test]
    fn test_investor_bot_creation() {
        let bot = InvestorBot::new(10000.0, test_price_fn);
        assert_eq!(bot.portfolio.cash_usd, 10000.0);
        assert!(bot.portfolio.positions.is_empty());
    }

    #[tokio::test]
    async fn test_portfolio_query() {
        let mut bot = InvestorBot::new(5000.0, test_price_fn);
        let response = bot.process_query("show my portfolio").await.unwrap();
        assert!(response.contains("Portfolio Summary"));
    }

    #[test]
    fn test_execute_investment() {
        let mut bot = InvestorBot::new(1000.0, test_price_fn);
        let result = bot.execute_investment("FDF-SEA", 500.0);
        assert!(result.is_ok());
        assert_eq!(bot.portfolio.positions.len(), 1);
        assert_eq!(bot.portfolio.cash_usd, 500.0);
    }
}
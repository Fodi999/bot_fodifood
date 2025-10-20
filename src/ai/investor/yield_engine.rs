//! 💰 Yield Engine - Passive Income Forecasting
//! 
//! Calculates expected passive income from revenue share, profit share, and staking

use super::portfolio::Position;

/// 📊 Input data for yield calculations
#[derive(Clone, Debug)]
pub struct YieldInputs {
    /// Monthly revenue (USD)
    pub rev_usd_30d: f64,
    /// Monthly profit (USD)
    pub profit_usd_30d: f64,
    /// Total tokens in circulation
    pub total_tokens: f64,
    /// Current token price (USD)
    pub price_now: f64,
}

impl YieldInputs {
    pub fn new(revenue: f64, profit: f64, total_tokens: f64, price: f64) -> Self {
        Self {
            rev_usd_30d: revenue,
            profit_usd_30d: profit,
            total_tokens,
            price_now: price,
        }
    }
}

/// 📈 Yield forecast results
#[derive(Clone, Debug)]
pub struct YieldForecast {
    /// Expected dividends in 30 days (USD)
    pub div_30d_usd: f64,
    /// Revenue share component (USD)
    pub revenue_share_usd: f64,
    /// Profit share component (USD)
    pub profit_share_usd: f64,
    /// Staking rewards component (USD)
    pub staking_rewards_usd: f64,
    /// Equivalent APR (annual percentage rate)
    pub apr_equiv: f64,
    /// Yield percentage for 30 days
    pub yield_30d_pct: f64,
}

impl YieldForecast {
    /// Display forecast details
    pub fn display(&self, position_symbol: &str) {
        println!("💰 **Yield Forecast for {}:**", position_symbol);
        println!("   • Total 30d Income: ${:.2}", self.div_30d_usd);
        println!("     ┌─ Revenue Share: ${:.2}", self.revenue_share_usd);
        println!("     ├─ Profit Share: ${:.2}", self.profit_share_usd);
        println!("     └─ Staking Rewards: ${:.2}", self.staking_rewards_usd);
        println!("   • 30d Yield: {:.2}%", self.yield_30d_pct);
        println!("   • Equivalent APR: {:.1}%", self.apr_equiv * 100.0);
    }
}

/// 🔮 Calculate yield forecast for a position
pub fn forecast(position: &Position, inputs: &YieldInputs) -> YieldForecast {
    // Calculate token share (position tokens / total tokens)
    let token_share = if inputs.total_tokens > 0.0 {
        position.tokens / inputs.total_tokens
    } else {
        0.0
    };

    // Revenue share calculation
    let revenue_share_usd = inputs.rev_usd_30d * position.revenue_share * token_share;

    // Profit share calculation  
    let profit_share_usd = inputs.profit_usd_30d * position.profit_share * token_share;

    // Staking rewards calculation (monthly APR)
    let monthly_apr = position.staking_apr / 12.0;
    let position_value = position.tokens * inputs.price_now;
    let staking_rewards_usd = position_value * monthly_apr;

    // Total dividends
    let div_30d_usd = revenue_share_usd + profit_share_usd + staking_rewards_usd;

    // Calculate equivalent APR
    let apr_equiv = if position_value > 0.0 {
        (div_30d_usd * 12.0) / position_value
    } else {
        0.0
    };

    // Calculate 30d yield percentage
    let yield_30d_pct = if position_value > 0.0 {
        (div_30d_usd / position_value) * 100.0
    } else {
        0.0
    };

    YieldForecast {
        div_30d_usd,
        revenue_share_usd,
        profit_share_usd,
        staking_rewards_usd,
        apr_equiv,
        yield_30d_pct,
    }
}

/// 💎 Advanced yield calculator with multiple scenarios
pub struct YieldCalculator {
    /// Base case inputs
    pub base_case: YieldInputs,
    /// Bull case (optimistic) inputs
    pub bull_case: Option<YieldInputs>,
    /// Bear case (pessimistic) inputs  
    pub bear_case: Option<YieldInputs>,
}

impl YieldCalculator {
    /// Create calculator with base case only
    pub fn new(base_case: YieldInputs) -> Self {
        Self {
            base_case,
            bull_case: None,
            bear_case: None,
        }
    }

    /// Add bull case scenario
    pub fn with_bull_case(mut self, bull_case: YieldInputs) -> Self {
        self.bull_case = Some(bull_case);
        self
    }

    /// Add bear case scenario
    pub fn with_bear_case(mut self, bear_case: YieldInputs) -> Self {
        self.bear_case = Some(bear_case);
        self
    }

    /// Calculate yield for all scenarios
    pub fn calculate_scenarios(&self, position: &Position) -> YieldScenarios {
        let base_forecast = forecast(position, &self.base_case);
        
        let bull_forecast = self.bull_case.as_ref()
            .map(|inputs| forecast(position, inputs));
            
        let bear_forecast = self.bear_case.as_ref()
            .map(|inputs| forecast(position, inputs));

        YieldScenarios {
            base: base_forecast,
            bull: bull_forecast,
            bear: bear_forecast,
        }
    }

    /// Display scenario analysis
    pub fn display_scenarios(&self, position: &Position) {
        let scenarios = self.calculate_scenarios(position);
        
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║  💎 Yield Scenario Analysis - {}                          ║", position.project_symbol);
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        // Base case
        println!("📊 **Base Case (Expected):**");
        println!("   • 30d Income: ${:.2}", scenarios.base.div_30d_usd);
        println!("   • APR: {:.1}%", scenarios.base.apr_equiv * 100.0);
        println!();

        // Bull case
        if let Some(bull) = &scenarios.bull {
            println!("🚀 **Bull Case (Optimistic):**");
            println!("   • 30d Income: ${:.2} (+{:.1}%)", 
                bull.div_30d_usd,
                ((bull.div_30d_usd / scenarios.base.div_30d_usd) - 1.0) * 100.0
            );
            println!("   • APR: {:.1}%", bull.apr_equiv * 100.0);
            println!();
        }

        // Bear case
        if let Some(bear) = &scenarios.bear {
            println!("🐻 **Bear Case (Pessimistic):**");
            println!("   • 30d Income: ${:.2} ({:.1}%)", 
                bear.div_30d_usd,
                ((bear.div_30d_usd / scenarios.base.div_30d_usd) - 1.0) * 100.0
            );
            println!("   • APR: {:.1}%", bear.apr_equiv * 100.0);
            println!();
        }

        // Income breakdown for base case
        println!("🔍 **Income Breakdown (Base Case):**");
        println!("   • Revenue Share: ${:.2} ({:.1}%)", 
            scenarios.base.revenue_share_usd,
            (scenarios.base.revenue_share_usd / scenarios.base.div_30d_usd) * 100.0
        );
        println!("   • Profit Share: ${:.2} ({:.1}%)", 
            scenarios.base.profit_share_usd,
            (scenarios.base.profit_share_usd / scenarios.base.div_30d_usd) * 100.0
        );
        println!("   • Staking Rewards: ${:.2} ({:.1}%)", 
            scenarios.base.staking_rewards_usd,
            (scenarios.base.staking_rewards_usd / scenarios.base.div_30d_usd) * 100.0
        );

        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
}

/// 📊 Multiple yield scenarios
#[derive(Clone, Debug)]
pub struct YieldScenarios {
    /// Base case forecast
    pub base: YieldForecast,
    /// Bull case forecast (optional)
    pub bull: Option<YieldForecast>,
    /// Bear case forecast (optional)
    pub bear: Option<YieldForecast>,
}

impl YieldScenarios {
    /// Get expected range of 30d income
    pub fn income_range(&self) -> (f64, f64, f64) {
        let base = self.base.div_30d_usd;
        let min = self.bear.as_ref().map(|b| b.div_30d_usd).unwrap_or(base);
        let max = self.bull.as_ref().map(|b| b.div_30d_usd).unwrap_or(base);
        (min, base, max)
    }

    /// Get expected APR range
    pub fn apr_range(&self) -> (f64, f64, f64) {
        let base = self.base.apr_equiv;
        let min = self.bear.as_ref().map(|b| b.apr_equiv).unwrap_or(base);
        let max = self.bull.as_ref().map(|b| b.apr_equiv).unwrap_or(base);
        (min, base, max)
    }
}

/// 📈 Portfolio-level yield analysis
pub fn portfolio_yield_forecast(
    positions: &[Position], 
    inputs_map: &std::collections::HashMap<String, YieldInputs>
) -> PortfolioYieldForecast {
    let mut total_30d_income = 0.0;
    let mut total_position_value = 0.0;
    let mut position_forecasts = Vec::new();

    for position in positions {
        if let Some(inputs) = inputs_map.get(&position.project_symbol) {
            let forecast = forecast(position, inputs);
            let position_value = position.tokens * inputs.price_now;
            
            total_30d_income += forecast.div_30d_usd;
            total_position_value += position_value;
            
            position_forecasts.push((position.project_symbol.clone(), forecast));
        }
    }

    let portfolio_apr = if total_position_value > 0.0 {
        (total_30d_income * 12.0) / total_position_value
    } else {
        0.0
    };

    let portfolio_yield_30d = if total_position_value > 0.0 {
        (total_30d_income / total_position_value) * 100.0
    } else {
        0.0
    };

    PortfolioYieldForecast {
        total_30d_income,
        total_position_value,
        portfolio_apr,
        portfolio_yield_30d,
        position_forecasts,
    }
}

/// 📊 Portfolio-level yield forecast
#[derive(Clone, Debug)]
pub struct PortfolioYieldForecast {
    /// Total expected 30d income across all positions
    pub total_30d_income: f64,
    /// Total value of all positions
    pub total_position_value: f64,
    /// Portfolio-level APR
    pub portfolio_apr: f64,
    /// Portfolio-level 30d yield percentage
    pub portfolio_yield_30d: f64,
    /// Individual position forecasts
    pub position_forecasts: Vec<(String, YieldForecast)>,
}

impl PortfolioYieldForecast {
    /// Display portfolio yield summary
    pub fn display(&self) {
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║  💼 Portfolio Yield Forecast                               ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        println!("📊 **Portfolio Summary:**");
        println!("   • Total Position Value: ${:.2}", self.total_position_value);
        println!("   • Expected 30d Income: ${:.2}", self.total_30d_income);
        println!("   • Portfolio Yield (30d): {:.2}%", self.portfolio_yield_30d);
        println!("   • Portfolio APR: {:.1}%", self.portfolio_apr * 100.0);
        println!();

        if !self.position_forecasts.is_empty() {
            println!("🔍 **Position Breakdown:**\n");
            
            for (symbol, forecast) in &self.position_forecasts {
                println!("   🔹 **{}**", symbol);
                println!("      • 30d Income: ${:.2}", forecast.div_30d_usd);
                println!("      • Contribution: {:.1}%", 
                    (forecast.div_30d_usd / self.total_30d_income) * 100.0
                );
                println!("      • Position APR: {:.1}%", forecast.apr_equiv * 100.0);
                println!();
            }
        }

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::investor::portfolio::Position;

    #[test]
    fn test_yield_forecast() {
        let position = Position::new("TEST".to_string(), "Test Project".to_string(), 1000.0, 2.0);
        let inputs = YieldInputs::new(100_000.0, 20_000.0, 1_000_000.0, 2.0);
        
        let forecast = forecast(&position, &inputs);
        
        assert!(forecast.div_30d_usd > 0.0);
        assert!(forecast.apr_equiv > 0.0);
    }

    #[test]
    fn test_yield_calculator() {
        let base_case = YieldInputs::new(100_000.0, 20_000.0, 1_000_000.0, 2.0);
        let bull_case = YieldInputs::new(150_000.0, 30_000.0, 1_000_000.0, 2.5);
        
        let calculator = YieldCalculator::new(base_case)
            .with_bull_case(bull_case);
        
        let position = Position::new("TEST".to_string(), "Test Project".to_string(), 1000.0, 2.0);
        let scenarios = calculator.calculate_scenarios(&position);
        
        assert!(scenarios.bull.is_some());
        assert!(scenarios.bull.unwrap().div_30d_usd > scenarios.base.div_30d_usd);
    }
}
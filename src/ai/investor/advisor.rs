//! 🎯 Investment Advisor - Capital Allocation and Recommendations
//! 
//! Smart allocation algorithms and portfolio management advice

use super::{
    opportunity::InvestmentOpportunity,
    portfolio::{Portfolio, Position},
};

/// 💰 Capital allocation recommendation
#[derive(Clone, Debug)]
pub struct Allocation {
    /// Project symbol
    pub symbol: String,
    /// Recommended USD amount to invest
    pub usd: f64,
    /// Percentage of total allocation
    pub percentage: f64,
    /// Rationale for this allocation
    pub rationale: String,
}

impl Allocation {
    pub fn new(symbol: String, usd: f64, percentage: f64, rationale: String) -> Self {
        Self {
            symbol,
            usd,
            percentage,
            rationale,
        }
    }
}

/// 🎯 Suggest capital allocations based on opportunities
pub fn suggest_allocations(
    cash: f64,
    opportunities: &[InvestmentOpportunity],
    max_positions: usize
) -> Vec<Allocation> {
    suggest_allocations_with_strategy(cash, opportunities, max_positions, &AllocationStrategy::Balanced)
}

/// 📊 Allocation strategies
#[derive(Clone, Debug)]
pub enum AllocationStrategy {
    /// Equal weight allocation
    EqualWeight,
    /// Score-weighted allocation
    Balanced,
    /// Aggressive (high-score focus)
    Aggressive,
    /// Conservative (risk-adjusted)
    Conservative,
    /// Growth-focused
    Growth,
}

/// 🎯 Advanced allocation with strategy
pub fn suggest_allocations_with_strategy(
    cash: f64,
    opportunities: &[InvestmentOpportunity],
    max_positions: usize,
    strategy: &AllocationStrategy,
) -> Vec<Allocation> {
    if cash <= 0.0 || opportunities.is_empty() {
        return Vec::new();
    }

    let top_opportunities: Vec<_> = opportunities.iter().take(max_positions).collect();

    match strategy {
        AllocationStrategy::EqualWeight => equal_weight_allocation(cash, &top_opportunities),
        AllocationStrategy::Balanced => balanced_allocation(cash, &top_opportunities),
        AllocationStrategy::Aggressive => aggressive_allocation(cash, &top_opportunities),
        AllocationStrategy::Conservative => conservative_allocation(cash, &top_opportunities),
        AllocationStrategy::Growth => growth_allocation(cash, &top_opportunities),
    }
}

/// Equal weight allocation
fn equal_weight_allocation(cash: f64, opportunities: &[&InvestmentOpportunity]) -> Vec<Allocation> {
    let per_position = cash / opportunities.len() as f64;
    let percentage = 100.0 / opportunities.len() as f64;

    opportunities.iter().map(|opp| {
        Allocation::new(
            opp.metrics.symbol.clone(),
            per_position,
            percentage,
            "Equal weight diversification".to_string(),
        )
    }).collect()
}

/// Balanced allocation (score-weighted)
fn balanced_allocation(cash: f64, opportunities: &[&InvestmentOpportunity]) -> Vec<Allocation> {
    let total_score: f64 = opportunities.iter().map(|opp| opp.score.max(1.0)).sum();

    opportunities.iter().map(|opp| {
        let weight = opp.score.max(1.0) / total_score;
        let usd = cash * weight;
        let percentage = weight * 100.0;
        
        let rationale = format!(
            "Score-weighted allocation (score: {:.1}/100)",
            opp.score
        );

        Allocation::new(opp.metrics.symbol.clone(), usd, percentage, rationale)
    }).collect()
}

/// Aggressive allocation (focus on highest scores)
fn aggressive_allocation(cash: f64, opportunities: &[&InvestmentOpportunity]) -> Vec<Allocation> {
    let squared_scores: Vec<f64> = opportunities.iter()
        .map(|opp| (opp.score.max(1.0) / 100.0).powi(2) * 100.0)
        .collect();
    
    let total_squared: f64 = squared_scores.iter().sum();

    opportunities.iter().zip(squared_scores.iter()).map(|(opp, &squared_score)| {
        let weight = squared_score / total_squared;
        let usd = cash * weight;
        let percentage = weight * 100.0;
        
        let rationale = format!(
            "Aggressive allocation - high score focus (score: {:.1}/100)",
            opp.score
        );

        Allocation::new(opp.metrics.symbol.clone(), usd, percentage, rationale)
    }).collect()
}

/// Conservative allocation (risk-adjusted)
fn conservative_allocation(cash: f64, opportunities: &[&InvestmentOpportunity]) -> Vec<Allocation> {
    let risk_adjusted_scores: Vec<f64> = opportunities.iter()
        .map(|opp| {
            let risk_factor = 1.0 - (opp.metrics.risk * 0.5); // Reduce allocation for high risk
            opp.score * risk_factor
        })
        .collect();
    
    let total_adjusted: f64 = risk_adjusted_scores.iter().sum();

    opportunities.iter().zip(risk_adjusted_scores.iter()).map(|(opp, &adj_score)| {
        let weight = adj_score / total_adjusted;
        let usd = cash * weight;
        let percentage = weight * 100.0;
        
        let rationale = format!(
            "Conservative allocation - risk-adjusted (risk: {:.1}/10)",
            opp.metrics.risk * 10.0
        );

        Allocation::new(opp.metrics.symbol.clone(), usd, percentage, rationale)
    }).collect()
}

/// Growth allocation (focus on growth metrics)
fn growth_allocation(cash: f64, opportunities: &[&InvestmentOpportunity]) -> Vec<Allocation> {
    let growth_scores: Vec<f64> = opportunities.iter()
        .map(|opp| {
            let growth_factor = (opp.metrics.sales_growth_30d + opp.metrics.orders_growth_30d) / 2.0;
            opp.score * growth_factor
        })
        .collect();
    
    let total_growth: f64 = growth_scores.iter().sum();

    opportunities.iter().zip(growth_scores.iter()).map(|(opp, &growth_score)| {
        let weight = growth_score / total_growth;
        let usd = cash * weight;
        let percentage = weight * 100.0;
        
        let rationale = format!(
            "Growth-focused allocation (sales: {:+.1}%, orders: {:+.1}%)",
            (opp.metrics.sales_growth_30d - 1.0) * 100.0,
            (opp.metrics.orders_growth_30d - 1.0) * 100.0
        );

        Allocation::new(opp.metrics.symbol.clone(), usd, percentage, rationale)
    }).collect()
}

/// 💼 Open positions based on allocations
pub fn open_positions(
    portfolio: &mut Portfolio,
    allocations: Vec<Allocation>,
    price_fn: &impl Fn(&str) -> f64,
) -> Result<Vec<String>, String> {
    let mut opened_positions = Vec::new();

    for allocation in allocations {
        let price = price_fn(&allocation.symbol);
        
        if price <= 0.0 {
            continue; // Skip if price not available
        }

        if allocation.usd <= 0.0 {
            continue; // Skip zero allocations
        }

        if portfolio.cash_usd < allocation.usd {
            return Err(format!(
                "Insufficient cash for {}: need ${:.2}, have ${:.2}",
                allocation.symbol, allocation.usd, portfolio.cash_usd
            ));
        }

        let tokens = allocation.usd / price;
        let position = Position::new(allocation.symbol.clone(), allocation.symbol.clone(), tokens, price);

        match portfolio.open_position(position) {
            Ok(()) => {
                opened_positions.push(format!(
                    "Opened {} position: {:.2} tokens at ${:.2} (${:.2} total)",
                    allocation.symbol, tokens, price, allocation.usd
                ));
            }
            Err(e) => return Err(e),
        }
    }

    Ok(opened_positions)
}

/// 🧠 Investment advisor with comprehensive analysis
#[derive(Clone, Debug)]
pub struct InvestmentAdvisor {
    strategy: AllocationStrategy,
    max_positions: usize,
    min_allocation_pct: f64,
}

impl InvestmentAdvisor {
    /// Create advisor with default settings
    pub fn new() -> Self {
        Self {
            strategy: AllocationStrategy::Balanced,
            max_positions: 5,
            min_allocation_pct: 5.0, // Minimum 5% per position
        }
    }

    /// Set allocation strategy
    pub fn with_strategy(mut self, strategy: AllocationStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Set maximum number of positions
    pub fn with_max_positions(mut self, max_positions: usize) -> Self {
        self.max_positions = max_positions;
        self
    }

    /// Set minimum allocation percentage
    pub fn with_min_allocation(mut self, min_pct: f64) -> Self {
        self.min_allocation_pct = min_pct;
        self
    }

    /// Generate investment recommendations
    pub fn recommend(&self, cash: f64, opportunities: &[InvestmentOpportunity]) -> InvestmentRecommendation {
        let allocations = suggest_allocations_with_strategy(
            cash,
            opportunities,
            self.max_positions,
            &self.strategy,
        );

        // Filter out allocations below minimum
        let filtered_allocations: Vec<_> = allocations.into_iter()
            .filter(|alloc| alloc.percentage >= self.min_allocation_pct)
            .collect();

        // Calculate metrics
        let total_allocated = filtered_allocations.iter().map(|a| a.usd).sum::<f64>();
        let remaining_cash = cash - total_allocated;
        let diversification_score = calculate_diversification_score(&filtered_allocations, opportunities);

        InvestmentRecommendation {
            allocations: filtered_allocations,
            total_cash: cash,
            total_allocated,
            remaining_cash,
            diversification_score,
            strategy: self.strategy.clone(),
        }
    }

    /// Display recommendation with analysis
    pub fn display_recommendation(&self, recommendation: &InvestmentRecommendation) {
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║  🎯 Investment Recommendation                               ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        println!("📊 **Strategy:** {:?}", recommendation.strategy);
        println!("💰 **Total Cash:** ${:.2}", recommendation.total_cash);
        println!("💼 **Total Allocated:** ${:.2} ({:.1}%)", 
            recommendation.total_allocated,
            (recommendation.total_allocated / recommendation.total_cash) * 100.0
        );
        println!("💵 **Remaining Cash:** ${:.2}", recommendation.remaining_cash);
        println!("🎲 **Diversification Score:** {:.1}/100\n", recommendation.diversification_score);

        if !recommendation.allocations.is_empty() {
            println!("🎯 **Recommended Allocations:**\n");

            for (i, allocation) in recommendation.allocations.iter().enumerate() {
                println!("   {}. **{}**", i + 1, allocation.symbol);
                println!("      • Amount: ${:.2} ({:.1}%)", allocation.usd, allocation.percentage);
                println!("      • Rationale: {}", allocation.rationale);
                println!();
            }
        }

        // Risk analysis
        let total_risk = recommendation.allocations.iter()
            .map(|alloc| alloc.percentage / 100.0)
            .sum::<f64>();

        println!("⚠️ **Risk Analysis:**");
        if total_risk > 0.8 {
            println!("   • High allocation ratio - consider keeping more cash");
        }
        if recommendation.allocations.len() < 3 {
            println!("   • Low diversification - consider more positions");
        }
        if recommendation.diversification_score < 50.0 {
            println!("   • Poor diversification across risk levels");
        }

        println!("\n💡 **Next Steps:**");
        println!("   1. Review individual company analysis");
        println!("   2. Consider your risk tolerance");
        println!("   3. Confirm allocation amounts");
        println!("   4. Execute trades in recommended order");

        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
}

impl Default for InvestmentAdvisor {
    fn default() -> Self {
        Self::new()
    }
}

/// 📋 Complete investment recommendation
#[derive(Clone, Debug)]
pub struct InvestmentRecommendation {
    /// Recommended allocations
    pub allocations: Vec<Allocation>,
    /// Total available cash
    pub total_cash: f64,
    /// Total amount to be allocated
    pub total_allocated: f64,
    /// Remaining cash after allocation
    pub remaining_cash: f64,
    /// Diversification score (0-100)
    pub diversification_score: f64,
    /// Strategy used
    pub strategy: AllocationStrategy,
}

/// Calculate diversification score based on allocations
fn calculate_diversification_score(
    allocations: &[Allocation],
    opportunities: &[InvestmentOpportunity],
) -> f64 {
    if allocations.is_empty() {
        return 0.0;
    }

    let mut score = 0.0;

    // Reward number of positions (up to optimal)
    let position_score = match allocations.len() {
        1 => 20.0,
        2 => 40.0,
        3 => 70.0,
        4 => 85.0,
        5..=7 => 100.0,
        _ => 90.0, // Too many positions
    };
    score += position_score * 0.4;

    // Reward balanced allocation (penalize concentration)
    let max_allocation = allocations.iter()
        .map(|a| a.percentage)
        .fold(0.0, f64::max);
    
    let concentration_score = if max_allocation > 50.0 {
        50.0
    } else if max_allocation > 40.0 {
        70.0
    } else if max_allocation > 30.0 {
        90.0
    } else {
        100.0
    };
    score += concentration_score * 0.3;

    // Reward risk diversification
    let risk_diversity = calculate_risk_diversity(allocations, opportunities);
    score += risk_diversity * 0.3;

    score.min(100.0)
}

/// Calculate risk diversity score
fn calculate_risk_diversity(
    allocations: &[Allocation],
    opportunities: &[InvestmentOpportunity],
) -> f64 {
    let opp_map: std::collections::HashMap<_, _> = opportunities.iter()
        .map(|opp| (&opp.metrics.symbol, opp))
        .collect();

    let risk_levels: Vec<f64> = allocations.iter()
        .filter_map(|alloc| opp_map.get(&alloc.symbol))
        .map(|opp| opp.metrics.risk)
        .collect();

    if risk_levels.is_empty() {
        return 0.0;
    }

    // Calculate standard deviation of risk levels
    let mean_risk: f64 = risk_levels.iter().sum::<f64>() / risk_levels.len() as f64;
    let variance: f64 = risk_levels.iter()
        .map(|risk| (risk - mean_risk).powi(2))
        .sum::<f64>() / risk_levels.len() as f64;
    
    let std_dev = variance.sqrt();
    
    // Convert to 0-100 score (higher std dev = better diversity)
    (std_dev * 200.0).min(100.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::investor::opportunity::CompanyMetrics;

    #[test]
    fn test_equal_weight_allocation() {
        let metrics1 = CompanyMetrics::new("A".to_string(), "Company A".to_string(), 1.0);
        let metrics2 = CompanyMetrics::new("B".to_string(), "Company B".to_string(), 2.0);
        
        let opportunities = vec![
            InvestmentOpportunity::new(metrics1, 80.0),
            InvestmentOpportunity::new(metrics2, 70.0),
        ];

        let allocations = suggest_allocations(1000.0, &opportunities, 2);
        
        assert_eq!(allocations.len(), 2);
        assert_eq!(allocations[0].usd, 500.0);
        assert_eq!(allocations[1].usd, 500.0);
    }

    #[test]
    fn test_investment_advisor() {
        let advisor = InvestmentAdvisor::new()
            .with_strategy(AllocationStrategy::Balanced)
            .with_max_positions(3);

        let metrics = CompanyMetrics::new("TEST".to_string(), "Test".to_string(), 1.0);
        let opportunities = vec![InvestmentOpportunity::new(metrics, 75.0)];

        let recommendation = advisor.recommend(1000.0, &opportunities);
        
        assert!(!recommendation.allocations.is_empty());
        assert!(recommendation.total_allocated > 0.0);
    }
}
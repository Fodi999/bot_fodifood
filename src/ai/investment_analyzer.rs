//! 💼 Investment Analyzer - AI CFO Module
//! 
//! This module provides comprehensive financial analysis for investment decisions:
//! - NPV (Net Present Value) calculation
//! - IRR (Internal Rate of Return) estimation
//! - ROI (Return on Investment) analysis
//! - Payback period calculation
//! - Investment recommendations
//! 
//! # Use Cases
//! - Restaurant expansion analysis
//! - New location investment decisions
//! - Equipment purchase evaluation
//! - Menu development ROI
//! - Marketing campaign effectiveness

use std::f64::consts::E;

/// 💰 Investment data structure
#[derive(Debug, Clone)]
pub struct InvestmentData {
    /// Initial investment amount
    pub initial_investment: f64,
    /// Projected cash flows per period (e.g., per year)
    pub cash_flows: Vec<f64>,
    /// Discount rate (e.g., 0.1 = 10%)
    pub discount_rate: f64,
}

/// 📊 Investment analysis report
#[derive(Debug, Clone)]
pub struct InvestmentReport {
    /// Net Present Value
    pub npv: f64,
    /// Internal Rate of Return
    pub irr: f64,
    /// Payback period in years
    pub payback_period: f64,
    /// Return on Investment (%)
    pub roi: f64,
    /// AI-powered recommendation
    pub recommendation: String,
    /// Risk assessment
    pub risk_level: RiskLevel,
}

/// 🎯 Risk level assessment
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl RiskLevel {
    pub fn as_emoji(&self) -> &str {
        match self {
            RiskLevel::Low => "🟢",
            RiskLevel::Medium => "🟡",
            RiskLevel::High => "🟠",
            RiskLevel::VeryHigh => "🔴",
        }
    }
    
    pub fn as_str(&self) -> &str {
        match self {
            RiskLevel::Low => "Низкий",
            RiskLevel::Medium => "Средний",
            RiskLevel::High => "Высокий",
            RiskLevel::VeryHigh => "Очень высокий",
        }
    }
}

/// 📈 Calculate NPV (Net Present Value)
/// 
/// Formula: NPV = -I₀ + Σ(CF_t / (1 + r)^t)
/// where I₀ is initial investment, CF_t is cash flow at time t, r is discount rate
pub fn calculate_npv(data: &InvestmentData) -> f64 {
    let mut npv = -data.initial_investment;
    
    for (t, &cash_flow) in data.cash_flows.iter().enumerate() {
        npv += cash_flow / (1.0 + data.discount_rate).powi((t + 1) as i32);
    }
    
    npv
}

/// 📉 Calculate IRR (Internal Rate of Return) using Newton-Raphson method
/// 
/// IRR is the discount rate that makes NPV = 0
/// Uses iterative approximation with 100 iterations
pub fn calculate_irr(data: &InvestmentData) -> f64 {
    let mut irr = 0.1; // Start with 10% guess
    
    for _ in 0..100 {
        let test_data = InvestmentData {
            initial_investment: data.initial_investment,
            cash_flows: data.cash_flows.clone(),
            discount_rate: irr,
        };
        
        let npv = calculate_npv(&test_data);
        
        if npv.abs() < 0.01 {
            return irr;
        }
        
        irr += npv.signum() * 0.001;
    }
    
    irr
}

/// 💵 Calculate ROI (Return on Investment)
/// 
/// Formula: ROI = (Total Return - Investment) / Investment
pub fn calculate_roi(data: &InvestmentData) -> f64 {
    let total_return: f64 = data.cash_flows.iter().sum();
    (total_return - data.initial_investment) / data.initial_investment
}

/// ⏳ Calculate payback period (in years)
/// 
/// Returns the time it takes for cumulative cash flows to exceed initial investment
pub fn calculate_payback_period(data: &InvestmentData) -> f64 {
    let mut cumulative = -data.initial_investment;
    
    for (t, &cf) in data.cash_flows.iter().enumerate() {
        cumulative += cf;
        if cumulative >= 0.0 {
            return (t + 1) as f64;
        }
    }
    
    data.cash_flows.len() as f64
}

/// 🎯 Assess risk level based on financial metrics
fn assess_risk(npv: f64, irr: f64, discount_rate: f64, payback: f64) -> RiskLevel {
    let irr_spread = irr - discount_rate;
    
    if npv > 50000.0 && irr_spread > 0.15 && payback < 2.0 {
        RiskLevel::Low
    } else if npv > 10000.0 && irr_spread > 0.05 && payback < 4.0 {
        RiskLevel::Medium
    } else if npv > 0.0 && irr_spread > 0.0 {
        RiskLevel::High
    } else {
        RiskLevel::VeryHigh
    }
}

/// 💼 Main investment analysis function
/// 
/// Performs comprehensive financial analysis and returns detailed report
/// 
/// # Examples
/// ```
/// let data = InvestmentData {
///     initial_investment: 50_000.0,
///     cash_flows: vec![15_000.0, 18_000.0, 22_000.0, 26_000.0, 30_000.0],
///     discount_rate: 0.1,
/// };
/// let report = analyze_investment(&data);
/// println!("NPV: ${:.2}", report.npv);
/// ```
pub fn analyze_investment(data: &InvestmentData) -> InvestmentReport {
    let npv = calculate_npv(data);
    let irr = calculate_irr(data);
    let roi = calculate_roi(data);
    let payback = calculate_payback_period(data);
    let risk = assess_risk(npv, irr, data.discount_rate, payback);
    
    let recommendation = generate_recommendation(npv, irr, data.discount_rate, roi, payback, &risk);
    
    InvestmentReport {
        npv,
        irr,
        payback_period: payback,
        roi,
        recommendation,
        risk_level: risk,
    }
}

/// 🧠 Generate AI-powered investment recommendation
fn generate_recommendation(
    npv: f64,
    irr: f64,
    discount_rate: f64,
    roi: f64,
    payback: f64,
    risk: &RiskLevel,
) -> String {
    let irr_spread = irr - discount_rate;
    
    if npv > 50000.0 && irr_spread > 0.15 && payback < 3.0 {
        format!(
            "✅ **ОТЛИЧНАЯ ИНВЕСТИЦИЯ**\n\n\
             NPV значительно положительная (${:.2}), IRR превышает ставку дисконтирования на {:.1}%, \
             окупаемость за {:.1} года. Риск: {}. \n\n\
             💡 Рекомендация: **Немедленно инвестировать**. Это высокодоходный проект с низким риском.",
            npv, irr_spread * 100.0, payback, risk.as_str()
        )
    } else if npv > 10000.0 && irr_spread > 0.05 && roi > 0.3 {
        format!(
            "✅ **ХОРОШАЯ ИНВЕСТИЦИЯ**\n\n\
             NPV положительная (${:.2}), IRR = {:.1}% (выше ставки на {:.1}%), ROI = {:.1}%. \
             Риск: {}.\n\n\
             💡 Рекомендация: **Рекомендуется к реализации** с учетом финансовых возможностей.",
            npv, irr * 100.0, irr_spread * 100.0, roi * 100.0, risk.as_str()
        )
    } else if npv > 0.0 && irr > discount_rate {
        format!(
            "⚠️ **ПРИЕМЛЕМАЯ ИНВЕСТИЦИЯ**\n\n\
             NPV слабо положительная (${:.2}), IRR = {:.1}% (чуть выше минимума). \
             Окупаемость: {:.1} года. Риск: {}.\n\n\
             💡 Рекомендация: Можно реализовать при отсутствии лучших альтернатив. \
             Рассмотрите возможность оптимизации плана.",
            npv, irr * 100.0, payback, risk.as_str()
        )
    } else if npv.abs() < 1000.0 {
        format!(
            "⚠️ **НЕЙТРАЛЬНЫЙ ПРОЕКТ**\n\n\
             NPV близка к нулю (${:.2}), IRR = {:.1}%. Риск: {}.\n\n\
             💡 Рекомендация: Проект на грани безубыточности. Требуется детальная проработка \
             и снижение издержек перед реализацией.",
            npv, irr * 100.0, risk.as_str()
        )
    } else {
        format!(
            "❌ **НЕВЫГОДНАЯ ИНВЕСТИЦИЯ**\n\n\
             NPV отрицательная (${:.2}), IRR = {:.1}% (ниже требуемой {:.1}%). Риск: {}.\n\n\
             💡 Рекомендация: **Не рекомендуется к реализации**. Проект убыточен. \
             Рассмотрите альтернативные варианты инвестирования.",
            npv, irr * 100.0, discount_rate * 100.0, risk.as_str()
        )
    }
}

/// 📊 Format report for display
pub fn format_report(data: &InvestmentData, report: &InvestmentReport) -> String {
    format!(
        "╔══════════════════════════════════════════════════════════════╗\n\
         ║  💼 AI CFO: Investment Analysis Report                      ║\n\
         ╚══════════════════════════════════════════════════════════════╝\n\
         \n\
         📥 **Исходные данные:**\n\
         • Инвестиция: ${:.2}\n\
         • Денежные потоки: {:?}\n\
         • Ставка дисконтирования: {:.1}%\n\
         \n\
         ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
         \n\
         📊 **Финансовые показатели:**\n\
         \n\
         • 💰 NPV (чистая приведенная стоимость): ${:.2}\n\
         • 📈 IRR (внутренняя норма доходности): {:.2}%\n\
         • ⏳ Срок окупаемости: {:.1} лет\n\
         • 💵 ROI (рентабельность инвестиций): {:.2}%\n\
         • {} Уровень риска: {}\n\
         \n\
         ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
         \n\
         💡 **Рекомендация AI CFO:**\n\
         \n\
         {}\n\
         \n\
         ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
        data.initial_investment,
        data.cash_flows,
        data.discount_rate * 100.0,
        report.npv,
        report.irr * 100.0,
        report.payback_period,
        report.roi * 100.0,
        report.risk_level.as_emoji(),
        report.risk_level.as_str(),
        report.recommendation
    )
}

/// 🎯 Demo function for testing
pub fn demo() {
    let data = InvestmentData {
        initial_investment: 50_000.0,
        cash_flows: vec![15_000.0, 18_000.0, 22_000.0, 26_000.0, 30_000.0],
        discount_rate: 0.1,
    };

    let report = analyze_investment(&data);
    println!("{}", format_report(&data, &report));
}

/// 🍽️ Restaurant-specific investment scenarios
pub mod scenarios {
    use super::*;
    
    /// New restaurant location
    pub fn new_location() -> InvestmentData {
        InvestmentData {
            initial_investment: 200_000.0,
            cash_flows: vec![40_000.0, 55_000.0, 70_000.0, 85_000.0, 100_000.0],
            discount_rate: 0.12,
        }
    }
    
    /// Kitchen equipment upgrade
    pub fn equipment_upgrade() -> InvestmentData {
        InvestmentData {
            initial_investment: 50_000.0,
            cash_flows: vec![15_000.0, 18_000.0, 22_000.0, 26_000.0, 30_000.0],
            discount_rate: 0.1,
        }
    }
    
    /// Menu development & marketing
    pub fn menu_development() -> InvestmentData {
        InvestmentData {
            initial_investment: 20_000.0,
            cash_flows: vec![8_000.0, 10_000.0, 12_000.0, 15_000.0, 18_000.0],
            discount_rate: 0.15,
        }
    }
    
    /// Delivery infrastructure
    pub fn delivery_infrastructure() -> InvestmentData {
        InvestmentData {
            initial_investment: 30_000.0,
            cash_flows: vec![10_000.0, 12_000.0, 14_000.0, 16_000.0, 18_000.0],
            discount_rate: 0.12,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_npv_calculation() {
        let data = InvestmentData {
            initial_investment: 10_000.0,
            cash_flows: vec![3_000.0, 4_000.0, 5_000.0],
            discount_rate: 0.1,
        };
        let npv = calculate_npv(&data);
        assert!(npv > 0.0, "NPV should be positive for profitable project");
    }
    
    #[test]
    fn test_roi_calculation() {
        let data = InvestmentData {
            initial_investment: 10_000.0,
            cash_flows: vec![5_000.0, 5_000.0, 5_000.0],
            discount_rate: 0.1,
        };
        let roi = calculate_roi(&data);
        assert_eq!(roi, 0.5, "ROI should be 50%");
    }
    
    #[test]
    fn test_payback_period() {
        let data = InvestmentData {
            initial_investment: 10_000.0,
            cash_flows: vec![3_000.0, 4_000.0, 5_000.0],
            discount_rate: 0.1,
        };
        let payback = calculate_payback_period(&data);
        assert_eq!(payback, 3.0, "Payback should be 3 years");
    }
}

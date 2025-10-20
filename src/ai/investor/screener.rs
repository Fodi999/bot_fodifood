// 🔍 Investment Screener - Скрининг инвестиционных возможностей
//
// Алгоритм скоринга, ранжирование проектов по метрикам

use super::opportunity::{CompanyMetrics, InvestmentOpportunity};

/// ⚖️ Веса для скоринга
#[derive(Clone, Debug)]
pub struct ScreenerWeights {
    /// Вес роста продаж (0.0..1.0)
    pub w_sales: f64,
    /// Вес роста заказов (0.0..1.0)
    pub w_orders: f64,
    /// Вес ROI кампаний (0.0..1.0)
    pub w_roi: f64,
    /// Вес retention (0.0..1.0)
    pub w_retention: f64,
    /// Вес маржи (0.0..1.0)
    pub w_margin: f64,
    /// Вес социальных метрик (0.0..1.0)
    pub w_social: f64,
    /// Штраф за риск (0.0..1.0)
    pub w_risk_penalty: f64,
}

impl Default for ScreenerWeights {
    fn default() -> Self {
        Self {
            w_sales: 0.22,
            w_orders: 0.18,
            w_roi: 0.18,
            w_retention: 0.16,
            w_margin: 0.14,
            w_social: 0.12,
            w_risk_penalty: 0.30,
        }
    }
}

impl ScreenerWeights {
    /// Создать веса по умолчанию
    pub fn new() -> Self {
        Self::default()
    }

    /// Установить приоритет на рост
    pub fn growth_focused() -> Self {
        Self {
            w_sales: 0.30,
            w_orders: 0.25,
            w_roi: 0.20,
            w_retention: 0.10,
            w_margin: 0.05,
            w_social: 0.10,
            w_risk_penalty: 0.25,
        }
    }

    /// Установить приоритет на стабильность
    pub fn stability_focused() -> Self {
        Self {
            w_sales: 0.15,
            w_orders: 0.15,
            w_roi: 0.20,
            w_retention: 0.25,
            w_margin: 0.20,
            w_social: 0.05,
            w_risk_penalty: 0.40,
        }
    }

    /// Установить приоритет на прибыльность
    pub fn profitability_focused() -> Self {
        Self {
            w_sales: 0.15,
            w_orders: 0.10,
            w_roi: 0.30,
            w_retention: 0.15,
            w_margin: 0.25,
            w_social: 0.05,
            w_risk_penalty: 0.35,
        }
    }
}

/// 🔍 Скринер инвестиций
pub struct InvestmentScreener {
    /// Веса для скоринга
    pub weights: ScreenerWeights,
}

impl InvestmentScreener {
    /// Создать скринер с весами по умолчанию
    pub fn new() -> Self {
        Self {
            weights: ScreenerWeights::default(),
        }
    }

    /// Создать скринер с кастомными весами
    pub fn with_weights(weights: ScreenerWeights) -> Self {
        Self { weights }
    }

    /// Нормализовать метрику в диапазон 0.0..1.0
    fn normalize(&self, value: f64, min: f64, max: f64) -> f64 {
        if max <= min {
            return 0.5;
        }
        ((value - min) / (max - min)).clamp(0.0, 1.0)
    }

    /// Рассчитать скоринг для компании
    ///
    /// Формула: 
    /// raw_score = Σ(weight_i * normalized_metric_i) - w_risk_penalty * risk
    /// final_score = raw_score * 100 (масштабирование в 0..100)
    pub fn score_company(&self, metrics: &CompanyMetrics) -> f64 {
        // Нормализуем метрики
        // Рост продаж: 0.5 (стагнация) → 2.0 (удвоение)
        let sales_norm = self.normalize(metrics.sales_growth_30d, 0.5, 2.0);
        
        // Рост заказов: 0.5 → 2.0
        let orders_norm = self.normalize(metrics.orders_growth_30d, 0.5, 2.0);
        
        // ROI: 0.0 → 3.0 (300%)
        let roi_norm = self.normalize(metrics.roi_last_campaign, 0.0, 3.0);
        
        // Retention уже в 0..1
        let retention_norm = metrics.retention_30d;
        
        // Margin: 0.0 → 0.5 (50%)
        let margin_norm = self.normalize(metrics.margin, 0.0, 0.5);
        
        // Social momentum уже в 0..1
        let social_norm = metrics.social_momentum;
        
        // Риск уже в 0..1 (но инвертируем для штрафа)
        let risk_penalty = metrics.risk;

        // Взвешенная сумма
        let raw_score = 
            self.weights.w_sales * sales_norm +
            self.weights.w_orders * orders_norm +
            self.weights.w_roi * roi_norm +
            self.weights.w_retention * retention_norm +
            self.weights.w_margin * margin_norm +
            self.weights.w_social * social_norm -
            self.weights.w_risk_penalty * risk_penalty;

        // Масштабируем в 0..100
        (raw_score * 100.0).clamp(0.0, 100.0)
    }

    /// Скринить список компаний и создать возможности
    pub fn screen(&self, companies: Vec<CompanyMetrics>) -> Vec<InvestmentOpportunity> {
        companies.into_iter()
            .map(|metrics| {
                let score = self.score_company(&metrics);
                InvestmentOpportunity::new(metrics, score)
            })
            .collect()
    }

    /// Скринить и отсортировать по скорингу
    pub fn screen_and_rank(&self, companies: Vec<CompanyMetrics>) -> Vec<InvestmentOpportunity> {
        let mut opportunities = self.screen(companies);
        opportunities.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        opportunities
    }

    /// Получить топ N возможностей
    pub fn screen_top_n(&self, companies: Vec<CompanyMetrics>, n: usize) -> Vec<InvestmentOpportunity> {
        let ranked = self.screen_and_rank(companies);
        ranked.into_iter().take(n).collect()
    }

    /// Отчет по скорингу (debug)
    pub fn score_breakdown(&self, metrics: &CompanyMetrics) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  🔍 Разбивка скоринга                                       ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        println!("🏢 Компания: {} ({})\n", metrics.name, metrics.symbol);

        let sales_norm = self.normalize(metrics.sales_growth_30d, 0.5, 2.0);
        let orders_norm = self.normalize(metrics.orders_growth_30d, 0.5, 2.0);
        let roi_norm = self.normalize(metrics.roi_last_campaign, 0.0, 3.0);
        let retention_norm = metrics.retention_30d;
        let margin_norm = self.normalize(metrics.margin, 0.0, 0.5);
        let social_norm = metrics.social_momentum;
        let risk_penalty = metrics.risk;

        println!("📊 **Компоненты скоринга:**\n");

        println!("   1. Рост продаж: {:.1}% → norm {:.2} × weight {:.2} = {:.2}",
            (metrics.sales_growth_30d - 1.0) * 100.0,
            sales_norm,
            self.weights.w_sales,
            sales_norm * self.weights.w_sales
        );

        println!("   2. Рост заказов: {:.1}% → norm {:.2} × weight {:.2} = {:.2}",
            (metrics.orders_growth_30d - 1.0) * 100.0,
            orders_norm,
            self.weights.w_orders,
            orders_norm * self.weights.w_orders
        );

        println!("   3. ROI кампаний: {:.1}% → norm {:.2} × weight {:.2} = {:.2}",
            metrics.roi_last_campaign * 100.0,
            roi_norm,
            self.weights.w_roi,
            roi_norm * self.weights.w_roi
        );

        println!("   4. Retention: {:.1}% → norm {:.2} × weight {:.2} = {:.2}",
            retention_norm * 100.0,
            retention_norm,
            self.weights.w_retention,
            retention_norm * self.weights.w_retention
        );

        println!("   5. Маржа: {:.1}% → norm {:.2} × weight {:.2} = {:.2}",
            metrics.margin * 100.0,
            margin_norm,
            self.weights.w_margin,
            margin_norm * self.weights.w_margin
        );

        println!("   6. Соцсети: {:.1}/10 → norm {:.2} × weight {:.2} = {:.2}",
            social_norm * 10.0,
            social_norm,
            self.weights.w_social,
            social_norm * self.weights.w_social
        );

        println!("\n   7. ШТРАФ за риск: {:.1}/10 × weight {:.2} = -{:.2}",
            risk_penalty * 10.0,
            self.weights.w_risk_penalty,
            risk_penalty * self.weights.w_risk_penalty
        );

        let total_score = self.score_company(metrics);
        println!("\n✨ **Итоговый скоринг: {:.1}/100**\n", total_score);

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
}

impl Default for InvestmentScreener {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        let screener = InvestmentScreener::new();
        assert_eq!(screener.normalize(0.5, 0.0, 1.0), 0.5);
        assert_eq!(screener.normalize(1.0, 0.0, 1.0), 1.0);
        assert_eq!(screener.normalize(0.0, 0.0, 1.0), 0.0);
    }

    #[test]
    fn test_score_company() {
        let screener = InvestmentScreener::new();
        let metrics = CompanyMetrics::new("TEST".to_string(), "Test".to_string(), 1.0)
            .with_growth(1.5, 1.3)
            .with_financials(0.4, 1.2)
            .with_operations(0.7, 0.2)
            .with_social(0.8);

        let score = screener.score_company(&metrics);
        assert!(score > 0.0);
        assert!(score <= 100.0);
    }

    #[test]
    fn test_screen_and_rank() {
        let screener = InvestmentScreener::new();
        
        let companies = vec![
            CompanyMetrics::new("A".to_string(), "A".to_string(), 1.0)
                .with_growth(1.2, 1.1)
                .with_financials(0.3, 0.8),
            CompanyMetrics::new("B".to_string(), "B".to_string(), 1.0)
                .with_growth(1.8, 1.5)
                .with_financials(0.4, 1.5),
        ];

        let ranked = screener.screen_and_rank(companies);
        assert!(ranked[0].score >= ranked[1].score);
    }
}

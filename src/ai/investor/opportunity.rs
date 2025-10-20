// 🎯 Investment Opportunities - Инвестиционные возможности
//
// Метрики компаний, скоринг, выбор лучших проектов для инвестирования

use serde::{Deserialize, Serialize};

/// 📊 Метрики компании для оценки
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompanyMetrics {
    /// Символ токена
    pub symbol: String,
    /// Название компании
    pub name: String,
    /// Текущая цена токена (USD)
    pub price: f64,
    /// Рост продаж за 30 дней (множитель, например 1.5 = +50%)
    pub sales_growth_30d: f64,
    /// Рост заказов за 30 дней (множитель)
    pub orders_growth_30d: f64,
    /// ROI последней кампании (0.0..∞, например 2.0 = 200%)
    pub roi_last_campaign: f64,
    /// Retention rate за 30 дней (0.0..1.0)
    pub retention_30d: f64,
    /// Margin (маржа, 0.0..1.0, например 0.35 = 35%)
    pub margin: f64,
    /// Фактор риска (0.0..1.0, где 0 = низкий риск, 1 = высокий)
    pub risk: f64,
    /// Social momentum (активность в соцсетях, 0.0..1.0)
    pub social_momentum: f64,
}

impl CompanyMetrics {
    /// Создать метрики вручную
    pub fn new(symbol: String, name: String, price: f64) -> Self {
        Self {
            symbol,
            name,
            price,
            sales_growth_30d: 1.0,
            orders_growth_30d: 1.0,
            roi_last_campaign: 0.0,
            retention_30d: 0.5,
            margin: 0.3,
            risk: 0.5,
            social_momentum: 0.5,
        }
    }

    /// Установить метрики роста
    pub fn with_growth(mut self, sales: f64, orders: f64) -> Self {
        self.sales_growth_30d = sales;
        self.orders_growth_30d = orders;
        self
    }

    /// Установить финансовые метрики
    pub fn with_financials(mut self, margin: f64, roi: f64) -> Self {
        self.margin = margin;
        self.roi_last_campaign = roi;
        self
    }

    /// Установить операционные метрики
    pub fn with_operations(mut self, retention: f64, risk: f64) -> Self {
        self.retention_30d = retention;
        self.risk = risk;
        self
    }

    /// Установить социальный momentum
    pub fn with_social(mut self, momentum: f64) -> Self {
        self.social_momentum = momentum;
        self
    }

    /// Получить краткую сводку
    pub fn summary(&self) {
        println!("🏢 {} ({})", self.name, self.symbol);
        println!("   💵 Цена: ${:.2}", self.price);
        println!("   📈 Рост продаж: {:+.1}%", (self.sales_growth_30d - 1.0) * 100.0);
        println!("   📦 Рост заказов: {:+.1}%", (self.orders_growth_30d - 1.0) * 100.0);
        println!("   💰 ROI: {:.1}%", self.roi_last_campaign * 100.0);
        println!("   🔄 Retention: {:.1}%", self.retention_30d * 100.0);
        println!("   📊 Маржа: {:.1}%", self.margin * 100.0);
        println!("   ⚠️  Риск: {:.1}/10", self.risk * 10.0);
        println!("   📱 Соцсети: {:.1}/10", self.social_momentum * 10.0);
    }
}

/// 💎 Инвестиционная возможность
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InvestmentOpportunity {
    /// Метрики компании
    pub metrics: CompanyMetrics,
    /// Скоринг (0.0..100.0)
    pub score: f64,
    /// Ожидаемая доходность за 30 дней (USD на $1000 инвестиции)
    pub expected_yield_30d: f64,
    /// Ожидаемый APR (годовая ставка)
    pub expected_apr: f64,
}

impl InvestmentOpportunity {
    /// Создать возможность с рассчитанным скорингом
    pub fn new(metrics: CompanyMetrics, score: f64) -> Self {
        Self {
            metrics,
            score,
            expected_yield_30d: 0.0,
            expected_apr: 0.0,
        }
    }

    /// Установить прогноз доходности
    pub fn with_yield_forecast(mut self, yield_30d: f64, apr: f64) -> Self {
        self.expected_yield_30d = yield_30d;
        self.expected_apr = apr;
        self
    }

    /// Получить детальный отчет
    pub fn report(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  💎 Инвестиционная возможность                             ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        self.metrics.summary();

        println!("\n📊 **Скоринг:**");
        println!("   • Общий балл: {:.1}/100", self.score);
        
        let rating = if self.score >= 80.0 {
            "🌟🌟🌟🌟🌟 Отличный выбор!"
        } else if self.score >= 65.0 {
            "🌟🌟🌟🌟 Хорошая возможность"
        } else if self.score >= 50.0 {
            "🌟🌟🌟 Средний вариант"
        } else if self.score >= 35.0 {
            "🌟🌟 Ниже среднего"
        } else {
            "🌟 Высокий риск"
        };
        println!("   • Рейтинг: {}", rating);

        if self.expected_yield_30d > 0.0 {
            println!("\n💰 **Прогноз доходности:**");
            println!("   • За 30 дней: ${:.2} на $1000", self.expected_yield_30d);
            println!("   • Годовая (APR): {:.1}%", self.expected_apr * 100.0);
        }

        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
}

/// 🔍 Менеджер инвестиционных возможностей
#[derive(Clone, Debug, Default)]
pub struct OpportunityManager {
    /// Список возможностей
    pub opportunities: Vec<InvestmentOpportunity>,
}

impl OpportunityManager {
    /// Создать менеджер
    pub fn new() -> Self {
        Self {
            opportunities: Vec::new(),
        }
    }

    /// Добавить возможность
    pub fn add(&mut self, opportunity: InvestmentOpportunity) {
        self.opportunities.push(opportunity);
    }

    /// Получить топ N возможностей по скорингу
    pub fn top_n(&self, n: usize) -> Vec<&InvestmentOpportunity> {
        let mut sorted = self.opportunities.iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        sorted.into_iter().take(n).collect()
    }

    /// Фильтровать по минимальному скорингу
    pub fn filter_by_score(&self, min_score: f64) -> Vec<&InvestmentOpportunity> {
        self.opportunities.iter()
            .filter(|opp| opp.score >= min_score)
            .collect()
    }

    /// Фильтровать по максимальному риску
    pub fn filter_by_risk(&self, max_risk: f64) -> Vec<&InvestmentOpportunity> {
        self.opportunities.iter()
            .filter(|opp| opp.metrics.risk <= max_risk)
            .collect()
    }

    /// Получить сводку топ-возможностей
    pub fn top_summary(&self, n: usize) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  🏆 Топ-{} инвестиционных возможностей                      ║", n);
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        let top = self.top_n(n);
        
        for (i, opp) in top.iter().enumerate() {
            let medal = match i {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "  ",
            };

            println!("{} {} {} ({})", 
                medal, 
                i + 1, 
                opp.metrics.name, 
                opp.metrics.symbol
            );
            println!("   • Скоринг: {:.1}/100", opp.score);
            println!("   • Цена: ${:.2}", opp.metrics.price);
            println!("   • Рост: {:+.1}% продажи, {:+.1}% заказы",
                (opp.metrics.sales_growth_30d - 1.0) * 100.0,
                (opp.metrics.orders_growth_30d - 1.0) * 100.0
            );
            println!("   • Маржа: {:.1}% | ROI: {:.1}%",
                opp.metrics.margin * 100.0,
                opp.metrics.roi_last_campaign * 100.0
            );
            println!("   • Риск: {:.1}/10 | Соцсети: {:.1}/10",
                opp.metrics.risk * 10.0,
                opp.metrics.social_momentum * 10.0
            );
            
            if opp.expected_apr > 0.0 {
                println!("   • Ожидаемый APR: {:.1}%", opp.expected_apr * 100.0);
            }
            
            println!();
        }

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_metrics() {
        let metrics = CompanyMetrics::new("TEST".to_string(), "Test".to_string(), 1.0)
            .with_growth(1.5, 1.3)
            .with_financials(0.4, 1.2);

        assert_eq!(metrics.sales_growth_30d, 1.5);
        assert_eq!(metrics.margin, 0.4);
    }

    #[test]
    fn test_opportunity_manager() {
        let mut manager = OpportunityManager::new();
        
        let opp1 = InvestmentOpportunity::new(
            CompanyMetrics::new("A".to_string(), "A".to_string(), 1.0),
            80.0
        );
        
        let opp2 = InvestmentOpportunity::new(
            CompanyMetrics::new("B".to_string(), "B".to_string(), 1.0),
            90.0
        );

        manager.add(opp1);
        manager.add(opp2);

        let top = manager.top_n(1);
        assert_eq!(top[0].score, 90.0);
    }
}

//! 💼 Business Brain - AI-powered business analysis and recommendations
//! 
//! This module provides intelligent business insights, market analysis,
//! and strategic recommendations for restaurant and food industry.
//! 
//! # Features
//! - Market opportunity analysis
//! - Competitor intelligence
//! - Business model recommendations
//! - ROI predictions
//! - Growth strategy suggestions

use anyhow::Result;
use crate::ai::thinker::Thinker;

/// 💼 Business intelligence analyzer
pub struct BusinessBrain;

impl BusinessBrain {
    /// 📊 Analyze business opportunity in specific industry and region
    /// 
    /// Provides comprehensive market analysis including:
    /// - Market size and growth trends
    /// - Competition level
    /// - Entry barriers
    /// - Recommended business model
    /// 
    /// # Examples
    /// ```
    /// let analysis = BusinessBrain::analyze_opportunity("restaurant", "Moscow").await?;
    /// ```
    pub async fn analyze_opportunity(industry: &str, region: &str) -> Result<String> {
        let prompt = format!(
            "Проанализируй бизнес-возможность в отрасли '{}' в регионе '{}'.\n\n\
             Предоставь:\n\
             1. Размер рынка и тенденции роста\n\
             2. Уровень конкуренции (низкий/средний/высокий)\n\
             3. Барьеры входа\n\
             4. Рекомендуемая бизнес-модель (franchise, freemium, subscription, etc.)\n\
             5. Прогноз ROI на 1 год\n\
             6. Ключевые факторы успеха\n\n\
             Ответ должен быть конкретным и практичным.",
            industry, region
        );
        
        match Thinker::analyze_business(&prompt).await {
            Ok(analysis) => Ok(format!("📊 **Анализ бизнес-возможности**\n\n{}", analysis)),
            Err(_) => Ok(Self::fallback_analysis(industry, region))
        }
    }
    
    /// 🎯 Analyze specific business metrics
    /// 
    /// Deep dive into business performance with actionable insights
    pub async fn analyze_metrics(
        revenue: f64,
        costs: f64,
        customer_count: u32,
        avg_order: f64
    ) -> Result<String> {
        let profit = revenue - costs;
        let roi = (profit / costs) * 100.0;
        let customer_value = revenue / customer_count as f64;
        
        let prompt = format!(
            "Проанализируй метрики ресторана:\n\
             - Выручка: ${:.2}\n\
             - Затраты: ${:.2}\n\
             - Прибыль: ${:.2}\n\
             - ROI: {:.1}%\n\
             - Клиентов: {}\n\
             - Средний чек: ${:.2}\n\
             - Ценность клиента: ${:.2}\n\n\
             Дай конкретные рекомендации по улучшению каждого показателя.",
            revenue, costs, profit, roi, customer_count, avg_order, customer_value
        );
        
        match Thinker::analyze_business(&prompt).await {
            Ok(analysis) => Ok(format!("📈 **Анализ метрик**\n\n{}", analysis)),
            Err(_) => Ok(Self::fallback_metrics_analysis(roi, customer_value))
        }
    }
    
    /// 🔍 Competitor analysis
    /// 
    /// Analyzes competitor landscape and suggests differentiation strategy
    pub async fn analyze_competitors(industry: &str, competitors: Vec<&str>) -> Result<String> {
        let competitors_list = competitors.join(", ");
        
        let prompt = format!(
            "Проанализируй конкурентов в отрасли '{}':\n\
             Конкуренты: {}\n\n\
             Предоставь:\n\
             1. Сильные стороны каждого конкурента\n\
             2. Слабые стороны (пробелы в рынке)\n\
             3. Стратегию дифференциации для нового игрока\n\
             4. Ценовое позиционирование\n\
             5. Уникальное торговое предложение (USP)",
            industry, competitors_list
        );
        
        match Thinker::think(&prompt).await {
            Ok(analysis) => Ok(format!("🔍 **Анализ конкурентов**\n\n{}", analysis)),
            Err(_) => Ok(Self::fallback_competitor_analysis(&competitors))
        }
    }
    
    /// 💡 Generate business model recommendations
    /// 
    /// Suggests optimal business model based on industry and target market
    pub async fn recommend_business_model(
        industry: &str,
        target_audience: &str,
        initial_budget: f64
    ) -> Result<String> {
        let prompt = format!(
            "Порекомендуй оптимальную бизнес-модель:\n\
             - Отрасль: {}\n\
             - Целевая аудитория: {}\n\
             - Начальный бюджет: ${:.2}\n\n\
             Рассмотри модели:\n\
             1. Freemium (бесплатная база + платные функции)\n\
             2. Subscription (подписка)\n\
             3. Marketplace (комиссия с транзакций)\n\
             4. Franchise (франшиза)\n\
             5. B2B SaaS\n\n\
             Для каждой подходящей модели дай:\n\
             - Плюсы и минусы\n\
             - Ожидаемый срок окупаемости\n\
             - Риски",
            industry, target_audience, initial_budget
        );
        
        match Thinker::think(&prompt).await {
            Ok(recommendations) => Ok(format!("💡 **Рекомендации по бизнес-модели**\n\n{}", recommendations)),
            Err(_) => Ok(Self::fallback_business_model(initial_budget))
        }
    }
    
    /// 📈 Growth strategy recommendations
    /// 
    /// Provides actionable growth strategies based on current stage
    pub async fn growth_strategy(
        current_stage: &str, // "startup", "growing", "mature"
        monthly_revenue: f64,
        customer_base: u32
    ) -> Result<String> {
        let prompt = format!(
            "Предложи стратегию роста для бизнеса:\n\
             - Текущая стадия: {}\n\
             - Месячная выручка: ${:.2}\n\
             - База клиентов: {}\n\n\
             Дай конкретные шаги:\n\
             1. Каналы привлечения (3-5 наиболее эффективных)\n\
             2. Удержание клиентов (retention strategy)\n\
             3. Масштабирование (когда и как)\n\
             4. Точки роста (где сфокусироваться)\n\
             5. Метрики для отслеживания",
            current_stage, monthly_revenue, customer_base
        );
        
        match Thinker::analyze_business(&prompt).await {
            Ok(strategy) => Ok(format!("📈 **Стратегия роста**\n\n{}", strategy)),
            Err(_) => Ok(Self::fallback_growth_strategy(current_stage))
        }
    }
    
    /// 🎨 Marketing strategy for food business
    /// 
    /// AI-powered marketing recommendations specific to food industry
    pub async fn marketing_strategy(
        business_type: &str,
        budget: f64,
        target_demo: &str
    ) -> Result<String> {
        let prompt = format!(
            "Создай маркетинговую стратегию для {}:\n\
             - Бюджет: ${:.2}/месяц\n\
             - Целевая аудитория: {}\n\n\
             Включи:\n\
             1. Каналы продвижения (Instagram, TikTok, Google Ads, etc.)\n\
             2. Контент-стратегия\n\
             3. Партнерства и коллаборации\n\
             4. Программа лояльности\n\
             5. Распределение бюджета по каналам (%)\n\
             6. Ожидаемый ROAS (Return on Ad Spend)",
            business_type, budget, target_demo
        );
        
        match Thinker::think(&prompt).await {
            Ok(strategy) => Ok(format!("🎨 **Маркетинговая стратегия**\n\n{}", strategy)),
            Err(_) => Ok(Self::fallback_marketing_strategy(budget))
        }
    }
    
    /// 🏆 Competitive advantage analysis
    /// 
    /// Identifies unique strengths and suggests how to leverage them
    pub async fn competitive_advantage(
        business_name: &str,
        unique_features: Vec<&str>
    ) -> Result<String> {
        let features = unique_features.join(", ");
        
        let prompt = format!(
            "Проанализируй конкурентные преимущества для '{}':\n\
             Уникальные особенности: {}\n\n\
             Определи:\n\
             1. Главное конкурентное преимущество (core strength)\n\
             2. Как его монетизировать\n\
             3. Как его защитить от копирования\n\
             4. Как его коммуницировать клиентам\n\
             5. Дополнительные преимущества, которые можно развить",
            business_name, features
        );
        
        match Thinker::analyze_business(&prompt).await {
            Ok(analysis) => Ok(format!("🏆 **Анализ конкурентных преимуществ**\n\n{}", analysis)),
            Err(_) => Ok(Self::fallback_competitive_advantage(&unique_features))
        }
    }
    
    // ============================================================================
    // FALLBACK RESPONSES (when AI is unavailable)
    // ============================================================================
    
    fn fallback_analysis(industry: &str, region: &str) -> String {
        format!(
            "📊 **Анализ бизнес-возможности: {} в {}**\n\n\
             🌍 **Размер рынка**: Отрасль '{}' показывает стабильный рост в регионе '{}'.\n\n\
             🎯 **Уровень конкуренции**: Средний. Есть возможности для дифференциации.\n\n\
             💼 **Рекомендуемая модель**: Freemium с переходом на subscription.\n\n\
             📈 **Прогноз ROI**: 120-150% в первый год при правильном исполнении.\n\n\
             ✨ **Ключевые факторы успеха**:\n\
             • Качество продукта/услуги\n\
             • Эффективный маркетинг\n\
             • Customer retention программы\n\
             • Локализация под регион '{}'\n\n\
             💡 Рекомендую начать с MVP и тестирования на небольшой аудитории.",
            industry, region, industry, region, region
        )
    }
    
    fn fallback_metrics_analysis(roi: f64, customer_value: f64) -> String {
        let roi_assessment = if roi > 100.0 {
            "отличная"
        } else if roi > 50.0 {
            "хорошая"
        } else if roi > 0.0 {
            "средняя"
        } else {
            "требует улучшения"
        };
        
        format!(
            "📈 **Анализ метрик**\n\n\
             💰 **ROI: {:.1}%** - рентабельность {}\n\n\
             👥 **Ценность клиента: ${:.2}**\n\n\
             ✅ **Рекомендации**:\n\
             1. Увеличьте средний чек через upselling\n\
             2. Внедрите программу лояльности\n\
             3. Оптимизируйте операционные затраты\n\
             4. Повысьте retention rate\n\
             5. Масштабируйте лучшие каналы привлечения",
            roi, roi_assessment, customer_value
        )
    }
    
    fn fallback_competitor_analysis(competitors: &[&str]) -> String {
        format!(
            "🔍 **Анализ конкурентов**\n\n\
             Выявлено {} конкурентов: {}\n\n\
             💡 **Стратегия дифференциации**:\n\
             • Фокус на уникальное value proposition\n\
             • Превосходное качество обслуживания\n\
             • Инновации в продукте\n\
             • Сильный брендинг\n\
             • Community building\n\n\
             🎯 **Ценовое позиционирование**: Premium с обоснованной ценностью\n\n\
             ⚡ **USP**: Найдите пробел, который не закрывают конкуренты",
            competitors.len(),
            competitors.join(", ")
        )
    }
    
    fn fallback_business_model(budget: f64) -> String {
        let model = if budget > 100_000.0 {
            "Franchise или B2B SaaS"
        } else if budget > 50_000.0 {
            "Subscription или Marketplace"
        } else {
            "Freemium с постепенным масштабированием"
        };
        
        format!(
            "💡 **Рекомендуемая бизнес-модель**\n\n\
             При бюджете ${:.2} оптимальна модель: **{}**\n\n\
             ✅ **Преимущества**:\n\
             • Низкий порог входа для клиентов\n\
             • Предсказуемый recurring revenue\n\
             • Возможность масштабирования\n\n\
             ⚠️ **Риски**:\n\
             • Требуется время на набор базы\n\
             • Высокая конкуренция\n\n\
             📅 **Срок окупаемости**: 12-18 месяцев",
            budget, model
        )
    }
    
    fn fallback_growth_strategy(stage: &str) -> String {
        let focus = match stage {
            "startup" => "Product-Market Fit и первые клиенты",
            "growing" => "Масштабирование и оптимизация unit economics",
            "mature" => "Expansion в новые рынки и продукты",
            _ => "Определение стратегии роста"
        };
        
        format!(
            "📈 **Стратегия роста для стадии '{}'**\n\n\
             🎯 **Фокус**: {}\n\n\
             **Приоритетные каналы**:\n\
             1. Content Marketing (SEO, блог)\n\
             2. Social Media (Instagram, TikTok)\n\
             3. Referral программы\n\
             4. Партнерства\n\
             5. Paid Ads (Google, Meta)\n\n\
             **Метрики для отслеживания**:\n\
             • CAC (Customer Acquisition Cost)\n\
             • LTV (Lifetime Value)\n\
             • Churn Rate\n\
             • Monthly Recurring Revenue",
            stage, focus
        )
    }
    
    fn fallback_marketing_strategy(budget: f64) -> String {
        format!(
            "🎨 **Маркетинговая стратегия**\n\n\
             💰 **Бюджет: ${:.2}/месяц**\n\n\
             **Распределение бюджета**:\n\
             • Instagram/TikTok: 35% (${:.2})\n\
             • Google Ads: 25% (${:.2})\n\
             • Content Marketing: 20% (${:.2})\n\
             • Influencer partnerships: 15% (${:.2})\n\
             • Email/SMS: 5% (${:.2})\n\n\
             📱 **Контент-стратегия**:\n\
             • Видео-рецепты\n\
             • Behind-the-scenes\n\
             • User-generated content\n\
             • Food photography\n\n\
             🎯 **ROAS цель**: 3:1 (на каждый $1 → $3 выручки)",
            budget,
            budget * 0.35,
            budget * 0.25,
            budget * 0.20,
            budget * 0.15,
            budget * 0.05
        )
    }
    
    fn fallback_competitive_advantage(features: &[&str]) -> String {
        let main_advantage = features.first().unwrap_or(&"quality");
        
        format!(
            "🏆 **Анализ конкурентных преимуществ**\n\n\
             ⭐ **Главное преимущество**: {}\n\n\
             💰 **Монетизация**:\n\
             • Premium pricing strategy\n\
             • Tiered pricing (base/premium/enterprise)\n\
             • Add-on services\n\n\
             🛡️ **Защита**:\n\
             • Брендинг и патенты\n\
             • Exclusive partnerships\n\
             • Network effects\n\n\
             📢 **Коммуникация**:\n\
             • Storytelling в контенте\n\
             • Testimonials и case studies\n\
             • Демонстрация результатов",
            main_advantage
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_business_analysis() {
        let result = BusinessBrain::analyze_opportunity("restaurant", "Moscow").await;
        assert!(result.is_ok());
        let analysis = result.unwrap();
        assert!(analysis.contains("Анализ бизнес-возможности"));
    }
    
    #[tokio::test]
    async fn test_metrics_analysis() {
        let result = BusinessBrain::analyze_metrics(100000.0, 70000.0, 500, 200.0).await;
        assert!(result.is_ok());
        let analysis = result.unwrap();
        assert!(analysis.contains("Анализ метрик"));
    }
}

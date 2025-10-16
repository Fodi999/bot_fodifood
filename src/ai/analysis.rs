use crate::services::go_client::BusinessMetrics;

/// 💡 Анализ метрик бизнеса с AI-рекомендациями
pub fn analyze_metrics(m: &BusinessMetrics) -> String {
    let trend = if m.price_change > 20.0 {
        "🚀 Цена растёт — бизнес в фазе роста!"
    } else if m.price_change < -10.0 {
        "📉 Цена падает — стоит быть осторожнее."
    } else {
        "⚖️ Стабильный рынок — можно держать позицию."
    };

    let investor_sentiment = if m.total_investors > 100 {
        "👥 Высокий интерес инвесторов!"
    } else if m.total_investors > 50 {
        "👥 Средний интерес инвесторов."
    } else {
        "👥 Низкий интерес инвесторов — возможно, ранняя стадия."
    };

    let roi_analysis = if m.avg_investor_roi > 50.0 {
        "💰 Отличная доходность! Инвесторы в плюсе."
    } else if m.avg_investor_roi > 0.0 {
        "💵 Положительная доходность."
    } else {
        "⚠️ Средняя доходность отрицательная."
    };

    format!(
        "💡 Анализ по токену {}:\n\
         \n\
         📊 Основные показатели:\n\
         • Текущая цена: ${:.2}\n\
         • Изменение цены: {:.1}%\n\
         • Инвесторов: {}\n\
         • Рыночная капитализация: ${:.2}\n\
         • ROI бизнеса: {:.1}%\n\
         • Средний ROI инвесторов: {:.1}%\n\
         \n\
         🎯 Выводы:\n\
         {}\n\
         {}\n\
         {}\n",
        m.token_symbol,
        m.current_price,
        m.price_change,
        m.total_investors,
        m.market_cap,
        m.roi,
        m.avg_investor_roi,
        trend,
        investor_sentiment,
        roi_analysis
    )
}

/// 📈 Рекомендация по инвестированию
pub fn investment_recommendation(m: &BusinessMetrics) -> String {
    let score = calculate_investment_score(m);

    let recommendation = match score {
        90..=100 => {
            "🟢 СИЛЬНАЯ ПОКУПКА\n\
             Отличные показатели роста и доходности. Рекомендуется инвестировать."
        }
        70..=89 => {
            "🟢 ПОКУПКА\n\
             Хорошие показатели, бизнес стабильно растёт."
        }
        50..=69 => {
            "🟡 ДЕРЖАТЬ\n\
             Стабильные показатели, но без явного роста."
        }
        30..=49 => {
            "🟠 ОСТОРОЖНО\n\
             Слабые показатели, рекомендуется подождать улучшения."
        }
        _ => {
            "🔴 НЕ РЕКОМЕНДУЕТСЯ\n\
             Неблагоприятные показатели для инвестиций."
        }
    };

    format!(
        "📊 Инвестиционная оценка: {}/100\n\
         \n\
         {}\n\
         \n\
         💡 Факторы:\n\
         • Динамика цены: {}\n\
         • Интерес инвесторов: {}\n\
         • Доходность: {}\n",
        score,
        recommendation,
        price_factor_analysis(m.price_change),
        investor_factor_analysis(m.total_investors),
        roi_factor_analysis(m.avg_investor_roi)
    )
}

/// 🔢 Расчёт инвестиционного балла (0-100)
fn calculate_investment_score(m: &BusinessMetrics) -> u8 {
    let mut score = 50; // Базовый балл

    // Фактор роста цены (±30 баллов)
    score = if m.price_change > 50.0 {
        score + 30
    } else if m.price_change > 20.0 {
        score + 20
    } else if m.price_change > 5.0 {
        score + 10
    } else if m.price_change < -20.0 {
        score - 30
    } else if m.price_change < -10.0 {
        score - 20
    } else {
        score
    };

    // Фактор количества инвесторов (±20 баллов)
    score = if m.total_investors > 200 {
        score + 20
    } else if m.total_investors > 100 {
        score + 15
    } else if m.total_investors > 50 {
        score + 10
    } else if m.total_investors < 10 {
        score - 10
    } else {
        score
    };

    // Фактор доходности (±30 баллов)
    score = if m.avg_investor_roi > 100.0 {
        score + 30
    } else if m.avg_investor_roi > 50.0 {
        score + 20
    } else if m.avg_investor_roi > 20.0 {
        score + 10
    } else if m.avg_investor_roi < -20.0 {
        score - 30
    } else if m.avg_investor_roi < 0.0 {
        score - 15
    } else {
        score
    };

    score.clamp(0, 100)
}

/// 📊 Анализ фактора цены
fn price_factor_analysis(price_change: f64) -> &'static str {
    if price_change > 50.0 {
        "Взрывной рост 🚀"
    } else if price_change > 20.0 {
        "Сильный рост 📈"
    } else if price_change > 5.0 {
        "Умеренный рост ↗️"
    } else if price_change > -5.0 {
        "Стабильность ➡️"
    } else if price_change > -20.0 {
        "Умеренное снижение ↘️"
    } else {
        "Сильное падение 📉"
    }
}

/// 👥 Анализ фактора инвесторов
fn investor_factor_analysis(total_investors: i64) -> &'static str {
    if total_investors > 200 {
        "Очень высокий 🔥"
    } else if total_investors > 100 {
        "Высокий ✨"
    } else if total_investors > 50 {
        "Средний ⭐"
    } else if total_investors > 20 {
        "Низкий 💫"
    } else {
        "Очень низкий ⚠️"
    }
}

/// 💰 Анализ фактора доходности
fn roi_factor_analysis(avg_roi: f64) -> &'static str {
    if avg_roi > 100.0 {
        "Отличная 💎"
    } else if avg_roi > 50.0 {
        "Хорошая 💰"
    } else if avg_roi > 20.0 {
        "Средняя 💵"
    } else if avg_roi > 0.0 {
        "Низкая 💸"
    } else {
        "Отрицательная ⚠️"
    }
}

/// 📊 Сравнительный анализ нескольких бизнесов
pub fn compare_businesses(businesses: Vec<(&str, &BusinessMetrics)>) -> String {
    if businesses.is_empty() {
        return "❌ Нет данных для сравнения.".to_string();
    }

    let mut result = String::from("📊 Сравнительный анализ бизнесов:\n\n");

    // Находим лучший по каждому показателю
    let best_price_change = businesses
        .iter()
        .max_by(|a, b| a.1.price_change.partial_cmp(&b.1.price_change).unwrap());
    let best_investors = businesses
        .iter()
        .max_by(|a, b| a.1.total_investors.cmp(&b.1.total_investors));
    let best_roi = businesses
        .iter()
        .max_by(|a, b| a.1.avg_investor_roi.partial_cmp(&b.1.avg_investor_roi).unwrap());

    result.push_str("🏆 Лидеры по показателям:\n");
    if let Some((name, m)) = best_price_change {
        result.push_str(&format!("• Рост цены: {} ({:.1}%)\n", name, m.price_change));
    }
    if let Some((name, m)) = best_investors {
        result.push_str(&format!("• Инвесторы: {} ({} чел.)\n", name, m.total_investors));
    }
    if let Some((name, m)) = best_roi {
        result.push_str(&format!("• ROI: {} ({:.1}%)\n", name, m.avg_investor_roi));
    }

    result.push_str("\n📈 Детальное сравнение:\n\n");

    for (name, metrics) in businesses.iter() {
        let score = calculate_investment_score(metrics);
        result.push_str(&format!(
            "🏢 {}\n\
             • Балл: {}/100\n\
             • Цена: ${:.2} ({:+.1}%)\n\
             • Инвесторы: {}\n\
             • ROI: {:.1}%\n\n",
            name, score, metrics.current_price, metrics.price_change, metrics.total_investors, metrics.avg_investor_roi
        ));
    }

    result
}

/// 🎯 Краткая сводка по бизнесу (для быстрого ответа)
pub fn quick_summary(m: &BusinessMetrics) -> String {
    let emoji = if m.price_change > 10.0 {
        "🟢"
    } else if m.price_change < -10.0 {
        "🔴"
    } else {
        "🟡"
    };

    format!(
        "{} {} - ${:.2} ({:+.1}%) | {} инвесторов | ROI: {:.1}%",
        emoji, m.token_symbol, m.current_price, m.price_change, m.total_investors, m.avg_investor_roi
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_metrics() -> BusinessMetrics {
        BusinessMetrics {
            token_symbol: "TEST".to_string(),
            current_price: 100.0,
            price_change: 25.0,
            total_investors: 150,
            market_cap: 1_000_000.0,
            roi: 30.0,
            avg_investor_roi: 45.0,
        }
    }

    #[test]
    fn test_analyze_metrics() {
        let metrics = create_test_metrics();
        let analysis = analyze_metrics(&metrics);
        assert!(analysis.contains("TEST"));
        assert!(analysis.contains("$100.00"));
    }

    #[test]
    fn test_investment_score() {
        let metrics = create_test_metrics();
        let score = calculate_investment_score(&metrics);
        assert!(score > 50); // Good metrics should score above 50
    }

    #[test]
    fn test_quick_summary() {
        let metrics = create_test_metrics();
        let summary = quick_summary(&metrics);
        assert!(summary.contains("TEST"));
        assert!(summary.contains("🟢")); // Price change > 10%
    }
}

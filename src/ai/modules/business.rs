use crate::ai::analysis::{analyze_metrics, investment_recommendation, quick_summary};
use crate::ai::intent_handler::{Context, IntentHandler};
use crate::services::{fetch_business_metrics, fetch_businesses};
use crate::state::AppState;
use async_trait::async_trait;

/// 📊 Обработчик анализа бизнеса
pub struct AnalyzeBusinessHandler;

#[async_trait]
impl IntentHandler for AnalyzeBusinessHandler {
    fn name(&self) -> &'static str {
        "analyzebusiness"
    }

    fn priority(&self) -> u8 {
        80 // Высокий приоритет для специализированного запроса
    }

    async fn handle(&self, input: &str, ctx: &mut Context, state: &AppState) -> Option<String> {
        tracing::info!("📊 Handling business analysis request for user: {}", ctx.user_id);

        // Пытаемся извлечь название или ID бизнеса из запроса
        let business_query = extract_business_name(input);

        // Если не указан конкретный бизнес, показываем список
        if business_query.is_empty() {
            return handle_list_businesses(state).await;
        }

        // Ищем бизнес по названию или ID
        match find_business(&business_query, state).await {
            Ok(Some((business_id, business_name))) => {
                tracing::info!("🔍 Found business: {} ({})", business_name, business_id);

                // Получаем метрики
                match fetch_business_metrics(&business_id).await {
                    Ok(metrics) => {
                        tracing::info!("✅ Metrics fetched for: {}", business_name);

                        // Генерируем полный анализ
                        let analysis = format!(
                            "🏢 **{}**\n\n{}\n\n{}{}",
                            business_name,
                            quick_summary(&metrics),
                            analyze_metrics(&metrics),
                            investment_recommendation(&metrics)
                        );

                        Some(analysis)
                    }
                    Err(e) => {
                        tracing::error!("❌ Failed to fetch metrics: {}", e);
                        Some(format!(
                            "❌ Не удалось получить метрики для бизнеса '{}'.\n\
                             Ошибка: {}",
                            business_name, e
                        ))
                    }
                }
            }
            Ok(None) => {
                tracing::warn!("❌ Business not found: {}", business_query);
                Some(format!(
                    "❌ Бизнес '{}' не найден.\n\n\
                     💡 Попробуйте:\n\
                     • Проверить название\n\
                     • Спросить 'покажи все бизнесы'\n\
                     • Указать точное название из списка",
                    business_query
                ))
            }
            Err(e) => {
                tracing::error!("❌ Error searching businesses: {}", e);
                Some(format!(
                    "❌ Ошибка при поиске бизнеса: {}\n\n\
                     Попробуйте позже или обратитесь в поддержку.",
                    e
                ))
            }
        }
    }
}

/// Извлечь название бизнеса из запроса
fn extract_business_name(input: &str) -> String {
    let input_lower = input.to_lowercase();

    // Убираем стоп-слова
    let stop_words = [
        "проанализируй",
        "анализ",
        "бизнес",
        "бизнеса",
        "покажи",
        "метрики",
        "для",
        "по",
        "analyze",
        "business",
        "show",
        "metrics",
        "for",
        // Для BusinessInsights
        "советы",
        "как",
        "улучшить",
        "рекомендации",
        "что",
        "делать",
        "insights",
        "advice",
        "recommendations",
        "improve",
    ];

    let words: Vec<&str> = input_lower
        .split_whitespace()
        .filter(|w| !stop_words.contains(w))
        .collect();

    words.join(" ").trim().to_string()
}

/// Показать список всех бизнесов
async fn handle_list_businesses(_state: &AppState) -> Option<String> {
    match fetch_businesses().await {
        Ok(businesses) if !businesses.is_empty() => {
            let mut result = String::from(
                "🏢 **Доступные бизнесы для анализа:**\n\n\
                 Выберите бизнес для получения детального анализа:\n\n",
            );

            for (i, business) in businesses.iter().enumerate().take(10) {
                result.push_str(&format!(
                    "{}. **{}**\n   • Категория: {}\n   • Город: {}\n   • Статус: {}\n\n",
                    i + 1,
                    business.name,
                    business.category.as_deref().unwrap_or("не указана"),
                    business.city.as_deref().unwrap_or("не указан"),
                    if business.is_active { "активен" } else { "неактивен" }
                ));
            }

            result.push_str(
                "💡 Для анализа скажите: 'проанализируй бизнес [название]'\n\
                 Например: 'проанализируй бизнес Tech Startup'",
            );

            Some(result)
        }
        Ok(_) => Some(
            "❌ Список бизнесов пуст.\n\n\
             Создайте бизнесы через админ-панель."
                .to_string(),
        ),
        Err(e) => {
            tracing::error!("❌ Failed to fetch businesses: {}", e);
            Some(format!(
                "❌ Не удалось получить список бизнесов: {}\n\n\
                 💡 Убедитесь, что Go backend запущен.",
                e
            ))
        }
    }
}

/// Найти бизнес по названию или ID
async fn find_business(
    query: &str,
    _state: &AppState,
) -> anyhow::Result<Option<(String, String)>> {
    let businesses = fetch_businesses().await?;

    // Поиск по точному совпадению
    if let Some(business) = businesses
        .iter()
        .find(|b| b.name.to_lowercase() == query.to_lowercase())
    {
        return Ok(Some((business.id.clone(), business.name.clone())));
    }

    // Поиск по частичному совпадению
    if let Some(business) = businesses
        .iter()
        .find(|b| b.name.to_lowercase().contains(query))
    {
        return Ok(Some((business.id.clone(), business.name.clone())));
    }

    // Поиск по ID
    if let Some(business) = businesses.iter().find(|b| b.id == query) {
        return Ok(Some((business.id.clone(), business.name.clone())));
    }

    Ok(None)
}

/// 🔄 Обработчик сравнения бизнесов
pub struct CompareBusinessesHandler;

#[async_trait]
impl IntentHandler for CompareBusinessesHandler {
    fn name(&self) -> &'static str {
        "comparebusinesses"
    }

    fn priority(&self) -> u8 {
        85 // Очень высокий приоритет для специализированного запроса
    }

    async fn handle(&self, input: &str, ctx: &mut Context, state: &AppState) -> Option<String> {
        tracing::info!("🔄 Handling business comparison request for user: {}", ctx.user_id);

        // Извлекаем названия бизнесов из запроса
        let business_names = extract_business_names_for_comparison(input);

        if business_names.len() < 2 {
            return Some(
                "❌ Для сравнения укажите минимум 2 бизнеса.\n\n\
                 💡 Пример: 'сравни Tech Startup и Fodi Sushi'\n\
                 или: 'сравни Tech Startup vs Fodi Sushi'".to_string()
            );
        }

        // Получаем бизнесы и их метрики
        let mut business_metrics = Vec::new();
        let mut not_found = Vec::new();

        for name in &business_names {
            match find_business(name, state).await {
                Ok(Some((business_id, business_name))) => {
                    match fetch_business_metrics(&business_id).await {
                        Ok(metrics) => {
                            business_metrics.push((business_name, metrics));
                        }
                        Err(e) => {
                            tracing::error!("❌ Failed to fetch metrics for {}: {}", business_name, e);
                            not_found.push(format!("{} (нет метрик)", business_name));
                        }
                    }
                }
                Ok(None) => {
                    not_found.push(name.clone());
                }
                Err(e) => {
                    tracing::error!("❌ Error searching business {}: {}", name, e);
                    not_found.push(name.clone());
                }
            }
        }

        if business_metrics.is_empty() {
            return Some(format!(
                "❌ Не удалось найти бизнесы для сравнения.\n\n\
                 Не найдены: {}\n\n\
                 💡 Проверьте названия или используйте команду 'покажи все бизнесы'",
                not_found.join(", ")
            ));
        }

        if business_metrics.len() == 1 {
            return Some(format!(
                "⚠️ Найден только один бизнес: {}\n\n\
                 Для сравнения нужно минимум 2 бизнеса.\n\
                 Не найдены: {}",
                business_metrics[0].0,
                not_found.join(", ")
            ));
        }

        // Используем функцию compare_businesses из analysis.rs
        use crate::ai::analysis::compare_businesses;
        
        let comparison_refs: Vec<(&str, &_)> = business_metrics
            .iter()
            .map(|(name, metrics)| (name.as_str(), metrics))
            .collect();

        let comparison = compare_businesses(comparison_refs);

        // Добавляем предупреждение если не все бизнесы найдены
        let result = if !not_found.is_empty() {
            format!(
                "⚠️ Некоторые бизнесы не найдены: {}\n\n{}", 
                not_found.join(", "),
                comparison
            )
        } else {
            comparison
        };

        Some(result)
    }
}

/// Извлечь названия бизнесов для сравнения
fn extract_business_names_for_comparison(input: &str) -> Vec<String> {
    let input_lower = input.to_lowercase();

    // Разделители для списка бизнесов
    let delimiters = [" и ", " or ", " vs ", " versus ", ", "];

    // Сначала пробуем найти разделители в оригинальном тексте
    let mut parts: Vec<String> = Vec::new();
    
    for delimiter in &delimiters {
        if input_lower.contains(delimiter) {
            parts = input_lower
                .split(delimiter)
                .map(|s| s.to_string())
                .collect();
            break;
        }
    }

    // Очищаем каждую часть от команд и стоп-слов
    let command_words = ["сравни", "сравнить", "сравнение", "compare", "comparison"];
    let stop_words = ["бизнес", "бизнесы", "бизнеса", "бизнесов", "business", "businesses"];
    
    parts.into_iter()
        .map(|part| {
            let mut cleaned = part;
            // Убираем команды
            for word in &command_words {
                cleaned = cleaned.replace(word, " ");
            }
            // Убираем стоп-слова
            for word in &stop_words {
                cleaned = cleaned.replace(word, " ");
            }
            // Очищаем лишние пробелы
            cleaned.split_whitespace().collect::<Vec<_>>().join(" ")
        })
        .filter(|s| !s.is_empty() && s.len() > 2)
        .collect()
}

/// 💡 Обработчик советов по бизнесу
pub struct BusinessInsightsHandler;

#[async_trait]
impl IntentHandler for BusinessInsightsHandler {
    fn name(&self) -> &'static str {
        "businessinsights"
    }

    fn priority(&self) -> u8 {
        82 // Высокий приоритет для аналитических советов
    }

    async fn handle(&self, input: &str, ctx: &mut Context, state: &AppState) -> Option<String> {
        tracing::info!("💡 Handling business insights request for user: {}", ctx.user_id);

        // Извлекаем название бизнеса
        let business_query = extract_business_name(input);

        if business_query.is_empty() {
            return Some(
                "💡 **Советы по улучшению бизнеса**\n\n\
                 Укажите название бизнеса для получения персональных рекомендаций.\n\n\
                 📌 Пример: 'советы для Tech Startup' или 'как улучшить Fodi Sushi'\n\n\
                 Я проанализирую метрики и дам конкретные советы по:\n\
                 • Увеличению ROI\n\
                 • Привлечению инвесторов\n\
                 • Стабилизации цены токена\n\
                 • Оптимизации расходов"
                .to_string()
            );
        }

        // Ищем бизнес
        match find_business(&business_query, state).await {
            Ok(Some((business_id, business_name))) => {
                tracing::info!("💡 Generating insights for: {}", business_name);

                // Получаем метрики
                match fetch_business_metrics(&business_id).await {
                    Ok(metrics) => {
                        use crate::ai::analysis::generate_business_insights;
                        
                        let insights = format!(
                            "💡 **Советы для бизнеса: {}**\n\n{}",
                            business_name,
                            generate_business_insights(&metrics)
                        );

                        Some(insights)
                    }
                    Err(e) => {
                        tracing::error!("❌ Failed to fetch metrics: {}", e);
                        Some(format!(
                            "❌ Не удалось получить метрики для '{}'.\n\
                             Ошибка: {}",
                            business_name, e
                        ))
                    }
                }
            }
            Ok(None) => {
                Some(format!(
                    "❌ Бизнес '{}' не найден.\n\n\
                     💡 Используйте 'покажи все бизнесы' для списка.",
                    business_query
                ))
            }
            Err(e) => {
                tracing::error!("❌ Error: {}", e);
                Some(format!("❌ Ошибка поиска: {}", e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_business_name() {
        assert_eq!(
            extract_business_name("проанализируй бизнес Tech Startup"),
            "tech startup"
        );
        assert_eq!(
            extract_business_name("analyze business Sushi Paradise"),
            "sushi paradise"
        );
        assert_eq!(extract_business_name("метрики Coffee Shop"), "coffee shop");
    }

    #[test]
    fn test_handler_name() {
        let handler = AnalyzeBusinessHandler;
        assert_eq!(handler.name(), "analyzebusiness");
    }

    #[test]
    fn test_handler_priority() {
        let handler = AnalyzeBusinessHandler;
        assert_eq!(handler.priority(), 80);
    }

    #[test]
    fn test_extract_business_names_for_comparison() {
        let names = extract_business_names_for_comparison("сравни Tech Startup и Fodi Sushi");
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"tech startup".to_string()));
        assert!(names.contains(&"fodi sushi".to_string()));

        let names2 = extract_business_names_for_comparison("compare Tech Startup vs Fodi Sushi");
        assert_eq!(names2.len(), 2);
    }

    #[test]
    fn test_compare_handler_name() {
        let handler = CompareBusinessesHandler;
        assert_eq!(handler.name(), "comparebusinesses");
    }

    #[test]
    fn test_compare_handler_priority() {
        let handler = CompareBusinessesHandler;
        assert_eq!(handler.priority(), 85);
    }

    #[test]
    fn test_insights_handler_name() {
        let handler = BusinessInsightsHandler;
        assert_eq!(handler.name(), "businessinsights");
    }

    #[test]
    fn test_insights_handler_priority() {
        let handler = BusinessInsightsHandler;
        assert_eq!(handler.priority(), 82);
    }
}

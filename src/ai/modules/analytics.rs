use async_trait::async_trait;

use crate::state::AppState;
use super::super::intent_handler::{IntentHandler, Context};

/// 📊 Check Ingredients Handler
pub struct CheckIngredientsHandler;

impl CheckIngredientsHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for CheckIngredientsHandler {
    fn name(&self) -> &'static str {
        "checkingredients"  // Match lowercase intent
    }

    fn priority(&self) -> u8 {
        85
    }

    async fn handle(&self, _input: &str, ctx: &mut Context, state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "📊 Handling check ingredients request for user: {}", ctx.user_id);

        if let Some(ingredient) = ctx.entities.first() {
            // Try to get ingredients from backend
            match state.backend.admin.get_ingredients(&ctx.user_id).await {
                Ok(ingredients) => {
                    // Find matching ingredient
                    let found = ingredients.iter().find(|i| {
                        i.name.to_lowercase().contains(&ingredient.to_lowercase())
                    });

                    if let Some(ing) = found {
                        Some(format!(
                            "📊 **Остатки: {}**\n\n\
                             📦 Количество: {}\n\
                             📏 Единица: {}\n\n\
                             ✅ В наличии",
                            ing.name,
                            ing.quantity,
                            ing.unit
                        ))
                    } else {
                        Some(format!(
                            "🔍 Ингредиент '{}' не найден в системе.\n\
                             Проверьте название.",
                            ingredient
                        ))
                    }
                }
                Err(e) => {
                    tracing::error!(target: "ai", "❌ Failed to get ingredients: {}", e);
                    Some(format!(
                        "📊 Проверяю остатки: **{}**\n\n\
                         ⚠️ Не удалось получить данные со склада.\n\
                         Попробуйте позже.",
                        ingredient
                    ))
                }
            }
        } else {
            Some(
                "📊 Для проверки остатков укажите название ингредиента:\n\
                 Например: 'проверь креветки' или 'остатки лосося'"
                    .to_string()
            )
        }
    }
}

/// 📦 Stock Status Handler
pub struct StockStatusHandler;

impl StockStatusHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for StockStatusHandler {
    fn name(&self) -> &'static str {
        "stockstatus"  // Match lowercase intent
    }

    fn priority(&self) -> u8 {
        80
    }

    async fn handle(&self, _input: &str, ctx: &mut Context, state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "📦 Handling stock status request for user: {}", ctx.user_id);

        match state.backend.admin.get_ingredients(&ctx.user_id).await {
            Ok(ingredients) => {
                if ingredients.is_empty() {
                    Some("📦 Склад пуст или нет доступа к данным.".to_string())
                } else {
                    let mut response = "📦 **Статус склада:**\n\n".to_string();
                    
                    for (i, ing) in ingredients.iter().enumerate().take(10) {
                        response.push_str(&format!(
                            "{}. {} — {} {}\n",
                            i + 1,
                            ing.name,
                            ing.quantity,
                            ing.unit
                        ));
                    }

                    if ingredients.len() > 10 {
                        response.push_str(&format!("\n...и ещё {} позиций", ingredients.len() - 10));
                    }

                    Some(response)
                }
            }
            Err(e) => {
                tracing::error!(target: "ai", "❌ Failed to get stock status: {}", e);
                Some(
                    "📦 **Статус склада:**\n\n\
                     ⚠️ Не удалось получить данные.\n\
                     Для проверки наличия используйте:\n\
                     'проверь ингредиент название'\n\n\
                     📊 Доступно для менеджеров и администраторов."
                        .to_string()
                )
            }
        }
    }
}

/// 📈 Get Statistics Handler
pub struct GetStatisticsHandler;

impl GetStatisticsHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for GetStatisticsHandler {
    fn name(&self) -> &'static str {
        "getstatistics"  // Match lowercase intent
    }

    fn priority(&self) -> u8 {
        90
    }

    async fn handle(&self, _input: &str, ctx: &mut Context, state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "📈 Handling statistics request for user: {}", ctx.user_id);

        match state.backend.admin.get_stats(&ctx.user_id).await {
            Ok(stats) => {
                Some(format!(
                    "📈 **Статистика продаж:**\n\n\
                     💰 Общий доход: {}₽\n\
                     📦 Количество заказов: {}\n\
                     👥 Пользователей: {}\n\
                     🍽️ Блюд в меню: {}\n\n\
                     ✅ Данные актуальны",
                    stats.revenue as i32,
                    stats.total_orders,
                    stats.total_users.unwrap_or(0),
                    stats.total_products.unwrap_or(0)
                ))
            }
            Err(e) => {
                tracing::error!(target: "ai", "❌ Failed to get statistics: {}", e);
                Some(
                    "📈 **Статистика продаж:**\n\n\
                     ⚠️ Не удалось получить данные.\n\n\
                     📊 Доступные отчёты:\n\
                     • Продажи за день/неделю/месяц\n\
                     • Топ продуктов\n\
                     • Средний чек\n\
                     • Динамика заказов\n\n\
                     🔐 Доступно только менеджерам."
                        .to_string()
                )
            }
        }
    }
}

/// 💰 Sales Analysis Handler
pub struct SalesAnalysisHandler;

impl SalesAnalysisHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for SalesAnalysisHandler {
    fn name(&self) -> &'static str {
        "salesanalysis"  // Match lowercase intent
    }

    fn priority(&self) -> u8 {
        85
    }

    async fn handle(&self, _input: &str, ctx: &mut Context, state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "💰 Handling sales analysis request for user: {}", ctx.user_id);

        match state.backend.admin.get_stats(&ctx.user_id).await {
            Ok(stats) => {
                let avg_check = if stats.total_orders > 0 {
                    stats.revenue / stats.total_orders as f64
                } else {
                    0.0
                };

                Some(format!(
                    "💰 **Анализ продаж:**\n\n\
                     📊 Основные метрики:\n\
                     • Выручка: {}₽\n\
                     • Количество заказов: {}\n\
                     • Средний чек: {:.0}₽\n\
                     • Пользователей: {}\n\n\
                     🔐 Полный отчёт доступен менеджерам.",
                    stats.revenue as i32,
                    stats.total_orders,
                    avg_check,
                    stats.total_users.unwrap_or(0)
                ))
            }
            Err(e) => {
                tracing::error!(target: "ai", "❌ Failed to get sales analysis: {}", e);
                Some(
                    "💰 **Анализ продаж:**\n\n\
                     ⚠️ Не удалось получить данные.\n\n\
                     📊 Основные метрики:\n\
                     • Выручка\n\
                     • Количество заказов\n\
                     • Популярные позиции\n\
                     • Пиковые часы\n\n\
                     Используйте 'статистика' для детального отчёта.\n\n\
                     🔐 Требуются права менеджера."
                        .to_string()
                )
            }
        }
    }
}

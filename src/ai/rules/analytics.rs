/// Аналитика, статистика и управление складом (для менеджеров)

pub fn check_ingredients_response(context: Option<&str>) -> String {
    if let Some(ingredient) = context {
        format!(
            "📊 Проверяю остатки: **{}**\n\n\
             Используйте команду:\n\
             `check_ingredients {}`\n\n\
             Это запросит данные из системы учёта склада.",
            ingredient, ingredient
        )
    } else {
        "📊 Для проверки остатков укажите название ингредиента:\n\
         `check_ingredients креветки`\n\
         `check_ingredients лосось`"
            .to_string()
    }
}

pub fn stock_status_response() -> String {
    "📦 **Статус склада:**\n\n\
     Для проверки наличия используйте:\n\
     `check_ingredients название`\n\n\
     📊 Доступно для менеджеров и администраторов."
        .to_string()
}

pub fn statistics_response() -> String {
    "📈 **Статистика продаж:**\n\n\
     Для получения аналитики используйте:\n\
     `get_stats`\n\n\
     📊 Доступные отчёты:\n\
     • Продажи за день/неделю/месяц\n\
     • Топ продуктов\n\
     • Средний чек\n\
     • Динамика заказов\n\n\
     🔐 Доступно только менеджерам."
        .to_string()
}

pub fn sales_analysis_response() -> String {
    "💰 **Анализ продаж:**\n\n\
     📊 Основные метрики:\n\
     • Выручка\n\
     • Количество заказов\n\
     • Популярные позиции\n\
     • Пиковые часы\n\n\
     Используйте `get_stats` для детального отчёта.\n\n\
     🔐 Требуются права менеджера."
        .to_string()
}

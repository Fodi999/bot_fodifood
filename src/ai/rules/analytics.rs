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

pub fn business_analysis_response(context: Option<&str>) -> String {
    if let Some(business) = context {
        format!(
            "📊 **Анализ бизнеса: {}**\n\n\
             Получаю метрики и формирую инвестиционную оценку...\n\n\
             💡 Анализ включает:\n\
             • Динамику цены токена\n\
             • Количество инвесторов\n\
             • ROI и доходность\n\
             • Рыночную капитализацию\n\
             • Инвестиционную рекомендацию",
            business
        )
    } else {
        "📊 **Анализ бизнеса**\n\n\
         Для получения детального анализа укажите название бизнеса:\n\n\
         Примеры:\n\
         • `проанализируй бизнес Tech Startup`\n\
         • `analyze business Sushi Paradise`\n\
         • `метрики бизнеса Coffee Shop`\n\n\
         💡 Или скажите 'покажи все бизнесы' для списка доступных."
            .to_string()
    }
}

/// 🔄 Сравнение бизнесов (fallback)
pub fn compare_businesses_response(_context: Option<&str>) -> String {
    "🔄 **Сравнение бизнесов**\n\n\
     Для сравнения укажите минимум 2 бизнеса.\n\n\
     📌 Примеры запросов:\n\
     • `сравни Tech Startup и Fodi Sushi`\n\
     • `compare Tech Startup vs Fodi Sushi`\n\
     • `какой бизнес лучше - Tech Startup или Fodi Sushi`\n\n\
     💡 Бот проанализирует ROI, количество инвесторов, \
     волатильность и даст рекомендацию."
        .to_string()
}

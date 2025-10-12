mod common;
mod menu;
mod orders;
mod recommendations;
mod analytics;
pub mod smalltalk;  // Публичный для использования в AIEngine

use super::intents::Intent;

/// Генератор ответов на основе правил и шаблонов
pub struct ResponseGenerator;

impl ResponseGenerator {
    /// Сгенерировать ответ на основе намерения
    pub fn generate(intent: &Intent, context: Option<&str>) -> String {
        match intent {
            // Общие диалоги (common.rs)
            Intent::Greeting => common::greeting_response(),
            Intent::Farewell => common::farewell_response(),
            Intent::Thanks => common::thanks_response(),
            Intent::Help => common::help_response(),
            Intent::WhoAmI => common::whoami_response(context),  // 👤 Новый intent
            Intent::Unknown => common::unknown_response(),
            
            // Меню и продукты (menu.rs)
            Intent::ViewMenu => menu::view_menu_response(),
            Intent::ProductInfo => menu::product_info_response(context),
            Intent::PriceInquiry => menu::price_inquiry_response(),
            Intent::ProductSearch => menu::product_search_response(context),  // 🔍 Поиск по ингредиенту
            
            // Заказы и доставка (orders.rs)
            Intent::OrderStatus => orders::order_status_response(context),
            Intent::CreateOrder => orders::create_order_response(),
            Intent::CancelOrder => orders::cancel_order_response(),
            Intent::DeliveryInfo => orders::delivery_info_response(),
            Intent::CourierStatus => orders::courier_status_response(),
            
            // Рекомендации (recommendations.rs)
            Intent::Recommendation => recommendations::recommendation_response(context),
            
            // Аналитика и склад (analytics.rs)
            Intent::CheckIngredients => analytics::check_ingredients_response(context),
            Intent::StockStatus => analytics::stock_status_response(),
            Intent::GetStatistics => analytics::statistics_response(),
            Intent::SalesAnalysis => analytics::sales_analysis_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting_response() {
        let response = ResponseGenerator::generate(&Intent::Greeting, None);
        assert!(response.contains("Привет"));
    }

    #[test]
    fn test_help_response() {
        let response = ResponseGenerator::generate(&Intent::Help, None);
        assert!(response.contains("команды"));
    }
}

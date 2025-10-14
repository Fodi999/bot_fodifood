pub mod menu;
pub mod orders;
pub mod smalltalk;
pub mod analytics;
pub mod recommendations;

use super::intent_handler::IntentRegistry;

/// � Register all intent handlers automatically
///
/// This function creates all available handlers and registers them
/// in the provided registry. Adding a new module is as simple as:
///
/// 1. Create new handler in its own file (e.g., `news.rs`)
/// 2. Add `pub mod news;` above
/// 3. Add registration line below
///
/// # Example
/// ```rust
/// let mut registry = IntentRegistry::new();
/// register_all_handlers(&mut registry);
///
/// // Now registry has all handlers ready to use
/// ```
pub fn register_all_handlers(registry: &mut IntentRegistry) {
    tracing::info!(target: "ai", "� Registering all intent handlers...");

    // Menu handlers
    registry.register(Box::new(menu::MenuHandler::new()));
    registry.register(Box::new(menu::SearchMenuHandler::new()));
    registry.register(Box::new(menu::FilterByIngredientHandler::new()));

    // Smalltalk handlers
    registry.register(Box::new(smalltalk::SmalltalkHandler::new()));
    registry.register(Box::new(smalltalk::HelpHandler::new()));
    registry.register(Box::new(smalltalk::DeliveryHandler::new()));

    // Order handlers
    registry.register(Box::new(orders::CreateOrderHandler::new()));
    registry.register(Box::new(orders::OrderStatusHandler::new()));
    registry.register(Box::new(orders::CancelOrderHandler::new()));

    // Analytics handlers
    registry.register(Box::new(analytics::CheckIngredientsHandler::new()));
    registry.register(Box::new(analytics::StockStatusHandler::new()));
    registry.register(Box::new(analytics::GetStatisticsHandler::new()));
    registry.register(Box::new(analytics::SalesAnalysisHandler::new()));

    // Recommendation handlers
    registry.register(Box::new(recommendations::RecommendationHandler::new()));

    tracing::info!(target: "ai", "✅ Registered {} handlers", registry.count());
    tracing::info!(target: "ai", "📝 Available handlers: {:?}", registry.registered_handlers());
}

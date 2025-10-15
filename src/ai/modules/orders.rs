use async_trait::async_trait;
use serde_json::json;

use super::super::intent_handler::{Context, IntentHandler};
use super::super::intents::IntentClassifier;
use crate::state::AppState;

/// 🛒 Create Order Intent Handler
pub struct CreateOrderHandler;

impl CreateOrderHandler {
    pub fn new() -> Self {
        Self
    }

    /// Parse items from user message
    fn parse_items(message: &str) -> Vec<String> {
        // Try to extract product name using IntentClassifier
        if let Some(product) = IntentClassifier::extract_product_name(message) {
            return vec![product];
        }

        // Fallback: split by common separators
        let items: Vec<String> = message
            .split(&[',', 'и', '\n'][..])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty() && s.len() > 2)
            .map(|s| s.to_string())
            .collect();

        items
    }
}

#[async_trait]
impl IntentHandler for CreateOrderHandler {
    fn name(&self) -> &'static str {
        "createorder"  // Match lowercase intent from classifier
    }

    fn priority(&self) -> u8 {
        100
    }

    async fn handle(&self, input: &str, ctx: &mut Context, state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "🛒 Handling create order request for user: {}", ctx.user_id);

        // Parse items from message or entities
        let items = if !ctx.entities.is_empty() {
            ctx.entities.clone()
        } else {
            Self::parse_items(input)
        };

        if items.is_empty() {
            return Some(
                "📦 Чтобы сделать заказ, напишите что хотите заказать.\n\
                Например: 'Хочу заказать Филадельфию и Калифорнию'"
                    .to_string(),
            );
        }

        // Get all products from backend
        let products = match state.backend.products.get_products().await {
            Ok(prods) => prods,
            Err(e) => {
                tracing::error!(target: "ai", "❌ Failed to fetch products: {}", e);
                return Some(
                    "⚠️ Не удалось загрузить меню.\n\
                    Пожалуйста, попробуйте позже.".to_string()
                );
            }
        };

        // Find matching products and build order items
        let mut order_items = Vec::new();
        let mut found_items = Vec::new();
        let mut not_found_items = Vec::new();

        for item_name in items {
            // Search for product by name (case-insensitive partial match)
            if let Some(product) = products.iter().find(|p| {
                p.name.to_lowercase().contains(&item_name.to_lowercase())
            }) {
                order_items.push(json!({
                    "product_id": product.id,
                    "name": product.name,
                    "quantity": 1,
                    "price": product.price
                }));
                found_items.push(product.name.clone());
            } else {
                not_found_items.push(item_name);
            }
        }

        // If no products found
        if order_items.is_empty() {
            return Some(format!(
                "😔 К сожалению, не нашел в меню: {}\n\n\
                Попробуйте:\n\
                • Проверить название блюда\n\
                • Посмотреть меню: 'покажи меню'\n\
                • Поискать по ингредиенту: 'блюда с лососем'",
                not_found_items.join(", ")
            ));
        }

        // Show warning if some items not found
        let warning = if !not_found_items.is_empty() {
            format!("⚠️ Не найдено в меню: {}\n\n", not_found_items.join(", "))
        } else {
            String::new()
        };

        let order_request = json!({
            "user_id": ctx.user_id,
            "name": "Тестовый клиент",
            "phone": "+7 900 000-00-00",
            "address": "Москва, ул. Примерная, д.1",
            "items": order_items
        });

        // Create order via Go backend
        match state.backend.orders.create_order(order_request).await {
            Ok(order) => {
                tracing::info!(target: "ai", "✅ Order created successfully: ID={}", order.id);
                
                Some(format!(
                    "{}✅ Заказ успешно создан! 🎉\n\n\
                    🆔 Номер заказа: {}\n\
                    📝 Позиции: {}\n\
                    💰 Сумма: {}₽\n\n\
                    📞 Наш менеджер свяжется с вами для подтверждения адреса и деталей доставки.\n\n\
                    Спасибо за заказ! 🚚",
                    warning, order.id, found_items.join(", "), order.total as i32
                ))
            }
            Err(e) => {
                tracing::error!(target: "ai", "❌ Failed to create order: {}", e);
                
                Some(format!(
                    "⚠️ Не удалось создать заказ в системе.\n\n\
                    📝 Вы хотели заказать: {}\n\n\
                    Пожалуйста, попробуйте позже или свяжитесь с нами напрямую:\n\
                    📱 +7 (XXX) XXX-XX-XX\n\n\
                    Приносим извинения за неудобства! 😞",
                    found_items.join(", ")
                ))
            }
        }
    }
}

/// 📦 Order Status Intent Handler
pub struct OrderStatusHandler;

impl OrderStatusHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for OrderStatusHandler {
    fn name(&self) -> &'static str {
        "orderstatus"  // Match lowercase intent from classifier
    }

    fn priority(&self) -> u8 {
        95
    }

    async fn handle(&self, _input: &str, ctx: &mut Context, state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "📦 Handling order status request for user: {}", ctx.user_id);

        // TODO: Need token, for now use user_id as token
        match state.backend.orders.get_recent_orders(&ctx.user_id).await {
            Ok(orders) => {
                if orders.is_empty() {
                    Some("У вас пока нет активных заказов 📭".to_string())
                } else {
                    let order = &orders[0];

                    Some(format!(
                        "📦 Ваш последний заказ:\n\
                        🆔 Номер: {}\n\
                        📊 Статус: {}\n\
                        💰 Сумма: {}₽\n\n\
                        Скоро свяжемся с вами!",
                        order.id, order.status, order.total as i32
                    ))
                }
            }
            Err(e) => {
                tracing::error!(target: "ai", "❌ Failed to get order status: {}", e);
                Some("Извините, не могу получить статус заказа. Попробуйте позже 😞".to_string())
            }
        }
    }
}

/// ❌ Cancel Order Intent Handler
pub struct CancelOrderHandler;

impl CancelOrderHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for CancelOrderHandler {
    fn name(&self) -> &'static str {
        "cancelorder"  // Match lowercase intent
    }

    fn priority(&self) -> u8 {
        90
    }

    async fn handle(&self, _input: &str, ctx: &mut Context, _state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "❌ Handling cancel order request for user: {}", ctx.user_id);

        Some(
            "❌ Для отмены заказа свяжитесь с нами:\n\
            📱 +7 (XXX) XXX-XX-XX\n\
            ✉️ support@fodifood.ru\n\n\
            Укажите номер заказа для быстрой обработки.".to_string()
        )
    }
}

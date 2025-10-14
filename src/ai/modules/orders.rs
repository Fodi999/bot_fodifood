use async_trait::async_trait;

use super::super::intent_handler::{Context, IntentHandler};
use crate::state::AppState;

/// 🛒 Create Order Intent Handler
pub struct CreateOrderHandler;

impl CreateOrderHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for CreateOrderHandler {
    fn name(&self) -> &'static str {
        "create_order"
    }

    fn priority(&self) -> u8 {
        100
    }

    async fn handle(&self, input: &str, ctx: &mut Context, _state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "🛒 Handling create order request for user: {}", ctx.user_id);

        let items_text = if !ctx.entities.is_empty() {
            ctx.entities.join(", ")
        } else {
            input.to_string()
        };

        if items_text.is_empty() || items_text.len() < 3 {
            return Some(
                "📦 Чтобы сделать заказ, напишите что хотите заказать.\n\
                Например: 'Хочу заказать Филадельфию и Калифорнию'"
                    .to_string(),
            );
        }

        Some(format!(
            "✅ Отлично! Начинаем оформление заказа.\n\n\
            📝 Вы хотите: {}\n\n\
            Для подтверждения укажите:\n\
            📍 Адрес доставки\n\
            📱 Номер телефона\n\
            💬 Комментарий (по желанию)",
            items_text
        ))
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
        "order_status"
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
        "cancel_order"
    }

    fn priority(&self) -> u8 {
        90
    }

    async fn handle(&self, _input: &str, ctx: &mut Context, _state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "❌ Handling cancel order request for user: {}", ctx.user_id);

        Some(format!(
            "😔 Хотите отменить заказ?\n\n\
            Пожалуйста, свяжитесь с поддержкой по телефону или через чат.\n\
            Мы поможем!"
        ))
    }
}

use async_trait::async_trait;

use super::super::intent_handler::{Context, IntentHandler};
use crate::state::AppState;

/// 💬 Smalltalk Intent Handler
pub struct SmalltalkHandler;

impl SmalltalkHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for SmalltalkHandler {
    fn name(&self) -> &'static str {
        "smalltalk"
    }

    fn priority(&self) -> u8 {
        50
    }

    async fn handle(&self, _input: &str, _ctx: &mut Context, _state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "💬 Handling smalltalk request");

        Some("👋 Привет! Чем могу помочь? Могу показать меню, оформить заказ или ответить на вопросы.".to_string())
    }
}

/// ❓ Help Intent Handler
pub struct HelpHandler;

impl HelpHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for HelpHandler {
    fn name(&self) -> &'static str {
        "help"
    }

    fn priority(&self) -> u8 {
        100
    }

    async fn handle(&self, _input: &str, _ctx: &mut Context, _state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "❓ Handling help request");

        Some(format!(
            "🤖 **Что я умею:**\n\n\
            📋 Показать меню\n\
            🔍 Найти блюдо\n\
            🛒 Оформить заказ\n\
            📦 Проверить статус заказа\n\
            🚚 Информация о доставке\n\n\
            Просто напиши, что тебе нужно!"
        ))
    }
}

/// 🚚 Delivery Info Handler
pub struct DeliveryHandler;

impl DeliveryHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for DeliveryHandler {
    fn name(&self) -> &'static str {
        "deliveryinfo"  // Match lowercase intent
    }

    fn priority(&self) -> u8 {
        80
    }

    async fn handle(&self, _input: &str, _ctx: &mut Context, _state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "🚚 Handling delivery info request");

        Some(format!(
            "🚚 **Доставка:**\n\n\
            ⏱️ Время: 30-60 минут\n\
            💰 Стоимость: бесплатно при заказе от 500₽\n\
            📍 Зона доставки: весь город\n\n\
            Минимальная сумма заказа: 300₽"
        ))
    }
}

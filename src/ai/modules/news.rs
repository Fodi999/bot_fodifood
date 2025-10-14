use async_trait::async_trait;

use crate::state::AppState;
use super::super::intent_handler::{IntentHandler, Context};

/// 📰 News Intent Handler - Example of adding new module
pub struct NewsHandler;

impl NewsHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for NewsHandler {
    fn name(&self) -> &'static str {
        "news"
    }

    fn priority(&self) -> u8 {
        70
    }

    async fn handle(&self, _input: &str, _ctx: &mut Context, _state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "📰 Handling news request");

        Some(format!(
            "📰 **Новости и акции:**\n\n\
            🔥 Скидка 20% на все роллы до конца недели!\n\
            🎉 Новое блюдо: Дракон Ролл\n\
            🚀 Бесплатная доставка при заказе от 1000₽\n\n\
            Следите за обновлениями!"
        ))
    }
}

use async_trait::async_trait;
use crate::ai::intent_handler::{Context, IntentHandler};
use crate::ai::core::{query_groq_with_system, GroqConfig, GroqModel};
use crate::state::AppState;

/// 🤖 Fallback Handler - uses GROQ AI for unknown intents
pub struct FallbackHandler;

impl FallbackHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for FallbackHandler {
    fn name(&self) -> &'static str {
        "fallback"
    }

    fn can_handle(&self, ctx: &Context) -> bool {
        // This is a catch-all fallback - always returns true
        // It will be used ONLY if no other handler matched
        // (thanks to low priority)
        true
    }

    fn priority(&self) -> u8 {
        // Lowest priority - only used if nothing else matches
        0
    }

    async fn handle(&self, input: &str, ctx: &mut Context, _state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "🤖 Fallback handler processing: {}", input);

        // Check if GROQ_API_KEY is available
        match std::env::var("GROQ_API_KEY") {
            Ok(key) if !key.is_empty() => {
                tracing::info!(target: "ai", "✅ GROQ_API_KEY found, using AI response");
            }
            _ => {
                tracing::warn!(target: "ai", "⚠️ GROQ_API_KEY not set, using default response");
                return Some("🤔 Извини, я ещё учусь понимать такие вопросы. Попробуй спросить иначе или выбери что-то из меню.".to_string());
            }
        };

        // Build context-aware prompt with real menu context
        let system_prompt = "Ты — дружелюбный AI-ассистент FodiFood, платформы доставки еды. \
            Твоя задача — помогать пользователям с заказами, меню, вопросами о еде и токенах FODI. \
            \n\n📋 Наше реальное меню:\
            \n- Роллы: Филадельфия (450₽), Калифорния (380₽)\
            \n- Пицца: Маргарита (350₽), Пепперони (420₽)\
            \n- Супы: Том Ям (320₽)\
            \n- Напитки: Coca-Cola (90₽)\
            \n\nОтвечай кратко, по делу, дружелюбно. Если не знаешь — признайся честно.";

        // Build user greeting (use username if available)
        let greeting = if let Some(ref name) = ctx.username {
            format!("Пользователь {} написал", name)
        } else {
            "Пользователь написал".to_string()
        };

        let user_prompt = format!(
            "{}: \"{}\"\n\n\
            Контекст:Intent = {}\n\n\
            Дай краткий, полезный ответ (1-3 предложения):",
            greeting,
            input,
            ctx.intent
        );

        // Configure GROQ
        let config = GroqConfig {
            model: GroqModel::Llama8B,
            temperature: 0.7,
            max_tokens: 150,
            top_p: 0.9,
        };

        // Call GROQ API
        match query_groq_with_system(&system_prompt, &user_prompt, &config).await {
            Ok(response) => {
                tracing::info!(target: "ai", "✅ GROQ response received: {} chars", response.len());
                Some(response.trim().to_string())
            }
            Err(e) => {
                tracing::error!(target: "ai", "❌ GROQ API error: {}", e);
                Some("🤔 Прости, возникла проблема с обработкой запроса. Попробуй ещё раз или выбери что-то из меню.".to_string())
            }
        }
    }
}

impl Default for FallbackHandler {
    fn default() -> Self {
        Self::new()
    }
}

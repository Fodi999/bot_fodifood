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
        // Handle ALL unknown intents
        ctx.intent == "unknown" || ctx.intent.is_empty()
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

        // Build context-aware prompt
        let system_prompt = "Ты — дружелюбный AI-ассистент FodiFood, платформы доставки еды. \
            Твоя задача — помогать пользователям с заказами, меню, вопросами о еде и токенах FODI. \
            Отвечай кратко, по делу, дружелюбно. Если не знаешь — признайся честно.";

        let user_prompt = format!(
            "Пользователь написал: \"{}\"\n\n\
            Контекст:\n\
            - User ID: {}\n\
            - Intent: {}\n\n\
            Дай краткий, полезный ответ (1-3 предложения):",
            input,
            ctx.user_id,
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

use async_trait::async_trait;
use std::collections::HashMap;

use crate::state::AppState;

/// 🎯 Unified Context for intent handling
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields are used by handlers, but rustc doesn't always detect it
pub struct Context {
    pub user_id: String,
    pub message: String,
    pub intent: String,
    pub entities: Vec<String>,
    pub metadata: HashMap<String, String>,
    // References to shared state (not cloned)
    // We'll pass AppState separately to avoid large clones
}

#[allow(dead_code)] // Methods are used in AI processing pipeline
impl Context {
    pub fn new(user_id: String, message: String, intent: String) -> Self {
        Self {
            user_id,
            message,
            intent,
            entities: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_entities(mut self, entities: Vec<String>) -> Self {
        self.entities = entities;
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

// Alias for backward compatibility
#[allow(dead_code)]
pub type IntentContext = Context;

/// 🧠 Intent Handler trait - implement this for each intent type
///
/// # Example
/// ```rust
/// pub struct NewsHandler;
///
/// #[async_trait]
/// impl IntentHandler for NewsHandler {
///     fn name(&self) -> &'static str { "news" }
///     
///     async fn handle(&self, input: &str, ctx: &mut Context, state: &AppState) -> Option<String> {
///         Some("📰 Сегодня акция на роллы!".into())
///     }
/// }
/// ```
#[async_trait]
#[allow(dead_code)] // Trait is used by all handler implementations
pub trait IntentHandler: Send + Sync {
    /// Get the handler name (usually matches intent name)
    fn name(&self) -> &'static str;

    /// Get priority (higher = processed first, default 100)
    fn priority(&self) -> u8 {
        100
    }

    /// Handle the intent and return a response
    ///
    /// # Arguments
    /// * `input` - The original user message
    /// * `ctx` - Mutable context with user_id, entities, metadata
    /// * `state` - Application state with backend clients
    ///
    /// # Returns
    /// * `Some(String)` - Response message if handled successfully
    /// * `None` - If this handler cannot process the request
    async fn handle(&self, input: &str, ctx: &mut Context, state: &AppState) -> Option<String>;

    /// Check if this handler can handle the given context
    /// Default implementation checks if intent matches name
    fn can_handle(&self, ctx: &Context) -> bool {
        ctx.intent == self.name()
    }
}

/// 📋 Intent Registry - manages all intent handlers
pub struct IntentRegistry {
    handlers: Vec<Box<dyn IntentHandler>>,
}

impl IntentRegistry {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    /// Register a new intent handler
    pub fn register(&mut self, handler: Box<dyn IntentHandler>) {
        tracing::info!("📝 Registering intent handler: {}", handler.name());
        self.handlers.push(handler);

        // Sort by priority (highest first)
        self.handlers
            .sort_by(|a, b| b.priority().cmp(&a.priority()));
    }

    /// Handle an intent using registered handlers
    #[allow(dead_code)] // Used by AI engine's process_with_plugins and process_with_insights
    pub async fn handle(&self, input: &str, ctx: &mut Context, state: &AppState) -> String {
        let start = std::time::Instant::now();
        tracing::debug!(target: "ai", "🔍 Looking for handler for intent: {}", ctx.intent);

        for handler in &self.handlers {
            if handler.can_handle(ctx) {
                tracing::info!(target: "ai", "✅ Found handler: {} for intent: {}", handler.name(), ctx.intent);

                match handler.handle(input, ctx, state).await {
                    Some(response) => {
                        let elapsed = start.elapsed();
                        tracing::info!(target: "ai", "⏱️  Intent '{}' handled in {:?}", ctx.intent, elapsed);
                        return response;
                    }
                    None => {
                        tracing::warn!(target: "ai", "⚠️  Handler {} returned None", handler.name());
                        continue;
                    }
                }
            }
        }

        let elapsed = start.elapsed();
        tracing::warn!(target: "ai", "❌ No handler found for intent: {} (took {:?})", ctx.intent, elapsed);
        "🤔 Не понял, попробуй иначе.".to_string()
    }

    /// Get all registered handler names
    pub fn registered_handlers(&self) -> Vec<String> {
        self.handlers.iter().map(|h| h.name().to_string()).collect()
    }

    /// Get number of registered handlers
    pub fn count(&self) -> usize {
        self.handlers.len()
    }
}

impl Default for IntentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHandler;

    #[async_trait]
    impl IntentHandler for TestHandler {
        fn name(&self) -> &'static str {
            "test"
        }

        async fn handle(
            &self,
            _input: &str,
            _ctx: &mut Context,
            _state: &AppState,
        ) -> Option<String> {
            Some("Test response".to_string())
        }
    }

    #[tokio::test]
    async fn test_registry_registration() {
        let mut registry = IntentRegistry::new();
        assert_eq!(registry.count(), 0);

        registry.register(Box::new(TestHandler));
        assert_eq!(registry.count(), 1);
        assert_eq!(registry.registered_handlers(), vec!["test"]);
    }
}

# 🚀 Плагинная AI система - Руководство

## Текущая архитектура

Система теперь полностью плагинная! Каждый модуль - это независимый обработчик интентов.

### Структура

```
src/ai/
  ├── intent_handler.rs  ← Core trait + Registry
  ├── modules/
  │   ├── mod.rs         ← Автоматическая регистрация
  │   ├── menu.rs        ← 3 хендлера для меню
  │   ├── orders.rs      ← 3 хендлера для заказов
  │   ├── smalltalk.rs   ← 3 хендлера для общения
  │   └── news.rs        ← 1 новый хендлер (пример!)
```

## 📝 Как добавить новый модуль (3 шага)

### 1. Создайте файл `src/ai/modules/my_feature.rs`

```rust
use async_trait::async_trait;

use crate::state::AppState;
use super::super::intent_handler::{IntentHandler, Context};

pub struct MyFeatureHandler;

impl MyFeatureHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for MyFeatureHandler {
    fn name(&self) -> &'static str {
        "my_feature"  // Intent name
    }

    fn priority(&self) -> u8 {
        100  // Higher = processed first
    }

    async fn handle(&self, input: &str, ctx: &mut Context, state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "✨ Handling my feature for user: {}", ctx.user_id);

        // Your logic here:
        // - Use ctx.user_id, ctx.entities, ctx.metadata
        // - Call state.backend.* for API access
        // - Return Some(response) or None

        Some("✨ Фича работает!".to_string())
    }
}
```

### 2. Добавьте `pub mod my_feature;` в `src/ai/modules/mod.rs`

```rust
pub mod menu;
pub mod smalltalk;
pub mod orders;
pub mod my_feature;  // ← Добавь эту строку
```

### 3. Зарегистрируйте в `register_all_handlers()`

```rust
pub fn register_all_handlers(registry: &mut IntentRegistry) {
    // ... existing handlers ...
    
    // My feature handlers
    registry.register(Box::new(my_feature::MyFeatureHandler::new()));
    
    tracing::info!(target: "ai", "✅ Registered {} handlers", registry.count());
}
```

## 🎯 Context API

### Доступные поля

```rust
pub struct Context {
    pub user_id: String,        // Telegram user ID
    pub message: String,        // Original message text
    pub intent: String,         // Classified intent name
    pub entities: Vec<String>,  // Extracted entities
    pub metadata: HashMap<String, String>,  // Custom data
}
```

### Методы Context

```rust
// Create context
let mut ctx = Context::new(user_id, message, intent);

// Add entities
ctx = ctx.with_entities(vec!["entity1".into(), "entity2".into()]);

// Add metadata
ctx = ctx.with_metadata("key".into(), "value".into());

// Get metadata
if let Some(value) = ctx.get_metadata("key") {
    // use value
}
```

## 🔧 AppState API

Доступ к бэкенду через `state.backend`:

```rust
// Products
let products = state.backend.products.get_products().await?;
let product = ProductsClient::find_product_by_name(&products, "Филадельфия");

// Orders
let orders = state.backend.orders.get_recent_orders(token).await?;

// Auth
let user = state.backend.auth.get_user_profile(token, user_id).await?;

// Admin
let stats = state.backend.admin.get_stats(token).await?;
```

## 📊 Логирование и метрики

Все хендлеры автоматически логируются:

```rust
tracing::info!(target: "ai", "✨ My feature triggered");
tracing::error!(target: "ai", "❌ Something went wrong: {}", error);
tracing::debug!(target: "ai", "🔍 Debug info: {:?}", data);
```

В `IntentRegistry::handle()` автоматически логируются:
- ⏱️ Время обработки каждого intent
- ✅ Найденный хендлер
- ❌ Отсутствие подходящего хендлера

## 🎛️ Priority система

Priority определяет порядок обработки (от большего к меньшему):

```
100 - High   (create_order, help)
 95 - Medium (search_menu, order_status)
 90 - Normal (show_menu, cancel_order)
 85 -        (filter_by_ingredient)
 80 -        (delivery_info)
 70 -        (news)
 50 - Low    (smalltalk - fallback)
```

## 🧪 Тестирование

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_my_handler() {
        let handler = MyFeatureHandler::new();
        assert_eq!(handler.name(), "my_feature");
        assert_eq!(handler.priority(), 100);
    }
}
```

## 🚀 Интеграция в AIEngine (TODO)

После всех модулей останется интегрировать registry в главный flow:

```rust
// src/ai/mod.rs
pub struct AIEngine {
    memory: BotMemory,
    backend: GoBackendClient,
    intent_registry: IntentRegistry,  // ← Добавить
}

impl AIEngine {
    pub fn new(config: &Config) -> Self {
        let mut registry = IntentRegistry::new();
        modules::register_all_handlers(&mut registry);
        
        Self {
            memory: BotMemory::new(),
            backend: GoBackendClient::new(config),
            intent_registry: registry,
        }
    }

    pub async fn process(&self, message: &str, user_id: &str) -> String {
        // 1. Classify intent
        let intent = classify_intent(message);
        
        // 2. Create context
        let mut ctx = Context::new(user_id.into(), message.into(), intent);
        
        // 3. Handle through registry
        self.intent_registry.handle(message, &mut ctx, &state).await
    }
}
```

## 📈 Следующие шаги

1. ✅ **Плагинная система** (сделано!)
2. 🔲 Интеграция registry в AIEngine
3. 🔲 Persistent Memory Service (sled/sqlite)
4. 🔲 Enhanced Metrics Dashboard
5. 🔲 Миграция старых rules/* → modules/*

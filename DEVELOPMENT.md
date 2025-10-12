# FodiFood Bot - Development Guide

## 🛠️ Локальная разработка

### Требования
- Rust 1.75+
- Cargo Shuttle CLI
- OpenAI API ключ (опционально для AI функций)
- Доступ к Go backend API

### Первый запуск

1. **Клонирование и setup**
```bash
git clone <your-repo>
cd bot_fodifood
chmod +x setup.sh
./setup.sh
```

2. **Конфигурация**
Отредактируйте `.env`:
```env
OPENAI_API_KEY=sk-your-key
GO_BACKEND_URL=http://localhost:8080/api  # или Koyeb URL
JWT_SECRET=your-secret-key
RUST_LOG=debug  # для детальных логов
```

3. **Запуск**
```bash
# Локальный запуск
cargo shuttle run

# С детальными логами
RUST_LOG=debug cargo shuttle run

# Только проверка компиляции
cargo check
```

### Hot Reload

Используйте `cargo-watch` для автоматической перезагрузки:
```bash
cargo install cargo-watch
cargo watch -x 'shuttle run'
```

## 🧪 Тестирование

### Unit тесты

Создайте файл `src/handlers/ws_test.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_detection() {
        assert_eq!(Intent::detect("покажи меню"), Intent::ViewMenu);
        assert_eq!(Intent::detect("хочу заказать"), Intent::CreateOrder);
    }
}
```

Запуск тестов:
```bash
cargo test
cargo test -- --nocapture  # с выводом println!
```

### Интеграционное тестирование

```bash
# Запустите бота локально
cargo shuttle run

# В другом терминале - тест WebSocket
websocat ws://localhost:8000/ws

# Отправьте тестовое сообщение
{"type":"ping"}
# Ожидаемый ответ: {"type":"pong"}
```

### Тестирование с фейковым Go backend

Создайте mock server с помощью `mockito`:
```rust
#[cfg(test)]
mod tests {
    use mockito::Server;

    #[tokio::test]
    async fn test_go_backend_client() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("GET", "/products")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"id":1,"name":"Test"}]"#)
            .create_async()
            .await;

        // Test your client
        mock.assert_async().await;
    }
}
```

## 📊 Мониторинг и отладка

### Логи в Shuttle

```bash
# Смотреть логи в реальном времени
cargo shuttle logs --follow

# Последние 100 строк
cargo shuttle logs --tail 100

# Логи с определенного времени
cargo shuttle logs --since 1h
```

### Уровни логирования

В коде используйте:
```rust
tracing::error!("Critical error: {}", e);
tracing::warn!("Warning: user {} disconnected", user_id);
tracing::info!("New connection from {}", addr);
tracing::debug!("Processing message: {:?}", msg);
tracing::trace!("Detailed state: {:?}", state);
```

Установите уровень через `.env`:
```env
# Все логи
RUST_LOG=trace

# Только ваше приложение
RUST_LOG=fodifood_bot=debug

# Отдельные модули
RUST_LOG=fodifood_bot::handlers::ws=trace,fodifood_bot::ai=debug
```

### Дебаггинг WebSocket

Используйте Chrome DevTools:
1. Откройте DevTools (F12)
2. Network tab → WS filter
3. Найдите соединение
4. Смотрите Messages

Или используйте специальные инструменты:
```bash
# websocat с детальным выводом
websocat -v ws://localhost:8000/ws

# wscat (Node.js)
npm install -g wscat
wscat -c ws://localhost:8000/ws
```

## 🏗️ Расширение функциональности

### Добавление нового Intent

1. **В `models/message.rs`:**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    // ... существующие
    RequestCallback,  // новый интент
}

impl Intent {
    pub fn detect(text: &str) -> Self {
        // ... существующие проверки
        
        if text_lower.contains("перезвоните") || text_lower.contains("callback") {
            return Intent::RequestCallback;
        }
        
        Intent::GeneralQuestion
    }
}
```

2. **В `handlers/ws.rs` - обработчик:**
```rust
async fn handle_chat_message(...) {
    match intent {
        // ... существующие
        
        Intent::RequestCallback => {
            let go_client = GoBackendClient::new(&state.config);
            match go_client.create_callback_request(user_id).await {
                Ok(_) => {
                    let response = OutgoingMessage::ChatResponse {
                        text: "Менеджер перезвонит вам в ближайшее время!".to_string(),
                        from_ai: false,
                    };
                    let _ = tx.send(response.to_json());
                }
                Err(e) => tracing::error!("Failed to create callback: {}", e),
            }
        }
    }
}
```

3. **В `api/go_backend.rs` - API метод:**
```rust
impl GoBackendClient {
    pub async fn create_callback_request(&self, user_id: &str) -> Result<()> {
        let url = format!("{}/callbacks", self.base_url);
        
        self.client
            .post(&url)
            .json(&serde_json::json!({ "user_id": user_id }))
            .send()
            .await?;
        
        Ok(())
    }
}
```

### Добавление новой команды

В `handlers/ws.rs`:
```rust
async fn handle_command(...) {
    match action {
        // ... существующие команды
        
        "get_user_orders" => {
            match go_client.get_user_orders(user_id).await {
                Ok(orders) => {
                    let response = OutgoingMessage::CommandResponse {
                        action: action.to_string(),
                        data: serde_json::to_value(orders).unwrap_or_default(),
                        success: true,
                    };
                    let _ = tx.send(response.to_json());
                }
                Err(e) => tracing::error!("Command failed: {}", e),
            }
        }
        
        _ => { /* ... */ }
    }
}
```

### Добавление кэширования

Добавьте в `Cargo.toml`:
```toml
moka = { version = "0.12", features = ["future"] }
```

В `state.rs`:
```rust
use moka::future::Cache;
use std::time::Duration;

pub struct AppState {
    pub config: Config,
    pub connections: Arc<DashMap<ClientId, ClientConnection>>,
    pub menu_cache: Cache<String, Vec<Product>>,  // новое
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            connections: Arc::new(DashMap::new()),
            menu_cache: Cache::builder()
                .time_to_live(Duration::from_secs(300))  // 5 минут
                .build(),
        }
    }
}
```

Использование:
```rust
// Попытка получить из кэша
if let Some(products) = state.menu_cache.get("products").await {
    return Ok(products);
}

// Запрос к API
let products = go_client.get_products().await?;

// Сохранение в кэш
state.menu_cache.insert("products".to_string(), products.clone()).await;
```

### Добавление Rate Limiting

Добавьте в `Cargo.toml`:
```toml
tower-governor = "0.3"
```

В `main.rs`:
```rust
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

let governor_conf = Box::new(
    GovernorConfigBuilder::default()
        .per_second(10)
        .burst_size(20)
        .finish()
        .unwrap(),
);

let app = Router::new()
    // ... routes
    .layer(GovernorLayer {
        config: Box::leak(governor_conf),
    });
```

## 🚀 Оптимизация производительности

### Профилирование

```bash
# Установка flamegraph
cargo install flamegraph

# Сборка с debug symbols
cargo build --release

# Профилирование
cargo flamegraph

# Откроется flamegraph.svg
```

### Размер бинарника

Добавьте в `Cargo.toml`:
```toml
[profile.release]
opt-level = "z"     # Оптимизация размера
lto = true          # Link Time Optimization
codegen-units = 1   # Лучшая оптимизация
strip = true        # Удалить символы
```

### Асинхронная оптимизация

```rust
// Параллельная обработка
use futures::future::join_all;

let futures = vec![
    go_client.get_products(),
    go_client.get_stats(),
    go_client.get_ingredients(),
];

let results = join_all(futures).await;
```

## 🔐 Безопасность

### Валидация входных данных

```rust
use validator::Validate;

#[derive(Deserialize, Validate)]
struct CreateOrderRequest {
    #[validate(length(min = 1, max = 100))]
    items: Vec<OrderItem>,
    
    #[validate(phone)]
    phone: String,
    
    #[validate(length(min = 5, max = 200))]
    delivery_address: String,
}
```

### Rate limiting по пользователям

```rust
use std::collections::HashMap;
use tokio::time::{Duration, Instant};

struct UserRateLimit {
    last_request: Instant,
    request_count: u32,
}

// В AppState
rate_limits: Arc<DashMap<String, UserRateLimit>>

// Проверка
fn check_rate_limit(user_id: &str) -> Result<(), &'static str> {
    let now = Instant::now();
    let mut limit = rate_limits
        .entry(user_id.to_string())
        .or_insert(UserRateLimit {
            last_request: now,
            request_count: 0,
        });

    if now.duration_since(limit.last_request) > Duration::from_secs(60) {
        limit.request_count = 1;
        limit.last_request = now;
        Ok(())
    } else if limit.request_count < 30 {
        limit.request_count += 1;
        Ok(())
    } else {
        Err("Rate limit exceeded")
    }
}
```

## 📦 Деплой

### Pre-deploy чеклист

- [ ] Все тесты проходят: `cargo test`
- [ ] Нет ошибок линтера: `cargo clippy`
- [ ] Код отформатирован: `cargo fmt`
- [ ] `.env` не коммитится (проверить `.gitignore`)
- [ ] Секреты настроены в Shuttle
- [ ] Go backend доступен
- [ ] OpenAI API ключ валиден

### Shuttle секреты

```bash
# Установка секретов
cargo shuttle secrets set OPENAI_API_KEY=sk-xxx
cargo shuttle secrets set GO_BACKEND_URL=https://your-backend.koyeb.app/api
cargo shuttle secrets set JWT_SECRET=your-secret

# Просмотр секретов
cargo shuttle secrets list
```

### CI/CD с GitHub Actions

Создайте `.github/workflows/deploy.yml`:
```yaml
name: Deploy to Shuttle

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install Shuttle
        run: cargo install cargo-shuttle
      
      - name: Deploy
        env:
          SHUTTLE_API_KEY: ${{ secrets.SHUTTLE_API_KEY }}
        run: cargo shuttle deploy --allow-dirty
```

## 📚 Дополнительные ресурсы

- [Axum Documentation](https://docs.rs/axum/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Shuttle Documentation](https://docs.shuttle.rs/)
- [OpenAI API Reference](https://platform.openai.com/docs/api-reference)

---

**Happy coding! 🦀**

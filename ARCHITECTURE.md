# 📊 FodiFood Bot - Технический обзор

## 🏗️ Архитектура

### Технологический стек
- **Язык**: Rust 🦀
- **Framework**: Axum + Tokio (async runtime)
- **WebSocket**: axum::extract::ws
- **HTTP Client**: reqwest
- **AI**: OpenAI GPT-4o-mini
- **Deployment**: Shuttle.rs
- **Serialization**: serde + serde_json

### Структура проекта (16 Rust файлов)

```
src/
├── main.rs                    # Entry point, Shuttle setup
├── config.rs                  # Configuration from env vars
├── state.rs                   # Shared app state, connections
│
├── handlers/                  # HTTP & WebSocket handlers
│   ├── ws.rs                 # WebSocket logic (auth, messages)
│   └── webhook.rs            # Webhook endpoint from Go backend
│
├── api/                       # External integrations
│   └── go_backend.rs         # REST client for Go API
│
├── ai/                        # AI integration
│   └── mod.rs                # OpenAI API wrapper
│
└── models/                    # Data models
    ├── message.rs            # WebSocket message types
    └── user.rs               # User, roles, JWT

```

## 🔄 Поток данных

### 1. Клиент → Бот (WebSocket)
```
Client (Next.js) --[WS: auth]-->  Rust Bot
                                      ↓
                                 Verify JWT
                                      ↓
                            [Go Backend: /auth/verify]
                                      ↓
                                Store connection
                                      ↓
                           Send auth_success
```

### 2. Клиент → Бот → Go Backend
```
Client --[WS: command]-->  Rust Bot
                               ↓
                        Parse command
                               ↓
                    [Go Backend: /api/orders]
                               ↓
                        Format response
                               ↓
            [WS: command_response]--> Client
```

### 3. Go Backend → Бот → Админы
```
Go Backend --[HTTP: /notify]--> Rust Bot
                                    ↓
                             Parse webhook
                                    ↓
                          Find admin connections
                                    ↓
                 [WS: notification]--> All Admins
```

### 4. Клиент → Бот → AI → Клиент
```
Client --[WS: chat]-->  Rust Bot
                            ↓
                     Detect intent
                            ↓
              Fetch data from Go Backend
                            ↓
           [OpenAI API: chat completion]
                            ↓
              Format AI response
                            ↓
        [WS: chat_response]--> Client
```

## 📡 API Endpoints

### WebSocket: `ws://localhost:8000/ws`

**Типы входящих сообщений:**
```typescript
type IncomingMessage = 
  | { type: "auth", token: string }
  | { type: "chat", text: string }
  | { type: "command", action: string, params?: any }
  | { type: "ping" }
```

**Типы исходящих сообщений:**
```typescript
type OutgoingMessage =
  | { type: "auth_success", user_id: string, role: string }
  | { type: "auth_failed", reason: string }
  | { type: "chat_response", text: string, from_ai: boolean }
  | { type: "command_response", action: string, data: any, success: boolean }
  | { type: "notification", event: string, data: any }
  | { type: "error", message: string }
  | { type: "pong" }
```

### HTTP Endpoints

| Method | Path      | Description                    |
|--------|-----------|--------------------------------|
| GET    | /         | Service info                   |
| GET    | /health   | Health check                   |
| GET    | /ws       | WebSocket upgrade              |
| POST   | /notify   | Webhook from Go backend        |

## 🧠 AI Интеллект

### Intent Detection
Автоматическое определение намерений пользователя:

| Intent | Ключевые слова | Действие |
|--------|----------------|----------|
| CreateOrder | заказ, создать, хочу | Помощь в оформлении |
| CheckOrderStatus | статус, где заказ | Проверка статуса |
| ViewMenu | меню, что есть | Показать меню + AI |
| GetRecommendation | порекоменду, что посовету | AI рекомендация |
| CheckInventory | остат, склад | Проверка запасов (staff) |
| ViewStats | статистик, продажи | Аналитика (staff) |
| GeneralQuestion | * | AI ответ |

### AI System Prompt
```
Ты — умный и дружелюбный менеджер ресторана морепродуктов FodiFood.
Ты помогаешь клиентам выбрать блюда, отвечаешь на вопросы о меню,
помогаешь оформить заказ. Также ты помогаешь менеджерам анализировать
продажи, остатки и давать рекомендации по закупкам.
Отвечай вежливо, по делу, кратко и информативно.
Используй эмодзи когда это уместно.
```

### AI API Parameters
- **Model**: `gpt-4o-mini`
- **Temperature**: `0.7` (баланс креативности/точности)
- **Max tokens**: `500` (ограничение длины ответа)

## 🔐 Безопасность

### Аутентификация
1. Client получает JWT от Go Backend
2. Client подключается к WebSocket
3. Client отправляет `{type: "auth", token: "..."}`
4. Bot проверяет токен через `POST /api/auth/verify`
5. Bot сохраняет connection с ролью

### Авторизация
```rust
match intent {
    Intent::CheckInventory if role.is_staff() => { /* allowed */ }
    Intent::ViewStats if role.is_staff() => { /* allowed */ }
    _ => { /* check permissions */ }
}
```

### Роли
| Роль | Доступ |
|------|--------|
| Client | Заказы, меню, вопросы |
| Admin | + уведомления, управление заказами |
| Manager | + статистика, аналитика |
| Courier | + уведомления о доставках |
| Cook | + уведомления на кухне |

## ⚡ Производительность

### Асинхронность
- **Tokio runtime** - полностью асинхронный
- **Concurrent connections** - тысячи одновременных WebSocket подключений
- **Non-blocking I/O** - все API запросы асинхронные

### Оптимизации
```toml
[profile.release]
opt-level = "z"      # Оптимизация размера
lto = true           # Link Time Optimization
codegen-units = 1    # Лучшая оптимизация
strip = true         # Удалить debug symbols
```

### Память
- **Connection pooling** - для HTTP клиента
- **Shared state** - Arc<DashMap> для потокобезопасности
- **Zero-copy** - где возможно

## 📊 Мониторинг

### Логирование
```rust
tracing::info!("User {} authenticated as {:?}", user_id, user_role);
tracing::warn!("Failed to fetch products: {}", e);
tracing::error!("Critical error: {}", e);
```

### Метрики (будущее)
- Количество активных подключений
- Среднее время ответа AI
- Количество запросов к Go Backend
- Error rate

## 🚀 Деплой

### Shuttle.rs Benefits
- ✅ Автоматический HTTPS/WSS
- ✅ Секреты через CLI
- ✅ Автоматическое масштабирование
- ✅ Встроенный мониторинг
- ✅ Бесплатный tier для разработки

### Environment Variables
```bash
OPENAI_API_KEY=sk-xxx     # Required
GO_BACKEND_URL=https://... # Required
JWT_SECRET=xxx            # Optional
RUST_LOG=info            # Optional
```

### Production URLs
```
WebSocket:  wss://fodifood-bot.shuttleapp.rs/ws
Health:     https://fodifood-bot.shuttleapp.rs/health
Webhook:    https://fodifood-bot.shuttleapp.rs/notify
```

## 📈 Расширяемость

### Добавление нового Intent
1. Добавить в `Intent` enum
2. Добавить детекцию в `Intent::detect()`
3. Добавить обработку в `handle_chat_message()`

### Добавление новой команды
1. Добавить в `handle_command()` match
2. Опционально добавить метод в `GoBackendClient`

### Добавление нового webhook события
1. Добавить в `webhook_handler()` match
2. Определить логику рассылки

## 🧪 Тестирование

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
# Запустить бота
cargo shuttle run

# В другом терминале
websocat ws://localhost:8000/ws
```

### Load Testing
```bash
artillery run load-test.yml
```

## 📦 Зависимости (ключевые)

| Crate | Version | Назначение |
|-------|---------|------------|
| axum | 0.7 | Web framework |
| tokio | 1.x | Async runtime |
| reqwest | 0.12 | HTTP client |
| serde | 1.0 | Serialization |
| dashmap | 6.0 | Concurrent HashMap |
| uuid | 1.0 | Unique IDs |

## 🎯 Ключевые особенности

1. ✨ **Полностью асинхронный** - высокая производительность
2. 🤖 **AI-powered** - умные ответы и аналитика
3. 🔐 **Безопасный** - JWT auth, role-based access
4. 📡 **Real-time** - WebSocket для мгновенных уведомлений
5. 🔄 **Интегрированный** - связь с Next.js и Go Backend
6. 🚀 **Production-ready** - готов к деплою
7. 📊 **Масштабируемый** - Shuttle автоматически масштабирует
8. 🛠️ **Расширяемый** - легко добавлять функции

---

**Создано с Rust 🦀 для FodiFood 🦐**

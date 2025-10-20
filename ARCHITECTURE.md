# 📊 FodiFood Bot - Технический обзор v2.3

## 🏗️ Архитектура

### Технологический стек
- **Язык**: Rust 🦀 (Edition 2021)
- **Framework**: Axum 0.8 + Tokio (async runtime)
- **WebSocket**: axum::extract::ws
- **HTTP Client**: reqwest 0.12
- **AI**: OpenAI GPT-4o-mini
- **Blockchain**: Solana SDK 2.0 + Metaplex
- **Storage**: sled (persistent memory)
- **Metrics**: Prometheus
- **Deployment**: Shuttle.rs 0.57
- **Serialization**: serde + serde_json

### Структура проекта v2.3 (85+ Rust файлов)

```
src/
├── main.rs                    # Entry point, Shuttle setup
├── lib.rs                     # Library root
├── config.rs                  # Configuration from env vars
├── state.rs                   # Shared app state (with orchestrator)
│
├── handlers/                  # HTTP & WebSocket handlers
│   ├── ws.rs                 # WebSocket logic (auth, messages)
│   ├── webhook.rs            # Webhook endpoint from Go backend
│   ├── insight_events.rs     # 📡 AI event types (9 types)
│   └── insight_broadcaster.rs # WebSocket broadcaster
│
├── api/                       # API Layer
│   ├── rest.rs               # REST endpoints + /api/v1/chat
│   ├── metrics.rs            # 📊 Metrics endpoints (Prometheus)
│   ├── admin_ws.rs           # Admin WebSocket
│   ├── insight_ws.rs         # 📡 AI Insight WebSocket
│   ├── backend_control.rs    # 🎯 Backend control API
│   ├── businesses.rs         # 💼 Business management (v2.3)
│   ├── user.rs               # 👤 User role management (v2.3)
│   ├── solana.rs             # 🪙 Solana blockchain API
│   └── go_backend/           # Go backend integration
│       ├── mod.rs
│       ├── auth.rs
│       ├── products.rs
│       ├── orders.rs
│       ├── admin.rs
│       └── types.rs
│
├── services/                  # 🔌 External Services
│   ├── mod.rs
│   └── go_client.rs          # 💼 Go Backend HTTP Client
│                             #    - fetch_businesses()
│                             #    - fetch_business_metrics()
│
├── ai/                        # 🧠 AI Engine v2.2
│   ├── mod.rs                # AIEngine with process_with_insights()
│   ├── intents.rs            # Intent classification (17 intents)
│   ├── intent_handler.rs     # Plugin system for handlers
│   ├── thinker.rs            # Cognitive analysis
│   ├── memory.rs             # In-memory context
│   ├── persistent_memory.rs  # Persistent storage (sled)
│   ├── analysis.rs           # 💼 Business analysis AI
│   ├── admin_assistant.rs    # Admin AI assistant
│   ├── modules/              # 📦 Intent Handlers (17 total)
│   │   ├── mod.rs
│   │   ├── menu.rs          # Menu queries
│   │   ├── orders.rs        # Order management
│   │   ├── recommendations.rs # Recommendations
│   │   ├── analytics.rs     # Statistics
│   │   ├── smalltalk.rs     # Small talk
│   │   ├── news.rs          # News
│   │   └── business.rs      # 💼 Business intelligence
│   └── rules/                # Rule-based responses
│       ├── mod.rs
│       ├── menu.rs
│       ├── orders.rs
│       ├── recommendations.rs
│       ├── analytics.rs
│       └── smalltalk.rs
│
├── solana/                    # 🪙 Solana Blockchain Integration
│   ├── mod.rs
│   ├── client.rs             # Solana RPC client
│   ├── token.rs              # SPL Token operations
│   ├── create_mint.rs        # Token creation
│   ├── add_metadata.rs       # Metaplex metadata
│   └── models.rs             # Blockchain types
│
├── orchestration/             # 🎯 Backend Orchestration
│   ├── mod.rs
│   ├── backend.rs            # Process lifecycle management
│   └── health.rs             # Health checker
│
├── metrics/                   # 📊 Metrics System
│   └── mod.rs                # MetricsCollector
│
├── models/                    # 📋 Data Models
│   ├── message.rs
│   ├── user.rs
│   └── mod.rs
│
├── tests/                     # 🧪 Internal tests
│   ├── mod.rs
│   └── test_solana_tx.rs
│
└── bin/                       # 🔧 Binaries
    ├── chat.rs               # CLI chat client
    ├── local.rs              # Local dev server
    ├── create_fodi_token.rs  # 🪙 Token creation utility
    └── add_fodi_metadata.rs  # 🎨 Metadata utility

tests/                         # External tests
└── fixtures/
    ├── test-keypair.json     # Test Solana keypair
    └── README.md

assets/                        # Token metadata
└── fodi-metadata.json        # FODI token metadata

examples/                      # Usage examples
├── business_analysis_demo.rs
└── go_client_demo.rs
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

### 4. Клиент → Бот → AI → Клиент (v2.2 with Insights)
```
Client --[WS: chat]-->  Rust Bot
                            ↓
                     Detect intent
                     ↓           ↓
              [AI Insights] [Fetch data from Go Backend]
                     ↓           ↓
           [OpenAI API: chat completion]
                     ↓           ↓
              Format AI response
                     ↓           ↓
        [WS: chat_response]--> Client
        [WS: insight]--> Admin Dashboard
```

### 5. Investor → Bot → Business Analytics (v2.2)
```
Investor --[POST /api/v1/chat]-->  Rust Bot
                                       ↓
                              Detect BusinessIntent
                                       ↓
                    [Go Backend: fetch businesses + metrics]
                                       ↓
                            [AI Analysis Engine]
                            • Investment scoring (0-100)
                            • Trend analysis
                            • ROI calculation
                            • Comparison logic
                                       ↓
                            [AI Recommendations]
                                       ↓
                        Format response with emojis
                                       ↓
                [JSON response]--> Investor
```

### 6. Admin → Backend Orchestration (v2.2)
```
Admin --[POST /api/v1/admin/backend/start]--> Rust Bot
                                                  ↓
                                    [Orchestrator: spawn Go process]
                                                  ↓
                                       [Monitor health every 30s]
                                                  ↓
                                    If unhealthy → restart
                                                  ↓
                            [Status tracking]--> Admin
```

### 7. User → Solana Token (NEW!)
```
User --[cargo run --bin create_fodi_token]--> Rust Bot
                                                  ↓
                                    [Solana RPC: create mint]
                                                  ↓
                                    [SPL Token: mint tokens]
                                                  ↓
                         [Metaplex: add metadata (name, symbol, URI)]
                                                  ↓
                                    [Explorer link]--> User
```

## 📡 API Endpoints v2.3

### WebSocket Endpoints

#### 1. `/ws` - Main WebSocket (Clients & Admins)
**Входящие сообщения:**
```typescript
type IncomingMessage = 
  | { type: "auth", token: string }
  | { type: "chat", text: string }
  | { type: "command", action: string, params?: any }
  | { type: "ping" }
```

**Исходящие сообщения:**
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

#### 2. `/api/v1/admin/ws` - Admin WebSocket
Управление и мониторинг для администраторов

#### 3. `/api/v1/insight` - AI Insights WebSocket (v2.2)
Real-time события обработки AI

**События:**
- `intent_classification_started`
- `intent_classified` (+ confidence)
- `entity_extraction`
- `handler_routing`
- `handler_execution_started`
- `handler_execution_completed`
- `context_updated`
- `processing_completed`
- `processing_error`

### HTTP REST Endpoints

#### Core Endpoints
| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Service info |
| GET | `/health` | Health check |
| POST | `/notify` | Webhook from Go backend |

#### Chat & AI (v2.2)
| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/v1/chat` | AI chat endpoint |
| GET | `/api/v1/insight` | AI insights WebSocket upgrade |

#### Metrics & Monitoring (v2.2)
| Method | Path | Description |
|--------|------|-------------|
| GET | `/metrics` | Prometheus metrics |
| GET | `/admin/metrics` | Web dashboard (HTML) |
| GET | `/admin/metrics/intents` | Intent statistics (JSON) |
| GET | `/admin/metrics/stats` | System stats (JSON) |

#### Backend Orchestration (v2.2)
| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/v1/admin/backend/start` | Start Go backend |
| POST | `/api/v1/admin/backend/stop` | Stop Go backend |
| POST | `/api/v1/admin/backend/restart` | Restart Go backend |
| GET | `/api/v1/admin/backend/status` | Backend status |
| GET | `/api/v1/admin/backend/health` | Orchestrator health |

#### Business Management (v2.3) 💼
| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/businesses` | List all businesses |
| POST | `/api/v1/businesses` | Create business (admin/owner) |

#### User Management (v2.3) 👤
| Method | Path | Description |
|--------|------|-------------|
| PATCH | `/api/v1/user/role` | Update user role (JWT) |

#### Solana Blockchain 🪙
| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/solana/token/{mint}` | Token info |
| POST | `/api/v1/solana/transfer` | Transfer tokens |

## 🧠 AI Интеллект v2.2

### Intent Detection - 17 Handlers

#### Core Intents (14)
| Intent | Ключевые слова | Действие |
|--------|----------------|----------|
| **Menu** | меню, что есть, показ, блюд | Показать меню + AI |
| **CreateOrder** | заказ, создать, хочу, закажи | Помощь в оформлении |
| **CheckOrderStatus** | статус, где заказ, когда | Проверка статуса |
| **GetRecommendation** | порекоменду, что посовету, подойдет | AI рекомендация |
| **CheckInventory** | остат, склад, запас | Проверка запасов (staff) |
| **ViewStats** | статистик, продажи, аналитика | Аналитика (staff) |
| **SmallTalk** | привет, как дела, спасибо | Обычный разговор |
| **News** | новост, акци, событ | Акции и события |
| **Reviews** | отзыв, оцен, feedback | Работа с отзывами |
| **Reservation** | забронир, столик, резерв | Бронирование |
| **Delivery** | доставк, когда привез | Трекинг доставки |
| **Promo** | промокод, скидк, купон | Активация промо |
| **Complaint** | жалоб, проблем, плох | Обработка жалоб |
| **FAQ** | как работа, режим, где наход | Частые вопросы |

#### Business Intelligence Intents (3) 💼 NEW!
| Intent | Ключевые слова | Действие |
|--------|----------------|----------|
| **AnalyzeBusiness** | проанализируй, метрики, оценка, analyze | Investment scoring (0-100) |
| **CompareBusinesses** | сравни, compare, vs, что лучше | Multi-business comparison |
| **BusinessInsights** | как улучшить, рекомендации, insights, советы | AI recommendations |

**Возможности Business Intelligence:**
- ✅ **Investment Scoring**: Автоматический балл 0-100
- ✅ **Trend Analysis**: Рост (+10%), стагнация (±5%), падение (-10%)
- ✅ **ROI Calculation**: Рентабельность для инвесторов
- ✅ **Multi-comparison**: Сравнение 2-10 бизнесов
- ✅ **AI Recommendations**: Приоритизированные советы
- ⚡ **Performance**: 80-550ms response time

### AI System Prompt v2.2
```
Ты — умный и дружелюбный менеджер ресторана морепродуктов FodiFood
и опытный бизнес-аналитик для инвесторов.

Для клиентов ресторана:
- Помогаешь выбрать блюда, отвечаешь на вопросы о меню
- Помогаешь оформить заказ
- Даешь рекомендации с учетом предпочтений

Для менеджеров:
- Анализируешь продажи, остатки
- Даешь рекомендации по закупкам
- Помогаешь оптимизировать бизнес

Для инвесторов:
- Анализируешь метрики бизнесов
- Рассчитываешь investment score (0-100)
- Сравниваешь несколько бизнесов
- Даешь рекомендации по инвестициям
- Определяешь тренды (рост/стагнация/падение)

Отвечай вежливо, по делу, кратко и информативно.
Используй эмодзи когда это уместно.
Для инвесторов используй: 📊📈💰🚀💡🏆
```

### AI API Parameters
- **Model**: `gpt-4o-mini`
- **Temperature**: `0.7` (баланс креативности/точности)
- **Max tokens**: `800` (для бизнес-анализа)
- **Stream**: `false`

### Cognitive Analysis (Thinker)
```rust
pub struct CognitiveState {
    pub complexity: f32,      // 0.0-1.0
    pub sentiment: Sentiment, // Positive/Neutral/Negative
    pub urgency: f32,         // 0.0-1.0
    pub confidence: f32,      // 0.0-1.0
}
```

### Memory System
- **In-Memory**: Текущая сессия (DashMap)
- **Persistent**: sled database
- **Context window**: Последние 10 сообщений
- **User preferences**: Долгосрочное хранение

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

## 📦 Зависимости (ключевые) v2.3

| Crate | Version | Назначение |
|-------|---------|------------|
| **Web & Async** | | |
| shuttle-runtime | 0.57 | Deployment platform |
| shuttle-axum | 0.57 | Axum integration |
| axum | 0.8 | Web framework |
| tokio | 1.48 | Async runtime |
| tower | 0.4/0.5 | Middleware |
| tower-http | 0.5/0.6 | HTTP middleware (CORS) |
| **Serialization** | | |
| serde | 1.0 | Serialization framework |
| serde_json | 1.0 | JSON support |
| bincode | 1.3 | Binary serialization |
| **HTTP Client** | | |
| reqwest | 0.12 | HTTP client |
| **WebSocket** | | |
| futures | 0.3 | Async utilities |
| futures-util | 0.3 | Stream utilities |
| **Auth & Security** | | |
| jsonwebtoken | 9.3 | JWT tokens |
| **Storage** | | |
| sled | 0.34 | Embedded database |
| dashmap | 6.0 | Concurrent HashMap |
| **Blockchain (Solana)** 🪙 | | |
| solana-client | 2.0 | Solana RPC client |
| solana-sdk | 2.0 | Solana SDK |
| solana-program | 2.0 | On-chain programs |
| spl-token | 7.0 | SPL Token standard |
| spl-associated-token-account | 6.0 | Associated accounts |
| mpl-token-metadata | 5.0 | Metaplex metadata |
| **Utilities** | | |
| uuid | 1.0 | Unique IDs |
| chrono | 0.4 | Date/time |
| rand | 0.8 | Random generation |
| anyhow | 1.0 | Error handling |
| thiserror | 1.0 | Custom errors |
| **Logging** | | |
| tracing | 0.1 | Structured logging |
| tracing-subscriber | 0.3 | Log subscriber |
| **Environment** | | |
| dotenvy | 0.15 | .env file support |
| **Async Traits** | | |
| async-trait | 0.1 | Async trait support |
| **Testing** | | |
| tempfile | 3.8 | Temporary files |

**Total Dependencies**: 85+ crates

## 🪙 Solana Blockchain Integration (NEW!)

### FODI Token
- **Mint Address**: `F9qcQ2HEmjDXmUygFiJjeiMHeF5PYSGnfzhRbETeP8Ek`
- **Symbol**: FODI
- **Name**: FODI Token
- **Decimals**: 9
- **Total Supply**: 1,000,000,000
- **Network**: Solana Devnet
- **Standard**: SPL Token (Token-2022)

### Метаданные (Metaplex)
```json
{
  "name": "FODI Token",
  "symbol": "FODI",
  "description": "Investment token for FodiFood platform",
  "image": "https://raw.githubusercontent.com/.../logo.png",
  "attributes": [
    {"trait_type": "Category", "value": "Investment"},
    {"trait_type": "Platform", "value": "FodiFood"},
    {"trait_type": "Total Supply", "value": "1,000,000,000"}
  ]
}
```

### Binaries
```bash
# Создать новый токен
cargo run --bin create_fodi_token

# Добавить метаданные
cargo run --bin add_fodi_metadata

# Параметры из .env.fodi
FODI_MINT_ADDRESS=F9qcQ2HEmjDXmUygFiJjeiMHeF5PYSGnfzhRbETeP8Ek
SOLANA_KEYPAIR_PATH=~/.config/solana/id.json
METADATA_URI=https://raw.githubusercontent.com/.../fodi-metadata.json
```

### Интеграция с кошельками
- ✅ Phantom Wallet
- ✅ Solflare
- ✅ Backpack
- ✅ Любой SPL-совместимый кошелек

### API Endpoints
```rust
// Информация о токене
GET /api/v1/solana/token/{mint}

// Перевод токенов
POST /api/v1/solana/transfer
{
  "from": "keypair_path",
  "to": "recipient_address",
  "amount": 100.0
}
```

## 📊 Metrics & Monitoring v2.2

### Prometheus Metrics
```prometheus
# Intent distribution
ai_intent_total{intent="menu_query"} 456
ai_intent_total{intent="order_create"} 234
ai_intent_total{intent="business_analysis"} 89

# Response times
ai_response_duration_seconds_bucket{le="0.5"} 234
ai_response_duration_seconds_sum 125.3
ai_response_duration_seconds_count 1024

# Connections
websocket_connections_active 12
websocket_messages_total 5678

# Backend health
backend_health_status 1.0
backend_uptime_seconds 3600
backend_restart_count 0

# Memory
ai_memory_size_bytes 1048576
ai_memory_users_total 45
```

### Web Dashboard
```
GET /admin/metrics
```

HTML dashboard с:
- 📈 Графики в реальном времени
- ⏱️ Средние времена ответа
- 🎯 Топ intents
- 👥 Активные пользователи
- ✅ Success rate

### AI Insights Events
Через WebSocket `/api/v1/insight`:
```json
{
  "event_type": "intent_classified",
  "timestamp": "2025-10-20T01:30:00Z",
  "data": {
    "intent": "AnalyzeBusiness",
    "confidence": 0.95,
    "user_id": "investor_123",
    "processing_time_ms": 245
  }
}
```

## 🎯 Backend Orchestration v2.2

### Process Management
```rust
pub struct BackendOrchestrator {
    process: Option<Child>,
    pid: Option<u32>,
    uptime: Instant,
    restart_count: u32,
    health_checker: Arc<HealthChecker>,
}
```

### Health Monitoring
- **Interval**: 30 секунд
- **Timeout**: 5 секунд
- **Auto-restart**: При 3 failed checks
- **Max restarts**: 5 в час

### Status Tracking
```json
{
  "status": "running",
  "pid": 12345,
  "uptime_secs": 3600,
  "restart_count": 0,
  "last_health_check": "healthy",
  "is_running": true
}
```

## 💼 Business Intelligence v2.2

### Investment Scoring Algorithm
```rust
fn calculate_score(metrics: &BusinessMetrics) -> f32 {
    let mut score = 0.0;
    
    // Price trend (40% weight)
    if price_change > 10.0 { score += 40.0; }
    else if price_change > 0.0 { score += 20.0; }
    
    // Investor count (30% weight)
    score += (investor_count as f32 * 5.0).min(30.0);
    
    // ROI (30% weight)
    if roi > 100.0 { score += 30.0; }
    else if roi > 50.0 { score += 20.0; }
    else if roi > 0.0 { score += 10.0; }
    
    score.min(100.0)
}
```

### Trend Classification
```rust
pub enum Trend {
    Rising,      // +10% or more
    Stagnant,    // ±5%
    Falling,     // -10% or less
}
```

### Performance
- **Single analysis**: 80-150ms
- **Comparison (2 businesses)**: 150-300ms
- **Comparison (5+ businesses)**: 300-550ms
- **Caching**: Go backend data cached

## 🎯 Ключевые особенности v2.3

1. ✨ **Полностью асинхронный** - высокая производительность (Tokio)
2. 🤖 **AI-powered** - 17 intent handlers + GPT-4o-mini
3. 💼 **Business Intelligence** - инвестиционный анализ с scoring
4. 🔐 **Безопасный** - JWT auth, RBAC, protected keypairs
5. 📡 **Real-time** - WebSocket для чата, уведомлений, AI insights
6. 📊 **Мониторинг** - Prometheus метрики, web dashboard
7. 🔄 **Интегрированный** - Next.js + Go Backend + Solana
8. 🪙 **Blockchain** - Solana SPL tokens + Metaplex
9. 🎯 **Orchestration** - автоматическое управление backend
10. 🚀 **Production-ready** - Shuttle deployment, auto-scaling
11. � **Масштабируемый** - 1000+ concurrent connections
12. 🛠️ **Расширяемый** - модульная архитектура, легко добавлять функции
13. 🧪 **Тестируемый** - 60+ unit tests, integration tests
14. 📚 **Документированный** - подробная документация + примеры

---

**Создано с Rust 🦀 для FodiFood 🦐**

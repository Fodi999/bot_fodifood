# FodiFood Intelligent Bot 🦐🤖

**v2.2** - Advanced AI Restaurant Bot with Backend Orchestration

Интеллектуальный бот на Rust для ресторана FodiFood - центральный коммуникационный узел между клиентами, администраторами и бизнес-логикой с расширенными возможностями мониторинга и управления.

## 🚀 Что нового в v2.2

### ✅ Завершено (90% готовности):

**Step 1: Rules Migration** ✅
- Миграция на модульную архитектуру intent handlers
- 17 специализированных обработчиков намерений
- Улучшенная система распознавания интентов

**Step 2: Metrics Dashboard** ✅
- Prometheus-совместимые метрики
- Веб-дашборд для мониторинга
- Отслеживание intent'ов и времени ответа
- JSON/Prometheus форматы экспорта

**Step 3: WebSocket Insight Layer** ✅
- Real-time стриминг AI событий обработки
- 9 типов событий (классификация, entity extraction, и т.д.)
- WebSocket endpoint `/api/v1/insight` для фронтенда
- Broadcast система для множественных клиентов

**Step 4: Go Backend Orchestration** ✅
- Управление жизненным циклом Go процесса
- Health monitoring с автоматическим перезапуском
- REST API для управления backend'ом
- Status tracking и uptime мониторинг

**Step 5: Business Intelligence Module** ✅ **NEW!**
- **Go Backend Integration**: HTTP клиент для бизнес-данных
- **AI Business Analysis**: Инвестиционный скоринг (0-100), тренды, ROI
- **3 специализированных интента**:
  - `AnalyzeBusiness` - Анализ метрик бизнеса
  - `CompareBusinesses` - Сравнение 2+ бизнесов
  - `BusinessInsights` - AI-советы по улучшению
- **Performance**: 80-550ms response time
- **Full Integration**: Neon PostgreSQL → Go API → Rust Analysis

**Step 6: Admin AI Assistant** 🔲 (В разработке)
- AI-ассистент для администраторов
- Команды управления через чат

## 🏗️ Архитектура

```
┌─────────────────┐    WebSocket + REST      ┌──────────────────────┐
│   Next.js       │◄─────────────────────────►│   Rust Bot v2.2      │
│   Frontend      │     wss:// + https://     │   (Shuttle.rs)       │
│   (Vercel)      │                            │                      │
└─────────────────┘                            │  🧠 AI Engine        │
                                               │  📊 Metrics          │
                                               │  📡 Insight WS       │
                                               │  🎯 Orchestrator     │
                                               └──────────┬───────────┘
                                                          │
                                                          │ REST API
                                                          │ Health Checks
                                                          ▼
                                                 ┌──────────────────┐
                                                 │   Go Backend     │
                                                 │   (Koyeb)        │
                                                 │   Business Logic │
                                                 └──────────────────┘
```

## ✨ Возможности

### 🤖 AI Engine (v2.2)
- **17 Intent Handlers**: Модульная архитектура обработчиков
  - 14 базовых (меню, заказы, аналитика, рекомендации)
  - **3 бизнес-интента** (анализ, сравнение, советы)
- **Cognitive Analysis**: Определение настроения, эмоций, сложности
- **Natural Language**: Понимание русского языка с контекстом
- **Smart Routing**: Автоматическая маршрутизация запросов
- **Memory System**: Персонализация на основе истории
- **Business Intelligence**: Инвестиционный анализ и AI-рекомендации

### 📊 Metrics & Monitoring
- **Prometheus Metrics**: `/metrics` endpoint
- **Web Dashboard**: `/admin/metrics` - визуальный дашборд
- **Intent Tracking**: Статистика использования intent'ов
- **Response Times**: Мониторинг производительности
- **Success Rates**: Отслеживание успешности обработки

### 📡 Real-time Insights
- **WebSocket Stream**: `/api/v1/insight` - события обработки AI
- **Event Types**: 9 типов событий (classification, extraction, routing, etc.)
- **Client Management**: Broadcast для множественных клиентов
- **Debugging**: Визуализация AI pipeline в реальном времени

### 🎯 Backend Orchestration
- **Process Control**: Start/Stop/Restart Go backend
- **Health Monitoring**: Автоматические health checks
- **Auto-restart**: Восстановление при сбоях
- **Status API**: `/api/v1/admin/backend/status`
- **PID Tracking**: Мониторинг процесса и uptime

### 🔐 Аутентификация
- JWT-токены от Go backend
- Роли: Client, Admin, Manager, Courier, Cook
- Автоматическая валидация при подключении

### 💬 WebSocket коммуникация
- Реал-тайм чат с клиентами
- Командный интерфейс для управления
- Автоматические уведомления админам
- AI Insight streaming

### 📡 Webhook система
- Получение событий от Go backend
- Рассылка уведомлений по ролям
- События: new_order, order_status_changed, low_inventory

### 🔄 Интеграция с Go Backend
- `/api/auth/verify` - проверка токенов
- `/api/products` - меню
- `/api/orders` - заказы
- `/api/ingredients` - склад
- `/api/stats` - статистика
- **`/api/businesses`** - список бизнесов (NEW!)
- **`/api/metrics/:id`** - метрики бизнеса (NEW!)

### 💼 Business Intelligence API (NEW!)
- **Investment Scoring**: Автоматический расчет балла 0-100
- **Trend Analysis**: Определение роста/стагнации/падения
- **ROI Calculation**: Рентабельность инвестиций
- **Multi-business Comparison**: Сравнение показателей
- **AI Recommendations**: Персонализированные советы по улучшению
- **Performance**: 80-550ms анализ с кэшированием

## 🚀 Быстрый старт

### Требования
- Rust 1.75+
- Shuttle CLI: `cargo install cargo-shuttle`

### Установка

1. **Клонируйте репозиторий**
```bash
cd bot_fodifood
```

2. **Настройте переменные окружения**
```bash
cp .env.example .env
```

Отредактируйте `.env`:
```env
OPENAI_API_KEY=sk-your-openai-key
GO_BACKEND_URL=https://your-go-backend.koyeb.app/api
JWT_SECRET=your-jwt-secret
RUST_LOG=info
```

3. **Локальная разработка**
```bash
cargo shuttle run
```

Бот будет доступен на `http://localhost:8000`

4. **Деплой на Shuttle**
```bash
# Логин (если первый раз)
cargo shuttle login

# Деплой
cargo shuttle deploy
```

После деплоя ваш WebSocket будет доступен:
```
wss://fodifood-bot.shuttleapp.rs/ws
```

## 📝 Примеры использования

### Интеллектуальный диалог с AI

```bash
# WebSocket чат (с AI insights)
wscat -c wss://your-app.shuttleapp.rs/ws
> {"type":"message","content":"Покажи меню на сегодня","user_id":"user123"}

# AI отправляет событие "intent_detected" через /api/v1/insight
# Затем приходит ответ с рекомендациями

# REST API
curl -X POST https://your-app.shuttleapp.rs/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Хочу заказать пиццу Маргарита",
    "user_id": "user123"
  }'
```

### Мониторинг AI системы (v2.2)

```bash
# Prometheus метрики
curl http://localhost:8000/api/v1/metrics/prometheus

# Метрики по интентам
curl http://localhost:8000/api/v1/metrics/intents
{
  "total_intents": 1523,
  "by_type": {
    "menu_query": 456,
    "order_create": 234,
    "recommendation_request": 189
  }
}

# AI Insights WebSocket (real-time events)
wscat -c ws://localhost:8000/api/v1/insight
# Получаете события: intent_detected, rule_matched, response_generated
```

### Управление Backend (v2.2)

```bash
# Запуск Go backend
curl -X POST http://localhost:8000/api/v1/admin/backend/start

# Статус системы
curl http://localhost:8000/api/v1/admin/backend/status
{
  "status": "running",
  "pid": 12345,
  "uptime_secs": 3600,
  "restart_count": 0
}

# Перезапуск при необходимости
curl -X POST http://localhost:8000/api/v1/admin/backend/restart
```

### Примеры AI Intent Handling

```python
# Меню запрос
User: "Что у вас есть из десертов?"
AI: [intent: menu_query, category: desserts]
Response: "🍰 У нас есть тирамису, чизкейк, панна-котта..."

# Создание заказа
User: "Закажи мне капучино и круассан"
AI: [intent: order_create, items: ["капучино", "круассан"]]
Response: "✅ Заказ создан: Капучино + Круассан (350₽)"

# Рекомендации
User: "Что посоветуешь к стейку?"
AI: [intent: recommendation_request, context: "стейк"]
Response: "🍷 К стейку отлично подойдет красное вино Cabernet..."

# Аналитика
User: "Покажи статистику заказов за неделю"
AI: [intent: analytics_query, period: "week"]
Response: "📊 За последнюю неделю: 234 заказа, средний чек 850₽..."
```

### 📊 Metrics Endpoints (v2.2)

#### GET `/metrics`
Prometheus-совместимые метрики

```bash
curl https://fodifood-bot.shuttleapp.rs/metrics
```

#### GET `/admin/metrics`
Веб-дашборд для мониторинга (HTML)

#### GET `/admin/metrics/intents`
Статистика по intent'ам (JSON)

```json
{
  "menu": {"count": 150, "avg_time_ms": 45},
  "order": {"count": 89, "avg_time_ms": 120}
}
```

#### GET `/admin/metrics/stats`
Общая статистика системы

```json
{
  "total_requests": 1024,
  "avg_response_time_ms": 78,
  "success_rate": 0.97
}
```

### � Business Intelligence API (v2.2) **NEW!**

Интеграция с Go backend для анализа бизнес-метрик и AI-рекомендаций.

#### POST `/api/v1/chat` - Анализ бизнеса

```bash
# Анализ одного бизнеса
curl -X POST https://fodifood-bot.shuttleapp.rs/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "investor1",
    "message": "проанализируй бизнес Tech Startup"
  }'

# Ответ
{
  "intent": "AnalyzeBusiness",
  "response": "🏢 **Tech Startup**\n\n🟢 TecT - $28.13 (+48.1%) | 2 инвесторов | ROI: 235.1%\n\n💡 Анализ:\n📊 Оценка: 90/100 - Отличные показатели\n🚀 Цена растёт на 48.1%\n💰 Высокая доходность (ROI 235.1%)\n\n💎 Рекомендация: ПОКУПАТЬ"
}
```

#### POST `/api/v1/chat` - Сравнение бизнесов

```bash
# Сравнение 2+ бизнесов
curl -X POST https://fodifood-bot.shuttleapp.rs/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "investor1",
    "message": "сравни Tech Startup и Fodi Sushi"
  }'

# Ответ
{
  "intent": "CompareBusinesses",
  "response": "📊 Сравнительный анализ:\n\n🏆 Лидеры:\n• Рост: Tech Startup (+48.1%)\n• Инвесторы: Tech Startup (2)\n• ROI: Tech Startup (235.1%)\n\n📈 Детали:\nTech Startup: 90/100 | $28.13\nFodi Sushi: 20/100 | $15.75"
}
```

#### POST `/api/v1/chat` - AI-советы

```bash
# Персонализированные рекомендации
curl -X POST https://fodifood-bot.shuttleapp.rs/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "investor1",
    "message": "как улучшить Fodi Sushi"
  }'

# Ответ
{
  "intent": "BusinessInsights",
  "response": "💡 **Советы для: Fodi Sushi**\n\n🎯 Оценка: 20/100\n\n📉 Падение цены (-17.1%)\n• Увеличьте маркетинг\n• PR-кампания\n\n👥 Мало инвесторов\n• Снизьте порог входа\n• Реферальная программа\n\nПриоритет: 1️⃣ Стабилизация цены"
}
```

**Возможности:**
- ✅ **Investment Scoring**: Автоматический балл 0-100 на основе метрик
- ✅ **Trend Analysis**: Определение роста (+10%), стагнации (±5%), падения (-10%)
- ✅ **ROI Calculation**: Расчёт рентабельности для инвесторов
- ✅ **Multi-comparison**: Сравнение 2-10 бизнесов одновременно
- ✅ **AI Recommendations**: Приоритизированные советы на основе слабых мест
- ✅ **Performance**: 80-550ms response time

**Ключевые слова:**
- Анализ: `"проанализируй"`, `"метрики"`, `"оценка"`, `"analyze"`
- Сравнение: `"сравни"`, `"compare"`, `"vs"`, `"что лучше"`
- Советы: `"как улучшить"`, `"рекомендации"`, `"insights"`, `"советы"`

### �📡 AI Insights WebSocket (v2.2)

#### WebSocket: `/api/v1/insight`
Real-time стриминг AI событий обработки

```javascript
const ws = new WebSocket('wss://fodifood-bot.shuttleapp.rs/api/v1/insight?client_id=admin_123');

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('AI Event:', data);
  // {type: "intent_classified", intent: "menu", confidence: 0.95, ...}
};
```

**Типы событий:**
- `intent_classification_started` - Начало классификации
- `intent_classified` - Intent определен (с confidence)
- `entity_extraction` - Извлечены сущности
- `handler_routing` - Маршрутизация к handler'у
- `handler_execution_started` - Начало выполнения
- `handler_execution_completed` - Выполнение завершено
- `context_updated` - Обновлен контекст
- `processing_completed` - Обработка завершена
- `processing_error` - Ошибка обработки

### 🎯 Backend Control Endpoints (v2.2)

#### POST `/api/v1/admin/backend/start`
Запустить Go backend процесс

```bash
curl -X POST https://fodifood-bot.shuttleapp.rs/api/v1/admin/backend/start
```

#### POST `/api/v1/admin/backend/stop`
Остановить Go backend процесс

#### POST `/api/v1/admin/backend/restart`
Перезапустить Go backend

#### GET `/api/v1/admin/backend/status`
Получить статус backend процесса

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

#### GET `/api/v1/admin/backend/health`
Health check оркестратора

```json
{
  "status": "ok",
  "service": "backend_orchestrator",
  "enabled": true
}
```

### Admin & Auth Endpoints

#### GET `/health`
Service health check

#### POST `/api/v1/auth/login`
Авторизация пользователя

#### GET `/api/v1/products`
Получить список продуктов

#### GET `/api/v1/admin/stats`
Административная статистика

#### GET `/api/v1/admin/orders`
Список всех заказов (admin only)

#### GET `/api/v1/admin/users`
Список пользователей (admin only)

#### WebSocket: `/api/v1/admin/ws`
Admin WebSocket для управления

### WebSocket: `/ws`

**Подключение:**
```javascript
const ws = new WebSocket('wss://fodifood-bot.shuttleapp.rs/ws');
```

**Аутентификация:**
```json
{
  "type": "auth",
  "token": "eyJhbGciOiJIUzI1NiIs..."
}
```

**Ответ:**
```json
{
  "type": "auth_success",
  "user_id": "user123",
  "role": "Client"
}
```

**Сообщение в чат:**
```json
{
  "type": "chat",
  "text": "Покажите меню"
}
```

**Команда:**
```json
{
  "type": "command",
  "action": "get_menu",
  "params": null
}
```

**Команды:**
- `get_menu` - получить меню
- `get_orders` - получить заказы (admin)
- `create_order` - создать заказ

### HTTP POST: `/notify`

Webhook для событий от Go backend:

```json
{
  "event": "new_order",
  "order_id": 128,
  "total": 5400,
  "user_id": "user123"
}
```

**События:**
- `new_order` - новый заказ
- `order_status_changed` - изменение статуса
- `low_inventory` - низкие остатки

### HTTP GET: `/health`

Проверка здоровья сервиса.

## 🧠 AI функции

### Детекция интентов
Бот автоматически определяет намерения пользователя:
- Создание заказа
- Проверка статуса
- Просмотр меню
- Запрос рекомендаций
- Проверка остатков (staff)
- Статистика (staff)

### Примеры взаимодействия

**Клиент:**
```
"Что у вас есть из креветок?"
→ AI формирует ответ на основе меню из Go backend
```

**Менеджер:**
```
"Какие блюда продаются лучше всего?"
→ AI анализирует статистику и дает рекомендации
```

## 🏗️ Структура проекта v2.2

```
src/
├── main.rs                  # Точка входа Shuttle
├── lib.rs                   # Library root
├── config.rs                # Конфигурация
├── state.rs                 # AppState с orchestrator
│
├── ai/                      # 🧠 AI Engine v2.2
│   ├── mod.rs               # AIEngine с process_with_insights()
│   ├── intents.rs           # Intent классификация (17 intents)
│   ├── intent_handler.rs    # Plugin system для handlers
│   ├── thinker.rs           # Cognitive analysis
│   ├── memory.rs            # In-memory context
│   ├── persistent_memory.rs # Persistent storage (sled)
│   ├── analysis.rs          # 💼 Business analysis AI (NEW!)
│   ├── modules/             # 📦 Intent Handlers (17 total)
│   │   ├── menu.rs          # Меню queries
│   │   ├── orders.rs        # Управление заказами
│   │   ├── recommendations.rs # Рекомендации
│   │   ├── analytics.rs     # Статистика
│   │   ├── smalltalk.rs     # Small talk
│   │   ├── news.rs          # Новости
│   │   └── business.rs      # 💼 Business intelligence (NEW!)
│   └── rules/               # Rule-based responses
│       ├── menu.rs
│       ├── orders.rs
│       ├── recommendations.rs
│       ├── analytics.rs
│       └── smalltalk.rs
│
├── api/                     # 🌐 API Layer
│   ├── rest.rs              # REST endpoints + /api/v1/chat
│   ├── metrics.rs           # 📊 Metrics endpoints
│   ├── admin_ws.rs          # Admin WebSocket
│   ├── insight_ws.rs        # 📡 AI Insight WebSocket
│   ├── backend_control.rs   # 🎯 Backend control API
│   └── go_backend/          # Go backend integration
│       ├── mod.rs
│       ├── auth.rs
│       ├── products.rs
│       ├── orders.rs
│       ├── admin.rs
│       └── types.rs
│
├── services/                # 🔌 External Services (NEW!)
│   ├── mod.rs
│   └── go_client.rs         # 💼 Go Backend HTTP Client
│                            #    - fetch_businesses()
│                            #    - fetch_business_metrics()
│                            #    - Business & BusinessMetrics types
│
├── handlers/                # 🔌 Protocol Handlers
│   ├── ws.rs                # WebSocket handler
│   ├── webhook.rs           # Webhook handler
│   ├── insight_events.rs    # 📡 AI event types
│   └── insight_broadcaster.rs # WebSocket broadcaster
│
├── orchestration/           # 🎯 Backend Orchestration (v2.2)
│   ├── mod.rs
│   ├── backend.rs           # Process lifecycle management
│   └── health.rs            # Health checker
│
├── metrics/                 # 📊 Metrics System (v2.2)
│   └── mod.rs               # MetricsCollector
│
├── models/                  # 📋 Data Models
│   ├── message.rs
│   └── user.rs
│
└── bin/                     # 🔧 Binaries
    ├── chat.rs              # CLI chat client
    └── local.rs             # Local dev server
```

## � Примеры использования

### Интеллектуальный диалог с AI

```bash
# WebSocket чат (с AI insights)
wscat -c wss://your-app.shuttleapp.rs/ws
> {"type":"message","content":"Покажи меню на сегодня","user_id":"user123"}

# AI отправляет событие "intent_detected" через /api/v1/insight
# Затем приходит ответ с рекомендациями

# REST API
curl -X POST https://your-app.shuttleapp.rs/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Хочу заказать пиццу Маргарита",
    "user_id": "user123"
  }'
```

### Мониторинг AI системы (v2.2)

```bash
# Prometheus метрики
curl http://localhost:8000/api/v1/metrics/prometheus

# Метрики по интентам
curl http://localhost:8000/api/v1/metrics/intents
{
  "total_intents": 1523,
  "by_type": {
    "menu_query": 456,
    "order_create": 234,
    "recommendation_request": 189
  }
}

# AI Insights WebSocket (real-time events)
wscat -c ws://localhost:8000/api/v1/insight
# Получаете события: intent_detected, rule_matched, response_generated
```

### Управление Backend (v2.2)

```bash
# Запуск Go backend
curl -X POST http://localhost:8000/api/v1/admin/backend/start

# Статус системы
curl http://localhost:8000/api/v1/admin/backend/status
{
  "status": "running",
  "pid": 12345,
  "uptime_secs": 3600,
  "restart_count": 0
}

# Перезапуск при необходимости
curl -X POST http://localhost:8000/api/v1/admin/backend/restart
```

### Примеры AI Intent Handling

```python
# Меню запрос
User: "Что у вас есть из десертов?"
AI: [intent: menu_query, category: desserts]
Response: "🍰 У нас есть тирамису, чизкейк, панна-котта..."

# Создание заказа
User: "Закажи мне капучино и круассан"
AI: [intent: order_create, items: ["капучино", "круассан"]]
Response: "✅ Заказ создан: Капучино + Круассан (350₽)"

# Рекомендации
User: "Что посоветуешь к стейку?"
AI: [intent: recommendation_request, context: "стейк"]
Response: "🍷 К стейку отлично подойдет красное вино Cabernet..."

# Аналитика
User: "Покажи статистику заказов за неделю"
AI: [intent: analytics_query, period: "week"]
Response: "📊 За последнюю неделю: 234 заказа, средний чек 850₽..."
```

## 🔧 Разработка и тестирование

### Локальная разработка

```bash
# Запуск локального сервера (порт 8000)
cargo run --bin local

# Запуск с полным логированием
RUST_LOG=debug cargo run --bin local

# Release build
cargo build --release

# Форматирование кода
cargo fmt

# Linting с warnings
cargo clippy -- -D warnings
```

### Тестирование компонентов v2.2

```bash
# Запуск всех тестов (60+ tests)
cargo test

# Тест AI Engine с insights
cargo test ai::tests --nocapture

# Тест метрик системы
cargo test metrics::tests

# Тест Backend Orchestrator
cargo test orchestration::tests --nocapture

# Тест WebSocket Insights
cargo test handlers::insight --nocapture

# Тест Business Intelligence (NEW!)
cargo test business --nocapture

# Тест с выводом логов
cargo test -- --nocapture
```

### Debug утилиты

```bash
# CLI чат клиент для отладки AI
cargo run --bin chat
> Привет, покажи меню
> Закажи мне кофе

# Тест метрик endpoints
curl http://localhost:8000/api/v1/metrics/dashboard | jq
curl http://localhost:8000/api/v1/metrics/intents | jq

# Подключение к Insight WebSocket (real-time AI events)
wscat -c ws://localhost:8000/api/v1/insight

# Проверка здоровья backend orchestrator
curl http://localhost:8000/api/v1/admin/backend/health | jq

# 💼 Business Intelligence тесты (NEW!)
# Тест анализа бизнеса
./test_api.sh

# Детальный тест всех 3 интентов
curl -X POST http://localhost:8000/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{"user_id":"test","message":"проанализируй бизнес Tech Startup"}' | jq '.'
  
curl -X POST http://localhost:8000/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{"user_id":"test","message":"сравни Tech Startup и Fodi Sushi"}' | jq '.'

curl -X POST http://localhost:8000/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{"user_id":"test","message":"как улучшить Fodi Sushi"}' | jq '.'
```

### Тестирование Backend Control API

```bash
# Статус backend (без запуска orchestrator)
curl http://localhost:8000/api/v1/admin/backend/status
# Response: {"status":"disabled","message":"Backend orchestration not enabled"}

# Если orchestrator включен:
curl -X POST http://localhost:8000/api/v1/admin/backend/start
curl -X POST http://localhost:8000/api/v1/admin/backend/stop
curl -X POST http://localhost:8000/api/v1/admin/backend/restart
```

## 📊 Monitoring & Analytics

### Метрики системы (v2.2)

**Доступные метрики:**
- ✅ **Intent distribution** - распределение по типам запросов
- ✅ **Response times** - latency metrics для AI обработки
- ✅ **Success/failure rates** - процент успешных ответов
- ✅ **Active connections** - количество WebSocket подключений
- ✅ **Backend health status** - состояние Go backend
- ✅ **Memory usage** - использование памяти AI контекстом

**Prometheus Integration:**
```bash
# Prometheus scrape endpoint
curl http://localhost:8000/api/v1/metrics/prometheus

# Пример метрик:
# ai_intent_total{intent="menu_query"} 456
# ai_response_duration_seconds_bucket{le="0.5"} 234
# websocket_connections_active 12
# backend_health_status 1.0
# ai_memory_size_bytes 1048576
```

### AI Insights Real-time Events

**События через WebSocket `/api/v1/insight`:**

| Event Type | Description | Данные |
|------------|-------------|--------|
| `intent_detected` | AI определил намерение | `{intent, confidence}` |
| `rule_matched` | Сработало правило | `{rule_id, priority}` |
| `context_updated` | Обновлен контекст памяти | `{memory_size}` |
| `response_generated` | AI сгенерировал ответ | `{response_time}` |
| `cognitive_analysis` | Завершен анализ Thinker | `{cognitive_state}` |
| `backend_called` | Вызов Go backend API | `{endpoint, status}` |
| `metric_recorded` | Записана метрика | `{metric_type, value}` |
| `processing_started` | Начало обработки запроса | `{request_id}` |
| `error` | Ошибка обработки | `{error_message}` |

**Пример подключения:**
```javascript
const ws = new WebSocket('ws://localhost:8000/api/v1/insight');

ws.onmessage = (event) => {
  const insight = JSON.parse(event.data);
  console.log(`[${insight.event_type}]`, insight.data);
  
  if (insight.event_type === 'intent_detected') {
    console.log(`Intent: ${insight.data.intent}, Confidence: ${insight.data.confidence}`);
  }
};
```

## �🔧 Настройка Go Backend

В вашем Go backend нужно настроить:

### 1. Webhook URL
После деплоя бота установите в Go backend:
```
WEBHOOK_URL=https://fodifood-bot.shuttleapp.rs/notify
```

### 2. Endpoint для верификации токенов
```go
POST /api/auth/verify
{
  "token": "jwt-token-here"
}

// Response:
{
  "valid": true,
  "user_id": "user123",
  "role": "client"
}
```

### 3. Отправка webhook событий
```go
type Event struct {
    Event   string      `json:"event"`
    Data    interface{} `json:"data"`
}

func notifyBot(event string, data interface{}) {
    payload := Event{Event: event, Data: data}
    http.Post(webhookURL+"/notify", "application/json", ...)
}

// Примеры:
notifyBot("new_order", map[string]interface{}{
    "order_id": 128,
    "total": 5400,
})

notifyBot("low_inventory", map[string]interface{}{
    "ingredient": "креветки",
    "quantity": 2.5,
})
```

## 🔐 Безопасность

- JWT токены проверяются через Go backend
- WebSocket требует аутентификации
- Роли проверяются для каждой команды
- CORS настроен (можно ограничить в production)

## 📊 Production Мониторинг

### Логи Shuttle
```bash
# Просмотр логов в production
cargo shuttle logs

# Логи с фильтром
cargo shuttle logs --follow

# Последние 100 строк
cargo shuttle logs --tail 100
```

### Уровни логирования
Настройте через переменную `RUST_LOG`:
- `error` - только критичные ошибки
- `warn` - предупреждения и ошибки
- `info` - информационные сообщения (по умолчанию)
- `debug` - детальная отладка AI engine
- `trace` - максимальная детализация (включая HTTP запросы)

**Пример:**
```bash
# В Secrets.toml для Shuttle
RUST_LOG = "info,fodifood_bot=debug"
```

## 🧪 Production Testing

### WebSocket тест (через websocat)
```bash
# Установка websocat
cargo install websocat

# Подключение к production
websocat wss://fodifood-bot.shuttleapp.rs/ws

# Отправка сообщений
{"type":"auth","token":"your-jwt-token"}
{"type":"chat","text":"Покажите меню"}

# Подключение к AI Insights
websocat wss://fodifood-bot.shuttleapp.rs/api/v1/insight
```

### Webhook тест
```bash
# Тестирование webhook endpoint
curl -X POST https://fodifood-bot.shuttleapp.rs/notify \
  -H "Content-Type: application/json" \
  -d '{"event":"new_order","order_id":999,"total":1200}'
```

### Health Check
```bash
# Проверка health endpoint
curl https://fodifood-bot.shuttleapp.rs/health

# Проверка backend orchestrator
curl https://fodifood-bot.shuttleapp.rs/api/v1/admin/backend/health | jq
```

## 🌐 Интеграция с Next.js

```typescript
// lib/websocket.ts
const WS_URL = 'wss://fodifood-bot.shuttleapp.rs/ws';

class BotClient {
  private ws: WebSocket | null = null;

  connect(token: string) {
    this.ws = new WebSocket(WS_URL);
    
    this.ws.onopen = () => {
      this.send({ type: 'auth', token });
    };

    this.ws.onmessage = (event) => {
      const message = JSON.parse(event.data);
      console.log('Received:', message);
      // Handle message based on type
    };
  }

  send(data: any) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(data));
    }
  }

  sendChat(text: string) {
    this.send({ type: 'chat', text });
  }

  sendCommand(action: string, params?: any) {
    this.send({ type: 'command', action, params });
  }
}

export const botClient = new BotClient();
```

## 📈 Масштабирование

Shuttle автоматически масштабирует приложение. Для улучшения производительности:

1. **Кэширование** - добавьте Redis для кэша меню/продуктов
2. **Rate limiting** - ограничьте частоту запросов к AI
3. **Message queue** - используйте очередь для webhook событий

## 🤝 Contributing

1. Fork проекта
2. Создайте feature branch (`git checkout -b feature/amazing-feature`)
3. Commit изменений (`git commit -m 'Add amazing feature'`)
4. Push в branch (`git push origin feature/amazing-feature`)
5. Создайте Pull Request

## 📄 Лицензия

MIT License - свободно используйте в своих проектах!

## 🆘 Поддержка

Если возникли вопросы:
- Изучите логи: `cargo shuttle logs`
- Проверьте переменные окружения
- Убедитесь что Go backend доступен
- Проверьте валидность OpenAI API ключа

## 📝 Changelog

### v2.2 (2025-10-16) - Business Intelligence Update 💼

**✨ Новые возможности:**
- 💼 **Business Intelligence Module**: Полная интеграция анализа бизнес-метрик
  - `AnalyzeBusiness` intent - анализ метрик отдельного бизнеса
  - `CompareBusinesses` intent - сравнение 2+ бизнесов
  - `BusinessInsights` intent - AI-советы по улучшению
- 🔗 **Go Backend Integration**: HTTP клиент для бизнес-данных
  - `services/go_client.rs` - полноценный клиент с типами
  - Интеграция с Neon PostgreSQL через Go API
- 🧠 **AI Analysis Engine**: 
  - Investment scoring (0-100 баллов)
  - Trend analysis (рост/стагнация/падение)
  - ROI calculation
  - Multi-business comparison
  - Prioritized recommendations
- 📈 **Performance**: 80-550ms response time для анализа

**🔧 Улучшения:**
- Обновлена система распознавания интентов (17 handlers)
- Исправлены ключевые слова для лучшего распознавания
- Добавлены 60+ unit tests
- Оптимизирована структура модулей

**📚 Документация:**
- Добавлены примеры Business Intelligence API
- Обновлены тестовые скрипты
- Расширена архитектурная документация

### v2.1 (Previous) - Backend Orchestration
- 🎯 Backend orchestrator с автоматическим управлением
- 📊 Prometheus metrics integration
- 📡 AI Insights WebSocket layer

### v2.0 - Modular Architecture
- 🧠 Migrация на intent handlers
- 📦 14 специализированных модулей
- 💾 Persistent memory с sled

---

**Сделано с ❤️ для FodiFood**
# bot_fodifood

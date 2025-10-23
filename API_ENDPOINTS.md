# 🌐 FodiFood Bot API Endpoints

Production URL: `https://bot-fodifood-lcon.shuttle.app`

## 📋 Table of Contents
- [Authentication](#-authentication)
- [User](#-user)
- [Chat & AI](#-chat--ai)
- [Products](#-products)
- [Business](#-business)
- [Admin](#-admin)
- [Multi-Agent System](#-multi-agent-system)
- [Metrics](#-metrics)
- [Health & Status](#-health--status)

---

## 🔐 Authentication

### POST `/api/v1/auth/register`
Регистрация нового пользователя

**Request:**
```json
{
  "email": "user@example.com",
  "password": "password123",
  "name": "John Doe"
}
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "user": {
    "id": "773dbd7f-3257-4d13-8832-a87a7acfc5f4",
    "email": "user@example.com",
    "name": "John Doe",
    "role": "user"
  }
}
```

**Test:**
```bash
curl -X POST https://bot-fodifood-lcon.shuttle.app/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"pass123","name":"Test User"}'
```

---

### POST `/api/v1/auth/login`
Вход пользователя

**Request:**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "user": {
    "id": "773dbd7f-3257-4d13-8832-a87a7acfc5f4",
    "email": "user@example.com",
    "name": "John Doe",
    "role": "user"
  }
}
```

**Test:**
```bash
curl -X POST https://bot-fodifood-lcon.shuttle.app/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"pass123"}'
```

---

## 👤 User

### GET `/api/v1/user/profile`
Получить профиль авторизованного пользователя

**Headers:**
```
Authorization: Bearer <JWT_TOKEN>
```

**Response:**
```json
{
  "id": "773dbd7f-3257-4d13-8832-a87a7acfc5f4",
  "email": "user@example.com",
  "name": "John Doe",
  "role": "user"
}
```

**Test:**
```bash
curl -X GET https://bot-fodifood-lcon.shuttle.app/api/v1/user/profile \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

---

## 💬 Chat & AI

### POST `/api/v1/chat`
Отправить сообщение AI боту (основной endpoint для взаимодействия)

**Aliases:**
- `/api/v1/chat` (primary)
- `/api/v1/chat/message` (frontend compatibility alias)

**Request:**
```json
{
  "user_id": "user_123",
  "message": "Покажи меню"
}
```

**Response:**
```json
{
  "intent": "ViewMenu",
  "response": "🍽️ **Актуальное меню с реальными ценами:**\n\n📂 **Роллы:**\n• **Филадельфия** — 450₽ (250г)...",
  "suggestions": [
    "Показать категорию Роллы",
    "Поиск по ингредиентам",
    "Сделать заказ"
  ],
  "products": [
    {
      "id": "1",
      "name": "Филадельфия",
      "price": 450.0,
      "description": "Лосось, сливочный сыр, огурец",
      "imageUrl": null,
      "category": "Роллы"
    }
  ]
}
```

**Supported Intents:**
- `Greeting` - Приветствие
- `ViewMenu` - Показать меню
- `CreateOrder` - Создать заказ
- `OrderStatus` - Статус заказа
- `CancelOrder` - Отменить заказ
- `DeliveryInfo` - Информация о доставке
- `SearchMenu` - Поиск блюд
- `SearchByIngredient` - Поиск по ингредиентам
- `CheckIngredients` - Проверка ингредиентов
- `Help` - Помощь
- `Statistics` - Статистика
- `SalesAnalysis` - Анализ продаж
- `AnalyzeBusiness` - Анализ бизнеса
- `CompareBusinesses` - Сравнение бизнесов
- `BusinessInsights` - Инсайты бизнеса
- `Recommendations` - Рекомендации
- `StockStatus` - Статус склада
- `Unknown` - Неизвестный (fallback to GROQ AI)

**Test Examples:**
```bash
# Greeting
curl -X POST https://bot-fodifood-lcon.shuttle.app/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{"user_id":"test","message":"Привет"}'

# View Menu
curl -X POST https://bot-fodifood-lcon.shuttle.app/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{"user_id":"test","message":"Покажи меню"}'

# Create Order
curl -X POST https://bot-fodifood-lcon.shuttle.app/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{"user_id":"test","message":"Хочу заказать Филадельфию"}'

# Search by Ingredient
curl -X POST https://bot-fodifood-lcon.shuttle.app/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{"user_id":"test","message":"Что есть с лососем?"}'
```

---

### GET `/api/v1/search`
Поиск блюд по ингредиенту (прямой endpoint)

**Query Parameters:**
- `ingredient` (string) - Название ингредиента

**Response:**
```json
{
  "products": [
    {
      "id": "1",
      "name": "Филадельфия",
      "price": 450.0,
      "description": "Лосось, сливочный сыр, огурец",
      "imageUrl": null,
      "category": "Роллы"
    }
  ]
}
```

**Test:**
```bash
curl "https://bot-fodifood-lcon.shuttle.app/api/v1/search?ingredient=лосось"
```

---

## 🍽️ Products

### GET `/api/v1/products`
Получить список всех продуктов из Go backend (или fallback menu при 404)

**Response:**
```json
[
  {
    "id": "1",
    "name": "Филадельфия",
    "description": "Лосось, сливочный сыр, огурец",
    "price": 450.0,
    "imageUrl": null,
    "weight": "250г",
    "category": "Роллы",
    "isVisible": true,
    "createdAt": "2025-10-22T12:00:00Z"
  }
]
```

**Test:**
```bash
curl https://bot-fodifood-lcon.shuttle.app/api/v1/products
```

**Fallback Menu** (when Go backend `/api/products` returns 404):
- Филадельфия (450₽)
- Калифорния (380₽)
- Маргарита (350₽)
- Пепперони (420₽)
- Том Ям (320₽)
- Coca-Cola (90₽)

---

## 💼 Business

### GET `/api/v1/businesses`
Получить список всех бизнесов

### POST `/api/v1/businesses`
Создать новый бизнес

### GET `/api/v1/businesses/{id}`
Получить информацию о бизнесе

### PUT `/api/v1/businesses/{id}`
Обновить информацию о бизнесе

### DELETE `/api/v1/businesses/{id}`
Удалить бизнес

**Note:** Требуется авторизация. Подробные схемы см. в `src/api/businesses.rs`

---

## 👨‍💼 Admin

### GET `/api/v1/admin/stats`
Получить статистику системы

**Headers:**
```
Authorization: Bearer <ADMIN_JWT_TOKEN>
```

**Test:**
```bash
curl -X GET https://bot-fodifood-lcon.shuttle.app/api/v1/admin/stats \
  -H "Authorization: Bearer ADMIN_TOKEN"
```

---

### GET `/api/v1/admin/orders`
Получить список всех заказов

---

### GET `/api/v1/admin/orders/recent`
Получить последние заказы

---

### GET `/api/v1/admin/users`
Получить список пользователей

---

### POST `/api/v1/admin/command`
Отправить команду AI админ-ассистенту

**Request:**
```json
{
  "command": "analyze user retention"
}
```

---

### WebSocket `/api/v1/admin/ws`
WebSocket для real-time admin updates

**Connect:**
```javascript
const ws = new WebSocket('wss://bot-fodifood-lcon.shuttle.app/api/v1/admin/ws');
```

---

## 🤖 Multi-Agent System

### GET `/api/v1/admin/agents`
Получить список всех AI агентов

**Response:**
```json
[
  {
    "id": "INV-PROD-001",
    "type": "Investor",
    "status": "active"
  },
  {
    "id": "BIZ-PROD-001",
    "type": "Business",
    "status": "active"
  },
  {
    "id": "USER-PROD-001",
    "type": "User",
    "status": "active"
  },
  {
    "id": "SYS-PROD-001",
    "type": "System",
    "status": "active"
  }
]
```

**Test:**
```bash
curl https://bot-fodifood-lcon.shuttle.app/api/v1/admin/agents
```

---

### GET `/api/v1/admin/agents/stats`
Получить статистику агентов

---

### GET `/api/v1/admin/agents/bus`
Получить статистику shared communication bus

---

### POST `/api/v1/admin/agents/coordinate`
Координация между агентами

**Request:**
```json
{
  "task": "analyze_market_opportunity",
  "agents": ["INV-PROD-001", "BIZ-PROD-001"]
}
```

---

### POST `/api/v1/admin/agents/subscribe`
Подписать агента на топики

**Request:**
```json
{
  "agent_id": "INV-PROD-001",
  "topics": ["market_analysis", "investment_opportunities"]
}
```

---

## 🎯 Backend Control

### POST `/api/v1/admin/backend/start`
Запустить Go backend (orchestrator)

---

### POST `/api/v1/admin/backend/stop`
Остановить Go backend

---

### POST `/api/v1/admin/backend/restart`
Перезапустить Go backend

---

### GET `/api/v1/admin/backend/status`
Получить статус Go backend

**Response:**
```json
{
  "status": "running",
  "url": "https://yeasty-madelaine-fodi999-671ccdf5.koyeb.app/api",
  "uptime": "2h 30m"
}
```

---

### GET `/api/v1/admin/backend/health`
Проверка здоровья orchestrator

---

## 📊 Metrics

### GET `/metrics`
Prometheus metrics

**Response:** Prometheus format
```
# HELP intent_requests_total Total number of intent classification requests
# TYPE intent_requests_total counter
intent_requests_total{intent="greeting"} 123
intent_requests_total{intent="viewmenu"} 456
...
```

**Test:**
```bash
curl https://bot-fodifood-lcon.shuttle.app/metrics
```

---

### GET `/admin/metrics`
Metrics dashboard (HTML)

---

### GET `/admin/metrics/intents`
Intent metrics (JSON)

**Response:**
```json
{
  "total_requests": 1234,
  "by_intent": {
    "greeting": 123,
    "viewmenu": 456,
    "createorder": 234
  },
  "avg_response_time_ms": 45.2
}
```

---

### GET `/admin/metrics/stats`
System metrics statistics

---

## 🏥 Health & Status

### GET `/`
Root endpoint (welcome message)

**Response:**
```json
{
  "service": "FodiFood Bot",
  "version": "0.1.0",
  "status": "running"
}
```

---

### GET `/health`
Health check

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2025-10-22T16:30:00Z"
}
```

**Test:**
```bash
curl https://bot-fodifood-lcon.shuttle.app/health
```

---

### GET `/api/v1/health`
API v1 health check

---

## 🔌 WebSocket Endpoints

### WebSocket `/ws`
Main chat WebSocket

**Connect:**
```javascript
const ws = new WebSocket('wss://bot-fodifood-lcon.shuttle.app/ws');
```

---

### WebSocket `/insight`
AI Insights WebSocket (real-time AI processing events)

**Connect (Legacy - UI compatibility):**
```javascript
const ws = new WebSocket('wss://bot-fodifood-lcon.shuttle.app/insight?token=JWT_TOKEN&channel=ui_events');
```

**Connect (New format):**
```javascript
const ws = new WebSocket('wss://bot-fodifood-lcon.shuttle.app/api/v1/insight?client_id=user123');
```

**Query Parameters:**
- `client_id` (optional) - Client identifier
- `token` (optional) - JWT token for authentication (legacy)
- `channel` (optional) - Channel name (e.g., `ui_events`) (legacy)

**Events:**
- Intent classification
- Entity extraction
- Handler routing
- Processing metrics

**Test:**
```bash
# Using websocat
websocat wss://bot-fodifood-lcon.shuttle.app/insight?channel=ui_events

# Using JavaScript
const ws = new WebSocket('wss://bot-fodifood-lcon.shuttle.app/insight?channel=ui_events');
ws.onmessage = (event) => console.log('AI Event:', event.data);
```

---

## 🔧 Go Backend Integration

**Go Backend URL:** `https://yeasty-madelaine-fodi999-671ccdf5.koyeb.app/api`

### Connected Endpoints:
- `GET /products` → Rust `state.backend.get_products()`
- `POST /orders` → Rust `state.backend.create_order()`
- `POST /auth/login` → Rust `state.backend.login()`
- `POST /auth/register` → Rust `state.backend.register()`
- `GET /user/profile` → Rust `state.backend.get_user_profile()`

**Fallback:** При 404 от Go backend, Rust использует встроенное fallback menu (6 продуктов)

---

## 🧪 Testing Summary

**1. Authentication Flow:**
```bash
# Register
curl -X POST https://bot-fodifood-lcon.shuttle.app/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"test123","name":"Test User"}'

# Login
curl -X POST https://bot-fodifood-lcon.shuttle.app/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"test123"}'

# Get Profile
curl -X GET https://bot-fodifood-lcon.shuttle.app/api/v1/user/profile \
  -H "Authorization: Bearer YOUR_TOKEN"
```

**2. Chat Flow:**
```bash
# Greeting
curl -X POST https://bot-fodifood-lcon.shuttle.app/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{"user_id":"user_123","message":"Привет"}'

# View Menu
curl -X POST https://bot-fodifood-lcon.shuttle.app/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{"user_id":"user_123","message":"Покажи меню"}'

# Order
curl -X POST https://bot-fodifood-lcon.shuttle.app/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{"user_id":"user_123","message":"Хочу заказать Филадельфию"}'
```

**3. Products:**
```bash
curl https://bot-fodifood-lcon.shuttle.app/api/v1/products
```

**4. Health:**
```bash
curl https://bot-fodifood-lcon.shuttle.app/health
```

---

## 📝 Notes

- ❌ `/api/v1/auth/signup` НЕ существует (используйте `/api/v1/auth/register`)
- ✅ GROQ AI fallback активен для unhandled intents (priority=0)
- ✅ 18 intent handlers зарегистрировано
- ✅ 4 AI агента активны (INV, BIZ, USER, SYS)
- ✅ Shared communication bus работает
- ✅ Fallback menu активен при недоступности Go backend
- ✅ JWT токены expires через 24 часа

---

## 🪙 FODI Token Info

- **Mint Address:** `F9qcQ2HEmjDXmUygFiJjeiMHeF5PYSGnfzhRbETeP8Ek`
- **Total Supply:** 1,000,000,000 FODI
- **Network:** Solana Devnet
- **Explorer:** [View on Solana Explorer](https://explorer.solana.com/address/F9qcQ2HEmjDXmUygFiJjeiMHeF5PYSGnfzhRbETeP8Ek?cluster=devnet)
- **Metadata:** [fodi-metadata.json](https://raw.githubusercontent.com/Fodi999/bot_fodifood/main/assets/fodi-metadata.json)

---

## 🚀 Production URLs

- **Rust Bot (Shuttle):** https://bot-fodifood-lcon.shuttle.app
- **Go Backend (Koyeb):** https://yeasty-madelaine-fodi999-671ccdf5.koyeb.app/api

---

**Last Updated:** 2025-10-22  
**Deployment:** depl_01K86C5ZDWYFW9M42RH1H347D3

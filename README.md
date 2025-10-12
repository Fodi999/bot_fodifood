# FodiFood Intelligent Bot 🦐🤖

Интеллектуальный бот на Rust для ресторана FodiFood - центральный коммуникационный узел между клиентами, администраторами и бизнес-логикой.

## 🏗️ Архитектура

```
┌─────────────────┐         WebSocket          ┌──────────────────┐
│   Next.js       │◄────────────────────────────►│   Rust Bot       │
│   Frontend      │      (wss://)               │  (Shuttle.rs)    │
│   (Vercel)      │                              │                  │
└─────────────────┘                              └────────┬─────────┘
                                                          │
                                                          │ REST API
                                                          │
                                                          ▼
                                                 ┌──────────────────┐
                                                 │   Go Backend     │
                                                 │   (Koyeb)        │
                                                 │   Business Logic │
                                                 └──────────────────┘
```

## ✨ Возможности

### 🔐 Аутентификация
- JWT-токены от Go backend
- Роли: Client, Admin, Manager, Courier, Cook
- Автоматическая валидация при подключении

### 💬 WebSocket коммуникация
- Реал-тайм чат с клиентами
- Командный интерфейс для управления
- Автоматические уведомления админам

### 🤖 AI интеграция (OpenAI GPT-4o-mini)
- Естественное общение с клиентами
- Анализ данных для менеджеров
- Персонализированные рекомендации
- Бизнес-аналитика и советы

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

## 📝 API Endpoints

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

## 🏗️ Структура проекта

```
src/
├── main.rs               # Точка входа, Shuttle setup
├── config.rs             # Конфигурация из env
├── state.rs              # Глобальное состояние приложения
├── handlers/
│   ├── ws.rs             # WebSocket обработчик
│   └── webhook.rs        # Webhook endpoints
├── api/
│   └── go_backend.rs     # REST клиент для Go API
├── ai/
│   └── mod.rs            # OpenAI интеграция
└── models/
    ├── message.rs        # Типы сообщений
    └── user.rs           # User, роли, JWT
```

## 🔧 Настройка Go Backend

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

## 📊 Мониторинг

Логи доступны через Shuttle:
```bash
cargo shuttle logs
```

Уровни логирования (переменная `RUST_LOG`):
- `error` - только ошибки
- `warn` - предупреждения
- `info` - информационные сообщения
- `debug` - детальная отладка
- `trace` - максимальная детализация

## 🧪 Тестирование

### WebSocket тест (через websocat)
```bash
# Установка websocat
cargo install websocat

# Подключение
websocat wss://fodifood-bot.shuttleapp.rs/ws

# Отправка сообщений
{"type":"auth","token":"your-jwt-token"}
{"type":"chat","text":"Покажите меню"}
```

### Webhook тест
```bash
curl -X POST https://fodifood-bot.shuttleapp.rs/notify \
  -H "Content-Type: application/json" \
  -d '{"event":"new_order","order_id":999,"total":1200}'
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

---

**Сделано с ❤️ для FodiFood**
# bot_fodifood

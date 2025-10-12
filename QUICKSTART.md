# 🦐 FodiFood Intelligent Bot - Быстрый старт

> Интеллектуальный бот на Rust для ресторана морепродуктов FodiFood

## ⚡ Быстрый старт (5 минут)

### 1. Клонирование и установка

```bash
# Клонируйте проект
cd bot_fodifood

# Запустите setup скрипт
./setup.sh

# Или вручную:
cargo install cargo-shuttle  # если ещё не установлен
cp .env.example .env
```

### 2. Конфигурация

Отредактируйте файл `.env`:

```env
# ОБЯЗАТЕЛЬНО: OpenAI API ключ для AI функций
OPENAI_API_KEY=sk-proj-xxxxxxxxxxxxxxxxxxxxxxxxxxxxx

# ОБЯЗАТЕЛЬНО: URL вашего Go backend
GO_BACKEND_URL=https://your-backend.koyeb.app/api

# Опционально: JWT секрет (используется для валидации токенов)
JWT_SECRET=your-super-secret-key-min-32-chars

# Опционально: уровень логирования
RUST_LOG=info
```

### 3. Локальный запуск

```bash
# Запуск сервера
cargo shuttle run

# Или с помощью make
make dev
```

Сервер запустится на `http://localhost:8000`

WebSocket доступен: `ws://localhost:8000/ws`

### 4. Тестирование

Откройте новый терминал и протестируйте:

```bash
# Проверка health endpoint
curl http://localhost:8000/health

# Тест WebSocket (используя websocat)
websocat ws://localhost:8000/ws

# В websocat отправьте:
{"type":"ping"}
# Ответ: {"type":"pong"}
```

### 5. Деплой на Shuttle

```bash
# Логин в Shuttle (если первый раз)
cargo shuttle login

# Установите секреты
cargo shuttle secrets set OPENAI_API_KEY=sk-xxxxx
cargo shuttle secrets set GO_BACKEND_URL=https://your-backend.koyeb.app/api

# Деплой!
cargo shuttle deploy

# Проверка статуса
cargo shuttle status

# Просмотр логов
cargo shuttle logs --follow
```

После деплоя ваш бот будет доступен:
```
wss://fodifood-bot.shuttleapp.rs/ws
```

## 📋 Что дальше?

### Интеграция с Next.js фронтендом

```typescript
// lib/bot.ts
const ws = new WebSocket('wss://fodifood-bot.shuttleapp.rs/ws');

ws.onopen = () => {
  // Аутентификация с JWT от Go backend
  ws.send(JSON.stringify({
    type: 'auth',
    token: yourJwtToken
  }));
};

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log('Bot:', message);
};

// Отправить сообщение
ws.send(JSON.stringify({
  type: 'chat',
  text: 'Покажите меню'
}));
```

### Настройка Go Backend

Добавьте в ваш Go backend:

```go
// После создания заказа, отправьте webhook:
http.Post(
    "https://fodifood-bot.shuttleapp.rs/notify",
    "application/json",
    bytes.NewBuffer([]byte(`{
        "event": "new_order",
        "order_id": 123,
        "total": 5400
    }`)),
)
```

### Endpoint для верификации токенов

```go
// POST /api/auth/verify
func VerifyToken(c *gin.Context) {
    var req struct {
        Token string `json:"token"`
    }
    c.BindJSON(&req)
    
    // Валидация JWT...
    
    c.JSON(200, gin.H{
        "valid": true,
        "user_id": "user123",
        "role": "client",
    })
}
```

## 🎯 Примеры использования

### Клиент спрашивает меню

```json
→ {"type":"chat","text":"Что у вас есть из креветок?"}
← {"type":"chat_response","text":"У нас есть...", "from_ai":true}
```

### Админ получает уведомление о новом заказе

```json
← {"type":"notification","event":"new_order","data":{"order_id":128,"total":5400}}
```

### Менеджер запрашивает статистику

```json
→ {"type":"chat","text":"Покажи продажи за сегодня"}
← {"type":"chat_response","text":"Сегодня продано...","from_ai":true}
```

## 🛠️ Полезные команды

```bash
# Разработка
make dev              # Запуск локально
make watch            # С auto-reload
make test             # Тесты
make lint             # Проверка кода

# Деплой
make deploy           # Деплой на Shuttle
make logs             # Просмотр логов
make status           # Статус проекта

# Проверки перед деплоем
./check.sh            # Полная проверка
make ci               # CI pipeline
```

## 📚 Документация

- [README.md](README.md) - Полная документация
- [EXAMPLES.md](EXAMPLES.md) - Примеры использования
- [DEVELOPMENT.md](DEVELOPMENT.md) - Руководство разработчика
- [CONTRIBUTING.md](CONTRIBUTING.md) - Как внести вклад

## 🐛 Проблемы?

### Проект не компилируется
```bash
cargo clean
cargo build
```

### WebSocket не подключается
- Проверьте логи: `cargo shuttle logs`
- Убедитесь что Go backend доступен
- Проверьте токен аутентификации

### AI не отвечает
- Проверьте `OPENAI_API_KEY` в `.env`
- Проверьте баланс OpenAI аккаунта
- Посмотрите логи на ошибки API

### Go backend недоступен
- Убедитесь что URL правильный в `.env`
- Проверьте что backend запущен
- Проверьте CORS настройки

## 💡 Совет

Для production обязательно:
- ✅ Используйте сильные секретные ключи
- ✅ Настройте CORS правильно
- ✅ Включите rate limiting
- ✅ Мониторьте логи
- ✅ Регулярно обновляйте зависимости

## 🚀 Готово!

Теперь у вас есть полнофункциональный интеллектуальный бот, готовый к работе!

---

**Нужна помощь?** Создайте issue или посмотрите полную документацию в README.md

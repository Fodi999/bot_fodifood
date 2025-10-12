# FodiFood Bot - Примеры использования

## 📝 Примеры сообщений WebSocket

### Клиентские запросы

#### Общие вопросы (AI)
```json
{
  "type": "chat",
  "text": "Что вы можете посоветовать из морепродуктов?"
}

{
  "type": "chat",
  "text": "Сколько стоит креветка королевская?"
}

{
  "type": "chat",
  "text": "Как долго готовится заказ?"
}
```

#### Создание заказа
```json
{
  "type": "chat",
  "text": "Хочу заказать креветки гриль и салат Цезарь"
}

{
  "type": "command",
  "action": "create_order",
  "params": {
    "items": [
      {"product_id": 5, "quantity": 2},
      {"product_id": 12, "quantity": 1}
    ],
    "delivery_address": "ул. Пушкина, д.10",
    "phone": "+79001234567"
  }
}
```

#### Проверка статуса
```json
{
  "type": "chat",
  "text": "Где мой заказ номер 128?"
}
```

### Административные команды

#### Просмотр всех заказов
```json
{
  "type": "command",
  "action": "get_orders"
}
```

#### Получение меню
```json
{
  "type": "command",
  "action": "get_menu"
}
```

### Менеджерские запросы (AI анализ)

```json
{
  "type": "chat",
  "text": "Покажи статистику продаж за сегодня"
}

{
  "type": "chat",
  "text": "Какие ингредиенты заканчиваются?"
}

{
  "type": "chat",
  "text": "Что продается лучше всего?"
}
```

## 🔔 Примеры Webhook событий

### От Go Backend к Bot

#### Новый заказ
```bash
curl -X POST https://fodifood-bot.shuttleapp.rs/notify \
  -H "Content-Type: application/json" \
  -d '{
    "event": "new_order",
    "order_id": 128,
    "user_id": "user123",
    "total": 5400,
    "items_count": 3
  }'
```

#### Изменение статуса заказа
```bash
curl -X POST https://fodifood-bot.shuttleapp.rs/notify \
  -H "Content-Type: application/json" \
  -d '{
    "event": "order_status_changed",
    "order_id": 128,
    "user_id": "user123",
    "old_status": "pending",
    "new_status": "cooking"
  }'
```

#### Низкий остаток ингредиентов
```bash
curl -X POST https://fodifood-bot.shuttleapp.rs/notify \
  -H "Content-Type: application/json" \
  -d '{
    "event": "low_inventory",
    "ingredient_id": 45,
    "ingredient_name": "Креветки королевские",
    "current_quantity": 2.5,
    "min_quantity": 10.0,
    "unit": "кг"
  }'
```

## 🌐 Интеграция с Next.js Frontend

### React Hook для WebSocket

```typescript
// hooks/useFodiBotSocket.ts
import { useEffect, useRef, useState } from 'react';

interface Message {
  type: string;
  [key: string]: any;
}

export function useFodiBotSocket(token: string | null) {
  const [connected, setConnected] = useState(false);
  const [messages, setMessages] = useState<Message[]>([]);
  const ws = useRef<WebSocket | null>(null);

  useEffect(() => {
    if (!token) return;

    const socket = new WebSocket('wss://fodifood-bot.shuttleapp.rs/ws');

    socket.onopen = () => {
      console.log('WebSocket connected');
      socket.send(JSON.stringify({ type: 'auth', token }));
    };

    socket.onmessage = (event) => {
      const message = JSON.parse(event.data);
      
      if (message.type === 'auth_success') {
        setConnected(true);
      } else if (message.type === 'auth_failed') {
        console.error('Auth failed:', message.reason);
        socket.close();
      } else {
        setMessages((prev) => [...prev, message]);
      }
    };

    socket.onclose = () => {
      setConnected(false);
      console.log('WebSocket disconnected');
    };

    ws.current = socket;

    return () => {
      socket.close();
    };
  }, [token]);

  const sendMessage = (text: string) => {
    if (ws.current?.readyState === WebSocket.OPEN) {
      ws.current.send(JSON.stringify({ type: 'chat', text }));
    }
  };

  const sendCommand = (action: string, params?: any) => {
    if (ws.current?.readyState === WebSocket.OPEN) {
      ws.current.send(JSON.stringify({ type: 'command', action, params }));
    }
  };

  return { connected, messages, sendMessage, sendCommand };
}
```

### Компонент чата

```typescript
// components/ChatBot.tsx
import { useState } from 'react';
import { useFodiBotSocket } from '@/hooks/useFodiBotSocket';

export function ChatBot({ token }: { token: string }) {
  const { connected, messages, sendMessage } = useFodiBotSocket(token);
  const [input, setInput] = useState('');

  const handleSend = () => {
    if (input.trim()) {
      sendMessage(input);
      setInput('');
    }
  };

  return (
    <div className="chat-container">
      <div className="status">
        {connected ? '🟢 Подключено' : '🔴 Не подключено'}
      </div>
      
      <div className="messages">
        {messages.map((msg, idx) => (
          <div key={idx} className="message">
            {msg.type === 'chat_response' && (
              <div className={msg.from_ai ? 'ai-message' : 'message'}>
                {msg.from_ai && '🤖 '}
                {msg.text}
              </div>
            )}
            
            {msg.type === 'notification' && (
              <div className="notification">
                🔔 {msg.event}: {JSON.stringify(msg.data)}
              </div>
            )}
            
            {msg.type === 'command_response' && (
              <div className="command-result">
                ✅ {msg.action}: {JSON.stringify(msg.data)}
              </div>
            )}
          </div>
        ))}
      </div>

      <div className="input-area">
        <input
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyPress={(e) => e.key === 'Enter' && handleSend()}
          placeholder="Спросите что-нибудь..."
        />
        <button onClick={handleSend}>Отправить</button>
      </div>
    </div>
  );
}
```

### Админская панель

```typescript
// components/AdminPanel.tsx
import { useFodiBotSocket } from '@/hooks/useFodiBotSocket';
import { useEffect } from 'react';

export function AdminPanel({ token }: { token: string }) {
  const { connected, messages, sendCommand } = useFodiBotSocket(token);

  useEffect(() => {
    // Загрузить заказы при подключении
    if (connected) {
      sendCommand('get_orders');
    }
  }, [connected]);

  const notifications = messages.filter(m => m.type === 'notification');

  return (
    <div className="admin-panel">
      <h2>Панель администратора</h2>
      
      <div className="notifications">
        <h3>Уведомления ({notifications.length})</h3>
        {notifications.map((notif, idx) => (
          <div key={idx} className="notification-card">
            <strong>{notif.event}</strong>
            <pre>{JSON.stringify(notif.data, null, 2)}</pre>
          </div>
        ))}
      </div>

      <div className="actions">
        <button onClick={() => sendCommand('get_orders')}>
          Обновить заказы
        </button>
        <button onClick={() => sendCommand('get_menu')}>
          Показать меню
        </button>
      </div>
    </div>
  );
}
```

## 🔧 Настройка Go Backend для отправки событий

### Пример на Go (Gin framework)

```go
// models/webhook.go
package models

type WebhookEvent struct {
    Event string                 `json:"event"`
    Data  map[string]interface{} `json:"data"`
}

// services/notifier.go
package services

import (
    "bytes"
    "encoding/json"
    "net/http"
    "os"
)

var botWebhookURL = os.Getenv("BOT_WEBHOOK_URL")

func NotifyBot(event string, data map[string]interface{}) error {
    payload := map[string]interface{}{
        "event": event,
    }
    
    // Merge data into payload
    for k, v := range data {
        payload[k] = v
    }

    jsonData, err := json.Marshal(payload)
    if err != nil {
        return err
    }

    _, err = http.Post(
        botWebhookURL+"/notify",
        "application/json",
        bytes.NewBuffer(jsonData),
    )
    
    return err
}

// handlers/orders.go
package handlers

import (
    "github.com/gin-gonic/gin"
    "yourapp/services"
)

func CreateOrder(c *gin.Context) {
    // ... создание заказа ...
    
    // Уведомить бота
    services.NotifyBot("new_order", map[string]interface{}{
        "order_id":    order.ID,
        "user_id":     order.UserID,
        "total":       order.Total,
        "items_count": len(order.Items),
    })
    
    c.JSON(200, order)
}

func UpdateOrderStatus(c *gin.Context) {
    // ... обновление статуса ...
    
    services.NotifyBot("order_status_changed", map[string]interface{}{
        "order_id":   order.ID,
        "user_id":    order.UserID,
        "old_status": oldStatus,
        "new_status": order.Status,
    })
    
    c.JSON(200, order)
}
```

### Endpoint для верификации токенов

```go
// handlers/auth.go
package handlers

import (
    "github.com/gin-gonic/gin"
    "github.com/golang-jwt/jwt/v5"
)

type VerifyTokenRequest struct {
    Token string `json:"token"`
}

type VerifyTokenResponse struct {
    Valid  bool   `json:"valid"`
    UserID string `json:"user_id,omitempty"`
    Role   string `json:"role,omitempty"`
}

func VerifyToken(c *gin.Context) {
    var req VerifyTokenRequest
    if err := c.BindJSON(&req); err != nil {
        c.JSON(400, gin.H{"error": "invalid request"})
        return
    }

    token, err := jwt.Parse(req.Token, func(token *jwt.Token) (interface{}, error) {
        return []byte(os.Getenv("JWT_SECRET")), nil
    })

    if err != nil || !token.Valid {
        c.JSON(200, VerifyTokenResponse{Valid: false})
        return
    }

    claims := token.Claims.(jwt.MapClaims)
    
    c.JSON(200, VerifyTokenResponse{
        Valid:  true,
        UserID: claims["sub"].(string),
        Role:   claims["role"].(string),
    })
}

// main.go
func main() {
    r := gin.Default()
    
    api := r.Group("/api")
    {
        api.POST("/auth/verify", handlers.VerifyToken)
        api.GET("/products", handlers.GetProducts)
        api.GET("/orders", handlers.GetOrders)
        api.POST("/orders", handlers.CreateOrder)
        api.GET("/ingredients", handlers.GetIngredients)
        api.GET("/stats", handlers.GetStats)
    }
    
    r.Run(":8080")
}
```

## 🧪 Тестирование

### Тест WebSocket через JavaScript (браузер)

```javascript
// Откройте консоль браузера и выполните:
const ws = new WebSocket('wss://fodifood-bot.shuttleapp.rs/ws');

ws.onopen = () => {
  console.log('Connected');
  
  // Аутентификация
  ws.send(JSON.stringify({
    type: 'auth',
    token: 'your-jwt-token-here'
  }));
};

ws.onmessage = (event) => {
  console.log('Received:', JSON.parse(event.data));
};

// Отправить сообщение
ws.send(JSON.stringify({
  type: 'chat',
  text: 'Покажите меню'
}));

// Команда
ws.send(JSON.stringify({
  type: 'command',
  action: 'get_menu'
}));
```

### Нагрузочное тестирование

```bash
# Установка artillery
npm install -g artillery

# Создайте файл load-test.yml
```

```yaml
# load-test.yml
config:
  target: "wss://fodifood-bot.shuttleapp.rs"
  phases:
    - duration: 60
      arrivalRate: 10
  engines:
    ws:
      query: "/ws"

scenarios:
  - engine: ws
    flow:
      - send:
          payload: '{"type":"auth","token":"test-token"}'
      - think: 2
      - send:
          payload: '{"type":"chat","text":"Покажите меню"}'
      - think: 3
```

```bash
# Запуск
artillery run load-test.yml
```

---

## 🔗 Интеграция с Go Backend

### Отправка заказа на бэкенд

Rust-бот может автоматически уведомлять Go backend о новых заказах:

```rust
use crate::ai::create_order;

// В обработчике WebSocket или AI-логике
let response = create_order("ORD-12345", 2500.0).await?;
println!("{}", response);
```

### Пример использования в AI

```rust
// src/ai/mod.rs
pub async fn process_message(
    &self,
    user_id: &str,
    message: &str,
) -> Result<String> {
    let intent = IntentClassifier::classify(message);
    
    match intent {
        Intent::CreateOrder => {
            // Извлечь данные заказа из сообщения
            let order_id = format!("ORD-{}", uuid::Uuid::new_v4());
            let total = 2500.0; // рассчитать из корзины
            
            // Отправить на Go backend
            create_order(&order_id, total).await?
        }
        _ => {
            ResponseGenerator::generate(&intent, None)
        }
    }
}
```

### Настройка для локального режима

В `.env` добавьте:

```env
GO_BACKEND_URL=http://127.0.0.1:8080
```

### Логи интеграции

При успешной отправке заказа вы увидите:

```
2025-10-12T15:30:45.123+02:00 [app] INFO fodifood_bot::ai: 🤖 AI: Создаю заказ ORD-12345 на сумму 2500.00 руб.
2025-10-12T15:30:45.234+02:00 [app] INFO fodifood_bot::api::go_backend: 📦 Sent order to backend → Status: 200
2025-10-12T15:30:45.235+02:00 [app] INFO fodifood_bot::ai: ✅ Заказ ORD-12345 успешно отправлен на backend
```

### Endpoint на Go backend

Go backend должен принимать POST-запросы на `/api/orders/notify`:

```go
// Go backend example
type OrderNotification struct {
    OrderID string  `json:"order_id"`
    Total   float64 `json:"total"`
}

func NotifyOrderHandler(c *gin.Context) {
    var notif OrderNotification
    if err := c.BindJSON(&notif); err != nil {
        c.JSON(400, gin.H{"error": err.Error()})
        return
    }
    
    log.Printf("📦 Получен заказ от Rust-бота: %s на сумму %.2f", 
        notif.OrderID, notif.Total)
    
    // Сохранить в БД, отправить уведомления и т.д.
    c.JSON(200, gin.H{"status": "received"})
}
```

---

**Готово к использованию! 🚀**

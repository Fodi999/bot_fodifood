# 🔐 Тестирование передачи JWT токена

## Способы передачи токена

### 1. Через Query Parameters (Рекомендуется)

```javascript
// Frontend (Next.js)
const token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
const ws = new WebSocket(`ws://127.0.0.1:8000/ws?token=${encodeURIComponent(token)}`);

ws.onopen = () => {
  console.log('✅ Подключено и автоматически аутентифицировано');
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  if (data.type === 'auth_success') {
    console.log(`✅ Аутентифицирован как ${data.user_id}, роль: ${data.role}`);
  }
};
```

**Логи на Rust-стороне:**
```
🌐 WebSocket connection attempt with params: WsParams { token: Some("eyJhbGci...") }
🔑 Token prefix received: eyJhbGciOiJIUzI1NiI...
🔐 Attempting auto-authentication with query token...
✅ Auto-authenticated user user123 as Client
```

### 2. Через WebSocket сообщение

```javascript
const ws = new WebSocket('ws://127.0.0.1:8000/ws');

ws.onopen = () => {
  // Отправляем токен после подключения
  ws.send(JSON.stringify({
    type: 'auth',
    token: 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...'
  }));
};
```

**Логи на Rust-стороне:**
```
🌐 WebSocket connection attempt with params: WsParams { token: None }
📝 No token in query params, expecting auth message
New WebSocket connection: 32b3cbbe-08d3-4ee4-8234-0f20c342a06b
✅ Auto-authenticated user user123 as Client
```

## Тестирование

### Использование HTML тестера

1. Открой файл `test_ws_token.html` в браузере
2. Введи JWT токен в поле "JWT Token"
3. Нажми кнопку тестирования
4. Проверь логи в браузере И в терминале Rust-бота

### Проверка логов Rust-бота

Запусти бота и следи за логами:

```bash
cargo shuttle run
```

При подключении ты увидишь:

```
2025-10-12T15:38:29.771+02:00 [app]  INFO fodifood_bot::handlers::ws: 🌐 WebSocket connection attempt with params: WsParams { token: Some("eyJ...") }
2025-10-12T15:38:29.772+02:00 [app]  INFO fodifood_bot::handlers::ws: 🔑 Token prefix received: eyJhbGciOiJIUzI1NiI...
2025-10-12T15:38:29.773+02:00 [app]  INFO fodifood_bot::handlers::ws: 🔐 Attempting auto-authentication with query token...
2025-10-12T15:38:29.775+02:00 [app]  INFO fodifood_bot::handlers::ws: ✅ Auto-authenticated user user123 as Client
```

### Диагностика проблем

#### Если видишь `token: None`:
```
🌐 WebSocket connection attempt with params: WsParams { token: None }
```
**Причина:** Фронтенд не передаёт токен в URL  
**Решение:** Проверь, что URL содержит `?token=...`

#### Если видишь "Invalid token":
```
⚠️ Invalid token received in query params
```
**Причина:** Go backend не признал токен валидным  
**Решение:** 
- Проверь, что Go backend запущен на `http://127.0.0.1:8080`
- Проверь, что токен сгенерирован с правильным секретом
- Проверь endpoint `/auth/verify` на Go backend

#### Если видишь "Auth server error":
```
❌ Token verification failed: Failed to send verify token request
```
**Причина:** Не удалось подключиться к Go backend  
**Решение:** 
- Убедись, что Go backend запущен
- Проверь `GO_BACKEND_URL` в `.env`
- Проверь сетевую доступность

## Формат токена для тестирования

Если Go backend не запущен, можно использовать тестовый токен:

```
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyX2lkIjoidGVzdDEyMyIsInJvbGUiOiJjbGllbnQifQ.test
```

**Внимание:** Для production используй реальные токены от Go backend!

## Интеграция с Next.js

```typescript
// components/ChatWidget.tsx
import { useEffect, useState } from 'react';

export default function ChatWidget() {
  const [ws, setWs] = useState<WebSocket | null>(null);
  
  useEffect(() => {
    // Получаем токен из localStorage или контекста
    const token = localStorage.getItem('jwt_token');
    
    if (!token) {
      console.error('No token found');
      return;
    }
    
    // Подключаемся с токеном в query
    const websocket = new WebSocket(
      `ws://localhost:8000/ws?token=${encodeURIComponent(token)}`
    );
    
    websocket.onopen = () => {
      console.log('✅ Connected to bot');
    };
    
    websocket.onmessage = (event) => {
      const data = JSON.parse(event.data);
      
      if (data.type === 'auth_success') {
        console.log(`Authenticated as ${data.user_id}`);
      } else if (data.type === 'chat_response') {
        console.log(`Bot: ${data.text}`);
      }
    };
    
    setWs(websocket);
    
    return () => {
      websocket.close();
    };
  }, []);
  
  const sendMessage = (text: string) => {
    if (ws && ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify({
        type: 'chat',
        text: text
      }));
    }
  };
  
  return (
    <div>
      {/* Your chat UI */}
    </div>
  );
}
```

## Полезные команды

```bash
# Запустить бота
cargo shuttle run

# Следить за логами с фильтрацией
cargo shuttle run 2>&1 | grep "WebSocket"

# Проверить, что бот слушает на порту 8000
lsof -i :8000

# Убить процесс на порту 8000
lsof -ti:8000 | xargs kill -9
```

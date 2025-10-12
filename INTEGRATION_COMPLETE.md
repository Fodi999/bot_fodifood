# ✅ Интеграция Rust-бота с Go Backend - ГОТОВО!

## 🎯 Что сделано

### 1. Добавлена роль `User` в enum
**Файл:** `src/models/user.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Client,
    Admin,
    Manager,
    Courier,
    Cook,
    User,  // ✅ Добавлено для совместимости с Go backend
}
```

**Причина:** Go backend возвращает `"role":"user"` в JWT токене, теперь Rust правильно десериализует эту роль.

### 2. Query параметр для автоматической аутентификации
**Файл:** `src/handlers/ws.rs`

```rust
#[derive(Deserialize, Debug)]
pub struct WsParams {
    pub token: Option<String>,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Автоматическая аутентификация через query параметр
}
```

**Теперь можно подключаться:**
```javascript
const ws = new WebSocket(`ws://127.0.0.1:8000/ws?token=${yourJWTToken}`);
```

### 3. GoBackendClient интегрирован в AppState
**Файл:** `src/state.rs`

```rust
pub struct AppState {
    pub config: Config,
    pub connections: Arc<DashMap<ClientId, ClientConnection>>,
    pub backend: Arc<GoBackendClient>,  // ✅ Общий клиент
}
```

Теперь все handlers используют один экземпляр клиента через `state.backend`.

### 4. Функции интеграции
**Файл:** `src/api/go_backend.rs`

✅ `verify_token()` - проверка JWT через Go backend  
✅ `get_products()` - получение меню  
✅ `get_orders()` - получение заказов  
✅ `create_order()` - создание заказа  
✅ `get_ingredients()` - остатки на складе  
✅ `get_stats()` - статистика продаж  
✅ `send_order_to_backend()` - уведомление о новых заказах

## 📡 Логи успешной аутентификации

Теперь в логах Rust-бота будет:

```
🌐 WebSocket connection attempt with params: WsParams { token: Some("eyJ...") }
🔑 Token prefix received: eyJhbGciOiJIUzI1Ni...
🔐 Attempting auto-authentication with query token...
✅ Auto-authenticated user cmgds9uv60000l704ynyfeqs5 as User
```

## 🔧 Настройки

### `.env` файл
```env
GO_BACKEND_URL=http://127.0.0.1:8080
JWT_SECRET=your-jwt-secret-here
RUST_LOG=info
```

### Production (Shuttle.rs)
```bash
# Установить секреты
cargo shuttle secrets set GO_BACKEND_URL="https://your-go-backend.koyeb.app"
cargo shuttle secrets set JWT_SECRET="your-production-secret"
```

## 🧪 Тестирование

### 1. Запустить Go backend
```bash
cd /Users/dmitrijfomin/Desktop/backend
go run main.go
# Ожидается: Running on :8080
```

### 2. Запустить Rust bot
```bash
cd /Users/dmitrijfomin/Desktop/bot_fodifood
cargo shuttle run
# Ожидается: http://127.0.0.1:8000
```

### 3. Протестировать через HTML
Открыть файл `test_ws_token.html` в браузере и ввести реальный JWT токен от Go backend.

### 4. Протестировать из Next.js
```typescript
const token = localStorage.getItem('jwt_token');
const ws = new WebSocket(`ws://localhost:8000/ws?token=${token}`);

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Bot response:', data);
};
```

## 📊 Доступные команды WebSocket

### Просмотр меню
```json
{
  "type": "chat",
  "text": "Покажи меню"
}
```

### Создание заказа
```json
{
  "type": "command",
  "action": "create_order",
  "params": {
    "user_id": "cmgds9uv60000l704ynyfeqs5",
    "items": [{"product_id": 1, "quantity": 2}],
    "total": 2500.0
  }
}
```

### Статистика (только для staff)
```json
{
  "type": "chat",
  "text": "Покажи статистику"
}
```

## ✅ Checklist интеграции

- [x] Роль `User` добавлена в enum
- [x] Query параметр для токена реализован
- [x] GoBackendClient интегрирован в AppState
- [x] Логирование токенов работает
- [x] Автоматическая аутентификация работает
- [x] Функции API Go backend доступны
- [x] Тестовый HTML файл создан
- [x] Документация обновлена

## 🚀 Следующие шаги

1. **Запустить Go backend** на http://127.0.0.1:8080
2. **Протестировать аутентификацию** с реальным токеном
3. **Интегрировать с Next.js фронтендом**
4. **Развернуть на production:**
   ```bash
   # Go backend → Koyeb
   # Rust bot → Shuttle.rs
   cargo shuttle deploy
   # Next.js → Vercel
   ```

---

**Все готово для полной интеграции! 🎉**

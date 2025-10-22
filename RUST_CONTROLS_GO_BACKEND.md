# 🎯 Rust Полностью Управляет Go Backend на Koyeb

## 📡 URL: `yeasty-madelaine-fodi999-671ccdf5.koyeb.app`

---

## 🏗️ Архитектура Управления

```
┌────────────────────────────────────────────────────────────────┐
│                    RUST BOT (Shuttle.rs)                       │
│                  fodifood_bot @ Shuttle.rs                     │
└────────────────────────────────────────────────────────────────┘
                            │
                            │ HTTP/REST API
                            │
    ┌───────────────────────┼───────────────────────┐
    │                       │                       │
    ▼                       ▼                       ▼
┌─────────┐          ┌──────────┐          ┌──────────────┐
│ Control │          │ Queries  │          │ Monitoring   │
│ Layer   │          │ (CRUD)   │          │ & Health     │
└─────────┘          └──────────┘          └──────────────┘
    │                       │                       │
    │                       │                       │
    ▼                       ▼                       ▼
┌────────────────────────────────────────────────────────────────┐
│              GO BACKEND (Koyeb)                                │
│   https://yeasty-madelaine-fodi999-671ccdf5.koyeb.app          │
│                                                                │
│   • PostgreSQL Database                                        │
│   • User Management                                            │
│   • Product Catalog                                            │
│   • Order Processing                                           │
│   • Business Logic                                             │
└────────────────────────────────────────────────────────────────┘
```

---

## 📁 Ключевые Файлы Управления

### 1️⃣ **Главный Контроллер** 
**`src/api/go_backend/mod.rs`** (200 строк)

```rust
/// 🌐 Go Backend Client - Unified facade for all services
pub struct GoBackendClient {
    pub auth: AuthClient,          // 🔐 Authentication
    pub products: ProductsClient,  // 🛍️ Products
    pub orders: OrdersClient,      // 📦 Orders
    pub admin: AdminClient,        // 👨‍💼 Admin
}

impl GoBackendClient {
    pub fn new(config: &Config) -> Self {
        let client = Client::new();
        let base_url = config.go_backend_url.clone(); // ← Koyeb URL
        
        Self {
            auth: AuthClient::new(client.clone(), base_url.clone()),
            products: ProductsClient::new(client.clone(), base_url.clone()),
            orders: OrdersClient::new(client.clone(), base_url.clone()),
            admin: AdminClient::new(client, base_url),
        }
    }
}
```

**Что делает:**
- ✅ Создаёт единую точку доступа ко всем Go сервисам
- ✅ Управляет аутентификацией пользователей
- ✅ CRUD операции с продуктами и заказами
- ✅ Admin панель (stats, users, orders)

---

### 2️⃣ **Аутентификация** 
**`src/api/go_backend/auth.rs`** (250 строк)

```rust
pub struct AuthClient {
    client: Client,
    base_url: String,  // https://yeasty-madelaine-fodi999-671ccdf5.koyeb.app
}

impl AuthClient {
    /// Login user with Go backend
    pub async fn login(&self, email: &str, password: &str) 
        -> Result<LoginResponse> 
    {
        let url = format!("{}/auth/login", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&serde_json::json!({
                "email": email,
                "password": password,
            }))
            .send()
            .await?;
        
        response.json::<LoginResponse>().await
    }
    
    /// Register new user
    pub async fn register(&self, email: &str, password: &str, name: &str)
        -> Result<LoginResponse>
    
    /// Verify JWT token
    pub async fn verify_token(&self, token: &str)
        -> Result<VerifyTokenResponse>
    
    /// Get user profile
    pub async fn get_user_profile(&self, token: &str)
        -> Result<UserProfile>
    
    /// Admin: Get all users
    pub async fn get_users(&self, token: &str)
        -> Result<Vec<UserProfile>>
    
    /// Admin: Update user
    pub async fn update_user(&self, token: &str, id: &str, data: Value)
        -> Result<UserProfile>
    
    /// Admin: Delete user
    pub async fn delete_user(&self, token: &str, id: &str)
        -> Result<()>
}
```

**API Endpoints на Go Backend:**
- `POST /auth/login` - Вход пользователя
- `POST /auth/register` - Регистрация
- `POST /auth/verify` - Проверка JWT токена
- `GET /user/profile` - Профиль пользователя
- `GET /admin/users` - Список пользователей (admin)
- `PUT /admin/users/{id}` - Обновить пользователя
- `DELETE /admin/users/{id}` - Удалить пользователя

---

### 3️⃣ **Продукты**
**`src/api/go_backend/products.rs`** (150 строк)

```rust
pub struct ProductsClient {
    client: Client,
    base_url: String,
}

impl ProductsClient {
    /// Get all products from Go backend
    pub async fn get_products(&self) -> Result<Vec<Product>> {
        let url = format!("{}/products", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        response.json::<Vec<Product>>().await
    }
    
    /// Find product by name
    pub fn find_product_by_name<'a>(
        products: &'a [Product], 
        query: &str
    ) -> Option<&'a Product>
    
    /// Filter products by ingredient
    pub fn filter_by_ingredient<'a>(
        products: &'a [Product],
        ingredient: &str
    ) -> Vec<&'a Product>
    
    /// Format products list for display
    pub fn format_products_list(products: &[Product]) -> String
}
```

**API Endpoints:**
- `GET /products` - Получить каталог продуктов

---

### 4️⃣ **Заказы**
**`src/api/go_backend/orders.rs`** (200 строк)

```rust
pub struct OrdersClient {
    client: Client,
    base_url: String,
}

impl OrdersClient {
    /// Get all orders (public)
    pub async fn get_orders(&self) -> Result<Vec<Order>>
    
    /// Get recent orders for user (authenticated)
    pub async fn get_recent_orders(&self, token: &str)
        -> Result<Vec<Order>>
    
    /// Create new order
    pub async fn create_order(&self, order_data: Value)
        -> Result<Order>
    
    /// Admin: Get all orders
    pub async fn get_all_orders_admin(&self, token: &str)
        -> Result<Vec<Order>>
    
    /// Admin: Update order status
    pub async fn update_order_status_admin(
        &self, 
        token: &str,
        id: i64,
        status: &str
    ) -> Result<Order>
    
    /// Send order notification to backend
    pub async fn send_order_notification(
        order_id: &str,
        total: f64
    ) -> Result<()>
}
```

**API Endpoints:**
- `GET /orders` - Все заказы
- `GET /orders/recent` - Недавние заказы пользователя
- `POST /orders` - Создать заказ
- `GET /admin/orders` - Все заказы (admin)
- `PUT /admin/orders/{id}` - Обновить статус заказа

---

### 5️⃣ **Админ Панель**
**`src/api/go_backend/admin.rs`** (150 строк)

```rust
pub struct AdminClient {
    client: Client,
    base_url: String,
}

impl AdminClient {
    /// Get statistics (admin only)
    pub async fn get_stats(&self, token: &str) -> Result<Stats> {
        let url = format!("{}/admin/stats", self.base_url);
        
        self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?
            .json()
            .await
    }
    
    /// Get ingredients/inventory
    pub async fn get_ingredients(&self, token: &str)
        -> Result<Vec<Ingredient>>
    
    /// Create new ingredient
    pub async fn create_ingredient(&self, token: &str, data: Value)
        -> Result<Ingredient>
    
    /// Update ingredient
    pub async fn update_ingredient(&self, token: &str, id: i64, data: Value)
        -> Result<Ingredient>
    
    /// Delete ingredient
    pub async fn delete_ingredient(&self, token: &str, id: i64)
        -> Result<()>
    
    /// Get ingredient movements
    pub async fn get_ingredient_movements(&self, token: &str, id: i64)
        -> Result<Vec<IngredientMovement>>
}
```

**API Endpoints:**
- `GET /admin/stats` - Статистика (users, orders, revenue)
- `GET /admin/ingredients` - Список ингредиентов
- `POST /admin/ingredients` - Создать ингредиент
- `PUT /admin/ingredients/{id}` - Обновить ингредиент
- `DELETE /admin/ingredients/{id}` - Удалить ингредиент
- `GET /admin/ingredients/{id}/movements` - История движений

---

### 6️⃣ **Оркестратор (Process Control)**
**`src/orchestration/backend.rs`** (400+ строк)

```rust
/// Go Backend Process Orchestrator
pub struct BackendOrchestrator {
    config: OrchestratorConfig,
    process: Arc<RwLock<Option<Child>>>,
    status: Arc<RwLock<BackendStatus>>,
    health_checker: Arc<HealthChecker>,
    start_time: Arc<RwLock<Option<Instant>>>,
    restart_count: Arc<RwLock<u32>>,
}

impl BackendOrchestrator {
    /// Start the Go backend process
    pub async fn start(&self) -> Result<()> {
        // Build command
        let mut cmd = Command::new(&self.config.binary_path);
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
        
        // Spawn process
        let child = cmd.spawn()?;
        let pid = child.id();
        
        // Store process handle
        *self.process.write().await = Some(child);
        
        // Perform health check
        let health = self.health_checker.check_with_retries(5, 1000).await;
        
        // Update status
        match health {
            HealthStatus::Healthy => {
                *self.status.write().await = BackendStatus::Running;
                Ok(())
            }
            _ => {
                *self.status.write().await = BackendStatus::Unhealthy;
                Ok(())
            }
        }
    }
    
    /// Stop the Go backend process
    pub async fn stop(&self) -> Result<()>
    
    /// Restart the Go backend process
    pub async fn restart(&self) -> Result<()>
    
    /// Start health monitoring task
    pub fn start_health_monitoring(self: Arc<Self>) 
        -> tokio::task::JoinHandle<()>
    
    /// Get backend status
    pub async fn get_status(&self) -> BackendStatus
    
    /// Get backend info (PID, uptime, restart count)
    pub async fn get_info(&self) -> BackendInfo
}
```

**Возможности:**
- ✅ Запуск/остановка/перезапуск Go backend
- ✅ Health monitoring каждые 30 секунд
- ✅ Auto-restart при падении (до 3 попыток)
- ✅ Отслеживание PID, uptime, restart count

---

### 7️⃣ **Backend Control API**
**`src/api/backend_control.rs`** (200 строк)

```rust
/// Start the Go backend
///
/// POST /api/v1/admin/backend/start
pub async fn start_backend(State(state): State<AppState>) 
    -> impl IntoResponse 
{
    if let Some(ref orchestrator) = state.backend_orchestrator {
        match orchestrator.start().await {
            Ok(_) => {
                let info = orchestrator.get_info().await;
                Json(json!({
                    "success": true,
                    "message": "Backend started successfully",
                    "pid": info.pid,
                    "status": info.status
                }))
            }
            Err(e) => {
                Json(json!({
                    "success": false,
                    "error": e.to_string()
                }))
            }
        }
    }
}

/// Stop the Go backend
/// POST /api/v1/admin/backend/stop
pub async fn stop_backend(State(state): State<AppState>)
    -> impl IntoResponse

/// Restart the Go backend
/// POST /api/v1/admin/backend/restart
pub async fn restart_backend(State(state): State<AppState>)
    -> impl IntoResponse

/// Get backend status
/// GET /api/v1/admin/backend/status
pub async fn get_backend_status(State(state): State<AppState>)
    -> impl IntoResponse

/// Health check
/// GET /api/v1/admin/backend/health
pub async fn backend_orchestrator_health(State(state): State<AppState>)
    -> impl IntoResponse
```

**REST API Endpoints:**
- `POST /api/v1/admin/backend/start` - Запустить Go backend
- `POST /api/v1/admin/backend/stop` - Остановить Go backend
- `POST /api/v1/admin/backend/restart` - Перезапустить Go backend
- `GET /api/v1/admin/backend/status` - Статус (running/stopped/crashed)
- `GET /api/v1/admin/backend/health` - Health check оркестратора

---

### 8️⃣ **Health Checker**
**`src/orchestration/health.rs`** (100+ строк)

```rust
pub struct HealthChecker {
    base_url: String,
    timeout_secs: u64,
    client: Client,
}

impl HealthChecker {
    /// Check backend health
    pub async fn check(&self) -> HealthStatus {
        let url = format!("{}/health", self.base_url);
        
        let response = timeout(
            Duration::from_secs(self.timeout_secs),
            self.client.get(&url).send()
        ).await;
        
        match response {
            Ok(Ok(resp)) if resp.status().is_success() => {
                HealthStatus::Healthy
            }
            Ok(Ok(resp)) => {
                HealthStatus::Unhealthy(
                    format!("HTTP {}", resp.status())
                )
            }
            _ => HealthStatus::Unhealthy("Connection failed".into())
        }
    }
    
    /// Check with retries
    pub async fn check_with_retries(
        &self,
        attempts: u32,
        delay_ms: u64
    ) -> HealthStatus
}
```

---

## 🔧 Конфигурация

### **`src/config.rs`**
```rust
pub struct Config {
    pub go_backend_url: String,  // ← URL Go backend на Koyeb
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            go_backend_url: env::var("GO_BACKEND_URL")
                .expect("GO_BACKEND_URL must be set"),
            // ...
        }
    }
}
```

### **`Secrets.toml`** (локально)
```toml
GO_BACKEND_URL = "https://yeasty-madelaine-fodi999-671ccdf5.koyeb.app"
JWT_SECRET = "your-secret"
OPENAI_API_KEY = "sk-..."
```

### **Shuttle Secrets** (production)
```rust
#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleAxum {
    // Load GO_BACKEND_URL from Shuttle Secrets
    if let Some(go_backend_url) = secrets.get("GO_BACKEND_URL") {
        std::env::set_var("GO_BACKEND_URL", go_backend_url);
    }
    
    let config = Config::from_env();
    let state = AppState::new(config);
    
    // Create Go Backend Client
    let go_client = GoBackendClient::new(&state.config);
}
```

---

## 🚀 Как Rust Управляет Go Backend

### **Сценарий 1: Пользователь логинится**

```
1. User → Rust Bot: POST /api/v1/auth/login
   └─ Body: {"email": "user@example.com", "password": "pass123"}

2. Rust Bot → Go Backend: POST https://yeasty-madelaine.../auth/login
   └─ AuthClient::login() делает HTTP запрос

3. Go Backend → PostgreSQL: SELECT * FROM users WHERE email=...
   └─ Проверяет credentials

4. Go Backend → Rust Bot: 200 OK
   └─ Body: {"token": "jwt-token-here", "user": {...}}

5. Rust Bot → User: 200 OK
   └─ Возвращает токен пользователю
```

---

### **Сценарий 2: Получить продукты**

```
1. User → Rust Bot: GET /api/v1/products

2. Rust Bot → Go Backend: GET https://yeasty-madelaine.../products
   └─ ProductsClient::get_products()

3. Go Backend → PostgreSQL: SELECT * FROM products

4. Go Backend → Rust Bot: [{"id": 1, "name": "Pizza", ...}, ...]

5. Rust Bot → User: Formatted product list
```

---

### **Сценарий 3: Admin запрашивает статистику**

```
1. Admin → Rust Bot: GET /api/v1/admin/stats
   └─ Header: Authorization: Bearer admin-jwt-token

2. Rust Bot → Go Backend: GET https://yeasty-madelaine.../admin/stats
   └─ AdminClient::get_stats(token)
   └─ Header: Authorization: Bearer admin-jwt-token

3. Go Backend:
   ├─ Проверяет JWT токен
   ├─ Проверяет роль (admin?)
   └─ Если admin: SELECT COUNT(*) FROM users, orders, ...

4. Go Backend → Rust Bot: 
   {
     "total_users": 150,
     "total_orders": 1234,
     "total_revenue": 45678.90
   }

5. Rust Bot → Admin: JSON stats
```

---

### **Сценарий 4: Создание заказа**

```
1. User → Rust Bot: POST /api/v1/orders
   └─ Body: {"items": [...], "total": 25.50}

2. Rust Bot → Go Backend: POST https://yeasty-madelaine.../orders
   └─ OrdersClient::create_order(order_data)

3. Go Backend → PostgreSQL:
   ├─ INSERT INTO orders (user_id, total, status, ...)
   └─ INSERT INTO order_items (order_id, product_id, quantity, ...)

4. Go Backend → Rust Bot: 
   {
     "id": 567,
     "status": "pending",
     "total": 25.50,
     "created_at": "2025-01-21T10:30:00Z"
   }

5. Rust Bot → User: Order confirmation
```

---

## 🎮 Control Layer (Управление Процессом)

### **Запуск Backend через API**

```bash
# Start backend
curl -X POST http://localhost:8000/api/v1/admin/backend/start

# Response:
{
  "success": true,
  "message": "Backend started successfully",
  "pid": 12345,
  "status": "running"
}
```

### **Проверка статуса**

```bash
# Get status
curl http://localhost:8000/api/v1/admin/backend/status

# Response:
{
  "status": "running",
  "pid": 12345,
  "uptime_secs": 3600,
  "restart_count": 0,
  "last_health_check": "healthy",
  "is_running": true
}
```

### **Перезапуск Backend**

```bash
# Restart backend
curl -X POST http://localhost:8000/api/v1/admin/backend/restart

# Response:
{
  "success": true,
  "message": "Backend restarted successfully",
  "restart_count": 1,
  "pid": 12346
}
```

---

## 🔄 Auto-Recovery (Автоматическое Восстановление)

**Health Monitoring Task:**
```rust
pub fn start_health_monitoring(self: Arc<Self>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(
            Duration::from_secs(30)  // Проверка каждые 30 секунд
        );
        
        loop {
            interval.tick().await;
            
            let health = self.health_checker.check().await;
            
            match health {
                HealthStatus::Healthy => {
                    // Backend OK
                }
                HealthStatus::Unhealthy(reason) => {
                    // Backend упал - попытка auto-restart
                    if self.config.auto_restart {
                        self.restart().await;
                    }
                }
            }
        }
    })
}
```

**Auto-Restart Logic:**
- ✅ Проверка здоровья каждые 30 секунд
- ✅ Если backend не отвечает → автоперезапуск
- ✅ Максимум 3 попытки перезапуска
- ✅ После 3 неудач → status: "crashed"

---

## 📊 Структура Data Flow

```
┌────────────────────────────────────────────────────────────┐
│                         USER                               │
└────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────────┐
│                   RUST BOT (Shuttle)                       │
│                                                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │ REST API     │  │ WebSocket    │  │ AI Assistant │    │
│  │ /api/v1/*    │  │ /ws          │  │ Groq LLM     │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│          │                 │                  │            │
│          └─────────────────┴──────────────────┘            │
│                           │                                │
│                  ┌────────▼────────┐                       │
│                  │ GoBackendClient │                       │
│                  └────────┬────────┘                       │
│                           │                                │
│     ┌─────────────────────┼─────────────────────┐         │
│     │                     │                     │         │
│     ▼                     ▼                     ▼         │
│ ┌─────────┐         ┌──────────┐         ┌─────────┐     │
│ │  Auth   │         │ Products │         │  Orders │     │
│ │ Client  │         │  Client  │         │  Client │     │
│ └─────────┘         └──────────┘         └─────────┘     │
└────────────────────────────────────────────────────────────┘
                           │
                           │ HTTP/REST
                           │
                           ▼
┌────────────────────────────────────────────────────────────┐
│           GO BACKEND (Koyeb)                               │
│   https://yeasty-madelaine-fodi999-671ccdf5.koyeb.app      │
│                                                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │ Auth Service │  │ Product Svc  │  │ Order Svc    │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│          │                 │                  │            │
│          └─────────────────┴──────────────────┘            │
│                           │                                │
│                           ▼                                │
│                  ┌────────────────┐                        │
│                  │   PostgreSQL   │                        │
│                  │   Database     │                        │
│                  └────────────────┘                        │
└────────────────────────────────────────────────────────────┘
```

---

## 🎯 Итог: Что Умеет Rust Bot

### ✅ **Полное Управление Go Backend:**
1. **CRUD операции** - создание/чтение/обновление/удаление всех сущностей
2. **Аутентификация** - login, register, verify token
3. **Бизнес-логика** - orders, products, users, stats
4. **Admin функции** - управление пользователями, заказами, инвентарём
5. **Process control** - start/stop/restart Go backend
6. **Health monitoring** - проверка здоровья каждые 30 секунд
7. **Auto-recovery** - автоматический перезапуск при падении

### ✅ **Единый Фасад:**
- Все запросы к Go backend идут через `GoBackendClient`
- Единая конфигурация (`GO_BACKEND_URL`)
- Централизованная обработка ошибок
- JWT токены для аутентификации

### ✅ **Production-Ready:**
- Retry логика для HTTP запросов
- Health checks с таймаутами
- Auto-restart до 3 попыток
- Structured logging (tracing)
- Graceful shutdown

---

## 📚 Полный Список Файлов

```
src/api/go_backend/
├── mod.rs               # Главный фасад GoBackendClient
├── auth.rs              # Аутентификация (login/register/verify)
├── products.rs          # Продукты (get/search/filter)
├── orders.rs            # Заказы (create/get/update status)
├── admin.rs             # Admin (stats/ingredients/movements)
└── types.rs             # Общие типы (LoginResponse, Product, Order, etc.)

src/orchestration/
├── mod.rs               # Exports
├── backend.rs           # BackendOrchestrator (start/stop/restart)
└── health.rs            # HealthChecker (проверка /health endpoint)

src/api/
├── backend_control.rs   # REST API для управления backend
├── rest.rs              # Основные REST endpoints
└── mod.rs               # API module exports

src/
├── config.rs            # Config::go_backend_url
├── state.rs             # AppState с GoBackendClient
└── main.rs              # Shuttle main - загрузка secrets, роутер
```

---

**🎉 Rust бот полностью управляет Go backend на Koyeb!**

- **URL**: `https://yeasty-madelaine-fodi999-671ccdf5.koyeb.app`
- **Protocol**: HTTP/REST API
- **Auth**: JWT Bearer tokens
- **Health**: Automatic monitoring + auto-restart
- **Control**: Full lifecycle management (start/stop/restart)

**Production-ready architecture с полным контролем над backend! 🚀**

# 🎮 Backend Orchestrator - Руководство по Использованию

## 📋 Что это?

**Backend Orchestrator** позволяет Rust боту **полностью управлять** Go backend процессом:
- ✅ Запуск/остановка/перезапуск
- ✅ Мониторинг здоровья каждые 30 секунд
- ✅ Автоматический перезапуск при падении
- ✅ REST API для удалённого управления

---

## 🚀 Quick Start

### 1. Настройка `.env`

Добавь в `.env` файл:

```bash
# Backend Orchestrator Configuration
ORCHESTRATOR_ENABLED=true
GO_BACKEND_BINARY=../backend/bin/server
GO_BACKEND_URL=http://127.0.0.1:3000
```

**Параметры:**
- `ORCHESTRATOR_ENABLED` - включить/выключить оркестратор
- `GO_BACKEND_BINARY` - путь к Go бинарнику (относительный или абсолютный)
- `GO_BACKEND_URL` - URL на котором будет работать Go backend

### 2. Запуск Rust бота

```bash
# Development
cargo run --bin local

# Production
cargo run --release --bin local
```

**Что произойдёт:**
1. Rust бот запустится на http://127.0.0.1:8000
2. Автоматически запустится Go backend на http://127.0.0.1:3000
3. Начнётся health monitoring каждые 30 секунд
4. В консоли появятся логи:

```
🚀 Starting FodiFood Bot (Local Mode)...
✅ Configuration loaded
🤖 Initializing Multi-Agent AI System...
🔧 Backend Orchestrator enabled
🚀 Starting Go backend process...
✅ Go backend started (PID: 12345)
💚 Backend health check: Healthy
🎯 Server listening on http://127.0.0.1:8000
```

---

## 🎯 REST API Управления

### Запустить Backend

```bash
curl -X POST http://127.0.0.1:8000/api/v1/admin/backend/start
```

**Response:**
```json
{
  "success": true,
  "message": "Backend started successfully",
  "pid": 12345,
  "status": "running"
}
```

---

### Проверить Статус

```bash
curl http://127.0.0.1:8000/api/v1/admin/backend/status
```

**Response:**
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

**Статусы:**
- `running` - процесс работает нормально
- `stopped` - процесс остановлен
- `starting` - процесс запускается
- `unhealthy` - процесс не отвечает на health check
- `crashed` - процесс упал

---

### Перезапустить Backend

```bash
curl -X POST http://127.0.0.1:8000/api/v1/admin/backend/restart
```

**Response:**
```json
{
  "success": true,
  "message": "Backend restarted successfully",
  "restart_count": 1,
  "pid": 12346
}
```

**Что происходит:**
1. Текущий процесс останавливается (graceful shutdown)
2. Ожидание 2 секунды
3. Запускается новый процесс
4. Проверка здоровья нового процесса

---

### Остановить Backend

```bash
curl -X POST http://127.0.0.1:8000/api/v1/admin/backend/stop
```

**Response:**
```json
{
  "success": true,
  "message": "Backend stopped successfully"
}
```

---

### Health Check Оркестратора

```bash
curl http://127.0.0.1:8000/api/v1/admin/backend/health
```

**Response:**
```json
{
  "orchestrator_enabled": true,
  "backend_status": "running",
  "health_check_interval": 30,
  "auto_restart": true,
  "max_restart_attempts": 3
}
```

---

## ⚙️ Расширенная Конфигурация

### Все Параметры `.env`

```bash
# Required
ORCHESTRATOR_ENABLED=true
GO_BACKEND_BINARY=../backend/bin/server
GO_BACKEND_URL=http://127.0.0.1:3000

# Optional - Health Monitoring
HEALTH_CHECK_INTERVAL=30      # Интервал проверки (секунды)
HEALTH_CHECK_TIMEOUT=5        # Таймаут запроса (секунды)

# Optional - Auto-Restart
AUTO_RESTART=true             # Автоматический перезапуск
MAX_RESTART_ATTEMPTS=3        # Максимум попыток

# Optional - Working Directory
BACKEND_WORKING_DIR=/tmp      # Рабочая директория для процесса
```

### Пример Production `.env`

```bash
# Production Configuration
ORCHESTRATOR_ENABLED=true
GO_BACKEND_BINARY=/opt/app/backend
GO_BACKEND_URL=http://0.0.0.0:8080
HEALTH_CHECK_INTERVAL=60
HEALTH_CHECK_TIMEOUT=10
AUTO_RESTART=true
MAX_RESTART_ATTEMPTS=5
```

---

## 🔍 Мониторинг и Логи

### Логи Rust бота

```bash
# Все логи
tail -f logs/bot.log

# Только оркестратор
tail -f logs/bot.log | grep "Backend"
```

**Примеры логов:**

```
INFO  Backend Orchestrator enabled
INFO  Starting Go backend process: /path/to/backend
INFO  Go backend started successfully (PID: 12345)
INFO  Backend health check: Healthy
WARN  Backend health check failed: Connection timeout
ERROR Backend crashed! Auto-restart attempt 1/3
INFO  Backend restarted successfully (PID: 12346)
```

---

### Prometheus Metrics

Backend оркестратор экспортирует метрики:

```bash
curl http://127.0.0.1:8000/metrics | grep backend
```

**Метрики:**
```
# Backend status (0=stopped, 1=running, 2=unhealthy, 3=crashed)
backend_status 1

# Uptime in seconds
backend_uptime_seconds 3600

# Restart count
backend_restart_count 0

# Health check success rate
backend_health_check_success_rate 1.0
```

---

## 🛠️ Troubleshooting

### Backend не запускается

**Проблема:**
```json
{
  "success": false,
  "error": "Failed to start backend: No such file or directory"
}
```

**Решение:**
1. Проверь путь к бинарнику:
   ```bash
   ls -la ../backend/bin/server
   ```
2. Проверь права на выполнение:
   ```bash
   chmod +x ../backend/bin/server
   ```
3. Попробуй абсолютный путь:
   ```bash
   GO_BACKEND_BINARY=/full/path/to/server
   ```

---

### Health Check постоянно падает

**Проблема:**
```
WARN Backend health check failed: Connection refused
```

**Решение:**
1. Проверь что Go backend слушает на правильном порту:
   ```bash
   netstat -an | grep 3000
   ```
2. Проверь health endpoint:
   ```bash
   curl http://127.0.0.1:3000/health
   ```
3. Увеличь таймаут:
   ```bash
   HEALTH_CHECK_TIMEOUT=10
   ```

---

### Auto-Restart не работает

**Проблема:**
```
ERROR Backend crashed! Maximum restart attempts (3) exceeded
```

**Решение:**
1. Проверь логи Go backend:
   ```bash
   ./backend/bin/server
   ```
2. Увеличь лимит:
   ```bash
   MAX_RESTART_ATTEMPTS=5
   ```
3. Отключи auto-restart для отладки:
   ```bash
   AUTO_RESTART=false
   ```

---

## 📊 Сценарии Использования

### Development

```bash
# .env
ORCHESTRATOR_ENABLED=true
GO_BACKEND_BINARY=../backend/bin/server
GO_BACKEND_URL=http://127.0.0.1:3000
AUTO_RESTART=true

# Запуск
cargo run --bin local
```

**Плюсы:**
- Автоматический запуск backend
- Быстрый restart при изменениях
- Удобные логи в одном терминале

---

### Production (Single Server)

```bash
# .env
ORCHESTRATOR_ENABLED=true
GO_BACKEND_BINARY=/opt/app/backend
GO_BACKEND_URL=http://0.0.0.0:8080
HEALTH_CHECK_INTERVAL=60
AUTO_RESTART=true
MAX_RESTART_ATTEMPTS=5

# Запуск как systemd service
systemctl start fodifood-bot
```

**Плюсы:**
- Автоматический recovery при падении
- Health monitoring
- Centralised управление через Rust API

---

### Production (Separate Servers)

```bash
# Rust Bot Server
ORCHESTRATOR_ENABLED=false  # Backend на другом сервере
GO_BACKEND_URL=http://backend.internal:8080

# Backend Server
# Запускается отдельно (systemd, docker, k8s)
```

**Плюсы:**
- Horizontal scaling
- Independent deployment
- Better resource isolation

---

### Docker Compose

```yaml
version: '3.8'

services:
  rust-bot:
    image: fodifood-bot:latest
    environment:
      - ORCHESTRATOR_ENABLED=false
      - GO_BACKEND_URL=http://go-backend:8080
    ports:
      - "8000:8000"
    depends_on:
      - go-backend

  go-backend:
    image: fodifood-backend:latest
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://...
```

---

## 🎯 Best Practices

### ✅ DO

1. **Используй абсолютные пути в production:**
   ```bash
   GO_BACKEND_BINARY=/opt/app/backend
   ```

2. **Настрой мониторинг:**
   ```bash
   # Prometheus scraping
   curl http://localhost:8000/metrics
   ```

3. **Логируй restart events:**
   ```rust
   // Логи автоматически пишутся в STDOUT
   ```

4. **Тестируй graceful shutdown:**
   ```bash
   curl -X POST http://localhost:8000/api/v1/admin/backend/stop
   ```

---

### ❌ DON'T

1. **Не используй orchestrator если backend на другом сервере:**
   ```bash
   # Если backend = https://backend.company.com
   ORCHESTRATOR_ENABLED=false  # ← Правильно!
   ```

2. **Не ставь слишком короткий timeout:**
   ```bash
   HEALTH_CHECK_TIMEOUT=1  # ❌ Слишком короткий
   HEALTH_CHECK_TIMEOUT=5  # ✅ Адекватный
   ```

3. **Не забывай про MAX_RESTART_ATTEMPTS:**
   ```bash
   # Без лимита может быть restart loop
   MAX_RESTART_ATTEMPTS=3  # ✅
   ```

---

## 🔗 Integration с CI/CD

### GitHub Actions

```yaml
name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Build Go Backend
        run: |
          cd backend
          go build -o bin/server
      
      - name: Build Rust Bot
        run: cargo build --release --bin local
      
      - name: Deploy
        run: |
          scp backend/bin/server server:/opt/app/
          scp target/release/local server:/opt/app/
          
          ssh server "systemctl restart fodifood-bot"
      
      - name: Health Check
        run: |
          sleep 10
          curl http://server:8000/api/v1/admin/backend/status
```

---

## 📝 Summary

**Orchestrator включается в 3 шага:**

1. **Настрой `.env`:**
   ```bash
   ORCHESTRATOR_ENABLED=true
   GO_BACKEND_BINARY=../backend/bin/server
   GO_BACKEND_URL=http://127.0.0.1:3000
   ```

2. **Запусти Rust bot:**
   ```bash
   cargo run --release --bin local
   ```

3. **Управляй через API:**
   ```bash
   curl http://127.0.0.1:8000/api/v1/admin/backend/status
   ```

**Результат:**
- ✅ Автоматический запуск Go backend
- ✅ Health monitoring каждые 30 секунд
- ✅ Auto-restart при падении
- ✅ REST API для управления
- ✅ Prometheus metrics
- ✅ Structured logging

---

**🎯 Production Ready! Go backend полностью под контролем Rust бота! 🚀**

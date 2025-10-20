# 🤖 FodiFood Bot - Полное Описание Программы

## 🎯 Что это за программа?

**FodiFood Bot** - это интеллектуальный backend-сервис на языке Rust, который служит центральным коммуникационным узлом для ресторанной платформы FodiFood. Программа объединяет клиентов, администраторов и бизнес-логику через AI-powered чат-бота.

## 🌟 Основное Назначение

### Для кого эта программа?

1. **🍽️ Рестораны и кафе** - автоматизация обслуживания клиентов
2. **👥 Клиенты** - удобное взаимодействие с рестораном через чат
3. **💼 Инвесторы** - анализ бизнес-метрик и принятие решений
4. **👨‍💼 Администраторы** - управление заказами и бизнесом
5. **📊 Аналитики** - мониторинг и статистика в реальном времени

### Что она умеет делать?

## 🧠 1. AI-Ассистент (Искусственный Интеллект)

### Понимание естественного языка
```
Клиент: "Что у вас есть из креветок?"
Бот: 🍤 У нас есть:
     • Креветки в соусе терияки - 890₽
     • Салат с тигровыми креветками - 650₽
     • Роллы с креветкой темпура - 420₽
```

### 17 типов намерений (intents)
1. **Меню** - поиск блюд, категории, цены
2. **Заказы** - создание, отслеживание, история
3. **Рекомендации** - AI предлагает блюда
4. **Аналитика** - статистика для админов
5. **Small talk** - обычный разговор
6. **Новости** - акции, события
7. **Анализ бизнеса** 💼 - инвестиционный скоринг
8. **Сравнение бизнесов** 📊 - сравнительный анализ
9. **Бизнес-советы** 💡 - AI рекомендации
10. **Управление складом** - проверка остатков
11. **Отзывы** - работа с feedback
12. **Резервирование столов**
13. **Доставка** - трекинг заказа
14. **Промокоды** - активация скидок
15. **Жалобы** - обработка проблем
16. **FAQ** - частые вопросы
17. **Управление профилем**

### Когнитивный анализ
- **Эмоции**: определяет настроение пользователя
- **Сложность**: понимает сложные запросы
- **Контекст**: помнит предыдущие сообщения
- **Персонализация**: адаптируется под пользователя

## 💼 2. Business Intelligence (Бизнес-Аналитика)

### Инвестиционный анализ
```
Инвестор: "проанализируй бизнес Tech Startup"

Бот: 🏢 Tech Startup
     📊 Оценка: 90/100 - Отличные показатели
     
     📈 Метрики:
     • Цена токена: $28.13 (+48.1% ⬆️)
     • Инвесторы: 2 чел.
     • ROI: 235.1%
     
     💡 Рекомендация: ПОКУПАТЬ ✅
     Высокий рост и доходность
```

### Возможности:
- ✅ **Investment Scoring** - балл от 0 до 100
- ✅ **Trend Analysis** - рост, стагнация, падение
- ✅ **ROI Calculation** - рентабельность инвестиций
- ✅ **Multi-comparison** - сравнение до 10 бизнесов
- ✅ **AI Recommendations** - умные советы
- ⚡ **Performance** - 80-550ms время ответа

## 🌐 3. WebSocket Коммуникация (Real-time)

### Для клиентов
```javascript
// Подключение к боту
const ws = new WebSocket('wss://bot.fodifood.com/ws');

// Аутентификация
ws.send(JSON.stringify({
  type: "auth",
  token: "your-jwt-token"
}));

// Отправка сообщения
ws.send(JSON.stringify({
  type: "chat",
  text: "Закажи мне пиццу Маргарита"
}));

// Получение ответа
ws.onmessage = (msg) => {
  const data = JSON.parse(msg.data);
  console.log(data.response); // "✅ Заказ создан!"
};
```

### Для администраторов
```javascript
// Admin WebSocket
const adminWs = new WebSocket('wss://bot.fodifood.com/api/v1/admin/ws');

// Получение уведомлений в реальном времени
adminWs.onmessage = (event) => {
  const notification = JSON.parse(event.data);
  // {type: "new_order", order_id: 128, total: 5400}
};
```

## 📊 4. Мониторинг и Метрики

### Prometheus метрики
```bash
# Экспорт метрик для Prometheus/Grafana
curl http://bot.fodifood.com/api/v1/metrics/prometheus

# Пример метрик:
ai_intent_total{intent="menu_query"} 456
ai_response_duration_seconds 0.245
websocket_connections_active 12
backend_health_status 1.0
```

### Web Dashboard
```
http://bot.fodifood.com/admin/metrics
```

Визуальный дашборд показывает:
- 📈 График запросов
- ⏱️ Время ответа AI
- 🎯 Распределение intents
- 👥 Активные подключения
- ✅ Success rate

### AI Insights (Real-time события)
```javascript
// Подключение к AI событиям
const insightWs = new WebSocket('ws://bot/api/v1/insight');

// Получение событий обработки
insightWs.onmessage = (event) => {
  const insight = JSON.parse(event.data);
  
  // Примеры событий:
  // {type: "intent_detected", intent: "menu", confidence: 0.95}
  // {type: "entity_extraction", entities: ["пицца", "маргарита"]}
  // {type: "response_generated", time_ms: 245}
};
```

## 🎯 5. Backend Orchestration (Управление Backend)

### Автоматическое управление Go процессом
```bash
# Запустить Go backend
curl -X POST http://bot/api/v1/admin/backend/start

# Статус
curl http://bot/api/v1/admin/backend/status
{
  "status": "running",
  "pid": 12345,
  "uptime_secs": 3600,
  "restart_count": 0,
  "is_running": true
}

# Остановить
curl -X POST http://bot/api/v1/admin/backend/stop

# Перезапустить
curl -X POST http://bot/api/v1/admin/backend/restart
```

### Health Monitoring
- ✅ Автоматические проверки каждые 30 сек
- ✅ Авто-перезапуск при сбоях
- ✅ Логирование событий
- ✅ PID tracking

## 🔐 6. Управление Пользователями и Бизнесом

### Роли пользователей
- **Client** - обычный клиент
- **Admin** - полный доступ
- **Manager** - управление заказами
- **Courier** - доставка
- **Cook** - кухня
- **Business Owner** - владелец бизнеса

### Обновление роли
```bash
curl -X PATCH http://bot/api/v1/user/role \
  -H "Authorization: Bearer TOKEN" \
  -d '{"role": "business_owner"}'
```

### Управление бизнесами
```bash
# Список всех бизнесов
curl http://bot/api/v1/businesses

# Создать новый бизнес (требует роль admin или business_owner)
curl -X POST http://bot/api/v1/businesses \
  -H "Authorization: Bearer TOKEN" \
  -d '{
    "name": "Sushi Bar",
    "category": "Japanese Restaurant",
    "city": "Gdansk"
  }'
```

## 🔄 7. Интеграция с Go Backend

### API Endpoints интеграции
```
✅ /api/auth/verify         - проверка JWT токенов
✅ /api/products            - меню ресторана
✅ /api/orders              - управление заказами
✅ /api/ingredients         - склад
✅ /api/stats               - статистика
✅ /api/businesses          - бизнесы (GET, POST)
✅ /api/metrics/:id         - метрики бизнеса
✅ /api/admin/users/update-role - обновление роли
```

### Webhook система
```
Go Backend → Rust Bot → Клиенты/Админы

События:
• new_order - новый заказ
• order_status_changed - статус изменился
• low_inventory - мало товара
• business_metrics_updated - обновление метрик
```

## 🪙 8. Solana Blockchain Integration

### Управление токенами
```bash
# Создать токен
cargo run --bin create_fodi_token

# Добавить метаданные
cargo run --bin add_fodi_metadata
```

### FODI Token
- **Mint Address**: `F9qcQ2HEmjDXmUygFiJjeiMHeF5PYSGnfzhRbETeP8Ek`
- **Symbol**: FODI
- **Supply**: 1,000,000,000
- **Decimals**: 9
- **Network**: Solana Devnet

### Функции:
- ✅ Создание SPL токенов
- ✅ Добавление метаданных (Metaplex)
- ✅ Управление supply
- ✅ Интеграция с кошельками (Phantom, Solflare)

## 🛠️ 9. Технические Возможности

### Асинхронная архитектура
- **Tokio runtime** - эффективная работа с async/await
- **Многопоточность** - параллельная обработка запросов
- **Connection pooling** - эффективное управление подключениями

### Производительность
- ⚡ **80-550ms** - время ответа AI
- ⚡ **<100ms** - REST API endpoints
- ⚡ **Real-time** - WebSocket латентность <50ms
- ⚡ **1000+** - одновременных подключений

### Безопасность
- 🔐 JWT аутентификация
- 🔐 Role-based access control (RBAC)
- 🔐 Защита приватных ключей
- 🔐 CORS настройки
- 🔐 Rate limiting

## 📈 10. Мониторинг и Отладка

### Логирование
```bash
# Уровни логов
RUST_LOG=debug cargo run

# Production логи (Shuttle)
cargo shuttle logs --follow
```

### Debug утилиты
```bash
# CLI чат для тестирования
cargo run --bin chat

# Локальный сервер
cargo run --bin local
```

### Тестирование
```bash
# 60+ unit tests
cargo test

# Тесты AI engine
cargo test ai::tests --nocapture

# Тесты Business Intelligence
cargo test business --nocapture
```

## 🌍 Примеры Использования

### 1. Ресторан "Суши Бар"
**Задача**: Автоматизировать обслуживание клиентов

**Решение**:
- Клиенты общаются с AI ботом в чате
- Бот показывает меню, принимает заказы
- Админы получают уведомления о новых заказах
- Метрики показывают популярные блюда

**Результат**: 40% меньше нагрузки на персонал

### 2. Инвестиционная платформа
**Задача**: Помочь инвесторам выбрать бизнес

**Решение**:
- AI анализирует метрики всех бизнесов
- Рассчитывает investment score (0-100)
- Сравнивает ROI, тренды, риски
- Дает персональные рекомендации

**Результат**: Более обоснованные инвестиционные решения

### 3. Сеть ресторанов
**Задача**: Мониторинг всей сети в реальном времени

**Решение**:
- WebSocket dashboard для админов
- Real-time метрики всех точек
- AI Insights показывают проблемные места
- Автоматические уведомления о проблемах

**Результат**: Быстрая реакция на инциденты

## 🚀 Деплой и Масштабирование

### Платформа
- **Shuttle.rs** - managed Rust hosting
- **Автоматическое масштабирование**
- **Zero-downtime deployments**
- **SSL из коробки**

### Команды
```bash
# Деплой
cargo shuttle deploy

# Логи
cargo shuttle logs

# Статус
cargo shuttle status
```

## 📊 Статистика Проекта

- **16,000+ строк кода** (Rust)
- **17 AI intent handlers**
- **60+ unit tests**
- **9 типов real-time событий**
- **10+ REST API endpoints**
- **3 WebSocket endpoints**
- **Solana blockchain** интеграция
- **OpenAI GPT-4o-mini** AI

## 🎯 Итого: Что умеет программа?

### Для клиентов:
✅ Принимать заказы через AI чат
✅ Показывать меню с рекомендациями
✅ Отвечать на вопросы о ресторане
✅ Отслеживать статус заказа
✅ Давать персональные советы

### Для бизнеса:
✅ Анализировать инвестиционную привлекательность
✅ Сравнивать метрики разных бизнесов
✅ Рассчитывать ROI и тренды
✅ Давать AI-рекомендации по улучшению
✅ Управлять токенами на Solana

### Для администраторов:
✅ Управлять заказами в реальном времени
✅ Мониторить метрики системы
✅ Получать уведомления о событиях
✅ Управлять пользователями и ролями
✅ Контролировать backend процессы

### Для разработчиков:
✅ REST API для интеграций
✅ WebSocket для real-time
✅ Prometheus метрики
✅ AI Insights для отладки
✅ Подробное логирование

---

**🎉 Вывод**: Это полноценная платформа для автоматизации ресторанного бизнеса с AI, blockchain интеграцией, бизнес-аналитикой и real-time коммуникацией!

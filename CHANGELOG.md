# Changelog

Все важные изменения в проекте будут документированы в этом файле.

Формат основан на [Keep a Changelog](https://keepachangelog.com/ru/1.0.0/),
и проект следует [Semantic Versioning](https://semver.org/lang/ru/).

## [Unreleased]

### Планируется
- [ ] Поддержка мультиязычности (RU/EN)
- [ ] Интеграция с Telegram Bot API
- [ ] Персистентное хранилище сообщений
- [ ] Аналитика и метрики
- [ ] Admin dashboard

## [0.1.0] - 2025-10-12

### Добавлено
- ✨ Базовая архитектура WebSocket сервера на Axum
- 🤖 Интеграция с OpenAI GPT-4o-mini
- 🔐 JWT аутентификация через Go backend
- 👥 Система ролей (Client, Admin, Manager, Courier, Cook)
- 💬 Обработка chat сообщений
- ⚡ Command система для структурированных запросов
- 🔔 Webhook endpoint для событий от Go backend
- 📡 REST клиент для Go backend API
- 🧠 AI-powered детекция интентов
- 📊 Поддержка событий: new_order, order_status_changed, low_inventory
- 📝 Подробная документация (README, EXAMPLES, DEVELOPMENT)
- 🛠️ Makefile для удобной разработки
- 🐳 Dockerfile для локальной разработки
- 🔄 GitHub Actions CI/CD
- 🚀 Готовность к деплою на Shuttle.rs

### Endpoints
- `GET /` - Информация о сервисе
- `GET /health` - Health check
- `GET /ws` - WebSocket endpoint
- `POST /notify` - Webhook для событий

### AI Возможности
- Генерация ответов клиентам
- Анализ бизнес-данных
- Персонализированные рекомендации
- Помощь менеджерам в принятии решений

### Интеграция с Go Backend
- `/api/auth/verify` - Верификация JWT токенов
- `/api/products` - Получение меню
- `/api/orders` - Управление заказами
- `/api/ingredients` - Остатки на складе
- `/api/stats` - Статистика продаж

---

## Формат записей

### Добавлено (Added)
Для новых функций.

### Изменено (Changed)
Для изменений в существующей функциональности.

### Устарело (Deprecated)
Для функций, которые скоро будут удалены.

### Удалено (Removed)
Для удаленных функций.

### Исправлено (Fixed)
Для исправлений багов.

### Безопасность (Security)
В случае уязвимостей.

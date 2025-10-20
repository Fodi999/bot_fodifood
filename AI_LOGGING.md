# 📝 AI Activity Logging - FodiFood Bot

## ✅ Что это делает

Все взаимодействия с Groq AI автоматически логируются в файл `ai_activity.log` для:
- 🐛 Отладки и диагностики
- 📊 Мониторинга активности AI
- 🔍 Анализа качества ответов
- 📈 Сбора данных для улучшения промптов

---

## 📂 Файл лога

**Расположение**: `/Users/dmitrijfomin/Desktop/bot_fodifood/ai_activity.log`

**Формат каждой записи**:
```
------------------------------------------------------------
⏰ Timestamp: 2025-10-20 15:35:49 UTC
🧠 Prompt: What are the top 3 seafood dishes?
💬 Response: Based on popularity and customer reviews...
```

---

## 🏷️ Типы логов (теги)

| Тег | Функция | Описание |
|-----|---------|----------|
| *(без тега)* | `Thinker::think()` | Основное мышление (Llama 3.3 70B) |
| `[FAST]` | `Thinker::think_fast()` | Быстрые ответы (Llama 3.1 8B) |
| `[BUSINESS]` | `Thinker::analyze_business()` | Бизнес-анализ |
| `[RECOMMEND]` | `Thinker::get_ai_recommendation()` | Персонализированные рекомендации |

---

## 🔧 Как использовать

### 1️⃣ Запустить тест логирования

```bash
cargo run --example ai_logging_test
```

### 2️⃣ Мониторинг в реальном времени

```bash
# В отдельном терминале
tail -f ai_activity.log
```

### 3️⃣ Просмотр всех логов

```bash
cat ai_activity.log
```

### 4️⃣ Поиск конкретных запросов

```bash
# Найти все бизнес-анализы
grep "\[BUSINESS\]" ai_activity.log

# Найти все ошибки
grep "ERROR:" ai_activity.log

# Найти все быстрые запросы
grep "\[FAST\]" ai_activity.log
```

### 5️⃣ Очистить старые логи

```bash
# Очистить файл
> ai_activity.log

# Или удалить полностью
rm ai_activity.log
```

---

## 📊 Примеры использования

### Отладка неудачного запроса

```bash
# Пользователь жалуется, что AI ответил неправильно
# Найди его запрос в логе:
grep "spicy seafood" ai_activity.log

# Результат:
🧠 Prompt: [RECOMMEND] Пользователь спрашивает: I want something spicy with seafood
💬 Response: Вы ищете что-то острое с морепродуктами?...
```

### Анализ качества ответов

```bash
# Посмотри последние 10 бизнес-анализов
grep -A 5 "\[BUSINESS\]" ai_activity.log | tail -60
```

### Мониторинг активности

```bash
# Сколько всего запросов?
grep "🧠 Prompt:" ai_activity.log | wc -l

# Сколько ошибок?
grep "ERROR:" ai_activity.log | wc -l

# Сколько быстрых запросов?
grep "\[FAST\]" ai_activity.log | wc -l
```

---

## 🎯 Интеграция в код

Логирование работает автоматически! Просто используй функции `Thinker`:

```rust
use crate::ai::thinker::Thinker;

// Все эти вызовы автоматически логируются
let response1 = Thinker::think("What is paella?").await?;
let response2 = Thinker::think_fast("Quick answer").await?;
let response3 = Thinker::analyze_business(data).await?;
let response4 = Thinker::get_ai_recommendation(context, prefs).await?;
```

---

## 🔒 Безопасность

### ⚠️ Важно: Не коммитить в Git!

Добавь в `.gitignore`:

```gitignore
# AI activity logs (may contain user data)
ai_activity.log
*.log
```

### 🔐 Для продакшена

Рекомендуется:
1. **Ротация логов**: Используй `logrotate` или аналог
2. **Шифрование**: Храни логи в зашифрованном виде
3. **Retention policy**: Удаляй логи старше N дней
4. **GDPR compliance**: Не логируй личные данные пользователей

---

## 📈 Продвинутое использование

### Структурированное логирование (будущее улучшение)

Можно улучшить до JSON-формата:

```rust
// Будущее улучшение:
fn log_activity_json(prompt: &str, response: &str) {
    let log_entry = json!({
        "timestamp": Utc::now().to_rfc3339(),
        "prompt": prompt,
        "response": response,
        "model": "llama-3.3-70b-versatile",
        "user_id": "optional",
    });
    // Write to JSON Lines format
}
```

### Метрики

Можно добавить:
- Время выполнения запроса
- Количество токенов
- Стоимость запроса
- ID сессии пользователя

---

## 🐛 Troubleshooting

### Проблема: Файл не создается

**Решение**: Проверь права доступа к директории:

```bash
ls -la ai_activity.log
# Если нет файла:
touch ai_activity.log
chmod 644 ai_activity.log
```

### Проблема: Лог слишком большой

**Решение**: Ротируй логи:

```bash
# Архивируй старый лог
mv ai_activity.log ai_activity.$(date +%Y%m%d).log

# Сожми
gzip ai_activity.20251020.log
```

### Проблема: Ошибки при записи

**Решение**: Проверь код в `thinker.rs`:

```rust
// OpenOptions обрабатывает ошибки тихо
if let Ok(mut log) = OpenOptions::new()
    .create(true)
    .append(true)
    .open("ai_activity.log")
{
    let _ = writeln!(log, "..."); // Игнорируем ошибки записи
}
```

---

## ✅ Примеры из реального лога

### Успешный запрос

```
------------------------------------------------------------
⏰ Timestamp: 2025-10-20 15:35:49 UTC
🧠 Prompt: What are the top 3 seafood dishes?
💬 Response: Based on popularity and customer reviews, the top 3 seafood dishes are:
1. **Grilled Salmon**: A classic favorite...
2. **Shrimp Scampi**: A flavorful and savory dish...
3. **Fish and Chips**: A comforting and crispy dish...
```

### Быстрый запрос

```
------------------------------------------------------------
⏰ Timestamp: 2025-10-20 15:35:50 UTC
🧠 Prompt: [FAST] What is sushi?
💬 Response: Sushi is a traditional Japanese dish...
```

### Бизнес-анализ (на русском)

```
------------------------------------------------------------
⏰ Timestamp: 2025-10-20 15:35:53 UTC
🧠 Prompt: [BUSINESS] Проанализируй бизнес-данные ресторана FodiFood:
Sales: $50000, Orders: 250, Top dish: Paella (78 orders)
💬 Response: **Анализ бизнес-данных ресторана FodiFood**
Рекомендации: ...
```

### Ошибка API

```
------------------------------------------------------------
⏰ Timestamp: 2025-10-20 15:40:00 UTC
🧠 Prompt: Test query
💬 Response: ERROR: Groq API error 429 Too Many Requests
```

---

## 🎓 Best Practices

1. ✅ **Регулярно проверяй логи** на наличие ошибок
2. ✅ **Анализируй типичные запро��ы** для улучшения промптов
3. ✅ **Мониторь время ответа** (можно добавить метрики)
4. ✅ **Очищай логи** старше 30 дней
5. ✅ **Не коммить** ai_activity.log в Git
6. ✅ **Защищай логи** от несанкционированного доступа

---

## 📚 Связанные файлы

- `src/ai/thinker.rs` - Основная логика логирования
- `examples/ai_logging_test.rs` - Тест логирования
- `ai_activity.log` - Файл с логами (не в Git!)

---

**Создано**: 20 октября 2025  
**Версия**: FodiFood Bot v0.1.0  
**AI**: Groq Llama 3.3 70B

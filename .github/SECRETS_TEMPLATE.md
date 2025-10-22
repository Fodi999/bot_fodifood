# GitHub Secrets Configuration

Для автоматического деплоя в Shuttle через GitHub Actions нужно добавить следующие секреты в репозитории:

**Settings → Secrets and variables → Actions → New repository secret**

---

## 🔑 Required Secrets

### 1. SHUTTLE_API_KEY
**Получение:**
```bash
# Login to Shuttle
shuttle login

# Get API key
cat ~/.config/shuttle/credentials.toml
```

**Значение:** Скопируй ключ из `api_key = "..."`

---

### 2. DATABASE_URL
**Источник:** Neon PostgreSQL

**Формат:**
```
postgresql://username:password@ep-xxx.us-east-2.aws.neon.tech/bot_fodifood?sslmode=require
```

**Где взять:**
1. Открой Neon Dashboard
2. Select project `bot_fodifood`
3. Copy Connection String (от PostgreSQL)

**Пример:**
```
postgresql://neondb_owner:npg_XXX@ep-cold-dawn-a6nfxdhy.us-east-2.aws.neon.tech/bot_fodifood?sslmode=require
```

---

### 3. GROQ_API_KEY
**Источник:** GroqCloud Console

**Формат:**
```
gsk_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

**Где взять:**
1. Открой https://console.groq.com/keys
2. Copy existing key ИЛИ Create new API key

---

### 4. GO_BACKEND_URL
**Источник:** Koyeb Deployment

**Значение:**
```
https://yeasty-madelaine-fodi999-671ccdf5.koyeb.app
```

**Проверка:**
```bash
curl https://yeasty-madelaine-fodi999-671ccdf5.koyeb.app/health
# Должно вернуть: {"status":"ok"}
```

---

### 5. SOLANA_KEYPAIR
**Источник:** Локальный файл `data/keypair.json`

**Формат:** JSON массив из 64 чисел
```json
[123,45,67,89,...]
```

**Где взять:**
```bash
# Прочитай keypair
cat data/keypair.json
```

**⚠️ ВАЖНО:** Это ВЕСЬ JSON, включая квадратные скобки

---

## 📝 Добавление секретов в GitHub

### Способ 1: Через Web UI

1. Открой репозиторий: https://github.com/Fodi999/bot_fodifood
2. Перейди в **Settings** → **Secrets and variables** → **Actions**
3. Нажми **New repository secret**
4. Введи имя (например, `DATABASE_URL`)
5. Вставь значение
6. Нажми **Add secret**
7. Повтори для всех 5 секретов

### Способ 2: Через GitHub CLI

```bash
# Install GitHub CLI (если нет)
brew install gh

# Login
gh auth login

# Добавь все секреты
gh secret set SHUTTLE_API_KEY < <(echo "your_shuttle_key")
gh secret set DATABASE_URL < <(echo "your_database_url")
gh secret set GROQ_API_KEY < <(echo "your_groq_key")
gh secret set GO_BACKEND_URL < <(echo "https://yeasty-madelaine-fodi999-671ccdf5.koyeb.app")
gh secret set SOLANA_KEYPAIR < data/keypair.json
```

---

## ✅ Проверка

После добавления всех секретов:

1. Перейди в **Actions** → **Deploy to Shuttle**
2. Нажми **Run workflow** → **Run workflow**
3. Дождись завершения (~5-7 минут)
4. Проверь статус:

```bash
# Health check
curl https://fodifood-bot.shuttleapp.rs/health

# Metrics
curl https://fodifood-bot.shuttleapp.rs/metrics

# Agents status
curl https://fodifood-bot.shuttleapp.rs/api/v1/admin/agents
```

---

## 🔒 Безопасность

- ❌ **НИКОГДА** не коммить секреты в репозиторий
- ✅ Используй GitHub Secrets для CI/CD
- ✅ Используй `Secrets.toml` для локальной разработки (в .gitignore)
- ✅ Ротируй ключи регулярно
- ✅ Используй разные ключи для development/production

---

## 🚨 Troubleshooting

### Secret не работает

```bash
# Проверь, что секрет добавлен
gh secret list

# Переустанови секрет
gh secret set DATABASE_URL < <(echo "new_value")
```

### Деплой падает

```bash
# Посмотри логи workflow
gh run list
gh run view <run-id>

# Проверь секреты в Actions logs
# (значения скрыты как ***)
```

### База данных не подключается

```bash
# Проверь DATABASE_URL локально
echo $DATABASE_URL

# Попробуй подключиться
psql "$DATABASE_URL" -c "SELECT 1"
```

---

## 📚 Дополнительные ресурсы

- [GitHub Secrets Documentation](https://docs.github.com/en/actions/security-guides/encrypted-secrets)
- [Shuttle Secrets Guide](https://docs.shuttle.rs/configuration/secrets)
- [Neon PostgreSQL](https://neon.tech/docs/get-started-with-neon/signing-up)
- [GroqCloud Console](https://console.groq.com/keys)

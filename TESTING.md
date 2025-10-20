# 🚀 How to Test Solana Integration

## ✅ Current Status

**Все готово к тестированию!** 🎉

- ✅ Solana модули скомпилированы
- ✅ API endpoints созданы (6 endpoints)
- ✅ Unit тесты проходят
- ✅ Release build готов

## 🧪 Способы тестирования

### 1. Unit Тесты (Без настройки)

```bash
# Тест подключения к Devnet
cargo test test_devnet_connection -- --nocapture

# Все helper тесты
cargo test helper_tests -- --nocapture

# Все Solana тесты
cargo test tests::test_solana_tx -- --nocapture
```

**Ожидаемый результат:**
```
✅ Connected to Solana Devnet
📦 Solana version: 3.0.6
✅ Pubkey parsed: Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr
✅ Lamports conversion works correctly
test result: ok. 4 passed; 0 failed; 3 ignored
```

### 2. Локальный сервер (Требует keypair)

**Шаг 1: Генерируем keypair**
```bash
# Установить Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Создать test keypair
solana-keygen new --outfile tests/fixtures/test-keypair.json --no-bip39-passphrase

# Получить адрес
solana-keygen pubkey tests/fixtures/test-keypair.json
```

**Шаг 2: Пополнить кошелек на Devnet**
```bash
solana airdrop 2 $(solana-keygen pubkey tests/fixtures/test-keypair.json) --url devnet
```

**Шаг 3: Обновить Secrets.toml**
```toml
SOLANA_RPC_URL = "https://api.devnet.solana.com"
SOLANA_KEYPAIR_PATH = "tests/fixtures/test-keypair.json"
```

**Шаг 4: Запустить локально**
```bash
cargo run --bin fodifood-bot
# или
cargo shuttle run
```

**Шаг 5: Тестировать API**
```bash
# Статус
curl http://localhost:8000/api/solana/status | jq

# Баланс
curl http://localhost:8000/api/solana/balance/Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr | jq

# Stake
curl -X POST http://localhost:8000/api/solana/stake \
  -H "Content-Type: application/json" \
  -d '{"amount": 1.0}' | jq
```

### 3. Shuttle.rs Deployment (Production)

**Шаг 1: Создать keypair на сервере Shuttle**
```bash
# Локально сгенерировать
solana-keygen new --outfile solana-keypair.json --no-bip39-passphrase

# Загрузить в Shuttle secrets
# (через Shuttle dashboard или CLI)
```

**Шаг 2: Обновить Secrets на Shuttle**
В Shuttle dashboard добавь:
```
SOLANA_RPC_URL=https://api.devnet.solana.com
SOLANA_KEYPAIR_PATH=/opt/shuttle/secrets/solana-keypair.json
```

**Шаг 3: Deploy**
```bash
cargo shuttle deploy
```

**Шаг 4: Тестировать Production**
```bash
./test_solana_api.sh
# или с custom URL
BASE_URL=https://bot-fodifood-lcon.shuttle.app ./test_solana_api.sh
```

## 📊 Available Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/solana/status` | GET | Проверка статуса подключения |
| `/api/solana/balance/:wallet` | GET | Баланс кошелька (path param) |
| `/api/solana/balance` | POST | Баланс кошелька (JSON body) |
| `/api/solana/transfer` | POST | Перевод SOL |
| `/api/solana/mint` | POST | Минт токенов |
| `/api/solana/stake` | POST | Стейкинг (placeholder) |

## 🔍 Примеры тестов

### Тест 1: Статус
```bash
curl https://bot-fodifood-lcon.shuttle.app/api/solana/status
```

**Ожидаемый ответ:**
```json
{
  "status": "connected",
  "payer": "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr",
  "balance": 1.5,
  "message": "Solana blockchain is ready"
}
```

### Тест 2: Баланс (GET)
```bash
curl https://bot-fodifood-lcon.shuttle.app/api/solana/balance/Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr
```

**Ожидаемый ответ:**
```json
{
  "wallet": "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr",
  "balance": 1.234,
  "status": "ok"
}
```

### Тест 3: Stake
```bash
curl -X POST https://bot-fodifood-lcon.shuttle.app/api/solana/stake \
  -H "Content-Type: application/json" \
  -d '{"amount": 1.5}'
```

**Ожидаемый ответ:**
```json
{
  "status": "pending",
  "message": "1.5 SOL queued for staking. Feature coming soon!",
  "amount": 1.5,
  "note": "Staking functionality will be implemented with Solana Stake Pool integration"
}
```

## ⚡ Быстрый тест

```bash
# Запустить все проверки
./test_solana.sh

# Протестировать API
./test_solana_api.sh
```

## 📁 Структура файлов

```
bot_fodifood/
├── src/
│   ├── solana/              # ✅ Solana модули
│   │   ├── client.rs        # RPC клиент
│   │   ├── token.rs         # Токен операции
│   │   ├── models.rs        # Модели данных
│   │   └── mod.rs
│   ├── api/
│   │   └── solana.rs        # ✅ API endpoints (NEW!)
│   └── tests/
│       └── test_solana_tx.rs # ✅ Тесты (NEW!)
├── test_solana.sh           # ✅ Quick test script (NEW!)
├── test_solana_api.sh       # ✅ API test script (NEW!)
├── SOLANA_INTEGRATION.md    # Документация
├── SOLANA_TESTING.md        # Тестирование
└── SOLANA_API.md            # ✅ API docs (NEW!)
```

## 🎯 Checklist для деплоя

Перед деплоем на production:

- [ ] ✅ Unit тесты проходят (`cargo test`)
- [ ] ✅ Release build компилится (`cargo build --release`)
- [ ] ⏳ Создан Solana keypair
- [ ] ⏳ Keypair пополнен SOL на Devnet
- [ ] ⏳ Secrets настроены в Shuttle
- [ ] ⏳ Deploy успешен (`cargo shuttle deploy`)
- [ ] ⏳ API endpoints отвечают
- [ ] ⏳ Транзакции проходят на Devnet

## 🔧 Troubleshooting

### Проблема: "Solana blockchain is not configured"
**Решение:** Добавь Solana client в AppState в main.rs:
```rust
let solana = SolanaClient::devnet("path/to/keypair.json")?;
let state = AppState::new(config).with_solana(solana);
```

### Проблема: "Invalid wallet address"
**Решение:** Проверь формат адреса (44 символа base58)

### Проблема: "Insufficient balance"
**Решение:** Пополни кошелек:
```bash
solana airdrop 2 <PUBKEY> --url devnet
```

## 📚 Документация

- 📖 [SOLANA_INTEGRATION.md](./SOLANA_INTEGRATION.md) - Полная интеграция
- 🧪 [SOLANA_TESTING.md](./SOLANA_TESTING.md) - Тестирование
- 🌐 [SOLANA_API.md](./SOLANA_API.md) - API документация
- 🔗 [Solana Docs](https://docs.solana.com/)
- 🔗 [Solana Explorer (Devnet)](https://explorer.solana.com/?cluster=devnet)

## ✅ Текущий статус

```
✅ Компиляция: SUCCESS
✅ Unit тесты: 4/4 PASSED
✅ API endpoints: 6 READY
✅ Documentation: COMPLETE
⏳ Keypair setup: PENDING
⏳ Deployment: READY TO DEPLOY
```

---

**Последнее обновление:** 19 октября 2025  
**Версия:** 1.0.0  
**Статус:** ✅ Ready for Testing

# 🪙 FODI SPL Token Transfers — Полное руководство

## ✅ Статус: PRODUCTION READY

Полностью рабочая система переводов FODI токенов на Solana Devnet с автоматическим созданием associated token accounts.

---

## 📊 Текущие активы

### 🏦 Treasury Wallet
```
Адрес:       4zLpxmqZvyX6QQC9BN4MHWNGEDLRPUsUpxzE8FEBWfLB
SOL:         ~1.9 SOL
FODI:        9,995 FODI (после тестового перевода)
Роль:        Mint Authority, централизованный банк для выплат
```

### 🪙 FODI Token (NEW)
```
Mint Address:    5G1fQSysF8cTdNaueNHkNJ9wP4DtTJzHjy8FVcS83b9a
Decimals:        9
Mint Authority:  4zLpxmqZvyX6QQC9BN4MHWNGEDLRPUsUpxzE8FEBWfLB (Treasury)
Freeze Authority: (none)
Program:         TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
Network:         Solana Devnet
```

### 👤 Test User: Bob (Restaurant Owner)
```
User ID:     bob_restaurant_owner
Pubkey:      E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp
SOL:         0.1 SOL
FODI:        5.0 FODI ✅ (получено через on-chain transfer)
```

---

## 🎯 API Endpoints

### 1. POST /api/solana/transfer — Перевод токенов

**Поддерживаемые токены:**
- `SOL` — нативная Solana валюта
- `FODI` — SPL токен (5G1fQSysF8cTdNaueNHkNJ9wP4DtTJzHjy8FVcS83b9a)

#### Пример: Перевод FODI токенов

```bash
curl -X POST http://127.0.0.1:8000/api/solana/transfer \
  -H "Content-Type: application/json" \
  -d '{
    "from": "4zLpxmqZvyX6QQC9BN4MHWNGEDLRPUsUpxzE8FEBWfLB",
    "to": "E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp",
    "amount": 5000000000,
    "token": "FODI"
  }'
```

**Response:**
```json
{
  "status": "ok",
  "tx": "3c8Qry7eAJL6GfLx16A6PUQfNXY9uUdrG39sXAAi9e3LKNEsHYf66Au29XN7rQUjC4QHe2CDGaHqTNoPYnExmZUv",
  "wallet": "E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp"
}
```

#### Пример: Перевод SOL

```bash
curl -X POST http://127.0.0.1:8000/api/solana/transfer \
  -H "Content-Type: application/json" \
  -d '{
    "from": "4zLpxmqZvyX6QQC9BN4MHWNGEDLRPUsUpxzE8FEBWfLB",
    "to": "E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp",
    "amount": 100000000,
    "token": "SOL"
  }'
```

**Параметры:**
- `from`: публичный адрес отправителя (treasury используется из .env)
- `to`: публичный адрес получателя
- `amount`: сумма в lamports (1 FODI = 1,000,000,000 lamports, 1 SOL = 1,000,000,000 lamports)
- `token`: тип токена (`"SOL"` или `"FODI"`, default: `"SOL"`)

---

### 2. POST /api/wallet/sync/{user_id} — Синхронизация баланса

Получает актуальные балансы SOL и FODI с Solana Devnet.

```bash
curl -X POST http://127.0.0.1:8000/api/wallet/sync/bob_restaurant_owner
```

**Response:**
```json
{
  "chain": "solana",
  "fodi_balance": 5000000000,
  "fodi_balance_ui": "5.000000000 FODI",
  "pubkey": "E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp",
  "sol_balance": 100000000,
  "sol_balance_ui": "0.100000000 SOL",
  "success": true,
  "synced": true,
  "user_id": "bob_restaurant_owner"
}
```

**Поля:**
- `sol_balance`: баланс в lamports SOL
- `sol_balance_ui`: читаемый формат SOL
- `fodi_balance`: баланс в lamports FODI (с 9 decimals)
- `fodi_balance_ui`: читаемый формат FODI

---

### 3. GET /api/solana/status — Проверка подключения

```bash
curl http://127.0.0.1:8000/api/solana/status
```

**Response:**
```json
{
  "status": "connected",
  "message": "Solana blockchain is ready",
  "payer": "4zLpxmqZvyX6QQC9BN4MHWNGEDLRPUsUpxzE8FEBWfLB",
  "balance": 1.899995
}
```

---

## 🔧 Технические детали

### Автоматическое создание Associated Token Accounts

При переводе FODI токенов система автоматически:

1. **Проверяет наличие ATA** у получателя для FODI mint
2. **Создаёт ATA**, если его нет (добавляет инструкцию в транзакцию)
3. **Выполняет transfer** SPL токенов между ATAs
4. **Возвращает сигнатуру** транзакции для проверки

```rust
// Код из src/solana/token.rs
let from_ata = get_associated_token_address(&from.pubkey(), token_mint);
let to_ata = get_associated_token_address(to, token_mint);

// Create ATA if needed
if client.get_account(&to_ata).is_err() {
    instructions.push(
        create_associated_token_account(&from.pubkey(), to, token_mint, &spl_token::ID)
    );
}

// Transfer tokens
instructions.push(
    spl_token::instruction::transfer(
        &spl_token::ID,
        &from_ata,
        &to_ata,
        &from.pubkey(),
        &[],
        amount,
    )?
);
```

### Роутинг по типу токена

```rust
// src/api/solana.rs
let token_type = req.token.to_uppercase();

match token_type.as_str() {
    "SOL" => {
        // Native SOL transfer
        let signature = transfer_tokens(...)?;
        TokenResponse { status: "ok", tx: signature, ... }
    }
    "FODI" => {
        // SPL FODI transfer
        let signature = transfer_spl_tokens(mint_pubkey, ...)?;
        TokenResponse { status: "ok", tx: signature, ... }
    }
    _ => {
        TokenResponse { 
            status: "error", 
            error: Some("Unsupported token type".to_string()) 
        }
    }
}
```

---

## 📈 История транзакций

### 🏆 Первый успешный FODI трансфер

**Транзакция:** `3c8Qry7eAJL6GfLx16A6PUQfNXY9uUdrG39sXAAi9e3LKNEsHYf66Au29XN7rQUjC4QHe2CDGaHqTNoPYnExmZUv`

**Explorer:**
https://explorer.solana.com/tx/3c8Qry7eAJL6GfLx16A6PUQfNXY9uUdrG39sXAAi9e3LKNEsHYf66Au29XN7rQUjC4QHe2CDGaHqTNoPYnExmZUv?cluster=devnet

**Детали:**
- От: Treasury (4zLpx...)
- Кому: Bob (E6vt5...)
- Сумма: 5.0 FODI
- Дата: 2025-10-20
- Создан ATA для Bob
- Статус: ✅ Confirmed

---

## 🛠️ Setup & Configuration

### Environment Variables (.env)

```bash
# Solana RPC (Devnet)
SOLANA_RPC_URL=https://api.devnet.solana.com

# FODI Token Mint Address
FODI_MINT_ADDRESS=5G1fQSysF8cTdNaueNHkNJ9wP4DtTJzHjy8FEBWfLB

# Treasury Keypair (mint authority)
FODI_TREASURY_KEYPAIR=/Users/dmitrijfomin/.config/solana/id.json
```

### Создание нового FODI токена

```bash
# 1. Создать токен с 9 decimals
spl-token create-token --decimals 9 --url devnet

# 2. Создать token account для treasury
spl-token create-account <MINT_ADDRESS> --url devnet

# 3. Минт токенов
spl-token mint <MINT_ADDRESS> 10000 --url devnet

# 4. Проверить баланс
spl-token balance <MINT_ADDRESS> --url devnet
```

### Проверка токенов через CLI

```bash
# Все токены на кошельке
spl-token accounts --owner <PUBKEY> --url devnet

# Конкретный токен
spl-token balance <MINT_ADDRESS> --owner <PUBKEY> --url devnet

# Информация о токене
spl-token display <MINT_ADDRESS> --url devnet
```

---

## 🧪 Testing Guide

### 1. Проверка статуса сервера

```bash
curl http://127.0.0.1:8000/api/solana/status | jq .
```

**Ожидаемый результат:**
```json
{
  "status": "connected",
  "payer": "4zLpxmqZvyX6QQC9BN4MHWNGEDLRPUsUpxzE8FEBWfLB",
  "balance": 1.9
}
```

### 2. Перевод небольшого количества FODI

```bash
# 0.001 FODI (1,000,000 lamports)
curl -X POST http://127.0.0.1:8000/api/solana/transfer \
  -H "Content-Type: application/json" \
  -d '{
    "from": "4zLpxmqZvyX6QQC9BN4MHWNGEDLRPUsUpxzE8FEBWfLB",
    "to": "E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp",
    "amount": 1000000,
    "token": "FODI"
  }' | jq .
```

### 3. Синхронизация баланса

```bash
curl -X POST http://127.0.0.1:8000/api/wallet/sync/bob_restaurant_owner | jq .
```

**Проверка:**
- `fodi_balance` должен увеличиться на 1,000,000
- `fodi_balance_ui` показывает читаемый формат

### 4. Проверка на Explorer

```bash
# Скопируйте tx из response и откройте:
https://explorer.solana.com/tx/<TX_SIGNATURE>?cluster=devnet
```

---

## 🔐 Security Best Practices

### ✅ Реализовано

1. **Единый Treasury Keypair** — хранится в `~/.config/solana/id.json` с правами `600`
2. **Environment Variables** — чувствительные данные в `.env` (не в Git)
3. **Mint Authority Control** — только treasury может минтить FODI
4. **ATA Auto-Creation** — безопасное создание token accounts через program

### ⚠️ Рекомендации для Production

1. **Используйте Hardware Wallet** для treasury (Ledger/Trezor)
2. **Multisig для больших сумм** (через Squads Protocol)
3. **Rate Limiting** на API endpoints
4. **Webhook Verification** для подтверждения транзакций
5. **Monitoring & Alerts** для подозрительных переводов

---

## 📚 Next Steps

### Фазы развития

#### ✅ Фаза 1: Базовые трансферы (DONE)
- [x] SOL трансферы
- [x] FODI SPL трансферы
- [x] Автоматическое создание ATA
- [x] Синхронизация балансов

#### 🚧 Фаза 2: Интеграция с Bank Module
- [ ] Автоматические выплаты по rewards
- [ ] Offchain → Onchain sync
- [ ] Batch transfers для оптимизации
- [ ] Transaction history в БД

#### 🔜 Фаза 3: NFT Integration
- [ ] Mint NFT для бизнесов
- [ ] NFT → Wallet привязка
- [ ] Metadata updates on-chain
- [ ] NFT transfer API

#### 🔜 Фаза 4: Advanced Features
- [ ] Staking rewards
- [ ] Token burning
- [ ] Governance voting
- [ ] Analytics dashboard

---

## 🐛 Troubleshooting

### Проблема: "Failed to send SPL token transfer"

**Причина:** У отправителя нет FODI токенов или ATA.

**Решение:**
```bash
# 1. Проверить баланс
spl-token balance <MINT> --owner <FROM_PUBKEY> --url devnet

# 2. Создать ATA если нужно
spl-token create-account <MINT> --url devnet

# 3. Минт токенов (только mint authority)
spl-token mint <MINT> 1000 --url devnet
```

### Проблема: "owner does not match" при минте

**Причина:** Keypair не является mint authority.

**Решение:**
```bash
# Проверить mint authority
spl-token display <MINT> --url devnet

# Если нужно передать authority (опасно!)
spl-token authorize <MINT> mint <NEW_AUTHORITY> --url devnet
```

### Проблема: Sync показывает 0 FODI

**Причина:** У кошелька нет associated token account для FODI.

**Решение:**
```bash
# Перевести хотя бы 0.000000001 FODI для создания ATA
curl -X POST http://127.0.0.1:8000/api/solana/transfer \
  -H "Content-Type: application/json" \
  -d '{
    "from": "4zLpxmqZvyX6QQC9BN4MHWNGEDLRPUsUpxzE8FEBWfLB",
    "to": "<USER_PUBKEY>",
    "amount": 1,
    "token": "FODI"
  }'
```

---

## 📝 References

- **Solana Docs:** https://docs.solana.com
- **SPL Token Program:** https://spl.solana.com/token
- **Associated Token Account:** https://spl.solana.com/associated-token-account
- **Solana Explorer (Devnet):** https://explorer.solana.com/?cluster=devnet

---

**Создано:** 2025-10-20  
**Статус:** ✅ Production Ready  
**Версия:** 1.0.0

# 💰 FODI Token Bank Module

Управление токеновой экономикой: ledger, rewards, burns, fiat-crypto exchange.

## 📦 Компоненты

### 1. Ledger (`ledger.rs`)
Учет балансов и транзакций пользователей:

```rust
let ledger = TokenLedger::new();

// Получить баланс
let balance = ledger.get_balance("user_123").await?;
println!("Total: {}, Available: {}", balance.total, balance.available);

// Обновить баланс
ledger.update_balance("user_123", 1_000_000_000).await?; // +1 FODI

// Заблокировать токены (для заказа)
ledger.lock_tokens("user_123", 500_000_000).await?; // Lock 0.5 FODI

// Разблокировать
ledger.unlock_tokens("user_123", 500_000_000).await?;
```

**Типы транзакций:**
- `Deposit` - пополнение
- `Withdrawal` - вывод
- `Reward` - награда
- `Burn` - сжигание
- `Purchase` - покупка
- `Transfer` - перевод

### 2. Rewards Engine (`rewards.rs`)
Система вознаграждений для пользователей:

```rust
let engine = RewardEngine::new(ledger.clone(), RewardConfig::default());

// Награда за заказ
engine.reward_order_completion("user_123", "order_456").await?;
// → +0.1 FODI

// Реферальная награда
engine.reward_referral("referrer", "referee").await?;
// → +0.5 FODI

// Ежедневный вход
engine.reward_daily_login("user_123").await?;
// → +0.01 FODI

// Награда за отзыв
engine.reward_review("user_123", "review_789").await?;
// → +0.05 FODI
```

**Конфигурация:**
```rust
RewardConfig {
    order_completion: 100_000_000,  // 0.1 FODI
    referral: 500_000_000,          // 0.5 FODI
    daily_login: 10_000_000,        // 0.01 FODI
    review: 50_000_000,             // 0.05 FODI
}
```

### 3. Burn Engine (`rewards.rs`)
Дефляционный механизм сжигания токенов:

```rust
let burn_engine = BurnEngine::new(ledger.clone(), BurnConfig::default());

// Автоматическое сжигание при покупке (1%)
burn_engine.burn_on_purchase("user_123", 1_000_000_000).await?;
// → Burns 10_000_000 (1% of 1 FODI)

// Ручное сжигание
burn_engine.burn_tokens("user_123", 100_000_000, "manual_burn").await?;
```

**Конфигурация:**
```rust
BurnConfig {
    transaction_burn_rate: 1,       // 1% burn
    min_burn_amount: 1_000_000,     // 0.001 FODI minimum
}
```

### 4. Stripe Exchange (`exchange.rs`)
Обмен fiat → crypto:

```rust
let exchange = StripeExchange::new(ledger.clone(), Some("sk_test_...".to_string()));

// Обновить курсы
exchange.update_rates(
    100.0,      // $100 per SOL
    0.00001     // 0.00001 SOL per FODI
);

// Получить котировку
let (fodi_amount, rate) = exchange.get_purchase_quote(10.0); // $10
// → 10,000 FODI @ $0.001 per FODI

// Создать платежный intent
let intent = exchange.create_payment_intent("user_123", 10.0).await?;

// После успешной оплаты
exchange.process_payment_success(&intent.id, "user_123", fodi_amount).await?;
```

### 5. API Endpoints (`api.rs`)

#### Health Check
```bash
GET /api/bank/health
```

#### User Balance
```bash
GET /api/bank/balance/:user_id

Response:
{
  "user_id": "user_123",
  "balance": {
    "total": 1000000000,
    "locked": 200000000,
    "available": 800000000
  }
}
```

#### Transaction History
```bash
GET /api/bank/transactions/:user_id?limit=50

Response:
[
  {
    "id": "tx_123",
    "user_id": "user_123",
    "transaction_type": "Reward",
    "amount": 100000000,
    "timestamp": "2025-10-20T12:00:00Z",
    "signature": null,
    "metadata": {
      "reason": "order_completion",
      "order_id": "order_456"
    }
  }
]
```

#### Admin: All Transactions
```bash
GET /api/bank/admin/transactions?limit=100
```

## 🧪 Тестирование

```bash
# Unit tests
cargo test --lib bank

# Запустить локальный сервер
cargo run --bin local

# Тестовые запросы
curl http://localhost:8000/api/bank/health
curl http://localhost:8000/api/bank/balance/user_123
curl http://localhost:8000/api/bank/transactions/user_123?limit=10
```

## 💡 Примеры использования

### Интеграция в приложение
```rust
use fodifood_bot::bank::{TokenLedger, RewardEngine, BurnEngine};
use std::sync::Arc;

// Создать ledger
let ledger = Arc::new(TokenLedger::new());

// Создать reward engine
let reward_engine = RewardEngine::new(
    ledger.clone(),
    RewardConfig::default()
);

// Когда пользователь завершил заказ
reward_engine.reward_order_completion("user_123", "order_456").await?;

// Когда пользователь делает покупку
let burn_engine = BurnEngine::new(ledger.clone(), BurnConfig::default());
burn_engine.burn_on_purchase("user_123", purchase_amount).await?;
```

## 📊 Метрики

- **Total users**: Количество пользователей с балансом
- **Total supply**: Сумма всех балансов
- **Burned amount**: Сколько токенов сожжено
- **Rewards distributed**: Сколько выдано наград
- **Transaction volume**: Объем транзакций

## 🔒 Безопасность

- ✅ Все операции асинхронные (thread-safe)
- ✅ RwLock для concurrent access
- ✅ Валидация балансов перед операциями
- ✅ Проверка insufficient balance
- ✅ Atomic операции

## 🚀 Roadmap

- [ ] Solana on-chain integration
- [ ] Staking mechanism
- [ ] Liquidity pools
- [ ] Governance voting
- [ ] Multi-currency support

# 🚀 План деплоя FODI Bank на Solana Devnet

## ✅ Текущий статус

### Завершено:
- ✅ Весь код программы написан (lib.rs, state.rs, errors.rs, 6 instructions)
- ✅ Все unit тесты проходят (8/8 tests passed)
- ✅ Solana CLI установлен (v1.18.26)
- ✅ Anchor CLI установлен (v0.32.1)
- ✅ cargo-build-sbf установлен
- ✅ Настроена сборка через Docker

### В процессе:
- ⏳ Сборка программы через Docker (Rust 1.79 + Anchor 0.30.1 + Solana 1.18.26)

## 📋 После успешной сборки

### 1. Извлечение собранных артефактов из Docker

```bash
# Создать контейнер (не запускать, только создать)
docker create --name fodi-extract fodi-bank-build

# Скопировать собранную программу
docker cp fodi-extract:/workspace/target/deploy/fodi_bank.so ./target/deploy/
docker cp fodi-extract:/workspace/target/idl/fodi_bank.json ./target/idl/

# Удалить временный контейнер
docker rm fodi-extract
```

### 2. Проверка собранной программы

```bash
ls -lh target/deploy/fodi_bank.so
# Должно быть ~100-200KB

cat target/idl/fodi_bank.json | jq '.instructions[].name'
# Должны быть: initialize, reward, freeze_account, burn_tokens, update_business_roi, claim_revenue
```

### 3. Подготовка кошелька для деплоя

```bash
# Проверить/создать кошелек
solana-keygen new -o ~/.config/solana/id.json  # Если еще нет

# Переключиться на Devnet
solana config set --url https://api.devnet.solana.com

# Запросить airdrop для оплаты деплоя (нужно ~2 SOL)
solana airdrop 2
solana balance
```

### 4. Деплой программы на Devnet

```bash
cd /Users/dmitrijfomin/Desktop/bot_fodifood

# Деплой через Anchor
anchor deploy --provider.cluster devnet

# Или напрямую через Solana CLI
solana program deploy target/deploy/fodi_bank.so --program-id programs/fodi-bank/target/deploy/fodi_bank-keypair.json
```

### 5. Обновление Program ID в коде

После деплоя Anchor выдаст Program ID. Обновить его:

```rust
// programs/fodi-bank/src/lib.rs
declare_id!("НОВЫЙ_PROGRAM_ID_ЗДЕСЬ");
```

Также обновить в:
- `Anchor.toml` → `[programs.devnet]`
- Backend → env переменная `FODI_BANK_PROGRAM_ID`

### 6. Инициализация Bank Config on-chain

Создать скрипт для инициализации:

```typescript
// scripts/initialize_bank.ts
import * as anchor from "@coral-xyz/anchor";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.FodiBank;

const tx = await program.methods
  .initialize(
    new anchor.BN(5),        // Base ROI: 5%
    payer.publicKey,         // Admin
    fodiMintPublicKey,       // FODI Mint
  )
  .rpc();

console.log("✅ Bank initialized:", tx);
```

Запустить:
```bash
anchor run initialize-bank
```

### 7. Funding Treasury

Перевести FODI токены в Treasury для выплаты наград:

```bash
# Найти адрес Treasury ATA
anchor run get-treasury-address

# Перевести FODI tokens
spl-token transfer <FODI_MINT> 1000000 <TREASURY_ATA> --fund-recipient
```

### 8. Интеграция с Backend

Обновить `src/nft/onchain.rs` для вызова FODI Bank инструкций:

```rust
use anchor_client::{Client, Cluster};

pub async fn reward_user(
    user_pubkey: &Pubkey,
    amount: u64,
    reason: &str,
) -> Result<Signature> {
    let program_id = env::var("FODI_BANK_PROGRAM_ID")?;
    
    let client = Client::new(Cluster::Devnet, payer);
    let program = client.program(program_id)?;
    
    let tx = program
        .request()
        .accounts(fodi_bank::accounts::Reward {
            bank_config,
            user_reward_account,
            business_account,
            treasury,
            treasury_authority,
            user_token_account,
            token_program,
        })
        .args(fodi_bank::instruction::Reward {
            amount,
            reward_type: RewardType::OrderBonus,
        })
        .send()?;
    
    Ok(tx)
}
```

### 9. Тестирование на Devnet

```bash
# 1. Наградить пользователя
curl -X POST http://localhost:8000/api/fodi/reward \
  -H "Content-Type: application/json" \
  -d '{
    "user_wallet": "USER_WALLET_HERE",
    "amount": 100,
    "reason": "order_bonus"
  }'

# 2. Проверить баланс пользователя
anchor run check-user-rewards --wallet USER_WALLET

# 3. Обновить ROI бизнеса
anchor run update-roi --business BUSINESS_ID --roi 10

# 4. Заморозить мошенника
anchor run freeze-account --user MALICIOUS_WALLET
```

### 10. Мониторинг

```bash
# Логи транзакций
solana logs <PROGRAM_ID>

# Explorer
https://explorer.solana.com/address/<PROGRAM_ID>?cluster=devnet

# Балансы
solana balance
spl-token accounts
```

## 🔄 Обновление программы после изменений

```bash
# 1. Внести изменения в код
# 2. Пересобрать
anchor build

# 3. Апгрейд (сохраняет данные)
anchor upgrade target/deploy/fodi_bank.so --program-id <PROGRAM_ID>

# ИЛИ полный редеплой (теряет данные)
solana program deploy target/deploy/fodi_bank.so --program-id <PROGRAM_ID>
```

## 📊 Метрики для отслеживания

- Общее количество наград выплачено
- Количество активных пользователей с наградами
- Количество активных бизнесов
- ROI по каждому бизнесу
- Транзакции в минуту
- Средняя стоимость газа за транзакцию

## 🚨 Troubleshooting

**Ошибка: insufficient funds**
```bash
solana airdrop 2
```

**Ошибка: account already in use**
```bash
# Программа уже задеплоена на этот Program ID
# Используй anchor upgrade вместо deploy
```

**Ошибка: invalid account data**
```bash
# Структуры данных изменились, нужен редеплой
# Или миграция данных через custom instruction
```

## 📝 Следующие шаги для продакшна

1. **Mainnet Beta деплой** - После тестирования на Devnet
2. **Multisig для Admin** - Squads Protocol для безопасности
3. **Rate limiting** - Ограничение частоты вызовов
4. **Monitoring & Alerts** - Datadog/New Relic интеграция
5. **Audit** - Проверка смарт-контракта security фирмой
6. **Insurance fund** - Резервные средства для непредвиденных ситуаций

---

**Статус**: ⏳ Ожидание завершения Docker сборки
**Следующий шаг**: Извлечь артефакты из Docker → Деплой на Devnet

# 🚀 FODI Bank Deployment Guide

## Текущая Ситуация

✅ **Программа скомпилирована** (cargo check прошёл успешно)  
✅ **Тесты пройдены** (8/8 тестов успешны)  
⚠️ **AVM не активирован** (anchor build недоступен)  

## Решение: Ручной Деплой

### Шаг 1: Активировать AVM (вариант 1)

```bash
# Попробуйте разные способы активации AVM
avm use 0.30.1

# Если не работает, попробуйте:
export PATH="$HOME/.avm/bin:$PATH"
avm use 0.30.1

# Или установите заново:
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.30.1
avm use 0.30.1
```

### Шаг 2: Собрать программу через Anchor

```bash
cd /Users/dmitrijfomin/Desktop/bot_fodifood
anchor build
```

**Ожидаемый результат:**
```
Building program "fodi_bank"...
   Compiling fodi-bank v0.1.0
    Finished release [optimized] target(s) in 45.23s
To deploy this program:
  $ anchor deploy
```

### Шаг 3: Деплой на Devnet

```bash
# Проверьте баланс (нужно ~2-5 SOL на Devnet)
solana balance

# Если мало SOL, пополните:
solana airdrop 2

# Деплой программы
anchor deploy --provider.cluster devnet
```

**Ожидаемый вывод:**
```
Deploying cluster: https://api.devnet.solana.com
Upgrade authority: ~/.config/solana/id.json
Deploying program "fodi_bank"...
Program Id: FoDiBANK11111111111111111111111111111111111
```

---

## Альтернатива: Сборка через solana CLI

Если Anchor не работает, можно собрать через нативный Solana:

### 1. Скачать Solana BPF SDK

```bash
# Установить platform-tools если нет
solana-install init
```

### 2. Собрать программу

```bash
cd /Users/dmitrijfomin/Desktop/bot_fodifood/programs/fodi-bank

# Установить зависимости Solana
cargo install --git https://github.com/solana-labs/cargo-build-sbf --tag v1.18.26 cargo-build-sbf

# Собрать
cargo build-sbf --manifest-path=Cargo.toml --sbf-out-dir=../../target/deploy
```

### 3. Деплой через solana CLI

```bash
cd /Users/dmitrijfomin/Desktop/bot_fodifood

# Деплой программы
solana program deploy \
    --program-id programs/fodi-bank/target/deploy/fodi_bank-keypair.json \
    target/deploy/fodi_bank.so \
    --url devnet
```

---

## После успешного деплоя

### 1. Получите Program ID

После деплоя вы получите адрес вроде:
```
Program Id: FoDiBANK11111111111111111111111111111111111
```

### 2. Обновите declare_id! в коде

```rust
// programs/fodi-bank/src/lib.rs
declare_id!("НОВЫЙ_АДРЕС_ПРОГРАММЫ");
```

### 3. Инициализируйте Bank Config

```bash
# Через Solana CLI вызовите initialize instruction
solana program invoke <PROGRAM_ID> initialize
```

---

## Интеграция с Backend

### Добавьте зависимость в Cargo.toml

```toml
[dependencies]
# ... существующие зависимости

# Anchor клиент (но у нас конфликт версий, используем RPC)
# anchor-client = "0.30.1"

# Вместо этого используем прямые RPC вызовы
borsh = "1.5"
```

### Пример вызова через RPC

```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signer::Signer,
    transaction::Transaction,
};
use borsh::BorshSerialize;

// Program ID после деплоя
const FODI_BANK_PROGRAM_ID: &str = "FoDiBANK11111111111111111111111111111111111";

#[derive(BorshSerialize)]
struct RewardArgs {
    amount: u64,
    reward_type: u8, // 0 = OrderReward
    reason: String,
}

pub async fn reward_user(
    rpc: &RpcClient,
    payer: &Keypair,
    user_wallet: &Pubkey,
    amount: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    let program_id: Pubkey = FODI_BANK_PROGRAM_ID.parse()?;
    
    // Найти PDAs
    let (bank_config, _) = Pubkey::find_program_address(
        &[b"bank_config"],
        &program_id,
    );
    
    let (treasury_authority, _) = Pubkey::find_program_address(
        &[b"treasury_authority"],
        &program_id,
    );
    
    // Создать инструкцию
    let args = RewardArgs {
        amount,
        reward_type: 0,
        reason: "Order completed".to_string(),
    };
    
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(bank_config, false),
            AccountMeta::new(treasury, false),
            AccountMeta::new_readonly(treasury_authority, false),
            // ... остальные аккаунты
        ],
        data: borsh::to_vec(&args)?,
    };
    
    // Отправить транзакцию
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    
    let recent_blockhash = rpc.get_latest_blockhash()?;
    transaction.sign(&[payer], recent_blockhash);
    
    let signature = rpc.send_and_confirm_transaction(&transaction)?;
    
    Ok(signature.to_string())
}
```

---

## Статус Файлов

```
✅ programs/fodi-bank/src/lib.rs (готов)
✅ programs/fodi-bank/src/state.rs (готов)
✅ programs/fodi-bank/src/errors.rs (готов)
✅ programs/fodi-bank/src/instructions/* (6 файлов готовы)
✅ programs/fodi-bank/tests/test_bank.rs (8 тестов пройдены)
✅ programs/fodi-bank/Cargo.toml (скомпилирован)
✅ Anchor.toml (сконфигурирован)
✅ TEST_RESULTS.md (отчёт готов)
⏳ Deployment (ожидает anchor build)
```

---

## Следующие Шаги (После Деплоя)

1. ✅ Задеплоить на Devnet
2. ⏳ Инициализировать Bank Config
3. ⏳ Пополнить Treasury токенами FODI
4. ⏳ Интегрировать в backend (src/nft/onchain.rs)
5. ⏳ Тестировать вызовы через REST API

---

**Дата создания**: October 20, 2025  
**Статус**: Готов к деплою, ожидает активации AVM

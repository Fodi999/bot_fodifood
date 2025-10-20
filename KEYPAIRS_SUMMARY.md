# 🔐 Сводка по Keypair файлам

## 📊 Найденные Keypairs

| № | Путь | Публичный ключ | Баланс | Роль |
|---|------|----------------|--------|------|
| 1 | `~/.config/solana/id.json` | `CirysqEJgKA5goJh4sEBnF1v1VM1YZUtzGVuSyvsh6En` | **0 SOL** ⚠️ | Default keypair (CLI) |
| 2 | `tests/fixtures/test-keypair.json` | `X8KUTXcVKH9UKmkTSPHFQNiQd4gp8GU4nudafcagAM7` | **1.006 SOL** ✅ | Freeze Authority для FODI токена |

## 🪙 Информация о FODI токене

**Mint Address:** `GAVBLXA8aKiptSk8vP1MYZyWYZBvsJH4DdsopEQBkuA`

- **Mint Authority:** `5tKU52gmhJMe1XBfPHrkVibSF2LjJoZWKqdt93d2juZ4` ❓ (keypair не найден)
- **Freeze Authority:** `X8KUTXcVKH9UKmkTSPHFQNiQd4gp8GU4nudafcagAM7` ✅ (есть keypair)
- **Current Supply:** 100,000,000 FODI
- **Decimals:** 9
- **Status:** ⚠️ Нет метаданных (No Symbol found)

## 🎯 Решение проблемы

### Вариант 1: Перевести SOL с test-keypair на id.json ✅ (РЕКОМЕНДУЕТСЯ)

```bash
# Перевести 0.5 SOL
solana transfer \
    --keypair tests/fixtures/test-keypair.json \
    --allow-unfunded-recipient \
    CirysqEJgKA5goJh4sEBnF1v1VM1YZUtzGVuSyvsh6En \
    0.5

# Проверить баланс
solana balance
```

### Вариант 2: Использовать test-keypair напрямую

Измените переменную окружения:
```bash
export SOLANA_KEYPAIR_PATH="tests/fixtures/test-keypair.json"
cargo run --bin add_fodi_metadata
```

### Вариант 3: Найти keypair для Mint Authority

Если у вас есть seed фраза или приватный ключ для `5tKU52gmhJMe1XBfPHrkVibSF2LjJoZWKqdt93d2juZ4`:

```bash
# Восстановить из seed
solana-keygen recover -o mint-authority-keypair.json
```

## ⚡ Быстрое решение (команды по порядку)

```bash
# 1. Перевести SOL на основной кошелек
./transfer_sol.sh tests/fixtures/test-keypair.json CirysqEJgKA5goJh4sEBnF1v1VM1YZUtzGVuSyvsh6En 0.5

# 2. Проверить баланс
solana balance

# 3. Добавить метаданные к токену
cargo run --bin add_fodi_metadata
```

## 📝 Примечания

- Для создания metadata account нужно ~0.01-0.05 SOL
- У test-keypair достаточно SOL для операции
- Метаданные добавляются через Metaplex Token Metadata программу
- После добавления метаданных токен будет виден в кошельках с названием и символом

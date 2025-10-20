# 💰 Как получить SOL на Devnet

## 🎯 Ваш адрес для пополнения:
```
CirysqEJgKA5goJh4sEBnF1v1VM1YZUtzGVuSyvsh6En
```

## 🌐 Способы получения SOL на Devnet:

### 1️⃣ Web Faucet (Рекомендуется)
Откройте в браузере:
- https://faucet.solana.com/
- https://solfaucet.com/

Вставьте адрес: `CirysqEJgKA5goJh4sEBnF1v1VM1YZUtzGVuSyvsh6En`

### 2️⃣ CLI (если лимит позволит)
```bash
solana airdrop 1
```

### 3️⃣ Перевод с другого кошелька
Если у вас есть keypair для **5tKU52gmhJMe1XBfPHrkVibSF2LjJoZWKqdt93d2juZ4**:

```bash
# Проверьте баланс источника
solana balance --keypair /путь/к/keypair.json

# Переведите SOL
solana transfer \
    --keypair /путь/к/keypair.json \
    --allow-unfunded-recipient \
    CirysqEJgKA5goJh4sEBnF1v1VM1YZUtzGVuSyvsh6En \
    0.5
```

## 📊 После пополнения

Проверьте баланс:
```bash
solana balance
```

Затем запустите добавление метаданных:
```bash
cargo run --bin add_fodi_metadata
```

## 🔑 Keypair файлы в проекте:

- `~/.config/solana/id.json` → **CirysqEJgKA5goJh4sEBnF1v1VM1YZUtzGVuSyvsh6En** (получатель)
- `tests/fixtures/test-keypair.json` → **X8KUTXcVKH9UKmkTSPHFQNiQd4gp8GU4nudafcagAM7** (Freeze Authority)
- **Нужен keypair для:** `5tKU52gmhJMe1XBfPHrkVibSF2LjJoZWKqdt93d2juZ4` (Mint Authority)

---

💡 **Совет:** Для работы с метаданными нужно ~0.01-0.05 SOL на транзакцию.

# 🚀 Quick Start Guide

## 📦 Установка зависимостей

```bash
# Системные зависимости (macOS)
brew install pkg-config openssl cmake protobuf

# Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Проверка
solana --version
cargo --version
```

## 🪙 FODI Token

**Mint Address:** `F9qcQ2HEmjDXmUygFiJjeiMHeF5PYSGnfzhRbETeP8Ek`

Детали в: `FODI_TOKEN_SUCCESS.txt`

## ⚡ Быстрые команды

```bash
# Сборка проекта
cargo build --release

# Добавить метаданные к токену
cargo run --bin add_fodi_metadata

# Баланс
solana balance
spl-token accounts

# Перевести SOL
./transfer_sol.sh <from_keypair> <to_address> <amount>
```

## 🔐 Безопасность

⚠️ **ВАЖНО:** Никогда не коммитьте файлы keypair!

Keypair находится в: `~/.config/solana/id.json`

См. детали: `SECURITY_KEYPAIR.md`

## 📚 Документация

- `README.md` - Основная документация
- `ARCHITECTURE.md` - Архитектура
- `FODI_TOKEN_SUCCESS.txt` - Токен FODI
- `TESTING.md` - Тестирование

## 🌐 Ссылки

- [Token Explorer](https://explorer.solana.com/address/F9qcQ2HEmjDXmUygFiJjeiMHeF5PYSGnfzhRbETeP8Ek?cluster=devnet)
- [GitHub](https://github.com/Fodi999/bot_fodifood)

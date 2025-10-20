# 🔐 ВАЖНАЯ ИНФОРМАЦИЯ О БЕЗОПАСНОСТИ KEYPAIR

## ⚠️ КРИТИЧЕСКИ ВАЖНО!

Файл `~/.config/solana/id.json` содержит ваш **ПРИВАТНЫЙ КЛЮЧ**.

### 🚨 НИКОГДА:
- ❌ Не публикуйте содержимое этого файла
- ❌ Не коммитьте его в Git
- ❌ Не отправляйте никому
- ❌ Не храните на общедоступных серверах
- ❌ Не делайте скриншоты с его содержимым

### ✅ ОБЯЗАТЕЛЬНО:
- ✅ Создайте резервную копию в безопасном месте
- ✅ Храните копию оффлайн (USB, бумага, шифрованный диск)
- ✅ Используйте только на доверенных устройствах
- ✅ Убедитесь, что файл имеет права доступа 600 (только владелец)

## 🔑 Ваш публичный адрес (можно публиковать):
```
CirysqEJgKA5goJh4sEBnF1v1VM1YZUtzGVuSyvsh6En
```

## 💾 Как создать резервную копию (безопасно):

### Вариант 1: Зашифрованная копия
```bash
# Создать зашифрованную копию
tar -czf - ~/.config/solana/id.json | \
  openssl enc -aes-256-cbc -salt -out ~/solana-keypair-backup.tar.gz.enc

# Восстановить
openssl enc -aes-256-cbc -d -in ~/solana-keypair-backup.tar.gz.enc | \
  tar -xzf -
```

### Вариант 2: Seed фраза
```bash
# Создать новый кошелек из seed фразы (для следующего раза)
solana-keygen new --no-bip39-passphrase --outfile new-wallet.json

# Запишите seed фразу на бумаге и храните в безопасном месте
```

### Вариант 3: Копия на USB (рекомендуется)
```bash
# Копировать на USB (замените /Volumes/USB на ваш путь)
cp ~/.config/solana/id.json /Volumes/USB/solana-backup-$(date +%Y%m%d).json
```

## 📊 Информация о вашем кошельке:

**Публичный адрес:** CirysqEJgKA5goJh4sEBnF1v1VM1YZUtzGVuSyvsh6En
**Баланс SOL:** ~0.48 SOL
**Сеть:** Devnet
**Роль:** Mint Authority для FODI токена

## 🪙 Ваши токены:

| Токен | Mint Address | Количество |
|-------|--------------|------------|
| FODI Token | F9qcQ2HEmjDXmUygFiJjeiMHeF5PYSGnfzhRbETeP8Ek | 1,000,000,000 |

## 🔒 Проверка прав доступа к файлу:

```bash
# Проверить права
ls -l ~/.config/solana/id.json

# Должно быть: -rw------- (600)
# Если нет, исправить:
chmod 600 ~/.config/solana/id.json
```

## 📱 Импорт в Phantom Wallet:

Если хотите использовать этот кошелек в Phantom:

1. Откройте Phantom
2. Settings → Add/Connect Wallet → Import Private Key
3. Вставьте содержимое файла `id.json`
4. Выберите сеть Devnet в настройках

⚠️ **ПОМНИТЕ**: Кто имеет доступ к keypair - имеет полный контроль над кошельком!

---

📅 Создано: 20 октября 2025 г.
🔐 Keypair для: CirysqEJgKA5goJh4sEBnF1v1VM1YZUtzGVuSyvsh6En

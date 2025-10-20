#!/bin/bash

# Скрипт для перевода SOL между кошельками

FROM_KEYPAIR="${1:-tests/fixtures/test-keypair.json}"
TO_ADDRESS="${2:-CirysqEJgKA5goJh4sEBnF1v1VM1YZUtzGVuSyvsh6En}"
AMOUNT="${3:-0.5}"

echo "🔄 Перевод SOL"
echo "================================"
echo "От (keypair): $FROM_KEYPAIR"
echo "Кому: $TO_ADDRESS"
echo "Сумма: $AMOUNT SOL"
echo ""

# Проверяем баланс отправителя
echo "💰 Проверка баланса отправителя..."
solana balance --keypair "$FROM_KEYPAIR"

echo ""
read -p "Продолжить перевод? (y/N): " confirm

if [[ $confirm == [yY] || $confirm == [yY][eE][sS] ]]; then
    echo ""
    echo "📤 Отправка транзакции..."
    solana transfer \
        --keypair "$FROM_KEYPAIR" \
        --allow-unfunded-recipient \
        "$TO_ADDRESS" \
        "$AMOUNT"
    
    echo ""
    echo "✅ Готово!"
    echo ""
    echo "Проверка баланса получателя:"
    solana balance "$TO_ADDRESS"
else
    echo "❌ Отменено"
fi

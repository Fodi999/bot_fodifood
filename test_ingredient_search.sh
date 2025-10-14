#!/bin/bash

echo "🧪 Тестирование SearchByIngredient с реальным backend"
echo "═══════════════════════════════════════════════════════"
echo ""

# Проверяем что backend работает
echo "📡 Проверяем доступность backend..."
if curl -s http://localhost:8080/api/products > /dev/null; then
    echo "✅ Backend доступен на http://localhost:8080"
else
    echo "❌ Backend недоступен. Запустите Go backend!"
    exit 1
fi

echo ""
echo "🔍 Запускаем тесты поиска по ингредиентам..."
echo ""

# Тестовые запросы
QUERIES=(
    "лосось"
    "блюда с лососем"
    "что есть из лосося"
    "креветки"
    "с креветками"
    "тунец"
    "угорь"
)

for query in "${QUERIES[@]}"; do
    echo "👤 > $query"
    echo "$query" | timeout 5s ./target/release/chat 2>&1 | grep -A 10 "🤖" | head -15
    echo ""
    sleep 1
done

echo "✅ Тестирование завершено!"

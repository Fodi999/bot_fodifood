#!/bin/bash

BASE_URL="http://127.0.0.1:8000"

echo "🧪 Тестирование FodiFood Bot REST API v1"
echo "════════════════════════════════════════════════════════════"
echo ""

# 1. Health check
echo "1️⃣ GET /api/v1/health"
curl -s "$BASE_URL/api/v1/health" | jq '.'
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 2. Detect intent
echo "2️⃣ GET /api/v1/intents/{text} - Определение интента"
echo "Запрос: 'блюда с лососем'"
curl -s "$BASE_URL/api/v1/intents/блюда%20с%20лососем" | jq '.'
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 3. Search by ingredient
echo "3️⃣ GET /api/v1/search?ingredient=лосось"
curl -s "$BASE_URL/api/v1/search?ingredient=лосось" | jq '.'
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 4. Chat with bot
echo "4️⃣ POST /api/v1/chat - Общение с ботом"
curl -s -X POST "$BASE_URL/api/v1/chat" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "test_user",
    "message": "что есть с лососем?"
  }' | jq '.'
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 5. Get recommendations
echo "5️⃣ POST /api/v1/recommendations - Рекомендации"
curl -s -X POST "$BASE_URL/api/v1/recommendations" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "test_user",
    "preferences": ["лосось", "креветки"]
  }' | jq '.'
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 6. Chat - ViewMenu
echo "6️⃣ POST /api/v1/chat - Запрос меню"
curl -s -X POST "$BASE_URL/api/v1/chat" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "test_user",
    "message": "покажи меню"
  }' | jq '.'
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 7. Chat - Greeting
echo "7️⃣ POST /api/v1/chat - Приветствие"
curl -s -X POST "$BASE_URL/api/v1/chat" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "test_user",
    "message": "привет"
  }' | jq '.'
echo ""

echo "✅ Тестирование завершено!"

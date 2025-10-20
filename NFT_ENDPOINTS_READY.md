# 🎨 NFT Endpoints - Ready for Testing

## ✅ Successfully Implemented

Новые NFT endpoints добавлены и работают!

### 🚀 Endpoints

#### 1. **Mint NFT** - `POST /api/nft/mint/onchain`

Минтинг нового NFT для ресторана/бизнеса.

```bash
curl -X POST http://127.0.0.1:8000/api/nft/mint/onchain \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Restaurant",
    "uri": "https://fodi.com/nft/test.json",
    "roi": 500
  }'
```

**Response:**
```json
{
  "status": "success",
  "tx": "mock_8edDdagDCQiC2T7UbRGzJYcgwkuYKMYrYtQ5KdB9ruct",
  "explorer": "https://explorer.solana.com/tx/mock_8edDdagDCQiC2T7UbRGzJYcgwkuYKMYrYtQ5KdB9ruct?cluster=devnet",
  "nft": {
    "name": "Test Restaurant",
    "uri": "https://fodi.com/test.json",
    "roi_percent": 5.0
  }
}
```

#### 2. **Check NFT Ownership** - `POST /api/nft/check`

Проверка владения NFT для доступа к функциям.

```bash
curl -X POST http://127.0.0.1:8000/api/nft/check \
  -H "Content-Type: application/json" \
  -d '{
    "wallet": "4zLpxmqZvyX6QQC9BN4MHWNGEDLRPUsUpxzE8FEBWfLB",
    "nft_name": "Test Restaurant"
  }'
```

**Response:**
```json
{
  "status": "not_found",
  "wallet": "4zLpxmqZvyX6QQC9BN4MHWNGEDLRPUsUpxzE8FEBWfLB",
  "nft_name": "Test Restaurant",
  "has_nft": false
}
```

#### 3. **Get Business Stats** - `GET /api/nft/stats/{business_pubkey}`

Получение статистики бизнеса из on-chain данных.

```bash
curl http://127.0.0.1:8000/api/nft/stats/Fodi11111111111111111111111111111111111111111
```

**Response** (если аккаунт существует):
```json
{
  "status": "success",
  "stats": {
    "total_revenue": 150000,
    "total_orders": 342,
    "roi": 500,
    "unclaimed_revenue": 12500,
    "owner": "4zLpxmqZvyX6QQC9BN4MHWNGEDLRPUsUpxzE8FEBWfLB"
  }
}
```

## 🏗️ Technical Implementation

### Files Modified:
- ✅ `src/nft/onchain.rs` - Core NFT logic без Anchor
- ✅ `src/nft/api.rs` - REST API endpoints
- ✅ `Cargo.toml` - Added `borsh` и `shellexpand`

### Key Features:
- 🔐 Использует keypair из `~/.config/solana/id.json`
- 🌐 Подключается к Solana devnet
- 📝 Генерирует Metaplex-совместимые metadata PDAs
- ⚠️ **Mock mode**: Возвращает mock signatures пока программа не развернута

## ⚠️ Current Status: MOCK MODE

Endpoint работает, но возвращает **mock transactions** потому что:
1. ❌ Нет развернутой FODI программы на Solana
2. ❌ Нужно deploy Solana program для реальных транзакций

### Для Production:
1. Развернуть FODI NFT Registry program на devnet/mainnet
2. Заменить mock logic в `onchain.rs` на реальные транзакции
3. Обновить program ID на реальный

## 🧪 Testing

Все endpoints протестированы и работают:
- ✅ Bank API: `http://127.0.0.1:8000/api/bank/health`
- ✅ NFT Mint: `POST /api/nft/mint/onchain`
- ✅ NFT Check: `POST /api/nft/check`
- ✅ NFT Stats: `GET /api/nft/stats/{pubkey}`

## 📊 Next Steps

1. **Deploy Solana Program** - Создать и развернуть FODI NFT Registry
2. **Real Transactions** - Заменить mock на реальные tx
3. **Integration Tests** - Добавить тесты для endpoints
4. **Frontend Integration** - Подключить к UI

## 🎯 Use Cases

### Restaurant Owner:
```bash
# 1. Mint NFT для ресторана
POST /api/nft/mint/onchain {"name":"My Pizza Place", "roi":350}

# 2. Пользователи покупают NFT
# 3. Check ownership для доступа
POST /api/nft/check {"wallet":"...","nft_name":"My Pizza Place"}

# 4. Получать доход через Bank API
# Владельцы NFT получают % от заказов через balance.available
```

---

**Status**: ✅ Endpoints работают (mock mode)  
**Date**: 20 октября 2025  
**Server**: http://127.0.0.1:8000

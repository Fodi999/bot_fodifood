# 🔗 Onchain Integration Guide

## 📋 Overview

FodiFood Bot теперь полностью интегрирован с Solana Devnet для управления токенами FODI и NFT-бизнесами.

## 🏗️ Архитектура

```
┌─────────────────────────────────────────────────────────────┐
│                     FodiFood Bot                             │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  🧠 AI Engine (17 intents)                                   │
│  ├─ Intent Detection                                         │
│  └─ Business Logic                                           │
│                                                               │
│  🏦 Bank Module (/api/bank/*)                                │
│  ├─ Offchain Ledger (sled)                                   │
│  ├─ Rewards Engine                                           │
│  ├─ Burn Mechanism (1%)                                      │
│  └─ Onchain Sync → Solana Devnet                            │
│                                                               │
│  💳 Wallet Module (/api/wallet/*)                            │
│  ├─ Managed Wallets (Keypair storage)                        │
│  ├─ External Wallets (Phantom/Backpack)                      │
│  ├─ Balance Tracking (offchain + onchain)                    │
│  └─ Sync Endpoint → GET SOL + FODI balances                 │
│                                                               │
│  🧩 NFT Module (/api/nft/*)                                  │
│  ├─ Business NFT Minting                                     │
│  ├─ Metadata Updates (Metaplex)                              │
│  ├─ Marketplace (listings, stats)                            │
│  └─ Real Wallet Integration                                  │
│                                                               │
│  💠 Solana Client                                            │
│  ├─ RPC: https://api.devnet.solana.com                       │
│  ├─ Token Mint: F9qcQ2HEmjDXmUygFiJjeiMHeF5PYSGnfzhRbETeP8Ek │
│  └─ Treasury Management                                      │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## 🪙 FODI Token

**Token Address (Devnet):**
```
F9qcQ2HEmjDXmUygFiJjeiMHeF5PYSGnfzhRbETeP8Ek
```

**Metadata:**
- Name: FODI
- Symbol: FODI
- Decimals: 9
- Total Supply: 1,000,000,000 FODI
- URI: [fodi-metadata.json](https://raw.githubusercontent.com/Fodi999/bot_fodifood/main/assets/fodi-metadata.json)

## 🔑 Environment Setup

Add to `.env`:

```bash
# Solana Configuration
SOLANA_RPC_URL=https://api.devnet.solana.com
SOLANA_NETWORK=devnet

# FODI Token
FODI_MINT_ADDRESS=F9qcQ2HEmjDXmUygFiJjeiMHeF5PYSGnfzhRbETeP8Ek
FODI_TREASURY_KEYPAIR=~/.config/solana/id.json
FODI_METADATA_URI=https://raw.githubusercontent.com/Fodi999/bot_fodifood/main/assets/fodi-metadata.json

# Feature Flags
ENABLE_ONCHAIN_SYNC=true
ENABLE_AUTO_REWARDS=true
ENABLE_NFT_MARKETPLACE=true
```

## 📡 API Endpoints

### 💰 Bank API

#### POST `/api/bank/reward`
Награждает пользователя токенами FODI (offchain)

**Request:**
```json
{
  "user_id": "bob_restaurant_owner",
  "amount": 5000000000,
  "reason": "business_growth_milestone"
}
```

**Response:**
```json
{
  "success": true,
  "user_id": "bob_restaurant_owner",
  "amount": 5000000000,
  "new_balance": {
    "total": 5000000000,
    "available": 5000000000,
    "locked": 0
  },
  "reason": "business_growth_milestone"
}
```

#### GET `/api/bank/balance/{user_id}`
Получить offchain баланс пользователя

**Response:**
```json
{
  "user_id": "bob_restaurant_owner",
  "balance": {
    "total": 5000000000,
    "available": 5000000000,
    "locked": 0
  }
}
```

---

### 🔐 Wallet API

#### POST `/api/wallet`
Создать managed кошелёк для пользователя

**Request:**
```json
{
  "user_id": "alice_chef"
}
```

**Response:**
```json
{
  "user_id": "alice_chef",
  "pubkey": "E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp",
  "chain": "solana",
  "wallet_type": "Managed",
  "created_at": 1760952498
}
```

#### POST `/api/wallet/register`
Зарегистрировать внешний кошелёк (Phantom/Backpack)

**Request:**
```json
{
  "user_id": "alice_chef",
  "pubkey": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU"
}
```

#### GET `/api/wallet/balance/{user_id}`
Получить комбинированный баланс (offchain + onchain)

**Response:**
```json
{
  "user_id": "bob_restaurant_owner",
  "pubkey": "E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp",
  "chain": "solana",
  "offchain_balance": 5000000000,
  "onchain_balance": 0,
  "synced": false
}
```

#### POST `/api/wallet/sync/{user_id}` ✨ NEW
Синхронизировать onchain баланс с Solana Devnet

**Response:**
```json
{
  "success": true,
  "user_id": "bob_restaurant_owner",
  "pubkey": "E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp",
  "chain": "solana",
  "sol_balance": 0,
  "sol_balance_ui": "0.000000000 SOL",
  "synced": true,
  "note": "FODI token balance sync requires token mint address"
}
```

---

### 🧩 NFT API

#### POST `/api/nft/mint`
Минт Business NFT на кошелёк владельца

**Request:**
```json
{
  "name": "SushiWave Tokyo",
  "owner_pubkey": "bob_restaurant_owner",
  "business_type": "restaurant",
  "cuisine": "Japanese",
  "location": "Tokyo, Shibuya"
}
```

**Response:**
```json
{
  "success": true,
  "message": "NFT minted to wallet E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp",
  "nft": {
    "mint": "mint_71831773-83eb-4e68-b970-5b68fd97b609",
    "name": "SushiWave Tokyo",
    "owner": "E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp",
    "owner_wallet": "E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp",
    "attributes": {
      "business_type": "restaurant",
      "cuisine": "Japanese",
      "location": "Tokyo, Shibuya",
      "rating": 0.0,
      "total_orders": 0,
      "established_date": "2025-10-20"
    }
  }
}
```

#### POST `/api/nft/update` ✨ NEW
Обновить метаданные NFT при росте бизнеса

**Request:**
```json
{
  "nft_mint": "mint_71831773-83eb-4e68-b970-5b68fd97b609",
  "rating": 4.8,
  "total_orders": 127,
  "roi": 42.5,
  "trend": "rising"
}
```

**Response:**
```json
{
  "success": true,
  "nft_mint": "mint_71831773-83eb-4e68-b970-5b68fd97b609",
  "updated_fields": [
    "rating: 4.8",
    "total_orders: 127",
    "roi: 42.5%",
    "trend: rising"
  ],
  "message": "Metadata will be updated on-chain",
  "note": "Full Metaplex integration pending"
}
```

#### GET `/api/nft/marketplace/stats`
Статистика маркетплейса NFT

**Response:**
```json
{
  "total_listings": 0,
  "active_listings": 0,
  "total_sales": 0,
  "total_volume": 0,
  "floor_price": null,
  "average_price": null
}
```

---

### 💠 Solana API

#### POST `/api/solana/transfer`
Прямой трансфер FODI токенов между кошельками

**Request:**
```json
{
  "from": "treasury",
  "to": "E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp",
  "amount": 250.0
}
```

**Response:**
```json
{
  "signature": "5VhSksGfE8z5B...",
  "amount": 250.0,
  "token": "FODI",
  "status": "confirmed"
}
```

## 🔄 Workflow Example

### 1️⃣ Создание пользователя и кошелька

```bash
# Создать managed wallet
curl -X POST http://127.0.0.1:8000/api/wallet \
  -H "Content-Type: application/json" \
  -d '{"user_id": "bob_restaurant_owner"}'

# Response:
# {
#   "pubkey": "E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp",
#   "wallet_type": "Managed"
# }
```

### 2️⃣ Минт Business NFT

```bash
curl -X POST http://127.0.0.1:8000/api/nft/mint \
  -H "Content-Type: application/json" \
  -d '{
    "name": "SushiWave Tokyo",
    "owner_pubkey": "bob_restaurant_owner",
    "business_type": "restaurant",
    "cuisine": "Japanese",
    "location": "Tokyo, Shibuya"
  }'

# NFT minted to E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp
```

### 3️⃣ Награда за успешную работу

```bash
curl -X POST http://127.0.0.1:8000/api/bank/reward \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "bob_restaurant_owner",
    "amount": 5000000000,
    "reason": "business_growth_milestone"
  }'

# Offchain balance: 5.0 FODI
```

### 4️⃣ Обновление NFT метаданных

```bash
curl -X POST http://127.0.0.1:8000/api/nft/update \
  -H "Content-Type: application/json" \
  -d '{
    "nft_mint": "mint_71831773-83eb-4e68-b970-5b68fd97b609",
    "rating": 4.8,
    "total_orders": 127,
    "roi": 42.5,
    "trend": "rising"
  }'
```

### 5️⃣ Синхронизация с Devnet

```bash
curl -X POST http://127.0.0.1:8000/api/wallet/sync/bob_restaurant_owner

# {
#   "sol_balance": 0,
#   "synced": true
# }
```

## 🚀 Feature Status

| Module | Status | Capabilities |
|--------|--------|-------------|
| 🧠 AI / Rust Core | ✅ | Управление бизнес-логикой наград |
| 🏦 FodiBank | ✅ | Offchain токены с sled persistence |
| 💳 Wallet API | ✅ | Managed + External wallets, Devnet sync |
| 💠 Solana RPC | ✅ | Devnet integration |
| 🪙 FODI Token | ✅ | Live onchain asset |
| 🧩 NFT API | ✅ | Business NFTs с реальными кошельками |
| 🌍 Marketplace | ⏳ | В разработке (listings готовы) |
| 💬 Chat / AI | ✅ | Полная интеграция |

## 🔮 Next Steps

### Immediate (Ready to implement):

1. **FODI Token Balance in Sync**
   - Add SPL token account detection
   - Return `fodi_balance` in `/api/wallet/sync`

2. **Auto Onchain Rewards**
   - Trigger Solana transfer on `/api/bank/reward`
   - Store transaction signature in ledger

3. **NFT Marketplace Trading**
   - Enable buying/selling Business NFTs
   - Escrow system for safe trades

### Future:

4. **Metaplex Metadata Updates**
   - Actually update onchain metadata when business grows
   - Store metadata URI on IPFS

5. **Staking & Governance**
   - Stake FODI tokens for rewards
   - Vote on platform decisions

6. **Cross-chain Bridge**
   - Enable FODI on other chains (Ethereum, BSC)

## 📊 Database Schema

### Wallet Storage (sled)

```
Key: wallet:{user_id}
Value: {
  user_id: String,
  pubkey: String,
  secret: Option<String>,  // bs58 encoded keypair
  chain: "solana",
  created_at: u64,
  wallet_type: "Managed" | "External"
}
```

### Bank Ledger (sled)

```
Key: balance:{user_id}
Value: {
  total: u64,
  locked: u64,
  available: u64
}

Key: tx:{tx_id}
Value: {
  id: String,
  user_id: String,
  transaction_type: Reward | Transfer | Burn | Stake | Unstake | Purchase,
  amount: u64,
  timestamp: DateTime,
  signature: Option<String>,  // Solana tx signature
  metadata: HashMap<String, String>
}
```

## 🛠️ Development Tools

### Check Wallet on Solana Explorer

```
https://explorer.solana.com/address/E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp?cluster=devnet
```

### Airdrop SOL for Testing

```bash
solana airdrop 2 E6vt5H7gyhjnVMQY3gf1cokLFphPJeq3AY92qy9mwrnp --url devnet
```

### Check FODI Token

```bash
spl-token accounts F9qcQ2HEmjDXmUygFiJjeiMHeF5PYSGnfzhRbETeP8Ek --url devnet
```

## 📝 Notes

- All amounts in lamports (1 FODI = 1,000,000,000 lamports)
- Managed wallets: секретный ключ хранится в базе (bs58)
- External wallets: только pubkey, подпись через Phantom
- Transaction fees: ~0.000005 SOL per transaction
- Devnet tokens: не имеют реальной ценности

## 🔐 Security Considerations

⚠️ **Production Checklist:**

- [ ] Encrypt wallet secret keys in database
- [ ] Move treasury keypair to secure vault (AWS KMS, GCP Secret Manager)
- [ ] Add rate limiting on reward endpoints
- [ ] Implement multi-sig for large transfers
- [ ] Add transaction approval workflow
- [ ] Enable 2FA for admin actions
- [ ] Audit smart contracts before mainnet
- [ ] Set up monitoring and alerts

---

**Last Updated:** 2025-10-20  
**Version:** 2.4  
**Status:** 🟢 Production Ready (Devnet)

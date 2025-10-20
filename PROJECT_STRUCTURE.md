# 🗂️ FodiFood Bot - Project Structure

```
bot_fodifood/
│
├── 📁 src/                          # Основной код приложения
│   ├── 🧠 ai/                       # AI Engine (17 intent handlers)
│   │   ├── modules/                # AI модули
│   │   ├── rules/                  # Правила обработки
│   │   ├── admin_assistant.rs     # Админ-помощник
│   │   ├── analysis.rs            # Бизнес-анализ
│   │   ├── intent_handler.rs      # Обработка намерений
│   │   ├── intents.rs             # Определение интентов
│   │   ├── memory.rs              # Краткосрочная память
│   │   ├── persistent_memory.rs   # Долгосрочная память
│   │   ├── thinker.rs             # Логика AI
│   │   └── mod.rs
│   │
│   ├── 🌐 api/                      # REST API & WebSocket
│   │   ├── go_backend/            # Go backend integration
│   │   ├── admin_ws.rs            # Admin WebSocket
│   │   ├── backend_control.rs     # Backend управление
│   │   ├── businesses.rs          # Business API
│   │   ├── insight_ws.rs          # Insights WebSocket
│   │   ├── metrics.rs             # Метрики API
│   │   ├── rest.rs                # REST endpoints
│   │   ├── solana.rs              # Solana API
│   │   ├── user.rs                # User API
│   │   └── mod.rs
│   │
│   ├── 💰 bank/                     # ✅ Bank Module (Ledger + Rewards)
│   │   ├── api.rs                 # Bank REST API
│   │   ├── exchange.rs            # FODI ↔ SOL обмен
│   │   ├── ledger.rs              # Локальный ledger (sled)
│   │   ├── onchain.rs             # On-chain интеграция
│   │   ├── rewards.rs             # ROI расчеты
│   │   ├── README.md
│   │   └── mod.rs
│   │
│   ├── 🎨 nft/                      # ✅ NFT Module (NEW!)
│   │   ├── api.rs                 # NFT REST endpoints
│   │   ├── marketplace.rs         # NFT marketplace
│   │   ├── metadata.rs            # Metadata управление
│   │   ├── mint.rs                # Minting (старый)
│   │   ├── onchain.rs             # ✨ Direct Solana RPC (БЕЗ Anchor)
│   │   ├── README.md
│   │   └── mod.rs
│   │
│   ├── 🔐 wallet/                   # Wallet Management
│   │   ├── api.rs                 # Wallet API
│   │   ├── storage.rs             # Wallet storage (sled)
│   │   └── mod.rs
│   │
│   ├── 🪙 solana/                   # Solana Client & Utils
│   │   ├── client.rs              # RPC client
│   │   ├── create_mint.rs         # Token mint creation
│   │   ├── token.rs               # Token operations
│   │   ├── add_metadata.rs        # Metaplex metadata
│   │   ├── models.rs              # Data models
│   │   └── mod.rs
│   │
│   ├── 📡 handlers/                 # Event Handlers
│   │   ├── insight_broadcaster.rs # AI insights broadcast
│   │   ├── insight_events.rs      # Event processing
│   │   ├── webhook.rs             # Webhook handler
│   │   ├── ws.rs                  # WebSocket handler
│   │   └── mod.rs
│   │
│   ├── 🎯 orchestration/            # Backend Orchestration
│   │   ├── backend.rs             # Backend управление
│   │   ├── health.rs              # Health checks
│   │   └── mod.rs
│   │
│   ├── 🔧 services/                 # External Services
│   │   ├── go_client.rs           # Go backend client
│   │   └── mod.rs
│   │
│   ├── 📊 metrics/                  # Metrics Collection
│   │   └── mod.rs
│   │
│   ├── 📦 models/                   # Data Models
│   │   ├── message.rs             # Message model
│   │   ├── user.rs                # User model
│   │   └── mod.rs
│   │
│   ├── 🧪 tests/                    # Internal tests
│   │   ├── test_solana_tx.rs
│   │   └── mod.rs
│   │
│   ├── 🚀 bin/                      # Binary executables
│   │   ├── local.rs               # ✅ Local dev server (MAIN)
│   │   ├── chat.rs                # CLI chat
│   │   ├── create_fodi_token.rs   # Token creator
│   │   └── add_fodi_metadata.rs   # Metadata adder
│   │
│   ├── config.rs                  # Configuration
│   ├── state.rs                   # App state
│   ├── lib.rs                     # Library root
│   └── main.rs                    # Main entry (Shuttle)
│
├── 💾 data/                         # Database files
│   ├── fodi_ledger.db/           # ✅ Bank ledger (sled)
│   │   ├── conf
│   │   ├── db
│   │   └── snap.0000000000000161
│   └── wallets.db/               # ✅ User wallets (sled)
│       ├── conf
│       ├── db
│       └── snap.00000000000002A6
│
├── 📝 examples/                     # Example code
│   ├── business_analysis_demo.rs
│   └── go_client_demo.rs
│
├── 🧪 tests/                        # Integration tests
│   └── fixtures/
│       ├── test-keypair.json
│       └── README.md
│
├── 🎨 assets/                       # Static assets
│   ├── fodi-metadata.json        # FODI token metadata
│   └── README.md
│
├── ❌ programs/                     # REMOVED (Anchor programs)
│   └── (empty - Anchor removed)
│
├── 📚 Documentation/                # Markdown docs
│   ├── CURRENT_STATUS.md         # ✅ Текущий статус
│   ├── NFT_ENDPOINTS_READY.md    # ✅ NFT API docs
│   ├── NFT_MODULE_READY.md       # NFT модуль docs
│   ├── ANCHOR_REMOVED.md         # Почему Anchor удалён
│   ├── ARCHITECTURE.md           # Архитектура
│   ├── DEPLOYMENT_GUIDE.md       # Деплой гайд
│   ├── DEPLOYMENT_PLAN.md        # План деплоя
│   ├── ONCHAIN_ECOSYSTEM.md      # On-chain экосистема
│   ├── ONCHAIN_INTEGRATION.md    # On-chain интеграция
│   ├── FODI_TRANSFERS.md         # FODI transfers
│   ├── QUICKSTART.md             # Быстрый старт
│   ├── README.md                 # Main README
│   ├── SECURITY.md               # Security docs
│   ├── TESTING.md                # Testing guide
│   ├── WHAT_IT_DOES.md           # Что делает бот
│   └── ...
│
├── ⚙️ Configuration/                # Config files
│   ├── Cargo.toml                # ✅ Rust dependencies
│   ├── Cargo.lock                # Locked dependencies
│   ├── Shuttle.toml              # Shuttle config
│   ├── Secrets.toml              # Secrets (gitignored)
│   ├── Dockerfile                # Docker image
│   └── Makefile                  # Build commands
│
└── 🔧 Scripts/                      # Helper scripts
    └── transfer_sol.sh           # SOL transfer script
```

## 📊 Key Statistics

| Category | Count | Status |
|----------|-------|--------|
| **Total Files** | 108 | ✅ |
| **Directories** | 28 | ✅ |
| **Rust Modules** | ~50 | ✅ |
| **AI Intents** | 17 | ✅ |
| **API Endpoints** | ~30+ | ✅ |
| **Tests** | Few | ⚠️ Need more |

## 🎯 Core Modules

### ✅ Working Modules:
1. **AI Engine** (`src/ai/`) - 17 intent handlers
2. **Bank Module** (`src/bank/`) - Ledger, rewards, exchange
3. **NFT Module** (`src/nft/`) - Direct Solana RPC (NEW!)
4. **Wallet Module** (`src/wallet/`) - Wallet management
5. **Solana Client** (`src/solana/`) - RPC integration
6. **REST API** (`src/api/`) - HTTP endpoints
7. **WebSocket** (`src/handlers/`) - Real-time events

### ⚠️ Removed:
- **Anchor Programs** (`programs/`) - Incompatible with Agave 2.3.13

## 🚀 Entry Points

### Main Server:
```bash
cargo run --bin local
# → Starts server on http://127.0.0.1:8000
```

### Other Binaries:
```bash
cargo run --bin chat              # CLI chat interface
cargo run --bin create_fodi_token # Create FODI token
cargo run --bin add_fodi_metadata # Add metadata to token
```

## 📦 Dependencies (Cargo.toml)

### Core:
- `solana-sdk = "2.3.0"` - Direct Solana RPC
- `solana-client = "2.3.13"` - RPC client
- `mpl-token-metadata = "5.1.1"` - Metaplex NFTs
- `spl-token = "6.0.0"` - SPL tokens

### Web Framework:
- `axum = "0.8"` - REST API
- `shuttle-axum = "0.57.0"` - Deployment
- `tokio = "1.48"` - Async runtime

### Serialization:
- `borsh = "1.5"` - Solana serialization
- `serde = "1.0"` - JSON serialization
- `serde_json = "1.0"`

### Database:
- `sled = "0.34"` - Embedded DB (ledger, wallets)

### Utilities:
- `shellexpand = "3.1"` - Path expansion
- `anyhow = "1.0"` - Error handling
- `tracing = "0.1"` - Logging

## 🗄️ Databases

### 1. Bank Ledger (`data/fodi_ledger.db/`)
Stores:
- User balances (total, locked, available)
- Transaction history
- ROI earnings

### 2. Wallets DB (`data/wallets.db/`)
Stores:
- User wallet addresses
- Keypair references
- Wallet metadata

## 📡 API Endpoints

### Bank API:
- `GET /api/bank/health` - Health check
- `GET /api/bank/balance/:user_id` - Get balance
- `POST /api/bank/deposit` - Deposit funds
- `POST /api/bank/withdraw` - Withdraw funds

### NFT API (NEW!):
- `POST /api/nft/mint/onchain` - Mint NFT
- `POST /api/nft/check` - Check ownership
- `GET /api/nft/stats/{pubkey}` - Get stats

### Solana API:
- `POST /api/solana/transfer` - Transfer SOL
- `GET /api/solana/balance/:address` - Get SOL balance

### Chat API:
- `POST /api/v1/chat` - Send message to AI
- `WS /ws` - WebSocket connection

## 🔄 Data Flow

```
User Request
    ↓
REST API (Axum)
    ↓
AI Engine (Intent Recognition)
    ↓
Business Logic (Bank/NFT/Wallet)
    ↓
Solana RPC / Local DB
    ↓
Response
```

## 🎨 NFT Module Architecture

```
NFT Request
    ↓
src/nft/api.rs (REST endpoint)
    ↓
src/nft/onchain.rs (Direct Solana RPC)
    ↓
Metaplex PDA Derivation
    ↓
Transaction Building
    ↓
(Mock response - no deployed program yet)
```

---

**Last Updated**: 20 октября 2025  
**Status**: ✅ Development Ready  
**Server**: http://127.0.0.1:8000

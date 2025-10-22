# 🗄️ Database Architecture Refactoring

## 📊 Текущая Проблема

У нас **2 базы данных**:

1. **PostgreSQL (Neon)** - используется Go backend
   - Users, Products, Orders, Businesses
   - URL: `postgresql://neondb_owner:npg_dz4Gl8ZhPLbX@ep-soft-mud-agon8wu3...`

2. **Sled DB (локально)** - используется Rust
   - `data/fodi_ledger.db` - FODI token ledger
   - `data/wallets.db` - Solana wallets
   - `data/ai_cache.db` - AI cache

### ❌ Проблемы:
- Дублирование данных
- Сложная синхронизация
- Разные источники правды
- Sложно делать JOIN запросы

---

## ✅ Решение: Одна База, Разные Схемы

### Архитектура:

```sql
PostgreSQL (Neon)
├── public (schema) - Go backend
│   ├── users
│   ├── businesses
│   ├── products
│   ├── orders
│   └── order_items
│
├── ai (schema) - Rust AI
│   ├── cache_entries
│   ├── conversations
│   ├── memory_facts
│   └── learning_data
│
├── blockchain (schema) - Rust Crypto
│   ├── fodi_transactions
│   ├── wallets
│   ├── nft_metadata
│   └── reward_history
│
└── analytics (schema) - Rust Analytics
    ├── metrics
    ├── events
    └── aggregations
```

---

## 🔧 Migration Plan

### Phase 1: Create Schemas

```sql
-- Create AI schema
CREATE SCHEMA IF NOT EXISTS ai;

-- Create blockchain schema
CREATE SCHEMA IF NOT EXISTS blockchain;

-- Create analytics schema
CREATE SCHEMA IF NOT EXISTS analytics;

-- Grant permissions
GRANT USAGE ON SCHEMA ai TO neondb_owner;
GRANT USAGE ON SCHEMA blockchain TO neondb_owner;
GRANT USAGE ON SCHEMA analytics TO neondb_owner;

GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA ai TO neondb_owner;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA blockchain TO neondb_owner;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA analytics TO neondb_owner;
```

---

### Phase 2: Create AI Tables

```sql
-- AI Cache (замена Sled cache)
CREATE TABLE ai.cache_entries (
    id SERIAL PRIMARY KEY,
    cache_key VARCHAR(255) NOT NULL UNIQUE,
    query TEXT NOT NULL,
    response TEXT NOT NULL,
    model VARCHAR(50) NOT NULL,
    cached_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    hit_count INTEGER DEFAULT 0,
    quality_score FLOAT,
    INDEX idx_cache_key (cache_key),
    INDEX idx_expires_at (expires_at)
);

-- AI Memory (persistent facts)
CREATE TABLE ai.memory_facts (
    id SERIAL PRIMARY KEY,
    user_id UUID REFERENCES public.users(id),
    business_id UUID REFERENCES public.businesses(id),
    fact_type VARCHAR(50) NOT NULL,
    fact_data JSONB NOT NULL,
    confidence FLOAT DEFAULT 1.0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    INDEX idx_user_id (user_id),
    INDEX idx_business_id (business_id)
);

-- AI Conversations (chat history)
CREATE TABLE ai.conversations (
    id SERIAL PRIMARY KEY,
    user_id UUID,
    session_id UUID NOT NULL,
    role VARCHAR(20) NOT NULL, -- user, assistant, system
    content TEXT NOT NULL,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    INDEX idx_session_id (session_id),
    INDEX idx_user_id (user_id)
);

-- AI Learning Data
CREATE TABLE ai.learning_data (
    id SERIAL PRIMARY KEY,
    category VARCHAR(100) NOT NULL,
    input_data JSONB NOT NULL,
    output_data JSONB NOT NULL,
    feedback_score FLOAT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

---

### Phase 3: Create Blockchain Tables

```sql
-- FODI Token Ledger (замена Sled ledger)
CREATE TABLE blockchain.fodi_transactions (
    id SERIAL PRIMARY KEY,
    tx_id VARCHAR(255) NOT NULL UNIQUE,
    from_address VARCHAR(255),
    to_address VARCHAR(255) NOT NULL,
    amount BIGINT NOT NULL,
    tx_type VARCHAR(50) NOT NULL, -- transfer, mint, burn, reward
    status VARCHAR(20) DEFAULT 'pending',
    blockchain_tx VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    confirmed_at TIMESTAMPTZ,
    INDEX idx_from_address (from_address),
    INDEX idx_to_address (to_address),
    INDEX idx_status (status)
);

-- Wallets (замена Sled wallets.db)
CREATE TABLE blockchain.wallets (
    id SERIAL PRIMARY KEY,
    user_id UUID REFERENCES public.users(id),
    public_key VARCHAR(255) NOT NULL UNIQUE,
    encrypted_private_key TEXT,
    wallet_type VARCHAR(50) DEFAULT 'solana',
    balance BIGINT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_sync TIMESTAMPTZ,
    INDEX idx_user_id (user_id),
    INDEX idx_public_key (public_key)
);

-- NFT Metadata
CREATE TABLE blockchain.nft_metadata (
    id SERIAL PRIMARY KEY,
    mint_address VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    symbol VARCHAR(10),
    uri TEXT,
    owner_address VARCHAR(255),
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    INDEX idx_mint_address (mint_address),
    INDEX idx_owner (owner_address)
);

-- Reward History
CREATE TABLE blockchain.reward_history (
    id SERIAL PRIMARY KEY,
    user_id UUID REFERENCES public.users(id),
    order_id INTEGER REFERENCES public.orders(id),
    amount BIGINT NOT NULL,
    reason VARCHAR(255),
    tx_id VARCHAR(255) REFERENCES blockchain.fodi_transactions(tx_id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    INDEX idx_user_id (user_id),
    INDEX idx_order_id (order_id)
);
```

---

### Phase 4: Create Analytics Tables

```sql
-- Metrics
CREATE TABLE analytics.metrics (
    id SERIAL PRIMARY KEY,
    metric_name VARCHAR(100) NOT NULL,
    value FLOAT NOT NULL,
    labels JSONB,
    recorded_at TIMESTAMPTZ DEFAULT NOW(),
    INDEX idx_metric_name (metric_name),
    INDEX idx_recorded_at (recorded_at)
);

-- Events
CREATE TABLE analytics.events (
    id SERIAL PRIMARY KEY,
    event_type VARCHAR(100) NOT NULL,
    user_id UUID,
    business_id UUID,
    event_data JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    INDEX idx_event_type (event_type),
    INDEX idx_user_id (user_id)
);
```

---

## 🔄 Code Migration

### Update Config

```rust
// src/config.rs
pub struct Config {
    pub database_url: String,  // PostgreSQL URL
    pub go_backend_url: String,
    
    // Remove Sled paths - no longer needed
    // pub db_path: String,  ❌ DELETE
    // pub wallet_db_path: String,  ❌ DELETE
}
```

### Create Database Module

```rust
// src/database/mod.rs
use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;

pub mod ai;
pub mod blockchain;
pub mod analytics;

pub struct DatabaseClient {
    pub pool: PgPool,
}

impl DatabaseClient {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;
        
        Ok(Self { pool })
    }
    
    // AI operations
    pub async fn cache_get(&self, key: &str) -> Result<Option<String>> {
        let result = sqlx::query_scalar!(
            "SELECT response FROM ai.cache_entries WHERE cache_key = $1 AND expires_at > NOW()",
            key
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(result)
    }
    
    pub async fn cache_set(&self, key: &str, response: &str, ttl_secs: i64) -> Result<()> {
        sqlx::query!(
            "INSERT INTO ai.cache_entries (cache_key, query, response, model, expires_at)
             VALUES ($1, $2, $3, $4, NOW() + INTERVAL '1 second' * $5)
             ON CONFLICT (cache_key) 
             DO UPDATE SET response = $3, cached_at = NOW(), hit_count = ai.cache_entries.hit_count + 1",
            key, "", response, "groq", ttl_secs
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    // Blockchain operations
    pub async fn create_fodi_transaction(
        &self,
        from: &str,
        to: &str,
        amount: i64
    ) -> Result<String> {
        let tx_id = uuid::Uuid::new_v4().to_string();
        
        sqlx::query!(
            "INSERT INTO blockchain.fodi_transactions (tx_id, from_address, to_address, amount, tx_type)
             VALUES ($1, $2, $3, $4, 'transfer')",
            tx_id, from, to, amount
        )
        .execute(&self.pool)
        .await?;
        
        Ok(tx_id)
    }
    
    pub async fn get_wallet_balance(&self, user_id: &str) -> Result<i64> {
        let balance = sqlx::query_scalar!(
            "SELECT balance FROM blockchain.wallets WHERE user_id = $1",
            uuid::Uuid::parse_str(user_id)?
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(balance.unwrap_or(0))
    }
}
```

---

## 📝 Migration Steps

### 1. Run SQL migrations
```bash
# Connect to Neon PostgreSQL
psql "postgresql://neondb_owner:npg_dz4Gl8ZhPLbX@ep-soft-mud-agon8wu3-pooler.c-2.eu-central-1.aws.neon.tech/neondb?sslmode=require"

# Run schema creation
\i migrations/001_create_schemas.sql
\i migrations/002_create_ai_tables.sql
\i migrations/003_create_blockchain_tables.sql
\i migrations/004_create_analytics_tables.sql
```

### 2. Update Cargo.toml
```toml
[dependencies]
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "macros", "uuid", "chrono"] }
```

### 3. Migrate Sled data to PostgreSQL
```bash
cargo run --bin migrate_sled_to_postgres
```

### 4. Update AI cache
```rust
// Replace src/ai/cache.rs Sled with PostgreSQL
impl AIResponseCache {
    pub async fn get(&self, query: &str, model: &str) -> Option<CachedResponse> {
        let key = self.cache_key(query, model);
        self.db.cache_get(&key).await.ok().flatten().map(|resp| {
            CachedResponse {
                response: resp,
                // ...
            }
        })
    }
}
```

### 5. Remove Sled dependencies
```bash
# Remove old databases
rm -rf data/fodi_ledger.db
rm -rf data/wallets.db
rm -rf data/ai_cache.db

# Update .env - remove DB_PATH, WALLET_DB_PATH
```

---

## ✅ Benefits

### Performance
- ✅ Single connection pool
- ✅ PostgreSQL indexes (faster than Sled)
- ✅ JOIN queries across schemas
- ✅ ACID transactions

### Data Integrity
- ✅ Foreign keys между схемами
- ✅ Single source of truth
- ✅ Automatic backups (Neon)
- ✅ No sync issues

### Scalability
- ✅ Horizontal scaling (read replicas)
- ✅ Better caching strategies
- ✅ Connection pooling
- ✅ Query optimization

### Development
- ✅ Easier debugging (pgAdmin)
- ✅ SQL migrations (version control)
- ✅ Schema visualization
- ✅ Standard tooling

---

## 📊 Example Queries

### Cross-schema JOIN
```sql
-- Get user with their wallet balance and AI conversations
SELECT 
    u.id,
    u.email,
    u.name,
    w.balance as fodi_balance,
    COUNT(c.id) as conversation_count
FROM public.users u
LEFT JOIN blockchain.wallets w ON u.id = w.user_id
LEFT JOIN ai.conversations c ON u.id = c.user_id
WHERE u.email = 'user@example.com'
GROUP BY u.id, w.balance;
```

### Analytics query
```sql
-- AI cache hit rate
SELECT 
    DATE(cached_at) as date,
    COUNT(*) as total_queries,
    AVG(hit_count) as avg_hits,
    SUM(CASE WHEN hit_count > 0 THEN 1 ELSE 0 END)::float / COUNT(*) as hit_rate
FROM ai.cache_entries
WHERE cached_at > NOW() - INTERVAL '7 days'
GROUP BY DATE(cached_at)
ORDER BY date DESC;
```

---

## 🚀 Next Steps

1. ✅ Create SQL migration files
2. ✅ Add sqlx to Cargo.toml
3. ✅ Create `src/database/` module
4. ✅ Migrate AI cache from Sled → PostgreSQL
5. ✅ Migrate FODI ledger from Sled → PostgreSQL
6. ✅ Migrate wallets from Sled → PostgreSQL
7. ✅ Update all references to Sled
8. ✅ Test thoroughly
9. ✅ Deploy to production

**Результат: Одна база, чистая архитектура, простая интеграция! 🎯**

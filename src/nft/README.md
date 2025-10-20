# 🧩 NFT Module - Business-as-NFT

Mint ресторанов/бизнесов как NFT, управление метаданными, marketplace для торговли.

## 📦 Компоненты

### 1. NFT Minter (`mint.rs`)
Создание бизнес NFT на Solana:

```rust
let minter = NftMinter::new(
    "https://api.devnet.solana.com".to_string(),
    keypair
);

// Создать NFT
let nft = minter.mint_business_nft(
    "biz_123",                          // business_id
    "Sushi Paradise".to_string(),      // name
    "BZNFT".to_string(),                // symbol
    "https://...metadata.json".to_string(), // uri
    BusinessAttributes {
        business_type: "restaurant".to_string(),
        cuisine: "sushi".to_string(),
        location: "Tokyo".to_string(),
        rating: 4.8,
        total_orders: 1000,
        established_date: "2024-01-01".to_string(),
    }
).await?;

println!("NFT minted: {}", nft.mint);
```

**Особенности NFT:**
- ✅ SPL Token standard (0 decimals)
- ✅ Unique mint address
- ✅ Metaplex compatible
- ✅ Business attributes embedded

### 2. Metadata Manager (`metadata.rs`)
Создание и обновление метаданных:

```rust
let updater = MetadataUpdater::new(
    "https://api.devnet.solana.com".to_string(),
    update_authority_keypair
);

// Создать off-chain metadata
let metadata = updater.create_business_metadata(
    "Sushi Paradise".to_string(),
    "Premium sushi restaurant in Tokyo".to_string(),
    "https://example.com/image.png".to_string(),
    attributes,
    creator_address
);

// Экспортировать в JSON
let json = updater.export_metadata_json(&metadata)?;
std::fs::write("metadata.json", json)?;

// Обновить атрибуты
updater.update_business_attributes(&mut metadata, new_attributes);
```

**Структура метаданных:**
```json
{
  "name": "Sushi Paradise",
  "symbol": "BZNFT",
  "description": "Premium sushi restaurant in Tokyo",
  "image": "https://example.com/image.png",
  "attributes": [
    {"trait_type": "Business Type", "value": "restaurant"},
    {"trait_type": "Cuisine", "value": "sushi"},
    {"trait_type": "Location", "value": "Tokyo"},
    {"trait_type": "Rating", "value": "4.8"},
    {"trait_type": "Total Orders", "value": "1000"}
  ],
  "properties": {
    "files": [{"uri": "...", "type": "image/png"}],
    "category": "image",
    "creators": [{"address": "...", "verified": true, "share": 100}]
  }
}
```

### 3. NFT Marketplace (`marketplace.rs`)
Торговля бизнес-NFT:

```rust
let marketplace = NftMarketplace::new(250); // 2.5% fee

// Создать листинг
let listing = marketplace.create_listing(
    nft,
    "seller_pubkey".to_string(),
    5_000_000_000,              // 5 FODI
    Currency::FODI,
    Some(30)                    // expires in 30 days
).await?;

// Получить активные листинги
let active = marketplace.get_active_listings().await?;

// Поиск по кухне
let sushi_nfts = marketplace.search_by_cuisine("sushi").await?;

// Купить NFT
let sale = marketplace.execute_sale(
    &listing.id,
    "buyer_pubkey".to_string(),
    "tx_signature".to_string()
).await?;

// Статистика маркетплейса
let stats = marketplace.get_stats().await?;
println!("Floor price: {:?}", stats.floor_price);
println!("Total sales: {}", stats.total_sales);
```

**Marketplace Features:**
- ✅ Создание листингов с expiration
- ✅ Отмена листингов
- ✅ Автоматическое управление статусом
- ✅ Floor price tracking
- ✅ Sales history
- ✅ Search & filtering
- ✅ Marketplace fee (configurable)

### 4. Business Attributes
```rust
pub struct BusinessAttributes {
    pub business_type: String,  // "restaurant", "cafe", "food_truck"
    pub cuisine: String,        // "sushi", "italian", "mexican"
    pub location: String,       // "Tokyo", "New York"
    pub rating: f32,            // 0.0 - 5.0
    pub total_orders: u64,      // Количество заказов
    pub established_date: String, // ISO date
}
```

## 🌐 API Endpoints

### Mint NFT
```bash
POST /api/nft/mint
Content-Type: application/json

{
  "business_id": "biz_123",
  "name": "Sushi Paradise",
  "description": "Premium sushi",
  "image_url": "https://...",
  "attributes": {
    "business_type": "restaurant",
    "cuisine": "sushi",
    "location": "Tokyo",
    "rating": 4.8,
    "total_orders": 1000,
    "established_date": "2024-01-01"
  }
}

Response:
{
  "mint": "ABC123...",
  "name": "Sushi Paradise",
  "owner": "XYZ789...",
  "metadata_uri": "https://..."
}
```

### Get Active Listings
```bash
GET /api/nft/listings

Response:
[
  {
    "id": "listing_123",
    "nft": {
      "mint": "ABC123...",
      "name": "Sushi Paradise",
      "attributes": {...}
    },
    "price": 5000000000,
    "currency": "FODI",
    "status": "Active",
    "created_at": "2025-10-20T12:00:00Z"
  }
]
```

### Get Marketplace Stats
```bash
GET /api/nft/marketplace/stats

Response:
{
  "total_listings": 50,
  "active_listings": 12,
  "total_sales": 35,
  "total_volume": 175000000000,
  "floor_price": 3000000000,
  "average_price": 5000000000
}
```

## 🧪 Тестирование

```bash
# Unit tests
cargo test --lib nft

# Интеграционные тесты
cargo test --test nft_integration

# Локальный сервер
cargo run --bin local

# Mint NFT
curl -X POST http://localhost:8000/api/nft/mint \
  -H "Content-Type: application/json" \
  -d '{
    "business_id": "biz_123",
    "name": "Sushi Paradise",
    "description": "Premium sushi restaurant",
    "image_url": "https://example.com/image.png",
    "attributes": {
      "business_type": "restaurant",
      "cuisine": "sushi",
      "location": "Tokyo",
      "rating": 4.8,
      "total_orders": 1000,
      "established_date": "2024-01-01"
    }
  }'
```

## 💡 Use Cases

### 1. Ресторан как инвестиция
Владелец ресторана минтит NFT → Продает долю инвесторам → Держатели NFT получают % от прибыли

### 2. Франчайзинг
Сеть ресторанов выпускает NFT для каждой локации → Франчайзи покупают права через NFT

### 3. Аренда бизнеса
Владелец NFT = владелец бизнеса → Может сдавать в аренду (временный transfer)

### 4. Коллекционирование
Инвесторы собирают портфолио успешных ресторанов в виде NFT

### 5. Динамические атрибуты
- **Rating** обновляется по отзывам
- **Total orders** растет со временем
- **Price history** отражает успешность бизнеса

## 📊 Метрики

- **Total NFTs minted**: Всего создано бизнес-NFT
- **Active listings**: Активные предложения
- **Floor price**: Минимальная цена
- **Trading volume**: Объем торгов
- **Average sale price**: Средняя цена продажи
- **Top cuisines**: Популярные кухни

## 🔒 Безопасность

- ✅ Только владелец может transfer NFT
- ✅ Marketplace escrow (planned)
- ✅ Verified creators
- ✅ Metadata integrity (IPFS pinning)
- ✅ Anti-scam measures

## 🚀 Roadmap

- [ ] IPFS metadata hosting
- [ ] Royalty enforcement
- [ ] Collection management
- [ ] Batch minting
- [ ] Fractional ownership
- [ ] Revenue sharing smart contracts
- [ ] NFT staking for rewards
- [ ] Cross-chain bridges

## 🎨 Metadata Standards

Совместимость с:
- ✅ Metaplex Token Metadata
- ✅ OpenSea metadata
- ✅ Magic Eden
- ✅ Tensor Trade

## 📚 Примеры

### Пример 1: Создание коллекции ресторанов
```rust
for business in businesses {
    let nft = minter.mint_business_nft(
        &business.id,
        business.name,
        "BZNFT".to_string(),
        business.metadata_uri,
        business.attributes
    ).await?;
    
    println!("Minted {} as {}", business.name, nft.mint);
}
```

### Пример 2: Обновление рейтинга
```rust
// Когда рейтинг изменяется
let mut metadata = load_metadata(&nft.uri)?;
metadata.attributes.rating = new_rating;

updater.update_business_attributes(&mut metadata, new_attributes);
let json = updater.export_metadata_json(&metadata)?;

// Upload to IPFS and update on-chain URI
updater.update_metadata_uri(&nft.mint, new_uri).await?;
```

---

**🧩 Business-as-NFT - революция в инвестировании в рестораны!**

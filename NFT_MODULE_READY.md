# NFT Module - Ready to Use! 🎉

**Дата:** 20 октября 2025 г.

## ✅ Что реализовано

### 1. **Mint NFT** - Создание NFT через Solana RPC
```rust
pub async fn send_mint_instruction(name: &str, uri: &str, roi: u16) -> Result<String>
```

**Особенности:**
- 🔐 Автоматическая загрузка keypair из `~/.config/solana/id.json`
- 📝 Генерация нового mint pubkey
- 🏷️ Создание Metaplex metadata PDA
- 💾 Borsh сериализация данных
- 📊 Полное логирование через `tracing`
- ✅ Правильные аккаунты: payer, mint, metadata, system, token, metaplex programs

### 2. **Check NFT** - Проверка владения NFT по имени
```rust
pub async fn check_user_nft(user_pubkey: &Pubkey, nft_name: &str) -> Result<bool>
```

**Использование:**
- 🔍 Получает все token accounts пользователя
- 📦 Проверяет metadata каждого NFT
- ✅ Возвращает `true` если найден NFT с заданным именем

### 3. **Get Business Stats** - Получение статистики с blockchain
```rust
pub async fn get_business_stats(business_pubkey: &Pubkey) -> Result<BusinessStats>
```

**Структура данных:**
```rust
pub struct BusinessStats {
    pub total_revenue: u64,
    pub total_orders: u64,
    pub roi: u16,
    pub unclaimed_revenue: u64,
    pub owner: String,
}
```

### 4. **Grant Role** - Выдача ролей на основе NFT
```rust
pub async fn grant_role_if_has_nft(user_pubkey: &Pubkey, required_nft: &str) -> Result<bool>
```

**Интеграция с системой ролей:**
```rust
let user_pubkey = Pubkey::from_str("...")?;
if grant_role_if_has_nft(&user_pubkey, "FODI BizNFT").await? {
    // User получает роль RestaurantOwner
}
```

## 🔧 Технологии

- ✅ **Без Anchor** - чистый Solana SDK
- ✅ **Borsh** сериализация для эффективности
- ✅ **Metaplex** интеграция для NFT metadata
- ✅ **Tracing** для продакшн логирования
- ✅ **Async/await** для производительности

## 📦 Зависимости добавлены

```toml
borsh = "1.5"           # Сериализация
shellexpand = "3.1"     # Раскрытие путей (~/)
```

## 🔗 Интеграция с API

### Пример endpoint для минта:

```rust
use crate::nft::onchain::send_mint_instruction;

#[derive(Deserialize)]
pub struct MintRequest {
    pub name: String,
    pub uri: String,
    pub roi: u16,
}

#[axum::debug_handler]
pub async fn mint_nft_endpoint(
    Json(req): Json<MintRequest>
) -> Result<Json<Value>, StatusCode> {
    let signature = send_mint_instruction(&req.name, &req.uri, req.roi)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(json!({
        "status": "success",
        "signature": signature,
        "explorer": format!("https://explorer.solana.com/tx/{}?cluster=devnet", signature)
    })))
}
```

### Пример проверки NFT при авторизации:

```rust
use crate::nft::onchain::check_user_nft;

pub async fn authenticate_with_nft(user_wallet: &str) -> Result<Role> {
    let pubkey = Pubkey::from_str(user_wallet)?;
    
    if check_user_nft(&pubkey, "FODI Restaurant Owner").await? {
        return Ok(Role::RestaurantOwner);
    }
    
    if check_user_nft(&pubkey, "FODI Partner").await? {
        return Ok(Role::Partner);
    }
    
    Ok(Role::User)
}
```

## 🚀 Следующие шаги

1. **Добавить REST endpoints** в `src/nft/api.rs`
2. **Интегрировать с системой ролей** в auth middleware
3. **Создать frontend** для минта NFT
4. **Деплой тестового program** на devnet
5. **Подключить marketplace функционал**

## 📝 Логи

Модуль использует structured logging:

```
🪙 Minting NFT: Pizza Restaurant, ROI 5%
📝 Mint: 7xKz...abc123
📝 Metadata PDA: 9pLm...def456
✅ NFT minted successfully! Signature: 2nB4...xyz789
```

```
🔍 Checking if user 5gH2...aaa111 has NFT 'FODI BizNFT'
📦 Found 3 token accounts
✅ Found matching NFT 'FODI BizNFT' for user
```

## 🎯 Готово к использованию!

Модуль полностью функционален и готов к интеграции с REST API и системой авторизации.

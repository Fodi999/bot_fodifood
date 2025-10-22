use sqlx::PgPool;
use anyhow::Result;
use chrono::{DateTime, Utc};

/// Blockchain operations for FODI transactions
pub struct BlockchainOps<'a> {
    pool: &'a PgPool,
}

impl<'a> BlockchainOps<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
    
    /// Create a FODI transaction
    pub async fn create_transaction(
        &self,
        tx_id: &str,
        from_address: Option<&str>,
        to_address: &str,
        amount: i64,
        tx_type: &str,
    ) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            "INSERT INTO blockchain.fodi_transactions (tx_id, from_address, to_address, amount, tx_type)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id"
        )
        .bind(tx_id)
        .bind(from_address)
        .bind(to_address)
        .bind(amount)
        .bind(tx_type)
        .fetch_one(self.pool)
        .await?;
        
        Ok(result.0)
    }
    
    /// Update transaction status
    pub async fn update_status(&self, tx_id: &str, status: &str, blockchain_tx: Option<&str>) -> Result<()> {
        sqlx::query(
            "UPDATE blockchain.fodi_transactions 
             SET status = $2, blockchain_tx = $3, confirmed_at = CASE WHEN $2 = 'confirmed' THEN NOW() ELSE confirmed_at END
             WHERE tx_id = $1"
        )
        .bind(tx_id)
        .bind(status)
        .bind(blockchain_tx)
        .execute(self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get transaction by ID
    pub async fn get_transaction(&self, tx_id: &str) -> Result<Option<FodiTransaction>> {
        let tx = sqlx::query_as::<_, FodiTransaction>(
            "SELECT id, tx_id, from_address, to_address, amount, tx_type, status, blockchain_tx, created_at, confirmed_at
             FROM blockchain.fodi_transactions
             WHERE tx_id = $1"
        )
        .bind(tx_id)
        .fetch_optional(self.pool)
        .await?;
        
        Ok(tx)
    }
    
    /// Get transactions by address (sent or received)
    pub async fn get_address_transactions(&self, address: &str, limit: i64) -> Result<Vec<FodiTransaction>> {
        let txs = sqlx::query_as::<_, FodiTransaction>(
            "SELECT id, tx_id, from_address, to_address, amount, tx_type, status, blockchain_tx, created_at, confirmed_at
             FROM blockchain.fodi_transactions
             WHERE from_address = $1 OR to_address = $1
             ORDER BY created_at DESC
             LIMIT $2"
        )
        .bind(address)
        .bind(limit)
        .fetch_all(self.pool)
        .await?;
        
        Ok(txs)
    }
}

/// Wallet operations
pub struct WalletOps<'a> {
    pool: &'a PgPool,
}

impl<'a> WalletOps<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
    
    /// Create a wallet
    pub async fn create_wallet(
        &self,
        user_id: uuid::Uuid,
        public_key: &str,
        encrypted_private_key: Option<&str>,
        wallet_type: &str,
    ) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            "INSERT INTO blockchain.wallets (user_id, public_key, encrypted_private_key, wallet_type)
             VALUES ($1, $2, $3, $4)
             RETURNING id"
        )
        .bind(user_id)
        .bind(public_key)
        .bind(encrypted_private_key)
        .bind(wallet_type)
        .fetch_one(self.pool)
        .await?;
        
        Ok(result.0)
    }
    
    /// Get wallet by public key
    pub async fn get_wallet(&self, public_key: &str) -> Result<Option<Wallet>> {
        let wallet = sqlx::query_as::<_, Wallet>(
            "SELECT id, user_id, public_key, encrypted_private_key, wallet_type, balance, created_at, last_sync
             FROM blockchain.wallets
             WHERE public_key = $1"
        )
        .bind(public_key)
        .fetch_optional(self.pool)
        .await?;
        
        Ok(wallet)
    }
    
    /// Get wallet by user ID
    pub async fn get_user_wallet(&self, user_id: uuid::Uuid) -> Result<Option<Wallet>> {
        let wallet = sqlx::query_as::<_, Wallet>(
            "SELECT id, user_id, public_key, encrypted_private_key, wallet_type, balance, created_at, last_sync
             FROM blockchain.wallets
             WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(self.pool)
        .await?;
        
        Ok(wallet)
    }
    
    /// Update wallet balance
    pub async fn update_balance(&self, public_key: &str, balance: i64) -> Result<()> {
        sqlx::query(
            "UPDATE blockchain.wallets 
             SET balance = $2, last_sync = NOW()
             WHERE public_key = $1"
        )
        .bind(public_key)
        .bind(balance)
        .execute(self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get total FODI supply
    pub async fn get_total_supply(&self) -> Result<i64> {
        let result: (Option<i64>,) = sqlx::query_as(
            "SELECT COALESCE(SUM(balance), 0)::BIGINT FROM blockchain.wallets"
        )
        .fetch_one(self.pool)
        .await?;
        
        Ok(result.0.unwrap_or(0))
    }
}

/// NFT operations
pub struct NFTOps<'a> {
    pool: &'a PgPool,
}

impl<'a> NFTOps<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
    
    /// Create NFT metadata
    pub async fn create_nft(
        &self,
        mint_address: &str,
        name: &str,
        symbol: Option<&str>,
        uri: Option<&str>,
        owner_address: Option<&str>,
        metadata: Option<serde_json::Value>,
    ) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            "INSERT INTO blockchain.nft_metadata (mint_address, name, symbol, uri, owner_address, metadata)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id"
        )
        .bind(mint_address)
        .bind(name)
        .bind(symbol)
        .bind(uri)
        .bind(owner_address)
        .bind(metadata)
        .fetch_one(self.pool)
        .await?;
        
        Ok(result.0)
    }
    
    /// Get NFT by mint address
    pub async fn get_nft(&self, mint_address: &str) -> Result<Option<NFTMetadata>> {
        let nft = sqlx::query_as::<_, NFTMetadata>(
            "SELECT id, mint_address, name, symbol, uri, owner_address, metadata, created_at
             FROM blockchain.nft_metadata
             WHERE mint_address = $1"
        )
        .bind(mint_address)
        .fetch_optional(self.pool)
        .await?;
        
        Ok(nft)
    }
    
    /// Get NFTs by owner
    pub async fn get_owner_nfts(&self, owner_address: &str) -> Result<Vec<NFTMetadata>> {
        let nfts = sqlx::query_as::<_, NFTMetadata>(
            "SELECT id, mint_address, name, symbol, uri, owner_address, metadata, created_at
             FROM blockchain.nft_metadata
             WHERE owner_address = $1
             ORDER BY created_at DESC"
        )
        .bind(owner_address)
        .fetch_all(self.pool)
        .await?;
        
        Ok(nfts)
    }
    
    /// Update NFT owner
    pub async fn transfer_nft(&self, mint_address: &str, new_owner: &str) -> Result<()> {
        sqlx::query(
            "UPDATE blockchain.nft_metadata 
             SET owner_address = $2
             WHERE mint_address = $1"
        )
        .bind(mint_address)
        .bind(new_owner)
        .execute(self.pool)
        .await?;
        
        Ok(())
    }
}

/// Reward operations
pub struct RewardOps<'a> {
    pool: &'a PgPool,
}

impl<'a> RewardOps<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
    
    /// Create reward entry
    pub async fn create_reward(
        &self,
        user_id: uuid::Uuid,
        order_id: Option<i32>,
        amount: i64,
        reason: Option<&str>,
        tx_id: Option<&str>,
    ) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            "INSERT INTO blockchain.reward_history (user_id, order_id, amount, reason, tx_id)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id"
        )
        .bind(user_id)
        .bind(order_id)
        .bind(amount)
        .bind(reason)
        .bind(tx_id)
        .fetch_one(self.pool)
        .await?;
        
        Ok(result.0)
    }
    
    /// Get user rewards
    pub async fn get_user_rewards(&self, user_id: uuid::Uuid) -> Result<Vec<RewardHistory>> {
        let rewards = sqlx::query_as::<_, RewardHistory>(
            "SELECT id, user_id, order_id, amount, reason, tx_id, created_at
             FROM blockchain.reward_history
             WHERE user_id = $1
             ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await?;
        
        Ok(rewards)
    }
    
    /// Get total rewards for user
    pub async fn get_user_total_rewards(&self, user_id: uuid::Uuid) -> Result<i64> {
        let result: (Option<i64>,) = sqlx::query_as(
            "SELECT COALESCE(SUM(amount), 0)::BIGINT FROM blockchain.reward_history WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;
        
        Ok(result.0.unwrap_or(0))
    }
}

// Data structures

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct FodiTransaction {
    pub id: i64,
    pub tx_id: String,
    pub from_address: Option<String>,
    pub to_address: String,
    pub amount: i64,
    pub tx_type: String,
    pub status: String,
    pub blockchain_tx: Option<String>,
    pub created_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Wallet {
    pub id: i64,
    pub user_id: uuid::Uuid,
    pub public_key: String,
    pub encrypted_private_key: Option<String>,
    pub wallet_type: String,
    pub balance: i64,
    pub created_at: DateTime<Utc>,
    pub last_sync: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NFTMetadata {
    pub id: i64,
    pub mint_address: String,
    pub name: String,
    pub symbol: Option<String>,
    pub uri: Option<String>,
    pub owner_address: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RewardHistory {
    pub id: i64,
    pub user_id: uuid::Uuid,
    pub order_id: Option<i32>,
    pub amount: i64,
    pub reason: Option<String>,
    pub tx_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

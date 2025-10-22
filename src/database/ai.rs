use sqlx::PgPool;
use anyhow::Result;
use chrono::{DateTime, Utc};

/// AI Cache operations
pub struct AICacheOps<'a> {
    pool: &'a PgPool,
}

impl<'a> AICacheOps<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
    
    /// Get cached response by key
    pub async fn get(&self, key: &str) -> Result<Option<CachedResponse>> {
        let result = sqlx::query_as::<_, CachedResponse>(
            "SELECT cache_key, query, response, model, cached_at, expires_at, hit_count, quality_score
             FROM ai.cache_entries 
             WHERE cache_key = $1 AND expires_at > NOW()"
        )
        .bind(key)
        .fetch_optional(self.pool)
        .await?;
        
        // Increment hit count if found
        if result.is_some() {
            sqlx::query(
                "UPDATE ai.cache_entries SET hit_count = hit_count + 1 WHERE cache_key = $1"
            )
            .bind(key)
            .execute(self.pool)
            .await?;
        }
        
        Ok(result)
    }
    
    /// Set cached response with TTL
    pub async fn set(&self, key: &str, query: &str, response: &str, model: &str, ttl_secs: i64) -> Result<()> {
        sqlx::query(
            "INSERT INTO ai.cache_entries (cache_key, query, response, model, expires_at)
             VALUES ($1, $2, $3, $4, NOW() + INTERVAL '1 second' * $5)
             ON CONFLICT (cache_key) DO UPDATE 
             SET response = $3, cached_at = NOW(), hit_count = ai.cache_entries.hit_count + 1, expires_at = NOW() + INTERVAL '1 second' * $5"
        )
        .bind(key)
        .bind(query)
        .bind(response)
        .bind(model)
        .bind(ttl_secs)
        .execute(self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Delete expired cache entries
    pub async fn cleanup_expired(&self) -> Result<i32> {
        let result: (i32,) = sqlx::query_as("SELECT ai.cleanup_expired_cache()")
            .fetch_one(self.pool)
            .await?;
        
        Ok(result.0)
    }
    
    /// Get cache statistics
    pub async fn stats(&self) -> Result<CacheStats> {
        let stats = sqlx::query_as::<_, CacheStats>(
            "SELECT 
                COUNT(*) as total_entries,
                SUM(hit_count) as total_hits,
                COUNT(CASE WHEN expires_at > NOW() THEN 1 END) as active_entries,
                AVG(quality_score) as avg_quality
             FROM ai.cache_entries"
        )
        .fetch_one(self.pool)
        .await?;
        
        Ok(stats)
    }
}

/// AI Memory operations
pub struct AIMemoryOps<'a> {
    pool: &'a PgPool,
}

impl<'a> AIMemoryOps<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
    
    /// Store a memory fact
    pub async fn store_fact(
        &self,
        user_id: Option<uuid::Uuid>,
        business_id: Option<uuid::Uuid>,
        fact_type: &str,
        fact_data: serde_json::Value,
        confidence: f64,
    ) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            "INSERT INTO ai.memory_facts (user_id, business_id, fact_type, fact_data, confidence)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id"
        )
        .bind(user_id)
        .bind(business_id)
        .bind(fact_type)
        .bind(fact_data)
        .bind(confidence)
        .fetch_one(self.pool)
        .await?;
        
        Ok(result.0)
    }
    
    /// Get facts by user
    pub async fn get_user_facts(&self, user_id: uuid::Uuid) -> Result<Vec<MemoryFact>> {
        let facts = sqlx::query_as::<_, MemoryFact>(
            "SELECT id, user_id, business_id, fact_type, fact_data, confidence, created_at, updated_at
             FROM ai.memory_facts
             WHERE user_id = $1
             ORDER BY updated_at DESC"
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await?;
        
        Ok(facts)
    }
    
    /// Get facts by business
    pub async fn get_business_facts(&self, business_id: uuid::Uuid) -> Result<Vec<MemoryFact>> {
        let facts = sqlx::query_as::<_, MemoryFact>(
            "SELECT id, user_id, business_id, fact_type, fact_data, confidence, created_at, updated_at
             FROM ai.memory_facts
             WHERE business_id = $1
             ORDER BY updated_at DESC"
        )
        .bind(business_id)
        .fetch_all(self.pool)
        .await?;
        
        Ok(facts)
    }
}

/// AI Conversation operations
pub struct AIConversationOps<'a> {
    pool: &'a PgPool,
}

impl<'a> AIConversationOps<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
    
    /// Store a conversation message
    pub async fn store_message(
        &self,
        user_id: Option<uuid::Uuid>,
        session_id: uuid::Uuid,
        role: &str,
        content: &str,
        metadata: Option<serde_json::Value>,
    ) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            "INSERT INTO ai.conversations (user_id, session_id, role, content, metadata)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id"
        )
        .bind(user_id)
        .bind(session_id)
        .bind(role)
        .bind(content)
        .bind(metadata)
        .fetch_one(self.pool)
        .await?;
        
        Ok(result.0)
    }
    
    /// Get conversation history by session
    pub async fn get_session_history(&self, session_id: uuid::Uuid, limit: i64) -> Result<Vec<ConversationMessage>> {
        let messages = sqlx::query_as::<_, ConversationMessage>(
            "SELECT id, user_id, session_id, role, content, metadata, created_at
             FROM ai.conversations
             WHERE session_id = $1
             ORDER BY created_at DESC
             LIMIT $2"
        )
        .bind(session_id)
        .bind(limit)
        .fetch_all(self.pool)
        .await?;
        
        Ok(messages)
    }
}

// Data structures

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CachedResponse {
    pub cache_key: String,
    pub query: String,
    pub response: String,
    pub model: String,
    pub cached_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub hit_count: i32,
    pub quality_score: Option<f64>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CacheStats {
    pub total_entries: Option<i64>,
    pub total_hits: Option<i64>,
    pub active_entries: Option<i64>,
    pub avg_quality: Option<f64>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MemoryFact {
    pub id: i64,
    pub user_id: Option<uuid::Uuid>,
    pub business_id: Option<uuid::Uuid>,
    pub fact_type: String,
    pub fact_data: serde_json::Value,
    pub confidence: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ConversationMessage {
    pub id: i64,
    pub user_id: Option<uuid::Uuid>,
    pub session_id: uuid::Uuid,
    pub role: String,
    pub content: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;

pub mod ai;
pub mod blockchain;
pub mod analytics;

/// Database client for PostgreSQL with multi-schema support
/// 
/// Schemas:
/// - `public`: Go backend (users, businesses, orders)
/// - `ai`: Rust AI (cache, memory, conversations, learning)
/// - `blockchain`: Rust Crypto (FODI transactions, wallets, NFTs, rewards)
/// - `analytics`: Rust Metrics (metrics, events, aggregations)
pub struct DatabaseClient {
    pub pool: PgPool,
}

impl DatabaseClient {
    /// Create a new database client with connection pool
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;
        
        Ok(Self { pool })
    }
    
    /// Get a reference to the connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_connection() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        let db = DatabaseClient::new(&database_url).await;
        assert!(db.is_ok());
    }
}

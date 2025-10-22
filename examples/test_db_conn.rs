use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenvy::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    
    println!("🔌 Connecting to PostgreSQL...");
    
    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;
    
    println!("✅ Connection pool created!");
    
    // Test 1: Check PostgreSQL version
    let version: (String,) = sqlx::query_as("SELECT version()")
        .fetch_one(&pool)
        .await?;
    
    println!("✅ Connected to {}", version.0.split_whitespace().take(2).collect::<Vec<_>>().join(" "));
    
    // Test 2: Check schemas
    let schemas: Vec<(String,)> = sqlx::query_as(
        "SELECT schema_name FROM information_schema.schemata 
         WHERE schema_name IN ('ai', 'blockchain', 'analytics', 'public') 
         ORDER BY schema_name"
    )
    .fetch_all(&pool)
    .await?;
    
    println!("\n📊 Available schemas:");
    for (schema,) in schemas {
        println!("   ✅ {}", schema);
    }
    
    // Test 3: Check AI tables
    let ai_tables: Vec<(String,)> = sqlx::query_as(
        "SELECT table_name FROM information_schema.tables 
         WHERE table_schema = 'ai' 
         ORDER BY table_name"
    )
    .fetch_all(&pool)
    .await?;
    
    println!("\n🤖 AI schema tables:");
    for (table,) in ai_tables {
        println!("   ✅ ai.{}", table);
    }
    
    // Test 4: Check blockchain tables
    let blockchain_tables: Vec<(String,)> = sqlx::query_as(
        "SELECT table_name FROM information_schema.tables 
         WHERE table_schema = 'blockchain' 
         ORDER BY table_name"
    )
    .fetch_all(&pool)
    .await?;
    
    println!("\n🔗 Blockchain schema tables:");
    for (table,) in blockchain_tables {
        println!("   ✅ blockchain.{}", table);
    }
    
    // Test 5: Check analytics tables
    let analytics_tables: Vec<(String,)> = sqlx::query_as(
        "SELECT table_name FROM information_schema.tables 
         WHERE table_schema = 'analytics' 
         ORDER BY table_name"
    )
    .fetch_all(&pool)
    .await?;
    
    println!("\n📈 Analytics schema tables:");
    for (table,) in analytics_tables {
        println!("   ✅ analytics.{}", table);
    }
    
    // Test 6: Write to cache_entries
    println!("\n💾 Testing write to ai.cache_entries...");
    
    let test_key = "test_connection_key";
    let test_response = "Connection test successful!";
    let test_query = "SELECT 1";
    
    sqlx::query(
        "INSERT INTO ai.cache_entries (cache_key, query, response, model, expires_at)
         VALUES ($1, $2, $3, $4, NOW() + INTERVAL '1 hour')
         ON CONFLICT (cache_key) DO UPDATE 
         SET response = $3, cached_at = NOW(), hit_count = ai.cache_entries.hit_count + 1"
    )
    .bind(test_key)
    .bind(test_query)
    .bind(test_response)
    .bind("groq")
    .execute(&pool)
    .await?;
    
    println!("✅ Cache write successful!");
    
    // Test 7: Read from cache_entries
    println!("📖 Testing read from ai.cache_entries...");
    
    let cached: Option<(String, i32)> = sqlx::query_as(
        "SELECT response, hit_count FROM ai.cache_entries WHERE cache_key = $1"
    )
    .bind(test_key)
    .fetch_optional(&pool)
    .await?;
    
    if let Some((response, hit_count)) = cached {
        println!("✅ Cache read successful!");
        println!("   Response: {}", response);
        println!("   Hit count: {}", hit_count);
    }
    
    // Test 8: Check cache count
    let cache_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ai.cache_entries")
        .fetch_one(&pool)
        .await?;
    
    println!("\n📊 Total cache entries: {}", cache_count.0);
    
    // Test 9: Test cleanup function
    println!("\n🧹 Testing cleanup function...");
    
    let deleted_count: (i32,) = sqlx::query_as("SELECT ai.cleanup_expired_cache()")
        .fetch_one(&pool)
        .await?;
    
    println!("✅ Cleanup function works! Deleted {} expired entries", deleted_count.0);
    
    println!("\n🎉 All tests passed!");
    println!("\n💡 Database is ready for:");
    println!("   🤖 AI cache & memory");
    println!("   🔗 FODI blockchain transactions");
    println!("   💰 Wallet management");
    println!("   📈 Analytics & metrics");
    
    Ok(())
}

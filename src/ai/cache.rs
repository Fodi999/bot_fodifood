//! 🧠 AI Response Cache - 3-Level Memory Architecture
//! 
//! Implements a hybrid caching system to reduce Groq API calls by 80-90%.
//! 
//! Architecture:
//! ```
//! User Request
//!     ↓
//! Level 1: In-Memory HashMap (0.001s) - Recent queries, 5-30 min TTL
//!     ↓ (miss)
//! Level 2: Sled DB (0.01s) - Persistent cache, 7 days TTL
//!     ↓ (miss)
//! Level 3: Groq API (0.5-2s) - Fresh AI response
//!     ↓
//! Save to Level 2 & Level 1 → Return to user
//! ```
//! 
//! Expected Performance:
//! - 80% requests: <0.01s (cache hit)
//! - 20% requests: 0.5-2s (API call)
//! - API cost reduction: 5-10x
//! - User experience: instant responses

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use sha2::{Sha256, Digest};

/// 📝 Cached response entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedResponse {
    /// Original query (for debugging)
    pub query: String,
    
    /// AI-generated response
    pub response: String,
    
    /// Model used (Llama70B, Llama8B, etc.)
    pub model: String,
    
    /// When this was cached
    pub cached_at: u64, // Unix timestamp
    
    /// How many times this was served from cache
    pub hit_count: u32,
    
    /// Average response quality score (0-100)
    pub quality_score: Option<u8>,
}

impl CachedResponse {
    pub fn new(query: String, response: String, model: String) -> Self {
        Self {
            query,
            response,
            model,
            cached_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            hit_count: 0,
            quality_score: None,
        }
    }

    pub fn is_expired(&self, ttl_seconds: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        now - self.cached_at > ttl_seconds
    }
}

/// 🎯 Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub l1_hits: u64,      // Memory hits
    pub l2_hits: u64,      // Sled hits
    pub misses: u64,       // API calls
    pub total_requests: u64,
    pub avg_response_time_ms: f64,
    pub cache_hit_rate: f64,
}

impl CacheStats {
    pub fn new() -> Self {
        Self {
            l1_hits: 0,
            l2_hits: 0,
            misses: 0,
            total_requests: 0,
            avg_response_time_ms: 0.0,
            cache_hit_rate: 0.0,
        }
    }

    pub fn calculate_hit_rate(&mut self) {
        if self.total_requests > 0 {
            self.cache_hit_rate = 
                ((self.l1_hits + self.l2_hits) as f64 / self.total_requests as f64) * 100.0;
        }
    }

    pub fn display(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  📊 AI CACHE STATISTICS                                     ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");
        
        println!("🎯 Cache Performance:");
        println!("   • Total Requests: {}", self.total_requests);
        println!("   • L1 Hits (Memory): {} ({:.1}%)", 
            self.l1_hits, 
            (self.l1_hits as f64 / self.total_requests as f64) * 100.0);
        println!("   • L2 Hits (Sled): {} ({:.1}%)", 
            self.l2_hits,
            (self.l2_hits as f64 / self.total_requests as f64) * 100.0);
        println!("   • Cache Misses (API): {} ({:.1}%)", 
            self.misses,
            (self.misses as f64 / self.total_requests as f64) * 100.0);
        println!("   • Overall Hit Rate: {:.1}%", self.cache_hit_rate);
        println!("   • Avg Response Time: {:.2}ms", self.avg_response_time_ms);
        println!();

        let api_savings = (self.l1_hits + self.l2_hits) as f64 / self.total_requests as f64;
        let cost_reduction = api_savings * 100.0;
        
        println!("💰 Cost Savings:");
        println!("   • API Calls Avoided: {}", self.l1_hits + self.l2_hits);
        println!("   • Cost Reduction: ~{:.0}%", cost_reduction);
        println!("   • Estimated Monthly Savings: ${:.2}", 
            (self.l1_hits + self.l2_hits) as f64 * 0.0001); // Rough estimate
        println!();

        let rating = if self.cache_hit_rate >= 80.0 {
            "⭐⭐⭐⭐⭐ EXCELLENT"
        } else if self.cache_hit_rate >= 60.0 {
            "⭐⭐⭐⭐ GOOD"
        } else if self.cache_hit_rate >= 40.0 {
            "⭐⭐⭐ FAIR"
        } else {
            "⭐⭐ NEEDS IMPROVEMENT"
        };
        
        println!("🎯 Performance Rating: {}", rating);
    }
}

/// 🧠 3-Level AI Response Cache
pub struct AIResponseCache {
    /// Level 1: In-memory cache (fastest, volatile)
    l1_cache: Arc<RwLock<HashMap<String, (CachedResponse, Instant)>>>,
    
    /// Level 2: Persistent cache (Sled DB)
    l2_cache: sled::Db,
    
    /// Cache statistics
    stats: Arc<RwLock<CacheStats>>,
    
    /// L1 TTL (time to live)
    l1_ttl: Duration,
    
    /// L2 TTL (time to live)
    l2_ttl_seconds: u64,
}

impl AIResponseCache {
    /// Create a new cache instance
    pub fn new(db_path: &str, l1_ttl_minutes: u64, l2_ttl_days: u64) -> Result<Arc<Self>> {
        let l2_cache = sled::open(db_path)?;
        
        Ok(Arc::new(Self {
            l1_cache: Arc::new(RwLock::new(HashMap::new())),
            l2_cache,
            stats: Arc::new(RwLock::new(CacheStats::new())),
            l1_ttl: Duration::from_secs(l1_ttl_minutes * 60),
            l2_ttl_seconds: l2_ttl_days * 24 * 60 * 60,
        }))
    }

    /// Generate cache key from query + model
    fn cache_key(query: &str, model: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(query.as_bytes());
        hasher.update(model.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    /// Get cached response (check L1 → L2 → None)
    pub async fn get(&self, query: &str, model: &str) -> Option<CachedResponse> {
        let key = Self::cache_key(query, model);
        let start = Instant::now();

        // Level 1: Check memory cache
        {
            let l1 = self.l1_cache.read().await;
            if let Some((cached, cached_at)) = l1.get(&key) {
                if cached_at.elapsed() < self.l1_ttl {
                    let mut stats = self.stats.write().await;
                    stats.l1_hits += 1;
                    stats.total_requests += 1;
                    stats.avg_response_time_ms = start.elapsed().as_millis() as f64;
                    stats.calculate_hit_rate();
                    
                    tracing::debug!("🎯 L1 Cache HIT: {} ({}ms)", 
                        &key[..8], start.elapsed().as_millis());
                    return Some(cached.clone());
                }
            }
        }

        // Level 2: Check persistent cache
        if let Ok(Some(data)) = self.l2_cache.get(&key) {
            if let Ok(mut cached) = bincode::deserialize::<CachedResponse>(&data) {
                if !cached.is_expired(self.l2_ttl_seconds) {
                    // Promote to L1
                    cached.hit_count += 1;
                    let mut l1 = self.l1_cache.write().await;
                    l1.insert(key.clone(), (cached.clone(), Instant::now()));

                    let mut stats = self.stats.write().await;
                    stats.l2_hits += 1;
                    stats.total_requests += 1;
                    stats.avg_response_time_ms = start.elapsed().as_millis() as f64;
                    stats.calculate_hit_rate();

                    tracing::debug!("🎯 L2 Cache HIT: {} ({}ms)", 
                        &key[..8], start.elapsed().as_millis());
                    
                    // Update L2 with incremented hit count
                    let _ = self.l2_cache.insert(&key, bincode::serialize(&cached).unwrap());
                    
                    return Some(cached);
                }
            }
        }

        // Cache miss
        let mut stats = self.stats.write().await;
        stats.misses += 1;
        stats.total_requests += 1;
        stats.calculate_hit_rate();
        
        tracing::debug!("❌ Cache MISS: {}", &key[..8]);
        None
    }

    /// Store response in both L1 and L2 caches
    pub async fn set(&self, query: &str, model: &str, response: String) -> Result<()> {
        let key = Self::cache_key(query, model);
        let cached = CachedResponse::new(query.to_string(), response, model.to_string());

        // Store in L1 (memory)
        {
            let mut l1 = self.l1_cache.write().await;
            l1.insert(key.clone(), (cached.clone(), Instant::now()));
        }

        // Store in L2 (persistent)
        let serialized = bincode::serialize(&cached)?;
        self.l2_cache.insert(&key, serialized)?;

        tracing::debug!("💾 Cached response: {}", &key[..8]);
        Ok(())
    }

    /// Clear L1 cache (memory) - keeps L2 intact
    pub async fn clear_l1(&self) {
        let mut l1 = self.l1_cache.write().await;
        l1.clear();
        tracing::info!("🗑️ L1 Cache cleared");
    }

    /// Clear both L1 and L2 caches
    pub async fn clear_all(&self) -> Result<()> {
        self.clear_l1().await;
        self.l2_cache.clear()?;
        tracing::info!("🗑️ All caches cleared");
        Ok(())
    }

    /// Get current statistics
    pub async fn get_stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }

    /// Display statistics
    pub async fn display_stats(&self) {
        let stats = self.stats.read().await;
        stats.display();
    }

    /// Clean expired entries from L1
    pub async fn cleanup_l1(&self) {
        let mut l1 = self.l1_cache.write().await;
        let now = Instant::now();
        l1.retain(|_, (_, cached_at)| now.duration_since(*cached_at) < self.l1_ttl);
        tracing::debug!("🧹 L1 Cache cleanup: {} entries remain", l1.len());
    }

    /// Clean expired entries from L2
    pub async fn cleanup_l2(&self) -> Result<u64> {
        let mut removed = 0;
        for item in self.l2_cache.iter() {
            if let Ok((key, value)) = item {
                if let Ok(cached) = bincode::deserialize::<CachedResponse>(&value) {
                    if cached.is_expired(self.l2_ttl_seconds) {
                        self.l2_cache.remove(&key)?;
                        removed += 1;
                    }
                }
            }
        }
        tracing::info!("🧹 L2 Cache cleanup: {} expired entries removed", removed);
        Ok(removed)
    }

    /// Get cache size info
    pub async fn get_size_info(&self) -> (usize, usize) {
        let l1_size = self.l1_cache.read().await.len();
        let l2_size = self.l2_cache.len() as usize;
        (l1_size, l2_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_basic() {
        let cache = AIResponseCache::new("data/test_cache.db", 5, 7).unwrap();
        
        // First call - should miss
        let result = cache.get("Hello", "llama-70b").await;
        assert!(result.is_none());

        // Store response
        cache.set("Hello", "llama-70b", "Hi there!".to_string()).await.unwrap();

        // Second call - should hit L1
        let result = cache.get("Hello", "llama-70b").await;
        assert!(result.is_some());
        assert_eq!(result.unwrap().response, "Hi there!");

        let stats = cache.get_stats().await;
        assert_eq!(stats.l1_hits, 1);
        assert_eq!(stats.misses, 1);
    }

    #[tokio::test]
    async fn test_cache_l2_promotion() {
        let cache = AIResponseCache::new("data/test_cache2.db", 5, 7).unwrap();
        
        // Store in cache
        cache.set("Test", "llama-8b", "Response".to_string()).await.unwrap();
        
        // Clear L1 to force L2 lookup
        cache.clear_l1().await;
        
        // Should hit L2 and promote to L1
        let result = cache.get("Test", "llama-8b").await;
        assert!(result.is_some());

        let stats = cache.get_stats().await;
        assert_eq!(stats.l2_hits, 1);
    }
}

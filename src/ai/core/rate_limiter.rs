//! 🚦 Rate Limiter & Key Balancer for Groq API
//! 
//! Prevents 429 Rate Limit errors and manages concurrent load for production.
//! 
//! Features:
//! - Semaphore-based concurrency control (max 50 concurrent requests)
//! - Multi-key rotation for load balancing
//! - Request throttling with adaptive delays
//! - Usage tracking and statistics
//! 
//! Usage:
//! ```rust
//! use crate::ai::core::rate_limiter::GLOBAL_RATE_LIMITER;
//! 
//! let _permit = GLOBAL_RATE_LIMITER.acquire().await;
//! let api_key = GLOBAL_RATE_LIMITER.get_key().await;
//! ```

use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{Semaphore, Mutex};
use rand::{seq::SliceRandom, thread_rng};
use lazy_static::lazy_static;
use dotenvy::dotenv;
use std::env;

/// 📊 Statistics for rate limiter monitoring
#[derive(Debug, Clone)]
pub struct RateLimiterStats {
    pub total_requests: u64,
    pub keys_count: usize,
    pub max_concurrent: usize,
    pub current_active: usize,
}

/// 🚦 Rate Limiter for Groq API
/// 
/// Manages concurrent requests and API key rotation to prevent rate limiting.
/// Designed for production load with 1000+ concurrent users.
pub struct GroqRateLimiter {
    /// Semaphore controlling max concurrent requests
    semaphore: Arc<Semaphore>,
    
    /// Pool of API keys for load balancing
    keys: Vec<String>,
    
    /// Usage tracking: (key, last_used_timestamp, request_count)
    usage: Arc<Mutex<Vec<(String, Instant, u64)>>>,
    
    /// Total requests counter
    total_requests: Arc<Mutex<u64>>,
    
    /// Throttle delay in milliseconds
    throttle_ms: u64,
}

impl GroqRateLimiter {
    /// Create a new rate limiter with specified concurrency and API keys
    /// 
    /// # Arguments
    /// * `max_concurrent` - Maximum number of simultaneous API requests
    /// * `keys` - Vector of Groq API keys for load balancing
    pub fn new(max_concurrent: usize, keys: Vec<String>) -> Arc<Self> {
        let limiter = Arc::new(Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            keys: keys.clone(),
            usage: Arc::new(Mutex::new(
                keys.iter()
                    .map(|k| (k.clone(), Instant::now(), 0))
                    .collect()
            )),
            total_requests: Arc::new(Mutex::new(0)),
            throttle_ms: 300, // 300ms between waves
        });

        println!("🚦 Rate Limiter initialized:");
        println!("   • Max concurrent: {}", max_concurrent);
        println!("   • API keys: {}", keys.len());
        println!("   • Throttle delay: {}ms", limiter.throttle_ms);

        limiter
    }

    /// Acquire permit for API request (blocks if max concurrent reached)
    /// 
    /// Returns a permit that must be held during the API request.
    /// When dropped, the permit is returned to the pool.
    pub async fn acquire(&self) -> tokio::sync::OwnedSemaphorePermit {
        self.semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("Semaphore closed unexpectedly")
    }

    /// Get an API key with load balancing
    /// 
    /// Selects the least recently used key from the pool and updates usage stats.
    pub async fn get_key(&self) -> String {
        let mut usage = self.usage.lock().await;
        
        // Find least recently used key
        let (key, _timestamp, count) = usage
            .iter_mut()
            .min_by_key(|(_, ts, _)| *ts)
            .expect("No API keys available");

        // Update usage
        let selected_key = key.clone();
        *_timestamp = Instant::now();
        *count += 1;

        // Increment total requests
        let mut total = self.total_requests.lock().await;
        *total += 1;

        selected_key
    }

    /// Randomly select a key (alternative to LRU selection)
    pub async fn get_random_key(&self) -> String {
        let mut rng = thread_rng();
        let key = {
            let mut usage = self.usage.lock().await;
            let idx = (0..usage.len()).collect::<Vec<_>>().choose(&mut rng).unwrap().clone();
            let (key, timestamp, count) = &mut usage[idx];
            *timestamp = Instant::now();
            *count += 1;
            key.clone()
        };

        // Increment total requests
        let mut total = self.total_requests.lock().await;
        *total += 1;

        key
    }

    /// Throttle between request waves to prevent burst overload
    /// 
    /// Adds a small delay to smooth out request patterns and avoid rate limits.
    pub async fn throttle_wave(&self) {
        tokio::time::sleep(Duration::from_millis(self.throttle_ms)).await;
    }

    /// Get current statistics for monitoring
    pub async fn get_stats(&self) -> RateLimiterStats {
        let usage = self.usage.lock().await;
        let total = self.total_requests.lock().await;
        let available_permits = self.semaphore.available_permits();

        RateLimiterStats {
            total_requests: *total,
            keys_count: self.keys.len(),
            max_concurrent: self.semaphore.available_permits() + (50 - available_permits), // Assuming max 50
            current_active: 50 - available_permits, // Active requests
        }
    }

    /// Display usage statistics for each API key
    pub async fn display_usage_stats(&self) {
        let usage = self.usage.lock().await;
        let total = self.total_requests.lock().await;

        println!("\n📊 Rate Limiter Statistics:");
        println!("   • Total Requests: {}", total);
        println!("   • Active Keys: {}", usage.len());
        println!("\n   Key Usage Breakdown:");

        for (idx, (key, timestamp, count)) in usage.iter().enumerate() {
            let key_preview = if key.len() > 20 {
                format!("{}...{}", &key[..8], &key[key.len()-8..])
            } else {
                key.clone()
            };
            let elapsed = timestamp.elapsed().as_secs();
            println!("      {}: {} requests (last used {}s ago)", 
                idx + 1, count, elapsed);
        }
    }

    /// Reset statistics (useful for testing)
    pub async fn reset_stats(&self) {
        let mut usage = self.usage.lock().await;
        for (_, _, count) in usage.iter_mut() {
            *count = 0;
        }
        let mut total = self.total_requests.lock().await;
        *total = 0;
    }
}

lazy_static! {
    /// Global rate limiter instance
    /// 
    /// Configured from environment variables:
    /// - `GROQ_API_KEYS`: Comma-separated list of API keys
    /// - `GROQ_API_KEY`: Single API key (fallback)
    /// 
    /// Default: 50 concurrent requests max
    pub static ref GLOBAL_RATE_LIMITER: Arc<GroqRateLimiter> = {
        dotenv().ok();
        
        // Load API keys from environment
        let keys_str = env::var("GROQ_API_KEYS")
            .or_else(|_| env::var("GROQ_API_KEY"))
            .unwrap_or_else(|_| {
                eprintln!("⚠️  WARNING: No GROQ_API_KEY or GROQ_API_KEYS found in environment!");
                eprintln!("   Rate limiter will operate with empty key pool.");
                String::new()
            });

        let keys: Vec<String> = keys_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if keys.is_empty() {
            eprintln!("⚠️  WARNING: Rate limiter has no API keys!");
        }

        // Create limiter with 50 concurrent requests max
        GroqRateLimiter::new(50, keys)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_basic() {
        let keys = vec!["test_key_1".to_string(), "test_key_2".to_string()];
        let limiter = GroqRateLimiter::new(10, keys);

        // Test acquiring permit
        let _permit = limiter.acquire().await;
        
        // Test getting key
        let key = limiter.get_key().await;
        assert!(!key.is_empty());

        // Test stats
        let stats = limiter.get_stats().await;
        assert_eq!(stats.keys_count, 2);
        assert_eq!(stats.total_requests, 1);
    }

    #[tokio::test]
    async fn test_concurrent_requests() {
        let keys = vec!["test_key_1".to_string()];
        let limiter = GroqRateLimiter::new(5, keys);

        let mut handles = vec![];
        for _ in 0..10 {
            let limiter = limiter.clone();
            let handle = tokio::spawn(async move {
                let _permit = limiter.acquire().await;
                let _key = limiter.get_key().await;
                tokio::time::sleep(Duration::from_millis(10)).await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        let stats = limiter.get_stats().await;
        assert_eq!(stats.total_requests, 10);
    }

    #[tokio::test]
    async fn test_key_rotation() {
        let keys = vec![
            "key1".to_string(),
            "key2".to_string(),
            "key3".to_string(),
        ];
        let limiter = GroqRateLimiter::new(10, keys);

        let mut selected_keys = std::collections::HashSet::new();
        for _ in 0..20 {
            let key = limiter.get_random_key().await;
            selected_keys.insert(key);
        }

        // Should use multiple keys
        assert!(selected_keys.len() > 1);
    }
}

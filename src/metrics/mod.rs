use std::sync::Arc;
use std::time::{Duration, Instant};
use dashmap::DashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;

/// Statistics snapshot from metrics collector
#[derive(Debug, Clone)]
pub struct MetricsStats {
    pub total_intents: u64,
    pub failed_intents: u64,
    pub active_connections: u64,
    pub total_connections: u64,
    pub avg_response_time: f64,
    pub min_response_time: f64,
    pub max_response_time: f64,
    pub intents_by_type: HashMap<String, u64>,
}

/// 📊 Metrics Collector for AI Intent Handlers
///
/// Collects and stores metrics for:
/// - Intent handler invocations
/// - Response times
/// - Success/error rates
/// - Handler performance
#[derive(Clone)]
pub struct MetricsCollector {
    /// Number of times each intent was invoked
    intent_counts: Arc<DashMap<String, AtomicU64>>,
    
    /// Response times for each intent (last 100 invocations)
    response_times: Arc<DashMap<String, Vec<Duration>>>,
    
    /// Error counts per intent
    error_counts: Arc<DashMap<String, AtomicU64>>,
    
    /// Success counts per intent
    success_counts: Arc<DashMap<String, AtomicU64>>,
    
    /// Total requests processed
    total_requests: Arc<AtomicU64>,
    
    /// Application start time
    start_time: Instant,
    
    /// Active WebSocket connections
    active_connections: Arc<AtomicU64>,
    
    /// Total connections (lifetime)
    total_connections: Arc<AtomicU64>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            intent_counts: Arc::new(DashMap::new()),
            response_times: Arc::new(DashMap::new()),
            error_counts: Arc::new(DashMap::new()),
            success_counts: Arc::new(DashMap::new()),
            total_requests: Arc::new(AtomicU64::new(0)),
            start_time: Instant::now(),
            active_connections: Arc::new(AtomicU64::new(0)),
            total_connections: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Record an intent invocation
    pub fn record_intent(&self, intent: &str) {
        self.intent_counts
            .entry(intent.to_string())
            .or_insert_with(|| AtomicU64::new(0))
            .fetch_add(1, Ordering::Relaxed);
        
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Record response time for an intent
    pub fn record_response_time(&self, intent: &str, duration: Duration) {
        let mut times = self.response_times
            .entry(intent.to_string())
            .or_insert_with(Vec::new);
        
        // Keep only last 100 measurements
        if times.len() >= 100 {
            times.remove(0);
        }
        times.push(duration);
    }

    /// Record a successful intent handling
    pub fn record_success(&self, intent: &str) {
        self.success_counts
            .entry(intent.to_string())
            .or_insert_with(|| AtomicU64::new(0))
            .fetch_add(1, Ordering::Relaxed);
    }

    /// Record an error in intent handling
    #[allow(dead_code)] // Will be used when error handling is enhanced
    pub fn record_error(&self, intent: &str) {
        self.error_counts
            .entry(intent.to_string())
            .or_insert_with(|| AtomicU64::new(0))
            .fetch_add(1, Ordering::Relaxed);
    }

    /// Get count for a specific intent
    pub fn get_intent_count(&self, intent: &str) -> u64 {
        self.intent_counts
            .get(intent)
            .map(|v| v.load(Ordering::Relaxed))
            .unwrap_or(0)
    }

    /// Get average response time for an intent
    pub fn get_avg_response_time(&self, intent: &str) -> Option<Duration> {
        self.response_times.get(intent).and_then(|times| {
            if times.is_empty() {
                None
            } else {
                let sum: Duration = times.iter().sum();
                Some(sum / times.len() as u32)
            }
        })
    }

    /// Get success rate for an intent (0.0 to 1.0)
    pub fn get_success_rate(&self, intent: &str) -> f64 {
        let success = self.success_counts
            .get(intent)
            .map(|v| v.load(Ordering::Relaxed))
            .unwrap_or(0);
        
        let errors = self.error_counts
            .get(intent)
            .map(|v| v.load(Ordering::Relaxed))
            .unwrap_or(0);
        
        let total = success + errors;
        if total == 0 {
            1.0
        } else {
            success as f64 / total as f64
        }
    }

    /// Get total number of requests
    pub fn total_requests(&self) -> u64 {
        self.total_requests.load(Ordering::Relaxed)
    }

    /// Get uptime duration
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get all tracked intents
    pub fn all_intents(&self) -> Vec<String> {
        self.intent_counts
            .iter()
            .map(|entry| entry.key().clone())
            .collect()
    }

    /// Increment active connections
    pub fn increment_connections(&self) {
        self.active_connections.fetch_add(1, Ordering::Relaxed);
        self.total_connections.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement active connections
    pub fn decrement_connections(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    /// Get metrics statistics snapshot
    pub fn get_stats(&self) -> MetricsStats {
        let mut intents_by_type = HashMap::new();
        let mut all_response_times = Vec::new();
        let mut total_errors = 0u64;

        for entry in self.intent_counts.iter() {
            let intent = entry.key().clone();
            let count = entry.value().load(Ordering::Relaxed);
            intents_by_type.insert(intent.clone(), count);

            // Collect response times
            if let Some(times) = self.response_times.get(&intent) {
                all_response_times.extend(times.iter().copied());
            }

            // Sum up errors
            if let Some(errors) = self.error_counts.get(&intent) {
                total_errors += errors.load(Ordering::Relaxed);
            }
        }

        let (avg_time, min_time, max_time) = if all_response_times.is_empty() {
            (0.0, 0.0, 0.0)
        } else {
            let sum: Duration = all_response_times.iter().sum();
            let avg = sum.as_secs_f64() / all_response_times.len() as f64;
            let min = all_response_times.iter().min().map(|d| d.as_secs_f64()).unwrap_or(0.0);
            let max = all_response_times.iter().max().map(|d| d.as_secs_f64()).unwrap_or(0.0);
            (avg, min, max)
        };

        MetricsStats {
            total_intents: self.total_requests.load(Ordering::Relaxed),
            failed_intents: total_errors,
            active_connections: self.active_connections.load(Ordering::Relaxed),
            total_connections: self.total_connections.load(Ordering::Relaxed),
            avg_response_time: avg_time,
            min_response_time: min_time,
            max_response_time: max_time,
            intents_by_type,
        }
    }

    /// Generate Prometheus metrics format
    pub fn to_prometheus(&self) -> String {
        let mut output = String::new();

        // Help text
        output.push_str("# HELP ai_intent_invocations_total Total number of intent invocations\n");
        output.push_str("# TYPE ai_intent_invocations_total counter\n");
        
        // Intent counts
        for entry in self.intent_counts.iter() {
            let intent = entry.key();
            let count = entry.value().load(Ordering::Relaxed);
            output.push_str(&format!(
                "ai_intent_invocations_total{{intent=\"{}\"}} {}\n",
                intent, count
            ));
        }

        output.push('\n');

        // Response times
        output.push_str("# HELP ai_intent_response_time_seconds Average response time in seconds\n");
        output.push_str("# TYPE ai_intent_response_time_seconds gauge\n");
        
        for entry in self.response_times.iter() {
            let intent = entry.key();
            if let Some(avg) = self.get_avg_response_time(intent) {
                output.push_str(&format!(
                    "ai_intent_response_time_seconds{{intent=\"{}\"}} {:.6}\n",
                    intent,
                    avg.as_secs_f64()
                ));
            }
        }

        output.push('\n');

        // Success rates
        output.push_str("# HELP ai_intent_success_rate Success rate for intents (0.0 to 1.0)\n");
        output.push_str("# TYPE ai_intent_success_rate gauge\n");
        
        for intent in self.all_intents() {
            let rate = self.get_success_rate(&intent);
            output.push_str(&format!(
                "ai_intent_success_rate{{intent=\"{}\"}} {:.4}\n",
                intent, rate
            ));
        }

        output.push('\n');

        // Total requests
        output.push_str("# HELP ai_requests_total Total number of AI requests processed\n");
        output.push_str("# TYPE ai_requests_total counter\n");
        output.push_str(&format!("ai_requests_total {}\n", self.total_requests()));

        output.push('\n');

        // Uptime
        output.push_str("# HELP ai_uptime_seconds Application uptime in seconds\n");
        output.push_str("# TYPE ai_uptime_seconds gauge\n");
        output.push_str(&format!("ai_uptime_seconds {:.0}\n", self.uptime().as_secs_f64()));

        output
    }

    /// Generate JSON metrics for admin dashboard
    pub fn to_json(&self) -> serde_json::Value {
        let intents: Vec<serde_json::Value> = self.all_intents()
            .iter()
            .map(|intent| {
                serde_json::json!({
                    "intent": intent,
                    "count": self.get_intent_count(intent),
                    "avg_response_time_ms": self.get_avg_response_time(intent)
                        .map(|d| d.as_millis())
                        .unwrap_or(0),
                    "success_rate": self.get_success_rate(intent),
                    "errors": self.error_counts.get(intent)
                        .map(|v| v.load(Ordering::Relaxed))
                        .unwrap_or(0),
                })
            })
            .collect();

        serde_json::json!({
            "total_requests": self.total_requests(),
            "uptime_seconds": self.uptime().as_secs(),
            "intents": intents,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        })
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_counting() {
        let metrics = MetricsCollector::new();
        
        metrics.record_intent("menu");
        metrics.record_intent("menu");
        metrics.record_intent("order");
        
        assert_eq!(metrics.get_intent_count("menu"), 2);
        assert_eq!(metrics.get_intent_count("order"), 1);
        assert_eq!(metrics.total_requests(), 3);
    }

    #[test]
    fn test_response_time() {
        let metrics = MetricsCollector::new();
        
        metrics.record_response_time("menu", Duration::from_millis(100));
        metrics.record_response_time("menu", Duration::from_millis(200));
        
        let avg = metrics.get_avg_response_time("menu").unwrap();
        assert_eq!(avg.as_millis(), 150);
    }

    #[test]
    fn test_success_rate() {
        let metrics = MetricsCollector::new();
        
        metrics.record_success("menu");
        metrics.record_success("menu");
        metrics.record_error("menu");
        
        let rate = metrics.get_success_rate("menu");
        assert!((rate - 0.6667).abs() < 0.001);
    }

    #[test]
    fn test_prometheus_format() {
        let metrics = MetricsCollector::new();
        
        metrics.record_intent("menu");
        metrics.record_success("menu");
        
        let prometheus = metrics.to_prometheus();
        
        assert!(prometheus.contains("ai_intent_invocations_total"));
        assert!(prometheus.contains("intent=\"menu\""));
        assert!(prometheus.contains("ai_requests_total"));
    }

    #[test]
    fn test_json_format() {
        let metrics = MetricsCollector::new();
        
        metrics.record_intent("menu");
        metrics.record_response_time("menu", Duration::from_millis(100));
        metrics.record_success("menu");
        
        let json = metrics.to_json();
        
        assert_eq!(json["total_requests"], 1);
        assert!(json["intents"].is_array());
        assert_eq!(json["intents"][0]["intent"], "menu");
    }
}

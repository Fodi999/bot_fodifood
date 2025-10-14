/// 🏥 Health Checker for Go Backend
///
/// Performs periodic health checks on the Go backend service

use anyhow::{Context, Result};
use reqwest::Client;
use std::time::Duration;

/// Health check result
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)] // Will be fully used when orchestration is enabled
pub enum HealthStatus {
    Healthy,
    Unhealthy(String),
    Unknown,
}

/// Health checker for Go backend
pub struct HealthChecker {
    client: Client,
    health_url: String,
    timeout: Duration,
}

#[allow(dead_code)] // All methods will be used when orchestration is active
impl HealthChecker {
    /// Create a new health checker
    ///
    /// # Arguments
    /// * `base_url` - Base URL of the Go backend (e.g., "http://localhost:8080")
    /// * `timeout_secs` - Timeout for health checks in seconds
    pub fn new(base_url: String, timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        let health_url = format!("{}/health", base_url);

        Self {
            client,
            health_url,
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    /// Perform a health check
    ///
    /// Returns HealthStatus indicating whether the backend is healthy
    pub async fn check(&self) -> HealthStatus {
        match self.check_internal().await {
            Ok(true) => {
                tracing::debug!(target: "orchestration", "✅ Backend health check: OK");
                HealthStatus::Healthy
            }
            Ok(false) => {
                tracing::warn!(target: "orchestration", "⚠️  Backend health check: Unhealthy response");
                HealthStatus::Unhealthy("Backend returned unhealthy status".to_string())
            }
            Err(e) => {
                tracing::error!(target: "orchestration", "❌ Backend health check failed: {}", e);
                HealthStatus::Unhealthy(format!("Health check error: {}", e))
            }
        }
    }

    /// Internal health check implementation
    async fn check_internal(&self) -> Result<bool> {
        let response = self
            .client
            .get(&self.health_url)
            .timeout(self.timeout)
            .send()
            .await
            .context("Failed to send health check request")?;

        // Check if status is 200 OK
        Ok(response.status().is_success())
    }

    /// Perform multiple health checks with retries
    ///
    /// # Arguments
    /// * `retries` - Number of retry attempts
    /// * `delay_ms` - Delay between retries in milliseconds
    ///
    /// Returns true if any check succeeds
    pub async fn check_with_retries(&self, retries: u32, delay_ms: u64) -> HealthStatus {
        for attempt in 1..=retries {
            let status = self.check().await;
            
            if matches!(status, HealthStatus::Healthy) {
                return status;
            }

            if attempt < retries {
                tracing::debug!(
                    target: "orchestration",
                    "Health check attempt {}/{} failed, retrying in {}ms...",
                    attempt,
                    retries,
                    delay_ms
                );
                tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            }
        }

        HealthStatus::Unhealthy(format!("Failed after {} retries", retries))
    }

    /// Get the health check timeout
    pub fn timeout(&self) -> Duration {
        self.timeout
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_checker_creation() {
        let checker = HealthChecker::new("http://localhost:8080".to_string(), 5);
        assert_eq!(checker.timeout(), Duration::from_secs(5));
    }

    #[test]
    fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_ne!(
            HealthStatus::Healthy,
            HealthStatus::Unhealthy("error".to_string())
        );
    }
}

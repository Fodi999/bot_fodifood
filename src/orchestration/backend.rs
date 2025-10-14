/// 🎯 Go Backend Process Orchestrator
///
/// Manages the lifecycle of the Go backend server process

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

use super::health::{HealthChecker, HealthStatus};

/// Backend process status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BackendStatus {
    /// Backend is not running
    Stopped,
    /// Backend is starting up
    Starting,
    /// Backend is running and healthy
    Running,
    /// Backend is running but unhealthy
    Unhealthy,
    /// Backend is stopping
    Stopping,
    /// Backend crashed unexpectedly
    Crashed(String),
}

/// Backend process information
#[derive(Debug, Clone, Serialize)]
pub struct BackendInfo {
    pub status: BackendStatus,
    pub pid: Option<u32>,
    pub uptime_secs: Option<u64>,
    pub restart_count: u32,
    pub last_health_check: Option<String>,
}

/// Configuration for backend orchestrator
#[derive(Debug, Clone)]
pub struct OrchestratorConfig {
    /// Path to the Go backend executable
    pub binary_path: String,
    /// Working directory for the process
    pub working_dir: Option<String>,
    /// Backend base URL for health checks
    pub base_url: String,
    /// Health check interval in seconds
    pub health_check_interval_secs: u64,
    /// Health check timeout in seconds
    pub health_check_timeout_secs: u64,
    /// Enable auto-restart on crash
    pub auto_restart: bool,
    /// Maximum restart attempts
    pub max_restart_attempts: u32,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            binary_path: "./go-backend/main".to_string(),
            working_dir: None,
            base_url: "http://localhost:8080".to_string(),
            health_check_interval_secs: 30,
            health_check_timeout_secs: 5,
            auto_restart: true,
            max_restart_attempts: 3,
        }
    }
}

/// Go Backend Orchestrator
pub struct BackendOrchestrator {
    config: OrchestratorConfig,
    process: Arc<RwLock<Option<Child>>>,
    status: Arc<RwLock<BackendStatus>>,
    health_checker: Arc<HealthChecker>,
    start_time: Arc<RwLock<Option<Instant>>>,
    restart_count: Arc<RwLock<u32>>,
}

impl BackendOrchestrator {
    /// Create a new backend orchestrator
    pub fn new(config: OrchestratorConfig) -> Self {
        let health_checker = Arc::new(HealthChecker::new(
            config.base_url.clone(),
            config.health_check_timeout_secs,
        ));

        Self {
            config,
            process: Arc::new(RwLock::new(None)),
            status: Arc::new(RwLock::new(BackendStatus::Stopped)),
            health_checker,
            start_time: Arc::new(RwLock::new(None)),
            restart_count: Arc::new(RwLock::new(0)),
        }
    }

    /// Start the Go backend process
    pub async fn start(&self) -> Result<()> {
        let mut status = self.status.write().await;
        
        if !matches!(*status, BackendStatus::Stopped | BackendStatus::Crashed(_)) {
            return Err(anyhow!("Backend is already running or starting"));
        }

        *status = BackendStatus::Starting;
        drop(status);

        tracing::info!(target: "orchestration", "🚀 Starting Go backend: {}", self.config.binary_path);

        // Build command
        let mut cmd = Command::new(&self.config.binary_path);
        
        if let Some(ref working_dir) = self.config.working_dir {
            cmd.current_dir(working_dir);
        }

        // Redirect stdout/stderr to null to avoid blocking
        cmd.stdout(Stdio::null()).stderr(Stdio::null());

        // Spawn process
        let child = cmd
            .spawn()
            .context("Failed to spawn Go backend process")?;

        let pid = child.id();
        tracing::info!(target: "orchestration", "✅ Go backend started with PID: {}", pid);

        // Store process handle
        let mut process = self.process.write().await;
        *process = Some(child);
        drop(process);

        // Record start time
        let mut start_time = self.start_time.write().await;
        *start_time = Some(Instant::now());
        drop(start_time);

        // Wait a bit for the process to initialize
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Perform initial health check with retries
        let health_status = self.health_checker.check_with_retries(5, 1000).await;

        let mut status = self.status.write().await;
        match health_status {
            HealthStatus::Healthy => {
                *status = BackendStatus::Running;
                tracing::info!(target: "orchestration", "✅ Go backend is healthy and running");
                Ok(())
            }
            _ => {
                *status = BackendStatus::Unhealthy;
                tracing::warn!(target: "orchestration", "⚠️  Go backend started but health check failed");
                Ok(())
            }
        }
    }

    /// Stop the Go backend process
    pub async fn stop(&self) -> Result<()> {
        let mut status = self.status.write().await;
        
        if matches!(*status, BackendStatus::Stopped) {
            return Ok(());
        }

        *status = BackendStatus::Stopping;
        drop(status);

        tracing::info!(target: "orchestration", "🛑 Stopping Go backend...");

        let mut process = self.process.write().await;
        
        if let Some(mut child) = process.take() {
            // Try graceful shutdown first
            match child.kill() {
                Ok(_) => {
                    tracing::info!(target: "orchestration", "✅ Go backend process terminated");
                }
                Err(e) => {
                    tracing::error!(target: "orchestration", "❌ Failed to kill process: {}", e);
                }
            }

            // Wait for process to exit
            let _ = child.wait();
        }

        drop(process);

        // Update status
        let mut status = self.status.write().await;
        *status = BackendStatus::Stopped;

        // Clear start time
        let mut start_time = self.start_time.write().await;
        *start_time = None;

        tracing::info!(target: "orchestration", "✅ Go backend stopped");
        Ok(())
    }

    /// Restart the Go backend process
    pub async fn restart(&self) -> Result<()> {
        tracing::info!(target: "orchestration", "🔄 Restarting Go backend...");
        
        // Increment restart counter
        let mut restart_count = self.restart_count.write().await;
        *restart_count += 1;
        let count = *restart_count;
        drop(restart_count);

        tracing::info!(target: "orchestration", "📊 Restart attempt #{}", count);

        self.stop().await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
        self.start().await?;

        Ok(())
    }

    /// Get current backend status
    pub async fn get_status(&self) -> BackendStatus {
        self.status.read().await.clone()
    }

    /// Get backend information
    pub async fn get_info(&self) -> BackendInfo {
        let status = self.status.read().await.clone();
        let process = self.process.read().await;
        let pid = process.as_ref().map(|p| p.id());
        drop(process);

        let start_time = self.start_time.read().await;
        let uptime_secs = start_time.as_ref().map(|t| t.elapsed().as_secs());
        drop(start_time);

        let restart_count = *self.restart_count.read().await;

        let last_health_check = match self.health_checker.check().await {
            HealthStatus::Healthy => Some("healthy".to_string()),
            HealthStatus::Unhealthy(reason) => Some(format!("unhealthy: {}", reason)),
            HealthStatus::Unknown => Some("unknown".to_string()),
        };

        BackendInfo {
            status,
            pid,
            uptime_secs,
            restart_count,
            last_health_check,
        }
    }

    /// Start health monitoring task
    ///
    /// Returns a task handle that can be used to stop monitoring
    pub fn start_health_monitoring(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        let interval_secs = self.config.health_check_interval_secs;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
            
            loop {
                interval.tick().await;

                let status = self.get_status().await;
                
                // Only check health if backend is supposed to be running
                if !matches!(status, BackendStatus::Running | BackendStatus::Unhealthy) {
                    continue;
                }

                tracing::debug!(target: "orchestration", "🏥 Performing health check...");
                
                let health = self.health_checker.check().await;
                
                match health {
                    HealthStatus::Healthy => {
                        let mut current_status = self.status.write().await;
                        if matches!(*current_status, BackendStatus::Unhealthy) {
                            tracing::info!(target: "orchestration", "✅ Backend recovered to healthy state");
                        }
                        *current_status = BackendStatus::Running;
                    }
                    HealthStatus::Unhealthy(reason) => {
                        tracing::warn!(target: "orchestration", "⚠️  Backend unhealthy: {}", reason);
                        let mut current_status = self.status.write().await;
                        *current_status = BackendStatus::Unhealthy;
                        
                        // Auto-restart if enabled
                        if self.config.auto_restart {
                            let restart_count = *self.restart_count.read().await;
                            if restart_count < self.config.max_restart_attempts {
                                drop(current_status);
                                tracing::warn!(target: "orchestration", "🔄 Attempting auto-restart...");
                                if let Err(e) = self.restart().await {
                                    tracing::error!(target: "orchestration", "❌ Auto-restart failed: {}", e);
                                }
                            } else {
                                tracing::error!(
                                    target: "orchestration",
                                    "❌ Max restart attempts ({}) reached, giving up",
                                    self.config.max_restart_attempts
                                );
                            }
                        }
                    }
                    HealthStatus::Unknown => {
                        tracing::debug!(target: "orchestration", "❓ Health status unknown");
                    }
                }
            }
        })
    }

    /// Check if backend is running
    pub async fn is_running(&self) -> bool {
        matches!(
            self.get_status().await,
            BackendStatus::Running | BackendStatus::Unhealthy
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_orchestrator_creation() {
        let config = OrchestratorConfig::default();
        let orchestrator = BackendOrchestrator::new(config);
        
        let status = orchestrator.get_status().await;
        assert_eq!(status, BackendStatus::Stopped);
    }

    #[tokio::test]
    async fn test_backend_info() {
        let config = OrchestratorConfig::default();
        let orchestrator = BackendOrchestrator::new(config);
        
        let info = orchestrator.get_info().await;
        assert_eq!(info.status, BackendStatus::Stopped);
        assert_eq!(info.pid, None);
        assert_eq!(info.restart_count, 0);
    }
}

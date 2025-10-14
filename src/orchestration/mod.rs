/// 🎯 Go Backend Orchestration Module
///
/// Manages the lifecycle of the Go backend process including:
/// - Starting/stopping/restarting the backend
/// - Health monitoring
/// - Automatic crash recovery
/// - Process supervision

pub mod backend;
pub mod health;

pub use backend::{BackendOrchestrator, BackendStatus};
pub use health::HealthChecker;

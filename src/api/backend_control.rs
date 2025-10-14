/// 🎯 Backend Control API Endpoints
///
/// REST API for managing the Go backend process lifecycle

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use serde_json::json;

use crate::state::AppState;

/// Response for backend start/stop/restart operations
#[derive(Debug, Serialize)]
pub struct OperationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Start the Go backend
///
/// POST /api/v1/admin/backend/start
pub async fn start_backend(State(state): State<AppState>) -> impl IntoResponse {
    tracing::info!(target: "backend_control", "📡 Received request to start backend");

    if let Some(ref orchestrator) = state.backend_orchestrator {
        match orchestrator.start().await {
            Ok(_) => {
                let info = orchestrator.get_info().await;
                tracing::info!(target: "backend_control", "✅ Backend started successfully: PID={:?}", info.pid);
                
                (
                    StatusCode::OK,
                    Json(json!({
                        "success": true,
                        "message": "Backend started successfully",
                        "pid": info.pid,
                        "status": info.status
                    })),
                )
            }
            Err(e) => {
                tracing::error!(target: "backend_control", "❌ Failed to start backend: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "success": false,
                        "message": "Failed to start backend",
                        "error": e.to_string()
                    })),
                )
            }
        }
    } else {
        tracing::warn!(target: "backend_control", "⚠️  Backend orchestrator not initialized");
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "success": false,
                "message": "Backend orchestrator not available",
                "error": "Orchestration service not enabled"
            })),
        )
    }
}

/// Stop the Go backend
///
/// POST /api/v1/admin/backend/stop
pub async fn stop_backend(State(state): State<AppState>) -> impl IntoResponse {
    tracing::info!(target: "backend_control", "📡 Received request to stop backend");

    if let Some(ref orchestrator) = state.backend_orchestrator {
        match orchestrator.stop().await {
            Ok(_) => {
                tracing::info!(target: "backend_control", "✅ Backend stopped successfully");
                (
                    StatusCode::OK,
                    Json(json!({
                        "success": true,
                        "message": "Backend stopped successfully"
                    })),
                )
            }
            Err(e) => {
                tracing::error!(target: "backend_control", "❌ Failed to stop backend: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "success": false,
                        "message": "Failed to stop backend",
                        "error": e.to_string()
                    })),
                )
            }
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "success": false,
                "message": "Backend orchestrator not available"
            })),
        )
    }
}

/// Restart the Go backend
///
/// POST /api/v1/admin/backend/restart
pub async fn restart_backend(State(state): State<AppState>) -> impl IntoResponse {
    tracing::info!(target: "backend_control", "📡 Received request to restart backend");

    if let Some(ref orchestrator) = state.backend_orchestrator {
        match orchestrator.restart().await {
            Ok(_) => {
                let info = orchestrator.get_info().await;
                tracing::info!(target: "backend_control", "✅ Backend restarted successfully");
                
                (
                    StatusCode::OK,
                    Json(json!({
                        "success": true,
                        "message": "Backend restarted successfully",
                        "restart_count": info.restart_count,
                        "pid": info.pid
                    })),
                )
            }
            Err(e) => {
                tracing::error!(target: "backend_control", "❌ Failed to restart backend: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "success": false,
                        "message": "Failed to restart backend",
                        "error": e.to_string()
                    })),
                )
            }
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "success": false,
                "message": "Backend orchestrator not available"
            })),
        )
    }
}

/// Get backend status
///
/// GET /api/v1/admin/backend/status
pub async fn get_backend_status(State(state): State<AppState>) -> impl IntoResponse {
    if let Some(ref orchestrator) = state.backend_orchestrator {
        let info = orchestrator.get_info().await;
        
        (
            StatusCode::OK,
            Json(json!({
                "status": info.status,
                "pid": info.pid,
                "uptime_secs": info.uptime_secs,
                "restart_count": info.restart_count,
                "last_health_check": info.last_health_check,
                "is_running": orchestrator.is_running().await
            })),
        )
    } else {
        (
            StatusCode::OK,
            Json(json!({
                "status": "disabled",
                "message": "Backend orchestration not enabled"
            })),
        )
    }
}

/// Health check endpoint for backend orchestrator itself
///
/// GET /api/v1/admin/backend/health
pub async fn backend_orchestrator_health(State(state): State<AppState>) -> impl IntoResponse {
    if state.backend_orchestrator.is_some() {
        (
            StatusCode::OK,
            Json(json!({
                "status": "ok",
                "service": "backend_orchestrator",
                "enabled": true
            })),
        )
    } else {
        (
            StatusCode::OK,
            Json(json!({
                "status": "ok",
                "service": "backend_orchestrator",
                "enabled": false
            })),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_response_serialization() {
        let response = OperationResponse {
            success: true,
            message: "Test".to_string(),
            error: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"success\":true"));
        assert!(!json.contains("error"));
    }
}

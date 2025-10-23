/// 📡 AI Insight WebSocket Endpoint
///
/// Provides real-time streaming of AI processing events to connected clients.
/// Clients can observe:
/// - Intent classification
/// - Entity extraction
/// - Handler routing and execution
/// - Processing metrics

use axum::{
    extract::{
        ws::{WebSocketUpgrade, WebSocket},
        Query, State,
    },
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct InsightQuery {
    /// Client identifier (e.g., user_id or session_id)
    pub client_id: Option<String>,
    /// JWT token (for authentication - optional, legacy frontend support)
    pub token: Option<String>,
    /// Channel name (e.g., ui_events - legacy frontend support)
    pub channel: Option<String>,
}

/// WebSocket endpoint for AI insights
///
/// # Example
/// ```
/// ws://localhost:8000/api/v1/insight?client_id=user123
/// ```
pub async fn ai_insight_ws(
    ws: WebSocketUpgrade,
    Query(params): Query<InsightQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Support both client_id and channel (for frontend compatibility)
    let client_id = params.client_id
        .or(params.channel.clone())
        .unwrap_or_else(|| format!("client_{}", uuid::Uuid::new_v4()));

    tracing::info!(
        "📡 AI Insight WebSocket upgrade request from: {} (channel: {:?}, token: {})",
        client_id,
        params.channel,
        params.token.is_some()
    );

    ws.on_upgrade(move |socket| handle_insight_socket(socket, client_id, state))
}

/// Handle individual WebSocket connection
async fn handle_insight_socket(socket: WebSocket, client_id: String, state: AppState) {
    tracing::info!("🔌 AI Insight WebSocket connected: {}", client_id);

    // Delegate to broadcaster
    state.insight_broadcaster.handle_connection(socket, client_id).await;
}

/// Health check endpoint for AI insight WebSocket
#[allow(dead_code)] // Used as fallback endpoint for WebSocket health checks
pub async fn ai_insight_health() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "service": "ai_insight_ws",
            "description": "WebSocket endpoint for real-time AI processing insights"
        })),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_parsing() {
        let query = InsightQuery {
            client_id: Some("user123".to_string()),
            token: None,
            channel: None,
        };

        assert_eq!(query.client_id.unwrap(), "user123");
    }
}

use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    Json,
};

use crate::state::AppState;

/// GET /metrics - Prometheus metrics endpoint
pub async fn prometheus_metrics(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let metrics = state.metrics.to_prometheus();
    
    (
        StatusCode::OK,
        [("Content-Type", "text/plain; version=0.0.4")],
        metrics,
    )
}

/// GET /admin/metrics - JSON metrics dashboard
pub async fn metrics_dashboard(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let metrics = state.metrics.to_json();
    Json(metrics)
}

/// GET /admin/metrics/intents - Intent-specific metrics
pub async fn intent_metrics(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let intents: Vec<serde_json::Value> = state.metrics.all_intents()
        .iter()
        .map(|intent| {
            serde_json::json!({
                "intent": intent,
                "count": state.metrics.get_intent_count(intent),
                "avg_response_time_ms": state.metrics.get_avg_response_time(intent)
                    .map(|d| d.as_millis())
                    .unwrap_or(0),
                "success_rate": state.metrics.get_success_rate(intent),
            })
        })
        .collect();

    Json(serde_json::json!({
        "intents": intents,
        "total": intents.len(),
    }))
}

/// GET /admin/metrics/stats - General statistics
pub async fn metrics_stats(
    State(state): State<AppState>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "total_requests": state.metrics.total_requests(),
        "uptime_seconds": state.metrics.uptime().as_secs(),
        "uptime_human": format_duration(state.metrics.uptime()),
        "intents_tracked": state.metrics.all_intents().len(),
    }))
}

/// Helper to format duration in human-readable format
fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

mod ai;
mod api;
mod config;
mod handlers;
mod models;
mod orchestration; // 🎯 Backend orchestration
mod state;
mod metrics;

use shuttle_axum::axum::{
    routing::{get, post},
    Router,
};
use shuttle_axum::ShuttleAxum;
use tower_http::cors::CorsLayer;

use crate::config::Config;
use crate::state::AppState;

#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    // Shuttle автоматически инициализирует tracing
    tracing::info!("🚀 FodiFood Intelligent Bot — запуск...");

    // === Конфигурация ===
    let config = Config::from_env();
    tracing::info!("✅ Конфигурация загружена");

    // === Общее состояние ===
    let state = AppState::new(config);

    // === Роутер ===
    let app = Router::new()
        // 🏠 Базовые endpoints
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        // 🌐 REST API v1
        .route("/api/v1/health", get(api::rest::health_check))
        .route("/api/v1/products", get(api::rest::get_products))
        // 🔐 Authentication
        .route("/api/v1/auth/login", post(api::rest::login_handler))
        .route("/api/v1/auth/register", post(api::rest::register_handler))
        // 👤 User Profile
        .route("/api/v1/user/profile", get(api::rest::get_user_profile))
        // 👨‍💼 Admin Endpoints
        .route("/api/v1/admin/stats", get(api::rest::get_admin_stats))
        .route(
            "/api/v1/admin/orders/recent",
            get(api::rest::get_recent_orders),
        )
        .route("/api/v1/admin/orders", get(api::rest::get_admin_orders))
        .route("/api/v1/admin/users", get(api::rest::get_admin_users))
        .route("/api/v1/admin/ws", get(api::admin_ws::admin_ws_handler))
        // 🎯 Backend Control Endpoints
        .route("/api/v1/admin/backend/start", post(api::backend_control::start_backend))
        .route("/api/v1/admin/backend/stop", post(api::backend_control::stop_backend))
        .route("/api/v1/admin/backend/restart", post(api::backend_control::restart_backend))
        .route("/api/v1/admin/backend/status", get(api::backend_control::get_backend_status))
        .route("/api/v1/admin/backend/health", get(api::backend_control::backend_orchestrator_health))
        // 📊 Metrics Endpoints
        .route("/metrics", get(api::metrics::prometheus_metrics))
        .route("/admin/metrics", get(api::metrics::metrics_dashboard))
        .route("/admin/metrics/intents", get(api::metrics::intent_metrics))
        .route("/admin/metrics/stats", get(api::metrics::metrics_stats))
        // �💬 Chat & AI
        .route("/api/v1/chat", post(api::rest::chat_handler))
        .route("/api/v1/search", get(api::rest::search_by_ingredient))
        .route(
            "/api/v1/recommendations",
            post(api::rest::get_recommendations),
        )
        .route("/api/v1/intents/{text}", get(api::rest::detect_intent))
        // 🔌 WebSocket & Webhooks
        .route("/ws", get(handlers::ws::websocket_handler))
        .route("/api/v1/insight", get(api::insight_ws::ai_insight_ws)) // 📡 AI Insights
        .route("/notify", post(handlers::webhook::webhook_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    tracing::info!("🤖 FodiFood Bot API запущен и готов!");
    tracing::info!("📡 REST API v1 доступен по адресу /api/v1/*");
    tracing::info!("👨‍💼 Admin endpoints: /api/v1/admin/*");

    Ok(app.into())
}

async fn root_handler() -> &'static str {
    "FodiFood Intelligent Bot API — WebSocket доступен по адресу /ws"
}

async fn health_handler() -> &'static str {
    "OK"
}

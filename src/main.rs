mod ai;
mod api;
mod config;
mod handlers;
mod models;
mod state;

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
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/ws", get(handlers::ws::websocket_handler))
        .route("/notify", post(handlers::webhook::webhook_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    tracing::info!("🤖 FodiFood Bot запущен и готов!");

    Ok(app.into())
}

async fn root_handler() -> &'static str {
    "FodiFood Intelligent Bot API — WebSocket доступен по адресу /ws"
}

async fn health_handler() -> &'static str {
    "OK"
}

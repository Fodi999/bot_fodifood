use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;

use fodifood_bot::{
    api, config::Config, handlers, state::AppState,
};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("🚀 Starting FodiFood Bot (Local Mode)...");

    // Load configuration
    let config = Config::from_env();
    tracing::info!("✅ Configuration loaded");
    tracing::info!("📡 Go Backend URL: {}", config.go_backend_url);

    // Initialize state
    let state = AppState::new(config);
    tracing::info!("✅ Application state initialized");
    tracing::info!("🧠 AI Engine ready with {} intent handlers", state.ai.registry_stats().0);
    tracing::info!("📊 Metrics collector initialized");

    // Build router
    let app = Router::new()
        // 🏠 Basic endpoints
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
        .route("/api/v1/admin/orders/recent", get(api::rest::get_recent_orders))
        .route("/api/v1/admin/orders", get(api::rest::get_admin_orders))
        .route("/api/v1/admin/users", get(api::rest::get_admin_users))
        .route("/api/v1/admin/ws", get(api::admin_ws::admin_ws_handler))
        
        // 📊 Metrics Endpoints
        .route("/metrics", get(api::metrics::prometheus_metrics))
        .route("/admin/metrics", get(api::metrics::metrics_dashboard))
        .route("/admin/metrics/intents", get(api::metrics::intent_metrics))
        .route("/admin/metrics/stats", get(api::metrics::metrics_stats))
        
        // 💬 Chat & AI
        .route("/api/v1/chat", post(api::rest::chat_handler))
        .route("/api/v1/search", get(api::rest::search_by_ingredient))
        .route("/api/v1/recommendations", post(api::rest::get_recommendations))
        .route("/api/v1/intents/{text}", get(api::rest::detect_intent))
        
        // 🔌 WebSocket & Webhooks
        .route("/ws", get(handlers::ws::websocket_handler))
        .route("/api/v1/insight", get(api::insight_ws::ai_insight_ws)) // 📡 AI Insights
        .route("/notify", post(handlers::webhook::webhook_handler))
        
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Bind to address
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    
    tracing::info!("🎯 Server listening on http://{}", addr);
    tracing::info!("");
    tracing::info!("📊 Metrics endpoints:");
    tracing::info!("   • Prometheus: http://{}/metrics", addr);
    tracing::info!("   • Dashboard:  http://{}/admin/metrics", addr);
    tracing::info!("   • Intents:    http://{}/admin/metrics/intents", addr);
    tracing::info!("   • Stats:      http://{}/admin/metrics/stats", addr);
    tracing::info!("");
    tracing::info!("💬 Chat API:      http://{}/api/v1/chat", addr);
    tracing::info!("🔌 WebSocket:     ws://{}/ws", addr);
    tracing::info!("");

    // Start server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    
    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

async fn root_handler() -> &'static str {
    "🍱 FodiFood Bot API - Running locally!"
}

async fn health_handler() -> &'static str {
    "OK"
}

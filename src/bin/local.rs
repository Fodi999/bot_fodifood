use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;
use std::sync::Arc;

use fodifood_bot::{
    api, config::Config, handlers, state::AppState,
    bank, nft, wallet, // 💰 🧩 🔐 Token modules
};
use fodifood_bot::orchestration::{BackendOrchestrator, backend::OrchestratorConfig};

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
    let mut state = AppState::new(config.clone());
    
    // Initialize Backend Orchestrator if enabled
    if config.orchestrator_enabled {
        tracing::info!("🎯 Backend Orchestrator enabled");
        
        let orchestrator_config = OrchestratorConfig {
            binary_path: config.go_backend_bin.clone(),
            working_dir: Some(".".to_string()),
            base_url: config.go_backend_url.clone(),
            health_check_interval_secs: 30,
            health_check_timeout_secs: 5,
            auto_restart: config.orchestrator_managed,
            max_restart_attempts: 3,
        };
        
        let orchestrator = Arc::new(BackendOrchestrator::new(orchestrator_config));
        state.backend_orchestrator = Some(orchestrator.clone());
        
        // Start health monitoring if managed
        if config.orchestrator_managed {
            tracing::info!("🔄 Auto-management enabled - will monitor and restart backend");
            let orch_clone = orchestrator.clone();
            tokio::spawn(async move {
                let _ = orch_clone.start_health_monitoring().await;
            });
        } else {
            tracing::info!("📊 Monitoring only - backend managed externally");
        }
        
        tracing::info!("✅ Backend Orchestrator initialized");
    } else {
        tracing::info!("⚠️  Backend Orchestrator disabled (set ORCHESTRATOR_ENABLED=true to enable)");
    }
    
    tracing::info!("✅ Application state initialized");
    tracing::info!("🧠 AI Engine ready with {} intent handlers", state.ai.registry_stats().0);
    tracing::info!("📊 Metrics collector initialized");

    // Initialize Solana client if configured
    if let Ok(solana_rpc) = std::env::var("SOLANA_RPC_URL") {
        if let Ok(keypair_path) = std::env::var("FODI_TREASURY_KEYPAIR") {
            match fodifood_bot::solana::SolanaClient::new(&solana_rpc, &keypair_path) {
                Ok(solana_client) => {
                    tracing::info!("✅ Solana client initialized: {}", solana_rpc);
                    state = state.with_solana(solana_client);
                }
                Err(e) => {
                    tracing::warn!("⚠️ Failed to initialize Solana client: {}", e);
                    tracing::warn!("   Solana API will be disabled");
                }
            }
        } else {
            tracing::warn!("⚠️ FODI_TREASURY_KEYPAIR not set, Solana API disabled");
        }
    } else {
        tracing::info!("ℹ️  SOLANA_RPC_URL not set, running without blockchain integration");
    }

    // Build router
    let app = Router::new()
        // 🏠 Basic endpoints
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        
        // 🌐 REST API v1
        .route("/api/v1/health", get(api::rest::health_check))
        .route("/api/v1/products", get(api::rest::get_products))
        .merge(api::businesses::routes()) // 💼 Business proxy
        .merge(api::user::routes()) // 👤 User management
        
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
        .route("/api/v1/admin/command", post(api::rest::admin_command_handler)) // 🤖 Admin AI
        
        // 🎯 Backend Control Endpoints
        .route("/api/v1/admin/backend/start", post(api::backend_control::start_backend))
        .route("/api/v1/admin/backend/stop", post(api::backend_control::stop_backend))
        .route("/api/v1/admin/backend/restart", post(api::backend_control::restart_backend))
        .route("/api/v1/admin/backend/status", get(api::backend_control::get_backend_status))
        .route("/api/v1/admin/backend/health", post(api::backend_control::backend_orchestrator_health))
        
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
        
        // 💠 Solana Blockchain API (before .with_state)
        .merge(api::solana::routes())
        
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Create shared ledger for bank and wallet
    let shared_ledger = Arc::new(
        bank::ledger::TokenLedger::with_persistence("data/fodi_ledger.db")
            .unwrap_or_else(|_| bank::ledger::TokenLedger::new())
    );

    // Create shared wallet database connection (used by wallet and NFT modules)
    let wallet_db = Arc::new(
        sled::open("data/wallets.db")
            .expect("Failed to open wallet database")
    );

    tracing::info!("💾 Shared wallet database initialized");

    // Add bank, wallet, and NFT routes with shared connections
    let app = app
        .nest("/api/bank", bank::api::routes_with_ledger(shared_ledger.clone()))
        .nest("/api/wallet", wallet::api::routes(shared_ledger, wallet_db.clone()))
        .nest("/api/nft", nft::api::routes(wallet_db));

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
    tracing::info!("💰 Bank API:      http://{}/api/bank/*", addr);
    tracing::info!("🔐 Wallet API:    http://{}/api/wallet/*", addr);
    tracing::info!("🧩 NFT API:       http://{}/api/nft/*", addr);
    tracing::info!("💠 Solana API:    http://{}/api/solana/*", addr);
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

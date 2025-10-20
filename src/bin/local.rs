use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;
use std::sync::Arc;

use fodifood_bot::{
    api, config::Config, handlers, state::AppState,
    bank, nft, wallet, // 💰 🧩 🔐 Token modules
    ai::{
        agent_manager::{AgentManager, AgentType},
        persistent_memory::PersistentMemory,
    },
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

    // Initialize Multi-Agent AI System
    tracing::info!("🤖 Initializing Multi-Agent AI System...");
    
    let memory = Arc::new(PersistentMemory::new("./data/local_agents.db").unwrap());
    let mut agent_manager = AgentManager::new(memory.clone()).await.unwrap();
    agent_manager.enable_shared_bus().await.unwrap();
    
    // Create specialized agents with automatic subscriptions
    let agents = [
        ("INV-LOCAL-001", AgentType::Investor, "💰 Investment Advisor", vec!["coordination", "investment_opportunities", "market_analysis"]),
        ("BIZ-LOCAL-001", AgentType::Business, "🏢 Business Strategist", vec!["coordination", "business_insights", "growth_campaigns"]),
        ("USER-LOCAL-001", AgentType::User, "👤 User Experience Agent", vec!["coordination", "user_interactions", "personalization"]),
        ("SYS-LOCAL-001", AgentType::System, "⚙️ System Administrator", vec!["coordination", "system_alerts", "admin_tasks"]),
    ];

    for (id, agent_type, description, topics) in &agents {
        agent_manager.create_agent(agent_type.clone(), id, None).await.unwrap();
        
        // Subscribe agent to its topics
        if let Some(bus) = agent_manager.get_shared_bus() {
            for topic in topics {
                match bus.subscribe(id, vec![topic.to_string()]).await {
                    Ok(_) => tracing::info!("📡 Agent {} subscribed to topic '{}'", id, topic),
                    Err(e) => tracing::warn!("⚠️ Failed to subscribe {} to '{}': {}", id, topic, e),
                }
            }
        }
        
        tracing::info!("✅ Created {}: {} with {} subscriptions", description, id, topics.len());
    }
    
    tracing::info!("🚌 Multi-Agent system with shared bus ready");

    // Initialize state with agent manager
    let mut state = AppState::new(config.clone()).with_agent_manager(Arc::new(agent_manager));
    
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
        
        // 🤖 Multi-Agent System Endpoints
        .route("/api/v1/admin/agents", get(agent_list_handler))
        .route("/api/v1/admin/agents/stats", get(agent_stats_handler))
        .route("/api/v1/admin/agents/bus", get(shared_bus_stats_handler))
        .route("/api/v1/admin/agents/coordinate", post(agent_coordinate_handler))
        .route("/api/v1/admin/agents/subscribe", post(agent_subscribe_handler))
        
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
    tracing::info!("🤖 Multi-Agent System:");
    tracing::info!("   • Agents List:  http://{}/api/v1/admin/agents", addr);
    tracing::info!("   • Agent Stats:  http://{}/api/v1/admin/agents/stats", addr);
    tracing::info!("   • Bus Stats:    http://{}/api/v1/admin/agents/bus", addr);
    tracing::info!("   • Subscribe:    POST http://{}/api/v1/admin/agents/subscribe", addr);
    tracing::info!("   • Coordinate:   POST http://{}/api/v1/admin/agents/coordinate", addr);
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

/// Subscribe agent to topics
async fn agent_subscribe_handler(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if let Some(agent_manager) = &state.agent_manager {
        let agent_id = payload.get("agent_id").and_then(|s| s.as_str()).unwrap_or("");
        let topics = payload.get("topics")
            .and_then(|t| t.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_else(Vec::new);
        
        if agent_id.is_empty() || topics.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "agent_id and topics are required".to_string()));
        }

        if let Some(bus) = agent_manager.get_shared_bus() {
            match bus.subscribe(agent_id, topics.clone()).await {
                Ok(_) => {
                    Ok(Json(serde_json::json!({
                        "status": "subscribed",
                        "agent_id": agent_id,
                        "topics": topics,
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    })))
                }
                Err(e) => {
                    Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Subscription failed: {}", e)))
                }
            }
        } else {
            Err((StatusCode::SERVICE_UNAVAILABLE, "SharedBus not enabled".to_string()))
        }
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, "Multi-Agent system not initialized".to_string()))
    }
}

async fn health_handler() -> &'static str {
    "OK"
}

// 🤖 Agent Management Handlers

/// List all active agents
async fn agent_list_handler(
    State(state): State<AppState>
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if let Some(agent_manager) = &state.agent_manager {
        let agent_ids = agent_manager.list_agents().await;
        let agent_info: Vec<serde_json::Value> = agent_ids.into_iter().map(|agent_id| {
            serde_json::json!({
                "id": agent_id,
                "status": "active"
            })
        }).collect();

        Ok(Json(serde_json::json!({
            "agents": agent_info,
            "total": agent_info.len(),
            "timestamp": chrono::Utc::now().to_rfc3339()
        })))
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, "Multi-Agent system not initialized".to_string()))
    }
}

/// Get agent system statistics
async fn agent_stats_handler(
    State(state): State<AppState>
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if let Some(agent_manager) = &state.agent_manager {
        let agent_ids = agent_manager.list_agents().await;

        Ok(Json(serde_json::json!({
            "total_agents": agent_ids.len(),
            "system_status": "operational",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })))
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, "Multi-Agent system not initialized".to_string()))
    }
}

/// Get SharedBus statistics
async fn shared_bus_stats_handler(
    State(state): State<AppState>
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if let Some(agent_manager) = &state.agent_manager {
        if let Some(bus) = agent_manager.get_shared_bus() {
            let stats = bus.get_stats().await;
            Ok(Json(serde_json::json!({
                "total_messages": stats.total_messages,
                "active_subscriptions": stats.active_subscriptions,
                "messages_per_topic": stats.messages_per_topic,
                "avg_processing_time_ms": stats.avg_processing_time_ms,
                "uptime_seconds": stats.uptime_seconds,
                "last_activity": stats.last_activity,
                "timestamp": chrono::Utc::now().to_rfc3339()
            })))
        } else {
            Err((StatusCode::SERVICE_UNAVAILABLE, "SharedBus not enabled".to_string()))
        }
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, "Multi-Agent system not initialized".to_string()))
    }
}

/// Trigger agent coordination
async fn agent_coordinate_handler(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if let Some(agent_manager) = &state.agent_manager {
        let coordinator = payload.get("coordinator").and_then(|s| s.as_str()).unwrap_or("system");
        let task_id = payload.get("task_id").and_then(|s| s.as_str()).unwrap_or("demo-task");
        let action = payload.get("action").and_then(|s| s.as_str()).unwrap_or("general_coordination");
        let participants = payload.get("participants")
            .and_then(|p| p.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_else(|| vec!["INV-LOCAL-001".to_string(), "BIZ-LOCAL-001".to_string()]);
        
        match agent_manager.coordinate_agents(coordinator, task_id, action, participants.clone()).await {
            Ok(_) => {
                Ok(Json(serde_json::json!({
                    "status": "coordination_initiated",
                    "coordinator": coordinator,
                    "task_id": task_id,
                    "action": action,
                    "participants": participants,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                })))
            }
            Err(e) => {
                Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Coordination failed: {}", e)))
            }
        }
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, "Multi-Agent system not initialized".to_string()))
    }
}

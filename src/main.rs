use fodifood_bot::{api, config, handlers, state};
// Note: bank, nft, wallet, solana modules available in local mode (src/bin/local.rs)

use shuttle_axum::axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use shuttle_axum::ShuttleAxum;
use shuttle_runtime::SecretStore;
use tower_http::cors::CorsLayer;

use config::Config;
use state::AppState;

// Import for agent handlers
use fodifood_bot::ai::shared_bus::{BusMessage, MessageType};
use fodifood_bot::ai::{
    agent_manager::{AgentManager, AgentType},
    persistent_memory::PersistentMemory,
};
use std::sync::Arc;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleAxum {
    // Shuttle автоматически инициализирует tracing
    tracing::info!("🚀 FodiFood Intelligent Bot — запуск...");

    // Set environment variables from Shuttle Secrets
    if let Some(go_backend_url) = secrets.get("GO_BACKEND_URL") {
        std::env::set_var("GO_BACKEND_URL", go_backend_url);
        tracing::info!("✅ GO_BACKEND_URL loaded from Shuttle Secrets");
    }
    if let Some(jwt_secret) = secrets.get("JWT_SECRET") {
        std::env::set_var("JWT_SECRET", jwt_secret);
    }
    if let Some(openai_key) = secrets.get("OPENAI_API_KEY") {
        std::env::set_var("OPENAI_API_KEY", openai_key);
    }
    if let Some(groq_key) = secrets.get("GROQ_API_KEY") {
        std::env::set_var("GROQ_API_KEY", groq_key);
        tracing::info!("✅ GROQ_API_KEY loaded from Shuttle Secrets");
    }
    if let Some(groq_model) = secrets.get("GROQ_MODEL") {
        std::env::set_var("GROQ_MODEL", groq_model);
    } else {
        // Default model if not specified
        std::env::set_var("GROQ_MODEL", "llama-3.1-8b-instant");
    }
    if let Some(orchestrator_enabled) = secrets.get("ORCHESTRATOR_ENABLED") {
        tracing::info!("✅ ORCHESTRATOR_ENABLED = {}", orchestrator_enabled);
        std::env::set_var("ORCHESTRATOR_ENABLED", orchestrator_enabled);
    }
    if let Some(orchestrator_managed) = secrets.get("ORCHESTRATOR_MANAGED") {
        tracing::info!("✅ ORCHESTRATOR_MANAGED = {}", orchestrator_managed);
        std::env::set_var("ORCHESTRATOR_MANAGED", orchestrator_managed);
    }
    if let Some(database_url) = secrets.get("DATABASE_URL") {
        std::env::set_var("DATABASE_URL", database_url);
        tracing::info!("✅ DATABASE_URL loaded");
    }

    // === Конфигурация ===
    let config = Config::from_env();
    tracing::info!("✅ Конфигурация загружена");

    // === Общее состояние ===
    let mut state = AppState::new(config.clone());

    // === Инициализация Multi-Agent системы (если включена) ===
    if config.orchestrator_enabled {
        tracing::info!("🤖 Initializing Multi-Agent AI System...");
        
        match PersistentMemory::new("/tmp/shuttle_agents.db") {
            Ok(memory) => {
                let memory = Arc::new(memory);
                match AgentManager::new(memory.clone()).await {
                    Ok(mut agent_manager) => {
                        // Enable SharedBus
                        if let Err(e) = agent_manager.enable_shared_bus().await {
                            tracing::error!("Failed to enable SharedBus: {}", e);
                        } else {
                            // Create production agents
                            let agents = [
                                ("INV-PROD-001", AgentType::Investor, "💰 Investment Advisor", vec!["coordination", "investment_opportunities", "market_analysis"]),
                                ("BIZ-PROD-001", AgentType::Business, "🏢 Business Strategist", vec!["coordination", "business_insights", "growth_campaigns"]),
                                ("USER-PROD-001", AgentType::User, "👤 User Experience Agent", vec!["coordination", "user_interactions", "personalization"]),
                                ("SYS-PROD-001", AgentType::System, "⚙️ System Administrator", vec!["coordination", "system_alerts", "admin_tasks"]),
                            ];

                            for (id, agent_type, description, topics) in &agents {
                                if let Ok(_) = agent_manager.create_agent(agent_type.clone(), id, None).await {
                                    // Subscribe agent to its topics
                                    if let Some(bus) = agent_manager.get_shared_bus() {
                                        for topic in topics {
                                            if let Ok(_) = bus.subscribe(id, vec![topic.to_string()]).await {
                                                tracing::info!("📡 Agent {} subscribed to topic '{}'", id, topic);
                                            }
                                        }
                                    }
                                    tracing::info!("✅ Created {}: {}", description, id);
                                }
                            }
                            
                            state.agent_manager = Some(Arc::new(agent_manager));
                            tracing::info!("🚌 Multi-Agent system with shared bus ready");
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to create AgentManager: {}", e);
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed to create PersistentMemory: {}", e);
            }
        }
    } else {
        tracing::info!("⚠️  Multi-Agent system disabled (set ORCHESTRATOR_ENABLED=true in Secrets.toml)");
    }

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
        // 💼 Business Management - merged routes from businesses module
        .merge(api::businesses::routes())
        // Note: Solana API available in local mode only
        // 👨‍💼 Admin Endpoints
        .route("/api/v1/admin/stats", get(api::rest::get_admin_stats))
        .route(
            "/api/v1/admin/orders/recent",
            get(api::rest::get_recent_orders),
        )
        .route("/api/v1/admin/orders", get(api::rest::get_admin_orders))
        .route("/api/v1/admin/users", get(api::rest::get_admin_users))
        .route("/api/v1/admin/ws", get(api::admin_ws::admin_ws_handler))
        .route("/api/v1/admin/command", post(api::rest::admin_command_handler)) // 🤖 Admin AI
        // 🤖 Multi-Agent System Endpoints
        .route("/api/v1/admin/agents", get(agent_list_handler))
        .route("/api/v1/admin/agents/stats", get(agent_stats_handler))
        .route("/api/v1/admin/agents/bus", get(shared_bus_stats_handler))
        .route("/api/v1/admin/agents/coordinate", post(agent_coordinate_handler))
        .route("/api/v1/admin/agents/subscribe", post(agent_subscribe_handler))
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
    // Note: Bank & NFT APIs available in local mode only (src/bin/local.rs)

    Ok(app.into())
}

async fn root_handler() -> &'static str {
    "FodiFood Intelligent Bot API — WebSocket доступен по адресу /ws"
}

async fn health_handler() -> &'static str {
    "OK"
}

// 🤖 Multi-Agent System Handlers

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
        // Return empty list if agent system not initialized
        Ok(Json(serde_json::json!({
            "agents": [],
            "total": 0,
            "message": "Multi-Agent system not initialized on Shuttle deployment",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })))
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
        Ok(Json(serde_json::json!({
            "total_agents": 0,
            "system_status": "not_initialized",
            "message": "Multi-Agent system available in local mode only",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })))
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
                "status": "operational",
                "timestamp": chrono::Utc::now().to_rfc3339()
            })))
        } else {
            Err((StatusCode::SERVICE_UNAVAILABLE, "SharedBus not enabled".to_string()))
        }
    } else {
        Ok(Json(serde_json::json!({
            "total_messages": 0,
            "active_subscriptions": 0,
            "status": "not_initialized",
            "message": "SharedBus available in local mode only",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })))
    }
}

/// Coordinate agents (send message to SharedBus)
async fn agent_coordinate_handler(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if let Some(agent_manager) = &state.agent_manager {
        let topic = payload.get("topic").and_then(|t| t.as_str()).unwrap_or("general");
        let message_text = payload.get("message").and_then(|m| m.as_str()).unwrap_or("");
        let sender = payload.get("sender").and_then(|s| s.as_str()).unwrap_or("system");

        if message_text.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "message is required".to_string()));
        }

        if let Some(bus) = agent_manager.get_shared_bus() {
            // Create BusMessage
            let bus_message = BusMessage {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: chrono::Utc::now(),
                from_agent: sender.to_string(),
                to_agent: None,
                topic: topic.to_string(),
                message_type: MessageType::Event,
                payload: serde_json::json!({
                    "text": message_text,
                    "source": "admin_api"
                }),
                priority: 5,
                requires_ack: false,
                ttl_seconds: Some(3600),
            };

            match bus.publish(bus_message).await {
                Ok(_) => {
                    Ok(Json(serde_json::json!({
                        "status": "published",
                        "topic": topic,
                        "sender": sender,
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    })))
                }
                Err(e) => {
                    Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Publish failed: {}", e)))
                }
            }
        } else {
            Err((StatusCode::SERVICE_UNAVAILABLE, "SharedBus not enabled".to_string()))
        }
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, "Multi-Agent system not initialized".to_string()))
    }
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

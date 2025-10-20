use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::ai::AIEngine;
use crate::api::go_backend::GoBackendClient;
use crate::config::Config;
use crate::metrics::MetricsCollector; // 📊 Metrics
use crate::handlers::InsightBroadcaster; // 📡 WebSocket Insights
use crate::solana::SolanaClient; // 🪙 Solana blockchain

// Import orchestrator
use crate::orchestration::BackendOrchestrator;

pub type ClientId = String;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)] // Может использоваться в будущих фичах
    pub config: Config,
    pub connections: Arc<DashMap<ClientId, ClientConnection>>,
    pub backend: Arc<GoBackendClient>,
    pub ai: Arc<AIEngine>, // 🧠 AI движок
    pub metrics: Arc<MetricsCollector>, // 📊 Metrics collector
    pub insight_broadcaster: InsightBroadcaster, // 📡 AI Insight broadcaster
    pub backend_orchestrator: Option<Arc<BackendOrchestrator>>, // 🎯 Backend lifecycle manager
    pub solana: Option<SolanaClient>, // 🪙 Solana blockchain (optional for graceful degradation)
    pub agent_manager: Option<Arc<crate::ai::AgentManager>>, // 🤖 Multi-Agent system
}

pub struct ClientConnection {
    #[allow(dead_code)]
    pub user_id: String,
    pub role: String,
    pub tx: mpsc::UnboundedSender<String>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let backend = Arc::new(GoBackendClient::new(&config));
        let ai = Arc::new(AIEngine::new(&config)); // 🧠 Создаём AI с config
        let metrics = Arc::new(MetricsCollector::new()); // 📊 Создаём metrics
        let insight_broadcaster = InsightBroadcaster::new(); // 📡 Создаём broadcaster

        Self {
            config,
            connections: Arc::new(DashMap::new()),
            backend,
            ai, // 🧠 Добавляем AI
            metrics, // 📊 Добавляем metrics
            insight_broadcaster, // 📡 Добавляем insight broadcaster
            backend_orchestrator: None, // 🎯 Оркестратор добавляется опционально
            solana: None, // 🪙 Solana будет добавлен через with_solana()
            agent_manager: None, // 🤖 Multi-Agent system добавляется опционально
        }
    }

    /// 🪙 Add Solana blockchain client (builder pattern)
    pub fn with_solana(mut self, solana: SolanaClient) -> Self {
        self.solana = Some(solana);
        self
    }

    /// 🤖 Add Multi-Agent system (builder pattern)
    pub fn with_agent_manager(mut self, agent_manager: Arc<crate::ai::AgentManager>) -> Self {
        self.agent_manager = Some(agent_manager);
        self
    }

    /// Broadcast message to all admins
    pub fn broadcast_to_admins(&self, message: &str) {
        for entry in self.connections.iter() {
            if entry.value().role == "admin" || entry.value().role == "manager" {
                let _ = entry.value().tx.send(message.to_string());
            }
        }
    }

    /// Send message to specific user
    pub fn send_to_user(&self, user_id: &str, message: &str) {
        if let Some(conn) = self.connections.get(user_id) {
            let _ = conn.tx.send(message.to_string());
        }
    }

    /// Get active connections count by role
    #[allow(dead_code)]
    pub fn get_connections_by_role(&self, role: &str) -> usize {
        self.connections
            .iter()
            .filter(|entry| entry.value().role == role)
            .count()
    }
}

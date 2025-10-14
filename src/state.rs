use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::ai::AIEngine;
use crate::api::go_backend::GoBackendClient;
use crate::config::Config;
use crate::models::user::UserRole;
use crate::metrics::MetricsCollector; // 📊 Metrics

pub type ClientId = String;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)] // Может использоваться в будущих фичах
    pub config: Config,
    pub connections: Arc<DashMap<ClientId, ClientConnection>>,
    pub backend: Arc<GoBackendClient>,
    pub ai: Arc<AIEngine>, // 🧠 AI движок
    pub metrics: Arc<MetricsCollector>, // 📊 Metrics collector
}

pub struct ClientConnection {
    #[allow(dead_code)]
    pub user_id: String,
    pub role: UserRole,
    pub tx: mpsc::UnboundedSender<String>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let backend = Arc::new(GoBackendClient::new(&config));
        let ai = Arc::new(AIEngine::new(&config)); // 🧠 Создаём AI с config
        let metrics = Arc::new(MetricsCollector::new()); // 📊 Создаём metrics

        Self {
            config,
            connections: Arc::new(DashMap::new()),
            backend,
            ai, // 🧠 Добавляем AI
            metrics, // 📊 Добавляем metrics
        }
    }

    /// Broadcast message to all admins
    pub fn broadcast_to_admins(&self, message: &str) {
        for entry in self.connections.iter() {
            if matches!(entry.value().role, UserRole::Admin | UserRole::Manager) {
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
    pub fn get_connections_by_role(&self, role: UserRole) -> usize {
        self.connections
            .iter()
            .filter(|entry| entry.value().role == role)
            .count()
    }
}

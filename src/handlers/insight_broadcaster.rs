/// 📡 AI Insight Broadcaster
///
/// Broadcasts AI processing events to all connected WebSocket clients.
/// Provides real-time visibility into AI pipeline execution.

use axum::extract::ws::{Message, WebSocket};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::mpsc;

use super::insight_events::AIInsightEvent;

/// 📡 Insight Broadcaster
#[derive(Clone)]
pub struct InsightBroadcaster {
    /// Connected clients (user_id -> sender)
    clients: Arc<DashMap<String, mpsc::UnboundedSender<AIInsightEvent>>>,
}

impl InsightBroadcaster {
    /// Create a new broadcaster
    pub fn new() -> Self {
        tracing::info!("📡 Creating AI Insight Broadcaster");
        Self {
            clients: Arc::new(DashMap::new()),
        }
    }

    /// Register a new WebSocket client
    pub fn register_client(&self, client_id: String, sender: mpsc::UnboundedSender<AIInsightEvent>) {
        tracing::info!("📡 Registering client: {}", client_id);
        self.clients.insert(client_id.clone(), sender);
        tracing::info!("📊 Active clients: {}", self.clients.len());
    }

    /// Unregister a WebSocket client
    pub fn unregister_client(&self, client_id: &str) {
        tracing::info!("📡 Unregistering client: {}", client_id);
        self.clients.remove(client_id);
        tracing::info!("📊 Active clients: {}", self.clients.len());
    }

    /// Broadcast event to all connected clients
    pub fn broadcast(&self, event: AIInsightEvent) {
        let event_type = match &event {
            AIInsightEvent::IntentClassificationStarted { .. } => "classification_started",
            AIInsightEvent::IntentClassified { .. } => "classified",
            AIInsightEvent::EntityExtraction { .. } => "entity_extraction",
            AIInsightEvent::HandlerRouting { .. } => "handler_routing",
            AIInsightEvent::HandlerExecutionStarted { .. } => "handler_started",
            AIInsightEvent::HandlerExecutionCompleted { .. } => "handler_completed",
            AIInsightEvent::ContextUpdated { .. } => "context_updated",
            AIInsightEvent::ProcessingCompleted { .. } => "processing_completed",
            AIInsightEvent::ProcessingError { .. } => "processing_error",
        };

        tracing::debug!("📡 Broadcasting event: {} to {} clients", event_type, self.clients.len());

        let mut dead_clients = Vec::new();

        for entry in self.clients.iter() {
            let client_id = entry.key();
            let sender = entry.value();

            if let Err(e) = sender.send(event.clone()) {
                tracing::warn!("❌ Failed to send to client {}: {}", client_id, e);
                dead_clients.push(client_id.clone());
            }
        }

        // Clean up disconnected clients
        for client_id in dead_clients {
            self.unregister_client(&client_id);
        }
    }

    /// Broadcast to specific user only
    #[allow(dead_code)] // Will be used for user-specific notifications
    pub fn broadcast_to_user(&self, user_id: &str, event: AIInsightEvent) {
        if let Some(sender) = self.clients.get(user_id) {
            if let Err(e) = sender.send(event) {
                tracing::warn!("❌ Failed to send to user {}: {}", user_id, e);
                self.unregister_client(user_id);
            }
        } else {
            tracing::debug!("📡 User {} not connected, skipping broadcast", user_id);
        }
    }

    /// Get number of connected clients
    #[allow(dead_code)] // Used for monitoring and debugging
    pub fn client_count(&self) -> usize {
        self.clients.len()
    }

    /// Get list of connected client IDs
    #[allow(dead_code)] // Used for admin dashboard
    pub fn connected_clients(&self) -> Vec<String> {
        self.clients.iter().map(|entry| entry.key().clone()).collect()
    }

    /// Handle WebSocket connection for a client
    pub async fn handle_connection(&self, mut socket: WebSocket, client_id: String) {
        tracing::info!("🔌 New insight connection from: {}", client_id);

        // Create channel for this client
        let (tx, mut rx) = mpsc::unbounded_channel::<AIInsightEvent>();

        // Register client
        self.register_client(client_id.clone(), tx);

        // Send welcome message
        let welcome = serde_json::json!({
            "type": "connected",
            "client_id": client_id,
            "message": "AI Insight stream connected",
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        if let Ok(welcome_json) = serde_json::to_string(&welcome) {
            if let Err(e) = socket.send(Message::Text(welcome_json.into())).await {
                tracing::error!("❌ Failed to send welcome message: {}", e);
                self.unregister_client(&client_id);
                return;
            }
        }

        // Handle incoming messages and outgoing events
        loop {
            tokio::select! {
                // Receive events from broadcaster
                Some(event) = rx.recv() => {
                    match event.to_json() {
                        Ok(json) => {
                            if let Err(e) = socket.send(Message::Text(json.into())).await {
                                tracing::error!("❌ Failed to send event to {}: {}", client_id, e);
                                break;
                            }
                        }
                        Err(e) => {
                            tracing::error!("❌ Failed to serialize event: {}", e);
                        }
                    }
                }

                // Receive messages from client
                Some(msg) = socket.recv() => {
                    match msg {
                        Ok(Message::Text(text)) => {
                            tracing::debug!("📨 Received from {}: {}", client_id, text);

                            // Handle client commands (e.g., subscribe/unsubscribe)
                            if text == "ping" {
                                let pong = serde_json::json!({
                                    "type": "pong",
                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                });
                                if let Ok(pong_json) = serde_json::to_string(&pong) {
                                    let _ = socket.send(Message::Text(pong_json.into())).await;
                                }
                            }
                        }
                        Ok(Message::Close(_)) => {
                            tracing::info!("🔌 Client {} closing connection", client_id);
                            break;
                        }
                        Ok(Message::Ping(data)) => {
                            let _ = socket.send(Message::Pong(data)).await;
                        }
                        Err(e) => {
                            tracing::error!("❌ WebSocket error from {}: {}", client_id, e);
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Cleanup
        self.unregister_client(&client_id);
        tracing::info!("🔌 Client {} disconnected", client_id);
    }
}

impl Default for InsightBroadcaster {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_broadcaster_creation() {
        let broadcaster = InsightBroadcaster::new();
        assert_eq!(broadcaster.client_count(), 0);
    }

    #[tokio::test]
    async fn test_client_registration() {
        let broadcaster = InsightBroadcaster::new();
        let (tx, _rx) = mpsc::unbounded_channel();

        broadcaster.register_client("user123".to_string(), tx);
        assert_eq!(broadcaster.client_count(), 1);
        assert!(broadcaster.connected_clients().contains(&"user123".to_string()));

        broadcaster.unregister_client("user123");
        assert_eq!(broadcaster.client_count(), 0);
    }

    #[tokio::test]
    async fn test_broadcast() {
        let broadcaster = InsightBroadcaster::new();
        let (tx, mut rx) = mpsc::unbounded_channel();

        broadcaster.register_client("user123".to_string(), tx);

        let event = AIInsightEvent::classification_started(
            "user123".to_string(),
            "покажи меню".to_string(),
        );

        broadcaster.broadcast(event.clone());

        // Verify event was received
        let received = rx.recv().await.unwrap();
        match received {
            AIInsightEvent::IntentClassificationStarted { user_id, message, .. } => {
                assert_eq!(user_id, "user123");
                assert_eq!(message, "покажи меню");
            }
            _ => panic!("Wrong event type"),
        }
    }
}

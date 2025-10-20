//! 🚌 Shared Communication Bus
//! 
//! Real-time communication system for multi-agent coordination.
//! Enables agents to publish events, subscribe to topics, and coordinate actions
//! through a centralized message bus with pub/sub pattern.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tokio::time::{Duration, Instant};
use uuid::Uuid;

/// Maximum number of messages to retain in bus channels
const MAX_CHANNEL_CAPACITY: usize = 1000;

/// Maximum age for message retention (in seconds)
const MAX_MESSAGE_AGE_SECONDS: u64 = 3600; // 1 hour

/// Communication bus for real-time agent coordination
pub struct SharedBus {
    /// Topic-based broadcast channels for pub/sub messaging
    topics: Arc<RwLock<HashMap<String, broadcast::Sender<BusMessage>>>>,
    /// Agent subscriptions tracker
    subscriptions: Arc<RwLock<HashMap<String, Vec<String>>>>, // agent_id -> topics
    /// Message history for debugging and replay
    message_history: Arc<RwLock<Vec<BusMessage>>>,
    /// Bus statistics
    stats: Arc<RwLock<BusStats>>,
    /// Cleanup task handle
    _cleanup_handle: tokio::task::JoinHandle<()>,
}

/// Message sent through the communication bus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusMessage {
    /// Unique message identifier
    pub id: String,
    /// Message timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Source agent identifier
    pub from_agent: String,
    /// Target agent (None for broadcast)
    pub to_agent: Option<String>,
    /// Message topic/category
    pub topic: String,
    /// Message type for routing
    pub message_type: MessageType,
    /// Message payload
    pub payload: serde_json::Value,
    /// Message priority (0 = low, 10 = critical)
    pub priority: u8,
    /// Time to live in seconds
    pub ttl_seconds: Option<u64>,
    /// Delivery confirmation required
    pub requires_ack: bool,
}

/// Types of messages that can be sent through the bus
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    /// General information broadcast
    Info,
    /// Request for action or data
    Request,
    /// Response to a request
    Response,
    /// Alert or notification
    Alert,
    /// Command for immediate action
    Command,
    /// Event notification
    Event,
    /// Heartbeat for monitoring
    Heartbeat,
    /// System coordination message
    Coordination,
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Info => write!(f, "Info"),
            MessageType::Request => write!(f, "Request"),
            MessageType::Response => write!(f, "Response"),
            MessageType::Alert => write!(f, "Alert"),
            MessageType::Command => write!(f, "Command"),
            MessageType::Event => write!(f, "Event"),
            MessageType::Heartbeat => write!(f, "Heartbeat"),
            MessageType::Coordination => write!(f, "Coordination"),
        }
    }
}

/// Message subscription configuration
#[derive(Debug)]
pub struct Subscription {
    /// Agent identifier
    pub agent_id: String,
    /// Topics to subscribe to
    pub topics: Vec<String>,
    /// Message type filters
    pub message_types: Option<Vec<MessageType>>,
    /// Minimum priority level
    pub min_priority: Option<u8>,
    /// Callback channel
    pub receiver: broadcast::Receiver<BusMessage>,
}

/// Coordination result from an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationResult {
    /// Task identifier this result belongs to
    pub task_id: String,
    /// Agent that produced this result
    pub agent_id: String,
    /// Result status
    pub status: CoordinationStatus,
    /// Result data or analysis
    pub result: serde_json::Value,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Optional next steps or recommendations
    pub next_steps: Option<Vec<String>>,
    /// Completion timestamp
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

/// Status of coordination task execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CoordinationStatus {
    /// Task completed successfully
    Success,
    /// Task failed with error
    Failed,
    /// Task partially completed
    Partial,
    /// Task is still in progress
    InProgress,
    /// Task was cancelled
    Cancelled,
}

/// Workflow step execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStepResult {
    /// Workflow identifier
    pub workflow_id: String,
    /// Step that was executed
    pub step: String,
    /// Agent that executed the step
    pub agent_id: String,
    /// Step execution status
    pub status: CoordinationStatus,
    /// Step output data
    pub output: serde_json::Value,
    /// Next step to execute (if any)
    pub next_step: Option<String>,
    /// Execution timestamp
    pub executed_at: chrono::DateTime<chrono::Utc>,
}

/// Bus statistics and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusStats {
    /// Total messages sent
    pub total_messages: u64,
    /// Messages per topic
    pub messages_per_topic: HashMap<String, u64>,
    /// Active subscriptions count
    pub active_subscriptions: u64,
    /// Average message processing time
    pub avg_processing_time_ms: f64,
    /// Bus uptime
    pub uptime_seconds: u64,
    /// Last activity timestamp
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

impl Default for BusStats {
    fn default() -> Self {
        Self {
            total_messages: 0,
            messages_per_topic: HashMap::new(),
            active_subscriptions: 0,
            avg_processing_time_ms: 0.0,
            uptime_seconds: 0,
            last_activity: chrono::Utc::now(),
        }
    }
}

impl SharedBus {
    /// Create a new shared communication bus
    pub async fn new() -> Result<Self> {
        let topics = Arc::new(RwLock::new(HashMap::new()));
        let subscriptions = Arc::new(RwLock::new(HashMap::new()));
        let message_history = Arc::new(RwLock::new(Vec::new()));
        let stats = Arc::new(RwLock::new(BusStats::default()));

        // Start cleanup task
        let cleanup_topics = Arc::clone(&topics);
        let cleanup_history = Arc::clone(&message_history);
        let cleanup_stats = Arc::clone(&stats);
        let cleanup_handle = tokio::spawn(async move {
            Self::cleanup_task(cleanup_topics, cleanup_history, cleanup_stats).await;
        });

        let bus = Self {
            topics,
            subscriptions,
            message_history,
            stats,
            _cleanup_handle: cleanup_handle,
        };

        tracing::info!("🚌 Shared communication bus initialized");
        Ok(bus)
    }

    /// Subscribe agent to topics
    pub async fn subscribe(&self, agent_id: &str, topics: Vec<String>) -> Result<broadcast::Receiver<BusMessage>> {
        let mut subscriptions = self.subscriptions.write().await;
        let mut topic_channels = self.topics.write().await;
        
        // Create topic channels if they don't exist
        for topic in &topics {
            if !topic_channels.contains_key(topic) {
                let (tx, _) = broadcast::channel(MAX_CHANNEL_CAPACITY);
                topic_channels.insert(topic.clone(), tx);
            }
        }

        // Create merged receiver for all subscribed topics
        let (merged_tx, merged_rx) = broadcast::channel(MAX_CHANNEL_CAPACITY);
        
        // Subscribe to each topic and forward to merged channel
        for topic in &topics {
            if let Some(topic_tx) = topic_channels.get(topic) {
                let mut topic_rx = topic_tx.subscribe();
                let forwarding_tx = merged_tx.clone();
                let topic_name = topic.clone();
                let subscriber_id = agent_id.to_string();
                
                tokio::spawn(async move {
                    while let Ok(message) = topic_rx.recv().await {
                        // Filter messages for this agent if targeted
                        if let Some(ref target) = message.to_agent {
                            if target != &subscriber_id {
                                continue;
                            }
                        }
                        
                        // Forward to merged channel
                        if forwarding_tx.send(message).is_err() {
                            tracing::warn!("Failed to forward message to agent {} on topic {}", subscriber_id, topic_name);
                            break;
                        }
                    }
                });
            }
        }

        // Track subscription
        subscriptions.insert(agent_id.to_string(), topics.clone());
        
        // Update stats
        let mut stats = self.stats.write().await;
        stats.active_subscriptions += 1;

        tracing::info!("🔔 Agent {} subscribed to topics: {:?}", agent_id, topics);
        Ok(merged_rx)
    }

    /// Publish message to the bus
    pub async fn publish(&self, message: BusMessage) -> Result<()> {
        let start_time = Instant::now();
        
        // Validate message
        if message.topic.is_empty() {
            return Err(anyhow::anyhow!("Message topic cannot be empty"));
        }

        // Get or create topic channel
        let mut topics = self.topics.write().await;
        let sender = topics.entry(message.topic.clone())
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(MAX_CHANNEL_CAPACITY);
                tx
            });

        // Send message
        if let Err(e) = sender.send(message.clone()) {
            tracing::warn!("Failed to send message to topic {}: {}", message.topic, e);
            return Err(anyhow::anyhow!("Failed to publish message: {}", e));
        }

        // Store in history
        let mut history = self.message_history.write().await;
        history.push(message.clone());

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_messages += 1;
        *stats.messages_per_topic.entry(message.topic.clone()).or_insert(0) += 1;
        stats.last_activity = chrono::Utc::now();
        
        let processing_time = start_time.elapsed().as_millis() as f64;
        stats.avg_processing_time_ms = 
            (stats.avg_processing_time_ms * (stats.total_messages - 1) as f64 + processing_time) 
            / stats.total_messages as f64;

        tracing::debug!("📨 Published message {} to topic {}", message.id, message.topic);
        Ok(())
    }

    /// Send targeted message to specific agent
    pub async fn send_to_agent(&self, from_agent: &str, to_agent: &str, topic: &str, payload: serde_json::Value) -> Result<()> {
        let message = BusMessage {
            id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            from_agent: from_agent.to_string(),
            to_agent: Some(to_agent.to_string()),
            topic: topic.to_string(),
            message_type: MessageType::Request,
            payload,
            priority: 5,
            ttl_seconds: Some(300), // 5 minutes
            requires_ack: false,
        };

        self.publish(message).await
    }

    /// Broadcast message to all subscribers of a topic
    pub async fn broadcast(&self, from_agent: &str, topic: &str, message_type: MessageType, payload: serde_json::Value) -> Result<()> {
        let message = BusMessage {
            id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            from_agent: from_agent.to_string(),
            to_agent: None,
            topic: topic.to_string(),
            message_type,
            payload,
            priority: 3,
            ttl_seconds: Some(600), // 10 minutes
            requires_ack: false,
        };

        self.publish(message).await
    }

    /// Send high-priority alert
    pub async fn send_alert(&self, from_agent: &str, topic: &str, alert_message: &str, priority: u8) -> Result<()> {
        let payload = serde_json::json!({
            "alert": alert_message,
            "severity": if priority >= 8 { "critical" } else if priority >= 6 { "warning" } else { "info" }
        });

        let message = BusMessage {
            id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            from_agent: from_agent.to_string(),
            to_agent: None,
            topic: topic.to_string(),
            message_type: MessageType::Alert,
            payload,
            priority,
            ttl_seconds: Some(3600), // 1 hour
            requires_ack: true,
        };

        self.publish(message).await
    }

    /// Send coordination message for multi-agent tasks
    pub async fn coordinate(&self, from_agent: &str, task_id: &str, action: &str, participants: Vec<String>) -> Result<()> {
        let payload = serde_json::json!({
            "task_id": task_id,
            "action": action,
            "participants": participants,
            "coordinator": from_agent
        });

        let message = BusMessage {
            id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            from_agent: from_agent.to_string(),
            to_agent: None,
            topic: "coordination".to_string(),
            message_type: MessageType::Coordination,
            payload,
            priority: 7,
            ttl_seconds: Some(1800), // 30 minutes
            requires_ack: true,
        };

        self.publish(message).await
    }

    /// Send coordination result back to the coordinator
    pub async fn send_coordination_result(&self, from_agent: &str, task_id: &str, result: CoordinationResult) -> Result<()> {
        let payload = serde_json::to_value(&result)?;

        let message = BusMessage {
            id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            from_agent: from_agent.to_string(),
            to_agent: None,
            topic: "coordination_result".to_string(),
            message_type: MessageType::Response,
            payload,
            priority: 6,
            ttl_seconds: Some(900), // 15 minutes
            requires_ack: false,
        };

        self.publish(message).await
    }

    /// Send workflow trigger to initiate chained actions
    pub async fn trigger_workflow(&self, from_agent: &str, workflow_id: &str, step: &str, data: serde_json::Value) -> Result<()> {
        let payload = serde_json::json!({
            "workflow_id": workflow_id,
            "step": step,
            "step_data": data,
            "initiator": from_agent,
            "timestamp": chrono::Utc::now()
        });

        let message = BusMessage {
            id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            from_agent: from_agent.to_string(),
            to_agent: None,
            topic: "workflow".to_string(),
            message_type: MessageType::Event,
            payload,
            priority: 5,
            ttl_seconds: Some(1200), // 20 minutes
            requires_ack: false,
        };

        self.publish(message).await
    }

    /// Send workflow step completion result
    pub async fn complete_workflow_step(&self, result: WorkflowStepResult) -> Result<()> {
        let payload = serde_json::to_value(&result)?;

        let message = BusMessage {
            id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            from_agent: result.agent_id.clone(),
            to_agent: None,
            topic: "workflow_result".to_string(),
            message_type: MessageType::Response,
            payload,
            priority: if result.status == CoordinationStatus::Failed { 8 } else { 5 },
            ttl_seconds: Some(900),
            requires_ack: false,
        };

        self.publish(message).await
    }

    /// Get message history for a topic
    pub async fn get_history(&self, topic: &str, limit: Option<usize>) -> Vec<BusMessage> {
        let history = self.message_history.read().await;
        let filtered: Vec<BusMessage> = history.iter()
            .filter(|msg| msg.topic == topic)
            .cloned()
            .collect();

        if let Some(limit) = limit {
            filtered.into_iter().rev().take(limit).collect()
        } else {
            filtered
        }
    }

    /// Get current bus statistics
    pub async fn get_stats(&self) -> BusStats {
        let mut stats = self.stats.read().await.clone();
        stats.uptime_seconds = chrono::Utc::now().timestamp() as u64 - stats.last_activity.timestamp() as u64;
        stats
    }

    /// List active topics
    pub async fn list_topics(&self) -> Vec<String> {
        let topics = self.topics.read().await;
        topics.keys().cloned().collect()
    }

    /// Get subscribers for a topic
    pub async fn get_topic_subscribers(&self, topic: &str) -> Vec<String> {
        let subscriptions = self.subscriptions.read().await;
        subscriptions.iter()
            .filter(|(_, topics)| topics.contains(&topic.to_string()))
            .map(|(agent_id, _)| agent_id.clone())
            .collect()
    }

    /// Unsubscribe agent from topics
    pub async fn unsubscribe(&self, agent_id: &str, topics: Option<Vec<String>>) -> Result<()> {
        let mut subscriptions = self.subscriptions.write().await;
        
        if let Some(specific_topics) = topics {
            // Unsubscribe from specific topics
            if let Some(agent_topics) = subscriptions.get_mut(agent_id) {
                agent_topics.retain(|topic| !specific_topics.contains(topic));
                if agent_topics.is_empty() {
                    subscriptions.remove(agent_id);
                }
            }
        } else {
            // Unsubscribe from all topics
            subscriptions.remove(agent_id);
        }

        // Update stats
        let mut stats = self.stats.write().await;
        if stats.active_subscriptions > 0 {
            stats.active_subscriptions -= 1;
        }

        tracing::info!("🔕 Agent {} unsubscribed", agent_id);
        Ok(())
    }

    /// Cleanup task to remove old messages and inactive topics
    async fn cleanup_task(
        topics: Arc<RwLock<HashMap<String, broadcast::Sender<BusMessage>>>>,
        history: Arc<RwLock<Vec<BusMessage>>>,
        stats: Arc<RwLock<BusStats>>,
    ) {
        let mut interval = tokio::time::interval(Duration::from_secs(300)); // Run every 5 minutes
        
        loop {
            interval.tick().await;
            
            // Clean up old messages from history
            let mut message_history = history.write().await;
            let cutoff_time = chrono::Utc::now() - chrono::Duration::seconds(MAX_MESSAGE_AGE_SECONDS as i64);
            message_history.retain(|msg| msg.timestamp > cutoff_time);
            drop(message_history);

            // Clean up inactive topics (topics with no subscribers)
            let mut topic_map = topics.write().await;
            topic_map.retain(|_, sender| sender.receiver_count() > 0);
            drop(topic_map);

            // Update stats
            let mut bus_stats = stats.write().await;
            bus_stats.uptime_seconds += 300; // Add 5 minutes
            drop(bus_stats);

            tracing::debug!("🧹 Bus cleanup completed");
        }
    }
}

/// Helper trait for agents to easily integrate with the shared bus
pub trait BusIntegration {
    /// Get agent ID for bus operations
    fn get_agent_id(&self) -> &str;
    
    /// Subscribe to bus topics
    async fn subscribe_to_bus(&self, bus: &SharedBus, topics: Vec<String>) -> Result<broadcast::Receiver<BusMessage>> {
        bus.subscribe(self.get_agent_id(), topics).await
    }
    
    /// Publish message to bus
    async fn publish_to_bus(&self, bus: &SharedBus, topic: &str, message_type: MessageType, payload: serde_json::Value) -> Result<()> {
        bus.broadcast(self.get_agent_id(), topic, message_type, payload).await
    }
    
    /// Send alert through bus
    async fn send_alert_to_bus(&self, bus: &SharedBus, topic: &str, message: &str, priority: u8) -> Result<()> {
        bus.send_alert(self.get_agent_id(), topic, message, priority).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_bus_creation() {
        let bus = SharedBus::new().await.unwrap();
        let stats = bus.get_stats().await;
        assert_eq!(stats.total_messages, 0);
        assert_eq!(stats.active_subscriptions, 0);
    }

    #[tokio::test]
    async fn test_subscribe_and_publish() {
        let bus = SharedBus::new().await.unwrap();
        let mut receiver = bus.subscribe("agent1", vec!["test_topic".to_string()]).await.unwrap();

        let message = BusMessage {
            id: "test_msg_1".to_string(),
            timestamp: chrono::Utc::now(),
            from_agent: "agent2".to_string(),
            to_agent: None,
            topic: "test_topic".to_string(),
            message_type: MessageType::Info,
            payload: serde_json::json!({"data": "test"}),
            priority: 5,
            ttl_seconds: Some(300),
            requires_ack: false,
        };

        bus.publish(message.clone()).await.unwrap();

        let received = timeout(Duration::from_millis(100), receiver.recv()).await
            .expect("Should receive message")
            .expect("Message should be valid");

        assert_eq!(received.id, "test_msg_1");
        assert_eq!(received.topic, "test_topic");
    }

    #[tokio::test]
    async fn test_targeted_messaging() {
        let bus = SharedBus::new().await.unwrap();
        let mut receiver1 = bus.subscribe("agent1", vec!["test_topic".to_string()]).await.unwrap();
        let mut receiver2 = bus.subscribe("agent2", vec!["test_topic".to_string()]).await.unwrap();

        bus.send_to_agent("sender", "agent1", "test_topic", serde_json::json!({"target": "agent1"})).await.unwrap();

        // Agent1 should receive the message
        let received1 = timeout(Duration::from_millis(100), receiver1.recv()).await
            .expect("Agent1 should receive message")
            .expect("Message should be valid");
        assert_eq!(received1.to_agent, Some("agent1".to_string()));

        // Agent2 should not receive the targeted message
        let result2 = timeout(Duration::from_millis(100), receiver2.recv()).await;
        assert!(result2.is_err(), "Agent2 should not receive targeted message");
    }

    #[tokio::test]
    async fn test_broadcast_messaging() {
        let bus = SharedBus::new().await.unwrap();
        let mut receiver1 = bus.subscribe("agent1", vec!["broadcast_topic".to_string()]).await.unwrap();
        let mut receiver2 = bus.subscribe("agent2", vec!["broadcast_topic".to_string()]).await.unwrap();

        bus.broadcast("sender", "broadcast_topic", MessageType::Info, serde_json::json!({"broadcast": true})).await.unwrap();

        // Both agents should receive the broadcast
        let received1 = timeout(Duration::from_millis(100), receiver1.recv()).await
            .expect("Agent1 should receive broadcast")
            .expect("Message should be valid");
        assert_eq!(received1.to_agent, None);

        let received2 = timeout(Duration::from_millis(100), receiver2.recv()).await
            .expect("Agent2 should receive broadcast")
            .expect("Message should be valid");
        assert_eq!(received2.to_agent, None);
    }

    #[tokio::test]
    async fn test_coordination_messaging() {
        let bus = SharedBus::new().await.unwrap();
        let mut receiver = bus.subscribe("agent1", vec!["coordination".to_string()]).await.unwrap();

        bus.coordinate("coordinator", "task_123", "start_analysis", vec!["agent1".to_string(), "agent2".to_string()]).await.unwrap();

        let received = timeout(Duration::from_millis(100), receiver.recv()).await
            .expect("Should receive coordination message")
            .expect("Message should be valid");

        assert_eq!(received.message_type, MessageType::Coordination);
        assert_eq!(received.topic, "coordination");
        assert!(received.requires_ack);
        assert_eq!(received.priority, 7);
    }
}
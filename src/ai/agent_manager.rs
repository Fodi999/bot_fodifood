//! 🤖 AI Agent Manager - Intelligent Multi-Agent System
//! 
//! Manages different types of AI agents with persistent memory and context.
//! Each agent type has specialized behavior and maintains its own state.

use std::collections::HashMap;
use crate::ai::agents::{InvestorAgent, BusinessAgent, UserAgent};
use crate::ai::persistent_memory::PersistentMemory;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::sync::Arc;

/// Types of AI agents in the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentType {
    /// Investment-focused agent for portfolio management
    Investor,
    /// Business-focused agent for company operations
    Business,
    /// User-focused agent for general interactions
    User,
    /// General purpose agent
    General,
    /// System agent for administrative tasks
    System,
}

/// Agent configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Maximum memory entries to store
    pub max_memory_size: usize,
    /// Enable AI thinking capabilities
    pub thinking_enabled: bool,
    /// Learning rate for preferences and patterns
    pub learning_rate: f64,
    /// Response timeout in seconds
    pub response_timeout_seconds: u32,
    /// Auto-save interval in minutes
    pub auto_save_interval_minutes: u32,
    /// Custom configuration parameters
    pub custom_params: HashMap<String, serde_json::Value>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_memory_size: 5000,
            thinking_enabled: true,
            learning_rate: 0.7,
            response_timeout_seconds: 30,
            auto_save_interval_minutes: 5,
            custom_params: HashMap::new(),
        }
    }
}

/// Agent state and status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    /// Agent unique identifier
    pub id: String,
    /// Agent type
    pub agent_type: AgentType,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last activity timestamp
    pub last_active: chrono::DateTime<chrono::Utc>,
    /// Total interactions count
    pub interaction_count: u64,
    /// Memory size
    pub memory_size: usize,
    /// Current status
    pub status: AgentStatus,
    /// Configuration version
    pub config_version: u32,
}

/// Agent status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    /// Agent is active and ready
    Active,
    /// Agent is idle
    Idle,
    /// Agent is archived
    Archived,
    /// Agent has encountered an error
    Error(String),
}

/// Multi-agent management system with persistent memory
pub struct AgentManager {
    /// Active agents by ID
    agents: Arc<RwLock<HashMap<String, Box<dyn AIEntityAgent>>>>,
    /// Persistent memory storage
    memory_store: Arc<PersistentMemory>,
    /// Agent interaction history
    interaction_log: Arc<RwLock<Vec<AgentInteraction>>>,
    /// Agent creation statistics
    stats: Arc<RwLock<AgentStats>>,
    /// Shared communication bus for real-time coordination
    shared_bus: Option<Arc<crate::ai::shared_bus::SharedBus>>,
}

/// Core trait for all AI agents
pub trait AIEntityAgent: Send + Sync {
    /// Get unique agent ID
    fn get_id(&self) -> &str;
    
    /// Get agent type
    fn get_type(&self) -> AgentType;
    
    /// Process input and generate response
    fn think(&mut self, input: &str) -> Result<String>;
    
    /// Recall memories and context
    fn recall(&self, query: Option<&str>) -> String;
    
    /// Store a memory
    fn memorize(&mut self, key: &str, value: &str) -> Result<()>;
    
    /// Get agent state summary
    fn get_state_summary(&self) -> AgentState;
    
    /// Receive message from another agent
    fn receive_message(&mut self, from_agent: &str, message: &str) -> Result<Option<String>>;
    
    /// Get agent capabilities
    fn get_capabilities(&self) -> Vec<String>;
    
    /// Update agent configuration
    fn update_config(&mut self, config: AgentConfig) -> Result<()>;
}

/// Agent interaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInteraction {
    /// Interaction timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Source agent ID
    pub agent_id: String,
    /// Agent type
    pub agent_type: AgentType,
    /// Input received
    pub input: String,
    /// Response generated
    pub response: String,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Memory keys accessed
    pub memory_accessed: Vec<String>,
    /// Memory keys updated
    pub memory_updated: Vec<String>,
}

/// Agent interaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStyle {
    /// Response length preference (0.0-1.0)
    pub verbosity: f64,
    /// Formality level (0.0-1.0)
    pub formality: f64,
    /// Technical detail level (0.0-1.0)
    pub technical_depth: f64,
    /// Emotional expression (0.0-1.0)
    pub emotional_tone: f64,
}

/// Memory management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySettings {
    /// Maximum memory items to retain
    pub max_items: usize,
    /// Memory retention period in days
    pub retention_days: u32,
    /// Auto-summarization threshold
    pub summarize_threshold: usize,
    /// Priority weighting for different memory types
    pub priority_weights: HashMap<String, f64>,
}

/// Agent system statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStats {
    /// Total agents created
    pub total_agents: u64,
    /// Active agents count
    pub active_agents: u64,
    /// Total interactions processed
    pub total_interactions: u64,
    /// Average response time in ms
    pub avg_response_time_ms: f64,
    /// Memory usage statistics
    pub memory_usage: MemoryUsageStats,
}

/// Memory usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsageStats {
    /// Total memory items stored
    pub total_items: u64,
    /// Average items per agent
    pub avg_items_per_agent: f64,
    /// Memory growth rate per day
    pub growth_rate_daily: f64,
}

impl Default for ResponseStyle {
    fn default() -> Self {
        Self {
            verbosity: 0.7,
            formality: 0.6,
            technical_depth: 0.5,
            emotional_tone: 0.4,
        }
    }
}

impl Default for MemorySettings {
    fn default() -> Self {
        let mut priority_weights = HashMap::new();
        priority_weights.insert("investment".to_string(), 1.0);
        priority_weights.insert("business".to_string(), 0.9);
        priority_weights.insert("personal".to_string(), 0.8);
        priority_weights.insert("system".to_string(), 0.6);
        
        Self {
            max_items: 1000,
            retention_days: 30,
            summarize_threshold: 100,
            priority_weights,
        }
    }
}

impl AgentManager {
    /// Create new agent manager
    pub async fn new(persistent_memory: Arc<PersistentMemory>) -> Result<Self> {
        Ok(Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            memory_store: persistent_memory,
            interaction_log: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(AgentStats {
                total_agents: 0,
                active_agents: 0,
                total_interactions: 0,
                avg_response_time_ms: 0.0,
                memory_usage: MemoryUsageStats {
                    total_items: 0,
                    avg_items_per_agent: 0.0,
                    growth_rate_daily: 0.0,
                },
            })),
            shared_bus: None,
        })
    }

    /// Get or create an agent
    pub async fn get_or_create_agent(&self, id: &str, agent_type: AgentType) -> Result<()> {
        let mut agents = self.agents.write().await;
        
        if !agents.contains_key(id) {
            let agent: Box<dyn AIEntityAgent> = match agent_type {
                AgentType::Investor => Box::new(InvestorAgent::new(id, self.memory_store.clone()).await?),
                AgentType::Business => Box::new(BusinessAgent::new(id, self.memory_store.clone()).await?),
                AgentType::User => Box::new(UserAgent::new(id, self.memory_store.clone()).await?),
                AgentType::General => Box::new(UserAgent::new(id, self.memory_store.clone()).await?), // Use UserAgent as General
                AgentType::System => Box::new(UserAgent::new(id, self.memory_store.clone()).await?), // Fallback
            };
            
            agents.insert(id.to_string(), agent);
            
            // Update statistics
            let mut stats = self.stats.write().await;
            stats.total_agents += 1;
            stats.active_agents += 1;
            
            tracing::info!("🤖 Created new {} agent: {}", 
                match agent_type {
                    AgentType::Investor => "Investor",
                    AgentType::Business => "Business", 
                    AgentType::User => "User",
                    AgentType::General => "General",
                    AgentType::System => "System",
                }, id);
        }
        
        Ok(())
    }

    /// Process input through specific agent
    pub async fn process_with_agent(&self, agent_id: &str, input: &str) -> Result<String> {
        let start_time = std::time::Instant::now();
        
        let mut agents = self.agents.write().await;
        let agent = agents.get_mut(agent_id)
            .ok_or_else(|| anyhow::anyhow!("Agent {} not found", agent_id))?;
        
        // Record memory state before processing
        let memory_before = agent.recall(None);
        let memory_keys_before: Vec<String> = memory_before.lines()
            .filter_map(|line| line.split(':').next().map(|s| s.trim().to_string()))
            .collect();
        
        // Process the input
        let response = agent.think(input)?;
        
        // Record memory state after processing
        let memory_after = agent.recall(None);
        let memory_keys_after: Vec<String> = memory_after.lines()
            .filter_map(|line| line.split(':').next().map(|s| s.trim().to_string()))
            .collect();
        
        let processing_time = start_time.elapsed();
        
        // Log the interaction
        let interaction = AgentInteraction {
            timestamp: chrono::Utc::now(),
            agent_id: agent_id.to_string(),
            agent_type: agent.get_type(),
            input: input.to_string(),
            response: response.clone(),
            processing_time_ms: processing_time.as_millis() as u64,
            memory_accessed: memory_keys_before.clone(),
            memory_updated: memory_keys_after.into_iter()
                .filter(|key| !memory_keys_before.contains(key))
                .collect(),
        };
        
        let mut log = self.interaction_log.write().await;
        log.push(interaction);
        
        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_interactions += 1;
        stats.avg_response_time_ms = (stats.avg_response_time_ms * (stats.total_interactions - 1) as f64 
            + processing_time.as_millis() as f64) / stats.total_interactions as f64;
        
        Ok(response)
    }

    /// Send message to a specific agent
    pub async fn send_message(&self, agent_id: &str, message: &str) -> Result<String> {
        self.process_with_agent(agent_id, message).await
    }

    /// Send message between agents
    pub async fn send_agent_message(&self, from_agent: &str, to_agent: &str, message: &str) -> Result<Option<String>> {
        let mut agents = self.agents.write().await;
        
        let recipient = agents.get_mut(to_agent)
            .ok_or_else(|| anyhow::anyhow!("Recipient agent {} not found", to_agent))?;
        
        recipient.receive_message(from_agent, message)
    }

    /// Get agent state information
    pub async fn get_agent_state(&self, agent_id: &str) -> Result<AgentState> {
        let agents = self.agents.read().await;
        let agent = agents.get(agent_id)
            .ok_or_else(|| anyhow::anyhow!("Agent {} not found", agent_id))?;
        
        Ok(agent.get_state_summary())
    }

    /// List all active agents
    pub async fn list_agents(&self) -> Vec<String> {
        let agents = self.agents.read().await;
        agents.keys().cloned().collect()
    }

    /// Create a new agent with optional configuration
    pub async fn create_agent(&mut self, agent_type: AgentType, agent_id: &str, config: Option<AgentConfig>) -> Result<()> {
        self.get_or_create_agent(agent_id, agent_type).await?;
        
        // If config is provided, update the agent
        if let Some(agent_config) = config {
            let mut agents = self.agents.write().await;
            if let Some(agent) = agents.get_mut(agent_id) {
                agent.update_config(agent_config)?;
            }
        }
        
        Ok(())
    }

    /// Get comprehensive system statistics
    pub async fn get_statistics(&self) -> AgentStats {
        let agents = self.agents.read().await;
        let log = self.interaction_log.read().await;
        
        let mut stats = self.stats.write().await;
        stats.total_agents = agents.len() as u64;
        stats.active_agents = agents.len() as u64; // All loaded agents are considered active
        
        // Calculate memory usage
        let total_memory: usize = agents.values()
            .map(|agent| agent.recall(None).len())
            .sum();
        
        stats.memory_usage = MemoryUsageStats {
            total_items: total_memory as u64,
            avg_items_per_agent: if agents.is_empty() { 0.0 } else { total_memory as f64 / agents.len() as f64 },
            growth_rate_daily: 0.0, // This would need historical data to calculate
        };
        
        stats.clone()
    }

    /// Get statistics for a specific agent
    pub async fn get_agent_statistics(&self, agent_id: &str) -> Option<AgentState> {
        let agents = self.agents.read().await;
        agents.get(agent_id).map(|agent| agent.get_state_summary())
    }

    /// Get capabilities of a specific agent
    pub async fn get_agent_capabilities(&self, agent_id: &str) -> Option<Vec<String>> {
        let agents = self.agents.read().await;
        agents.get(agent_id).map(|agent| agent.get_capabilities())
    }

    /// Get interaction history for agent or all agents
    pub async fn get_interaction_history(&self, limit: Option<usize>) -> Result<Vec<AgentInteraction>> {
        let log = self.interaction_log.read().await;
        let mut interactions: Vec<AgentInteraction> = log.clone();
        
        interactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        if let Some(limit_val) = limit {
            interactions.truncate(limit_val);
        }
        
        Ok(interactions)
    }

    /// Get interaction history for specific agent
    pub async fn get_agent_interaction_history(&self, agent_id: &str, limit: Option<usize>) -> Vec<AgentInteraction> {
        let log = self.interaction_log.read().await;
        let mut interactions: Vec<AgentInteraction> = log.iter()
            .filter(|interaction| interaction.agent_id == agent_id)
            .cloned()
            .collect();
        
        interactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        if let Some(limit) = limit {
            interactions.truncate(limit);
        }
        
        interactions
    }

    /// Archive inactive agents
    pub async fn archive_inactive_agents(&self, inactive_days: u32) -> Result<u32> {
        let cutoff_time = chrono::Utc::now() - chrono::Duration::days(inactive_days as i64);
        let mut agents = self.agents.write().await;
        let mut archived_count = 0;
        
        let agent_ids: Vec<String> = agents.keys().cloned().collect();
        
        for agent_id in agent_ids {
            if let Some(agent) = agents.get(&agent_id) {
                let state = agent.get_state_summary();
                if state.last_active < cutoff_time {
                    agents.remove(&agent_id);
                    archived_count += 1;
                    tracing::info!("📦 Archived inactive agent: {}", agent_id);
                }
            }
        }
        
        // Update statistics
        let mut stats = self.stats.write().await;
        stats.active_agents = agents.len() as u64;
        
        Ok(archived_count)
    }

    /// Get system statistics
    pub async fn get_stats(&self) -> AgentStats {
        self.stats.read().await.clone()
    }

    /// Cleanup old interaction logs
    pub async fn cleanup_logs(&self, retain_days: u32) -> Result<u32> {
        let cutoff_time = chrono::Utc::now() - chrono::Duration::days(retain_days as i64);
        let mut log = self.interaction_log.write().await;
        
        let initial_count = log.len();
        log.retain(|interaction| interaction.timestamp > cutoff_time);
        let removed_count = initial_count - log.len();
        
        tracing::info!("🧹 Cleaned up {} old interaction logs", removed_count);
        Ok(removed_count as u32)
    }

    /// Export agent data for analysis
    pub async fn export_agent_data(&self, agent_id: &str) -> Result<serde_json::Value> {
        let agents = self.agents.read().await;
        let agent = agents.get(agent_id)
            .ok_or_else(|| anyhow::anyhow!("Agent {} not found", agent_id))?;
        
        let state = agent.get_state_summary();
        let memory = agent.recall(None);
        let interactions = match self.get_interaction_history(Some(100)).await {
            Ok(interactions) => interactions,
            Err(_) => vec![], // Empty on error
        };
        
        Ok(serde_json::json!({
            "state": state,
            "memory": memory,
            "recent_interactions": interactions,
            "capabilities": agent.get_capabilities(),
            "export_timestamp": chrono::Utc::now()
        }))
    }

    /// Enable shared bus for real-time agent coordination
    pub async fn enable_shared_bus(&mut self) -> Result<()> {
        let bus = Arc::new(crate::ai::shared_bus::SharedBus::new().await?);
        self.shared_bus = Some(bus);
        tracing::info!("🚌 Shared communication bus enabled for agent manager");
        Ok(())
    }

    /// Get reference to shared bus
    pub fn get_shared_bus(&self) -> Option<Arc<crate::ai::shared_bus::SharedBus>> {
        self.shared_bus.clone()
    }

    /// Broadcast message to all agents via shared bus
    pub async fn broadcast_to_agents(&self, from_agent: &str, topic: &str, message: serde_json::Value) -> Result<()> {
        if let Some(ref bus) = self.shared_bus {
            bus.broadcast(from_agent, topic, crate::ai::shared_bus::MessageType::Info, message).await?;
            tracing::debug!("📡 Broadcasted message from {} to topic {}", from_agent, topic);
        } else {
            tracing::warn!("🚌 Shared bus not enabled - message not sent");
        }
        Ok(())
    }

    /// Send coordination message between agents
    pub async fn coordinate_agents(&self, coordinator: &str, task_id: &str, action: &str, participants: Vec<String>) -> Result<()> {
        if let Some(ref bus) = self.shared_bus {
            bus.coordinate(coordinator, task_id, action, participants).await?;
            tracing::info!("🔄 Coordination initiated by {} for task {}", coordinator, task_id);
        } else {
            tracing::warn!("🚌 Shared bus not enabled - coordination not sent");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_creation() {
        let memory = Arc::new(PersistentMemory::new("test_agents.db").unwrap());
        let manager = AgentManager::new(memory).await.unwrap();
        
        manager.get_or_create_agent("INVESTOR-123", AgentType::Investor).await.unwrap();
        manager.get_or_create_agent("BUSINESS-SEA", AgentType::Business).await.unwrap();
        
        let agents = manager.list_agents().await;
        assert_eq!(agents.len(), 2);
        assert!(agents.contains(&"INVESTOR-123".to_string()));
        assert!(agents.contains(&"BUSINESS-SEA".to_string()));
    }

    #[tokio::test]
    async fn test_agent_interaction() {
        let memory = Arc::new(PersistentMemory::new("test_interaction.db").unwrap());
        let manager = AgentManager::new(memory).await.unwrap();
        manager.get_or_create_agent("TEST-AGENT", AgentType::User).await.unwrap();
        
        let response = manager.process_with_agent("TEST-AGENT", "Hello, how are you?").await.unwrap();
        assert!(!response.is_empty());
        
        let history = manager.get_interaction_history(Some(1)).await.unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].input, "Hello, how are you?");
    }
}
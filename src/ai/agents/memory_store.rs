//! 🧠 Memory Store - Persistent Storage for AI Agents
//! 
//! Handles long-term memory storage and retrieval for AI agents.
//! Supports structured memory with semantic search and categorization.

use crate::ai::persistent_memory::PersistentMemory;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Memory entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Unique memory ID
    pub id: String,
    /// Agent ID that owns this memory
    pub agent_id: String,
    /// Memory category (investment, business, personal, etc.)
    pub category: String,
    /// Memory key for retrieval
    pub key: String,
    /// Memory content
    pub value: String,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last accessed timestamp
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    /// Access count
    pub access_count: u32,
    /// Memory importance score (0.0-1.0)
    pub importance: f64,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Related memory IDs
    pub related_memories: Vec<String>,
}

/// Memory query parameters
#[derive(Debug, Clone)]
pub struct MemoryQuery {
    /// Agent ID filter
    pub agent_id: Option<String>,
    /// Category filter
    pub category: Option<String>,
    /// Search text
    pub search_text: Option<String>,
    /// Tags filter
    pub tags: Vec<String>,
    /// Minimum importance threshold
    pub min_importance: Option<f64>,
    /// Maximum number of results
    pub limit: Option<usize>,
    /// Sort by (created_at, accessed_at, importance, access_count)
    pub sort_by: MemorySortBy,
}

/// Memory sorting options
#[derive(Debug, Clone)]
pub enum MemorySortBy {
    CreatedAt,
    LastAccessed,
    Importance,
    AccessCount,
    Relevance,
}

/// Advanced memory store with semantic capabilities
pub struct MemoryStore {
    /// Underlying persistent storage
    storage: Arc<PersistentMemory>,
    /// In-memory cache for frequent access
    cache: Arc<RwLock<HashMap<String, MemoryEntry>>>,
    /// Memory statistics
    stats: Arc<RwLock<MemoryStats>>,
}

/// Memory usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Total memory entries
    pub total_entries: u64,
    /// Entries by category
    pub entries_by_category: HashMap<String, u64>,
    /// Entries by agent
    pub entries_by_agent: HashMap<String, u64>,
    /// Average importance score
    pub avg_importance: f64,
    /// Most accessed memories
    pub top_accessed: Vec<String>,
    /// Recent activity
    pub recent_activity: Vec<MemoryActivity>,
}

/// Memory activity record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryActivity {
    /// Activity timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Activity type
    pub activity_type: MemoryActivityType,
    /// Agent ID
    pub agent_id: String,
    /// Memory ID
    pub memory_id: String,
    /// Additional context
    pub context: Option<String>,
}

/// Types of memory activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryActivityType {
    Created,
    Accessed,
    Updated,
    Deleted,
    Tagged,
    Linked,
}

impl MemoryStore {
    /// Create new memory store
    pub async fn new(storage: Arc<PersistentMemory>) -> Result<Self> {
        Ok(Self {
            storage,
            cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(MemoryStats {
                total_entries: 0,
                entries_by_category: HashMap::new(),
                entries_by_agent: HashMap::new(),
                avg_importance: 0.0,
                top_accessed: Vec::new(),
                recent_activity: Vec::new(),
            })),
        })
    }

    /// Store a memory entry
    pub async fn store(&self, agent_id: &str, category: &str, key: &str, value: &str) -> Result<()> {
        let entry = MemoryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            agent_id: agent_id.to_string(),
            category: category.to_string(),
            key: key.to_string(),
            value: value.to_string(),
            created_at: chrono::Utc::now(),
            last_accessed: chrono::Utc::now(),
            access_count: 0,
            importance: self.calculate_importance(category, value),
            tags: self.extract_tags(value),
            related_memories: Vec::new(),
        };

        let storage_key = format!("memory:{}:{}:{}", agent_id, category, key);
        
        // Use the preference system as a simple key-value store
        self.storage.save_preference("memory", &storage_key, &serde_json::to_string(&entry)?).await?;

        Ok(())
    }

    /// Retrieve a specific memory by ID
    pub async fn retrieve(&self, memory_id: &str) -> Result<Option<MemoryEntry>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(entry) = cache.get(memory_id) {
                // Update access statistics
                self.update_access_stats(memory_id).await;
                return Ok(Some(entry.clone()));
            }
        }

        // Load from persistent storage
        let storage_key = format!("memory:{}", memory_id);
        if let Some(data) = self.storage.retrieve(&storage_key).await? {
            let mut entry: MemoryEntry = serde_json::from_str(&data)?;
            
            // Update access information
            entry.last_accessed = chrono::Utc::now();
            entry.access_count += 1;

            // Update cache
            let mut cache = self.cache.write().await;
            cache.insert(memory_id.to_string(), entry.clone());

            // Record activity
            self.record_activity(MemoryActivity {
                timestamp: chrono::Utc::now(),
                activity_type: MemoryActivityType::Accessed,
                agent_id: entry.agent_id.clone(),
                memory_id: memory_id.to_string(),
                context: None,
            }).await;

            Ok(Some(entry))
        } else {
            Ok(None)
        }
    }

    /// Search memories with query parameters
    pub async fn search(&self, _query: MemoryQuery) -> Result<Vec<MemoryEntry>> {
        // Placeholder for search functionality
        // Would need to implement proper search using PersistentMemory's preference system
        Ok(Vec::new())
    }

    /// Get all memories for an agent
    pub async fn get_agent_memories(&self, agent_id: &str) -> Result<Vec<MemoryEntry>> {
        // For now, return empty vec as we need to redesign storage
        // This is a placeholder until we properly integrate with PersistentMemory
        Ok(Vec::new())
    }

    /// Get memories by category
    pub async fn get_memories_by_category(&self, agent_id: &str, category: &str) -> Result<Vec<MemoryEntry>> {
        self.search(MemoryQuery {
            agent_id: Some(agent_id.to_string()),
            category: Some(category.to_string()),
            search_text: None,
            tags: Vec::new(),
            min_importance: None,
            limit: None,
            sort_by: MemorySortBy::Importance,
        }).await
    }

    /// Update existing memory
    pub async fn update(&self, memory_id: &str, value: &str) -> Result<()> {
        if let Some(mut entry) = self.retrieve(memory_id).await? {
            entry.value = value.to_string();
            entry.last_accessed = chrono::Utc::now();
            entry.importance = self.calculate_importance(&entry.category, value);
            entry.tags = self.extract_tags(value);

            // Update storage
            let storage_key = format!("memory:{}", memory_id);
            self.storage.store(&storage_key, &serde_json::to_string(&entry)?).await?;

            // Update cache
            let mut cache = self.cache.write().await;
            cache.insert(memory_id.to_string(), entry.clone());

            // Record activity
            self.record_activity(MemoryActivity {
                timestamp: chrono::Utc::now(),
                activity_type: MemoryActivityType::Updated,
                agent_id: entry.agent_id,
                memory_id: memory_id.to_string(),
                context: Some("Memory content updated".to_string()),
            }).await;

            tracing::debug!("📝 Updated memory: {}", memory_id);
        }

        Ok(())
    }

    /// Delete memory
    pub async fn delete(&self, memory_id: &str) -> Result<()> {
        let storage_key = format!("memory:{}", memory_id);
        
        // Remove from storage
        self.storage.delete(&storage_key).await?;

        // Remove from cache
        let mut cache = self.cache.write().await;
        if let Some(entry) = cache.remove(memory_id) {
            // Record activity
            self.record_activity(MemoryActivity {
                timestamp: chrono::Utc::now(),
                activity_type: MemoryActivityType::Deleted,
                agent_id: entry.agent_id,
                memory_id: memory_id.to_string(),
                context: None,
            }).await;
        }

        tracing::debug!("🗑️ Deleted memory: {}", memory_id);
        Ok(())
    }

    /// Calculate importance score based on content and category
    fn calculate_importance(&self, category: &str, content: &str) -> f64 {
        let base_importance: f64 = match category {
            "investment" => 0.9,
            "business" => 0.8,
            "personal" => 0.7,
            "system" => 0.5,
            _ => 0.6,
        };

        // Adjust based on content characteristics
        let content_multiplier: f64 = if content.contains("$") || content.contains("profit") || content.contains("loss") {
            1.2 // Financial content is more important
        } else if content.len() > 200 {
            1.1 // Longer content might be more detailed
        } else {
            1.0
        };

        (base_importance * content_multiplier).min(1.0)
    }

    /// Extract tags from content using simple heuristics
    fn extract_tags(&self, content: &str) -> Vec<String> {
        let mut tags = Vec::new();
        let content_lower = content.to_lowercase();

        // Financial tags
        if content_lower.contains("invest") || content_lower.contains("portfolio") {
            tags.push("investment".to_string());
        }
        if content_lower.contains("profit") || content_lower.contains("revenue") {
            tags.push("financial".to_string());
        }
        if content_lower.contains("campaign") || content_lower.contains("marketing") {
            tags.push("marketing".to_string());
        }
        if content_lower.contains("risk") || content_lower.contains("warning") {
            tags.push("risk".to_string());
        }

        // Extract currency amounts as tags
        if content.contains("$") {
            tags.push("monetary".to_string());
        }

        tags
    }

    /// Check if memory entry matches query criteria
    fn matches_query(&self, entry: &MemoryEntry, query: &MemoryQuery) -> bool {
        // Agent ID filter
        if let Some(ref agent_id) = query.agent_id {
            if entry.agent_id != *agent_id {
                return false;
            }
        }

        // Category filter
        if let Some(ref category) = query.category {
            if entry.category != *category {
                return false;
            }
        }

        // Search text filter
        if let Some(ref search_text) = query.search_text {
            let search_lower = search_text.to_lowercase();
            if !entry.key.to_lowercase().contains(&search_lower) 
                && !entry.value.to_lowercase().contains(&search_lower) {
                return false;
            }
        }

        // Tags filter
        if !query.tags.is_empty() {
            let has_matching_tag = query.tags.iter()
                .any(|tag| entry.tags.contains(tag));
            if !has_matching_tag {
                return false;
            }
        }

        // Importance threshold
        if let Some(min_importance) = query.min_importance {
            if entry.importance < min_importance {
                return false;
            }
        }

        true
    }

    /// Sort memories based on criteria
    fn sort_memories(&self, memories: &mut Vec<MemoryEntry>, sort_by: MemorySortBy) {
        match sort_by {
            MemorySortBy::CreatedAt => {
                memories.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            }
            MemorySortBy::LastAccessed => {
                memories.sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));
            }
            MemorySortBy::Importance => {
                memories.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap_or(std::cmp::Ordering::Equal));
            }
            MemorySortBy::AccessCount => {
                memories.sort_by(|a, b| b.access_count.cmp(&a.access_count));
            }
            MemorySortBy::Relevance => {
                // Combine importance and access count for relevance
                memories.sort_by(|a, b| {
                    let score_a = a.importance + (a.access_count as f64 * 0.1);
                    let score_b = b.importance + (b.access_count as f64 * 0.1);
                    score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
    }

    /// Update statistics after creating a memory
    async fn update_stats_after_create(&self, entry: &MemoryEntry) {
        let mut stats = self.stats.write().await;
        
        stats.total_entries += 1;
        
        *stats.entries_by_category.entry(entry.category.clone()).or_insert(0) += 1;
        *stats.entries_by_agent.entry(entry.agent_id.clone()).or_insert(0) += 1;
        
        // Recalculate average importance
        stats.avg_importance = (stats.avg_importance * (stats.total_entries - 1) as f64 + entry.importance) / stats.total_entries as f64;
    }

    /// Update access statistics
    async fn update_access_stats(&self, memory_id: &str) {
        // This would be implemented to track most accessed memories
        // For now, just a placeholder
        tracing::trace!("📊 Access recorded for memory: {}", memory_id);
    }

    /// Record memory activity
    async fn record_activity(&self, activity: MemoryActivity) {
        let mut stats = self.stats.write().await;
        stats.recent_activity.push(activity);

        // Keep only recent activities (last 100)
        if stats.recent_activity.len() > 100 {
            stats.recent_activity.remove(0);
        }
    }

    /// Get memory statistics
    pub async fn get_stats(&self) -> MemoryStats {
        self.stats.read().await.clone()
    }

    /// Cleanup old or unused memories
    pub async fn cleanup(&self, _retention_days: u32, _min_access_count: u32) -> Result<u32> {
        // Placeholder for cleanup functionality
        // Would need to implement proper cleanup using PersistentMemory's methods
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_store_creation() {
        let persistent_memory = Arc::new(PersistentMemory::new("test_memory.db").unwrap());
        let memory_store = MemoryStore::new(persistent_memory).await.unwrap();
        
        // Test basic store operation
        let result = memory_store.store(
            "TEST-AGENT",
            "investment",
            "last_purchase",
            "Bought 1000 shares of FDF-SEA at $2.45"
        ).await;
        
        assert!(result.is_ok());
    }
}
/// 💾 Persistent Memory Service
///
/// Provides persistent storage for conversation context using sled database.
/// This replaces the in-memory BotMemory with a disk-backed solution.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::intent_handler::Context;

/// Conversation entry stored in database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationEntry {
    pub user_id: String,
    pub message: String,
    pub intent: String,
    pub timestamp: i64,
    pub entities: Vec<String>,
}

/// Persistent memory backend using sled
pub struct PersistentMemory {
    db: sled::Db,
}

impl PersistentMemory {
    /// Create a new persistent memory instance
    ///
    /// # Arguments
    /// * `path` - Path to the database directory
    ///
    /// # Example
    /// ```
    /// let memory = PersistentMemory::new("./data/memory")?;
    /// ```
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();
        let db = sled::open(path_ref)?;
        tracing::info!("📦 Persistent memory initialized at: {:?}", path_ref);
        Ok(Self { db })
    }

    /// Save conversation context
    pub async fn save_context(&self, user_id: &str, ctx: &Context) -> Result<()> {
        let entry = ConversationEntry {
            user_id: ctx.user_id.clone(),
            message: ctx.message.clone(),
            intent: ctx.intent.clone(),
            timestamp: chrono::Utc::now().timestamp(),
            entities: ctx.entities.clone(),
        };

        let key = format!("ctx:{}:{}", user_id, entry.timestamp);
        let value = bincode::serialize(&entry)?;
        
        self.db.insert(key.as_bytes(), value)?;
        self.db.flush_async().await?;

        tracing::debug!(target: "memory", "💾 Saved context for user: {}", user_id);
        Ok(())
    }

    /// Get conversation history for a user
    ///
    /// # Arguments
    /// * `user_id` - User identifier
    /// * `limit` - Maximum number of entries to return (most recent first)
    pub async fn get_history(&self, user_id: &str, limit: usize) -> Result<Vec<ConversationEntry>> {
        let prefix = format!("ctx:{}:", user_id);
        let mut entries = Vec::new();

        for item in self.db.scan_prefix(prefix.as_bytes()) {
            let (_key, value) = item?;
            let entry: ConversationEntry = bincode::deserialize(&value)?;
            entries.push(entry);
        }

        // Sort by timestamp (newest first)
        entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        entries.truncate(limit);

        tracing::debug!(target: "memory", "📖 Retrieved {} entries for user: {}", entries.len(), user_id);
        Ok(entries)
    }

    /// Clear history for a user
    pub async fn clear(&self, user_id: &str) -> Result<()> {
        let prefix = format!("ctx:{}:", user_id);
        let mut batch = sled::Batch::default();

        for item in self.db.scan_prefix(prefix.as_bytes()) {
            let (key, _) = item?;
            batch.remove(key);
        }

        self.db.apply_batch(batch)?;
        self.db.flush_async().await?;

        tracing::info!(target: "memory", "🗑️  Cleared history for user: {}", user_id);
        Ok(())
    }

    /// Get total number of conversations
    pub fn total_conversations(&self) -> usize {
        self.db.len()
    }

    /// Save user preference
    pub async fn save_preference(&self, user_id: &str, key: &str, value: &str) -> Result<()> {
        let pref_key = format!("pref:{}:{}", user_id, key);
        self.db.insert(pref_key.as_bytes(), value.as_bytes())?;
        self.db.flush_async().await?;

        tracing::debug!(target: "memory", "💡 Saved preference for {}: {}={}", user_id, key, value);
        Ok(())
    }

    /// Get user preference
    pub async fn get_preference(&self, user_id: &str, key: &str) -> Result<Option<String>> {
        let pref_key = format!("pref:{}:{}", user_id, key);
        
        if let Some(value) = self.db.get(pref_key.as_bytes())? {
            let pref = String::from_utf8(value.to_vec())?;
            Ok(Some(pref))
        } else {
            Ok(None)
        }
    }

    /// Get database stats
    pub fn stats(&self) -> (usize, usize) {
        let total = self.db.len();
        let size_on_disk = self.db.size_on_disk().unwrap_or(0);
        (total, size_on_disk as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_save_and_retrieve() {
        let dir = tempdir().unwrap();
        let memory = PersistentMemory::new(dir.path()).unwrap();

        let ctx = Context::new(
            "user123".into(),
            "hello".into(),
            "greeting".into(),
        );

        memory.save_context("user123", &ctx).await.unwrap();
        
        let history = memory.get_history("user123", 10).await.unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].message, "hello");
    }

    #[tokio::test]
    async fn test_preferences() {
        let dir = tempdir().unwrap();
        let memory = PersistentMemory::new(dir.path()).unwrap();

        memory.save_preference("user456", "favorite", "sushi").await.unwrap();
        
        let pref = memory.get_preference("user456", "favorite").await.unwrap();
        assert_eq!(pref, Some("sushi".to_string()));
    }

    #[tokio::test]
    async fn test_clear_history() {
        let dir = tempdir().unwrap();
        let memory = PersistentMemory::new(dir.path()).unwrap();

        let ctx = Context::new("user789".into(), "test".into(), "test".into());
        memory.save_context("user789", &ctx).await.unwrap();

        memory.clear("user789").await.unwrap();
        
        let history = memory.get_history("user789", 10).await.unwrap();
        assert_eq!(history.len(), 0);
    }
}

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Простая память бота для хранения контекста диалогов
#[derive(Clone)]
pub struct BotMemory {
    /// Хранилище контекста по ID пользователя
    contexts: Arc<RwLock<HashMap<String, UserContext>>>,
}

/// Контекст пользователя
#[derive(Debug, Clone)]
pub struct UserContext {
    /// История последних сообщений
    pub message_history: Vec<String>,
    
    /// Последнее намерение
    pub last_intent: Option<String>,
    
    /// Предпочтения пользователя
    pub preferences: HashMap<String, String>,
    
    /// Временные данные сессии
    #[allow(dead_code)]
    pub session_data: HashMap<String, String>,
    
    /// Счётчик сообщений
    pub message_count: usize,
    
    /// 🔄 Состояние диалога (для контекстных разговоров)
    #[allow(dead_code)]
    pub conversation_state: Option<String>,
}

impl Default for UserContext {
    fn default() -> Self {
        Self {
            message_history: Vec::new(),
            last_intent: None,
            preferences: HashMap::new(),
            session_data: HashMap::new(),
            message_count: 0,
            conversation_state: None,  // 🔄 Изначально нет состояния
        }
    }
}

impl BotMemory {
    /// Создать новую память бота
    pub fn new() -> Self {
        Self {
            contexts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Получить контекст пользователя
    pub async fn get_context(&self, user_id: &str) -> UserContext {
        let contexts = self.contexts.read().await;
        contexts.get(user_id).cloned().unwrap_or_default()
    }
    
    /// Обновить контекст пользователя
    pub async fn update_context<F>(&self, user_id: &str, updater: F)
    where
        F: FnOnce(&mut UserContext),
    {
        let mut contexts = self.contexts.write().await;
        let context = contexts.entry(user_id.to_string()).or_default();
        updater(context);
    }
    
    /// Добавить сообщение в историю
    pub async fn add_message(&self, user_id: &str, message: String) {
        self.update_context(user_id, |ctx| {
            ctx.message_history.push(message);
            ctx.message_count += 1;
            
            // Ограничиваем историю последними 10 сообщениями
            if ctx.message_history.len() > 10 {
                ctx.message_history.remove(0);
            }
        }).await;
    }
    
    /// Сохранить последнее намерение
    pub async fn set_last_intent(&self, user_id: &str, intent: String) {
        self.update_context(user_id, |ctx| {
            ctx.last_intent = Some(intent);
        }).await;
    }
    
    /// Получить последнее намерение
    #[allow(dead_code)]
    pub async fn get_last_intent(&self, user_id: &str) -> Option<String> {
        let context = self.get_context(user_id).await;
        context.last_intent
    }
    
    /// Сохранить предпочтение пользователя
    #[allow(dead_code)]
    pub async fn set_preference(&self, user_id: &str, key: String, value: String) {
        self.update_context(user_id, |ctx| {
            ctx.preferences.insert(key, value);
        }).await;
    }
    
    /// Получить предпочтение пользователя
    pub async fn get_preference(&self, user_id: &str, key: &str) -> Option<String> {
        let context = self.get_context(user_id).await;
        context.preferences.get(key).cloned()
    }
    
    /// 🧠 Автоматически извлечь и сохранить предпочтения из текста
    pub async fn extract_and_save_preferences(&self, user_id: &str, text: &str) {
        let text_lower = text.to_lowercase();
        
        // Определяем предпочтение по остроте
        if text_lower.contains("острое") || text_lower.contains("остр") || text_lower.contains("чили") || text_lower.contains("spicy") {
            self.set_preference(user_id, "spicy".to_string(), "true".to_string()).await;
            tracing::info!("🌶️ Запомнил: пользователь {} любит острое", user_id);
        }
        
        // Диета / ПП
        if text_lower.contains("диета") || text_lower.contains(" пп ") || text_lower.contains("легкое") || text_lower.contains("калории") {
            self.set_preference(user_id, "healthy".to_string(), "true".to_string()).await;
            tracing::info!("🥗 Запомнил: пользователь {} предпочитает здоровое питание", user_id);
        }
        
        // Вегетарианство
        if text_lower.contains("вегетариан") || text_lower.contains("без мяса") || text_lower.contains("vegetarian") {
            self.set_preference(user_id, "vegetarian".to_string(), "true".to_string()).await;
            tracing::info!("🌱 Запомнил: пользователь {} вегетарианец", user_id);
        }
        
        // Любовь к морепродуктам
        if text_lower.contains("креветки") || text_lower.contains("shrimp") || text_lower.contains("prawns") {
            self.set_preference(user_id, "favorite".to_string(), "shrimp".to_string()).await;
            tracing::info!("🦐 Запомнил: пользователь {} любит креветки", user_id);
        }
        if text_lower.contains("лосось") || text_lower.contains("salmon") {
            self.set_preference(user_id, "favorite".to_string(), "salmon".to_string()).await;
            tracing::info!("🐟 Запомнил: пользователь {} любит лосося", user_id);
        }
        
        // Компания / праздник
        if text_lower.contains("компания") || text_lower.contains("праздник") || text_lower.contains("гостей") || text_lower.contains("celebration") {
            self.set_preference(user_id, "occasion".to_string(), "party".to_string()).await;
            tracing::info!("🎉 Запомнил: пользователь {} планирует праздник", user_id);
        }
    }
    
    /// 💡 Получить контекстную подсказку для рекомендаций
    pub async fn get_recommendation_context(&self, user_id: &str) -> Option<String> {
        let context = self.get_context(user_id).await;
        
        // Собираем все предпочтения в одну строку
        let mut hints = Vec::new();
        
        if context.preferences.get("spicy") == Some(&"true".to_string()) {
            hints.push("острое");
        }
        if context.preferences.get("healthy") == Some(&"true".to_string()) {
            hints.push("диета");
        }
        if context.preferences.get("vegetarian") == Some(&"true".to_string()) {
            hints.push("вегетарианское");
        }
        if let Some(fav) = context.preferences.get("favorite") {
            hints.push(fav.as_str());
        }
        if context.preferences.get("occasion") == Some(&"party".to_string()) {
            hints.push("праздник");
        }
        
        if hints.is_empty() {
            None
        } else {
            Some(hints.join(", "))
        }
    }
    
    /// 📝 Получить краткую сводку о пользователе для персонализации
    #[allow(dead_code)]  // Зарезервировано для будущих фич
    pub async fn get_user_summary(&self, user_id: &str) -> String {
        let context = self.get_context(user_id).await;
        
        let mut summary = Vec::new();
        
        if context.message_count == 0 {
            return "Новый пользователь".to_string();
        }
        
        summary.push(format!("Сообщений: {}", context.message_count));
        
        if !context.preferences.is_empty() {
            let prefs: Vec<String> = context.preferences
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            summary.push(format!("Предпочтения: {}", prefs.join(", ")));
        }
        
        if let Some(intent) = &context.last_intent {
            summary.push(format!("Последнее: {}", intent));
        }
        
        summary.join(" | ")
    }
    
    /// Сохранить данные сессии
    #[allow(dead_code)]
    pub async fn set_session_data(&self, user_id: &str, key: String, value: String) {
        self.update_context(user_id, |ctx| {
            ctx.session_data.insert(key, value);
        }).await;
    }
    
    /// Получить данные сессии
    #[allow(dead_code)]
    pub async fn get_session_data(&self, user_id: &str, key: &str) -> Option<String> {
        let context = self.get_context(user_id).await;
        context.session_data.get(key).cloned()
    }
    
    /// Очистить сессию пользователя
    #[allow(dead_code)]
    pub async fn clear_session(&self, user_id: &str) {
        self.update_context(user_id, |ctx| {
            ctx.session_data.clear();
        }).await;
    }
    
    /// Получить историю сообщений
    #[allow(dead_code)]
    pub async fn get_history(&self, user_id: &str) -> Vec<String> {
        let context = self.get_context(user_id).await;
        context.message_history
    }
    
    /// Получить количество сообщений пользователя
    pub async fn get_message_count(&self, user_id: &str) -> usize {
        let context = self.get_context(user_id).await;
        context.message_count
    }
    
    /// 👤 Установить имя пользователя (сохраняется при аутентификации)
    pub async fn set_user_name(&self, user_id: &str, name: String) {
        self.set_preference(user_id, "user_name".to_string(), name).await;
        tracing::info!("👤 Сохранено имя пользователя {} в память", user_id);
    }
    
    /// 👤 Получить имя пользователя из памяти
    pub async fn get_user_name(&self, user_id: &str) -> Option<String> {
        self.get_preference(user_id, "user_name").await
    }
    
    /// ❤️ Установить эмоциональное состояние пользователя
    pub async fn set_emotional_state(&self, user_id: &str, mood: &str, emotion: Option<&str>) {
        self.update_context(user_id, |ctx| {
            ctx.preferences.insert("last_mood".to_string(), mood.to_string());
            if let Some(em) = emotion {
                ctx.preferences.insert("last_emotion".to_string(), em.to_string());
            }
        }).await;
        tracing::info!("❤️ Сохранено настроение {} = {}, эмоция: {:?}", user_id, mood, emotion);
    }
    
    /// ❤️ Получить последнее настроение пользователя
    pub async fn get_last_mood(&self, user_id: &str) -> Option<String> {
        self.get_preference(user_id, "last_mood").await
    }
    
    /// ❤️ Получить последнюю эмоцию пользователя
    #[allow(dead_code)]  // Зарезервировано для будущих фич
    pub async fn get_last_emotion(&self, user_id: &str) -> Option<String> {
        self.get_preference(user_id, "last_emotion").await
    }
    
    /// Проверить, новый ли пользователь
    #[allow(dead_code)]
    pub async fn is_new_user(&self, user_id: &str) -> bool {
        let contexts = self.contexts.read().await;
        !contexts.contains_key(user_id)
    }
    
    /// Удалить контекст пользователя
    #[allow(dead_code)]
    pub async fn remove_context(&self, user_id: &str) {
        let mut contexts = self.contexts.write().await;
        contexts.remove(user_id);
    }
}

/// Статистика памяти
#[derive(Debug)]
#[allow(dead_code)]
pub struct MemoryStats {
    pub total_users: usize,
    pub total_messages: usize,
}

impl Default for BotMemory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_operations() {
        let memory = BotMemory::new();
        let user_id = "user123";
        
        // Проверка нового пользователя
        assert!(memory.is_new_user(user_id).await);
        
        // Добавление сообщения
        memory.add_message(user_id, "Hello".to_string()).await;
        assert!(!memory.is_new_user(user_id).await);
        
        // Проверка счётчика
        assert_eq!(memory.get_message_count(user_id).await, 1);
        
        // Сохранение предпочтения
        memory.set_preference(user_id, "cuisine".to_string(), "seafood".to_string()).await;
        assert_eq!(
            memory.get_preference(user_id, "cuisine").await,
            Some("seafood".to_string())
        );
    }
}

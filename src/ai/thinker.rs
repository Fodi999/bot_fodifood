//! Cognitive Layer - Emotional analysis, personalization, and Groq AI integration
//! 
//! This module serves as the "consciousness" of FodiFood AI:
//! - Groq integration for LLM-powered reasoning
//! - Emotional analysis and mood detection  
//! - Context extraction and personalization
//! - Complexity analysis
//! - Activity logging for debugging and monitoring

use crate::ai::core::{query_groq_with_config, query_groq_with_system, GroqConfig, GroqModel};
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

/// Analyzer for mood, emotions, and AI-powered reasoning
pub struct Thinker;

impl Thinker {
    /// 📝 Logs AI activity to file for debugging and monitoring
    /// 
    /// Creates/appends to ai_activity.log in the project root
    fn log_activity(prompt: &str, response: &str) {
        if let Ok(mut log) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("ai_activity.log")
        {
            let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
            let _ = writeln!(log, "------------------------------------------------------------");
            let _ = writeln!(log, "⏰ Timestamp: {}", timestamp);
            let _ = writeln!(log, "🧠 Prompt: {}", prompt);
            let _ = writeln!(log, "💬 Response: {}", response);
            let _ = writeln!(log, "");
        }
    }
    /// 🎭 Определяет настроение пользователя по тексту
    pub fn detect_mood(message: &str) -> &'static str {
        let text = message.to_lowercase();

        // Позитивные маркеры
        if text.contains("спасибо")
            || text.contains("классно")
            || text.contains("отлично")
            || text.contains("супер")
            || text.contains("круто")
            || text.contains("thanks")
            || text.contains("great")
        {
            return "positive";
        }

        // Негативные маркеры
        if text.contains("плохо")
            || text.contains("ужас")
            || text.contains("не нравится")
            || text.contains("разочаров")
            || text.contains("terrible")
            || text.contains("bad")
        {
            return "negative";
        }

        // Нейтральное
        "neutral"
    }

    /// 😋 Извлекает эмоцию или физическое состояние
    pub fn extract_emotion(message: &str) -> Option<&'static str> {
        let lower = message.to_lowercase();

        if lower.contains("голод")
            || lower.contains("есть хочу")
            || lower.contains("проголодался")
            || lower.contains("hungry")
        {
            return Some("hungry");
        }

        if lower.contains("устал") || lower.contains("tired") || lower.contains("выматывающий день")
        {
            return Some("tired");
        }

        if lower.contains("праздник") || lower.contains("отмечаю") || lower.contains("celebration")
        {
            return Some("celebrating");
        }

        if lower.contains("один") || lower.contains("одиночест") || lower.contains("alone")
        {
            return Some("alone");
        }

        if lower.contains("компания")
            || lower.contains("друзья")
            || lower.contains("гости")
            || lower.contains("with friends")
        {
            return Some("with_company");
        }

        None
    }

    /// 🎨 Комбинирует базовый ответ с эмоциональным контекстом
    pub fn personalize(base: &str, mood: &str, emotion: Option<&str>) -> String {
        let mut response = base.to_string();

        // Реакция на настроение
        match mood {
            "positive" => {
                response.push_str("\n\n😊 Рад, что тебе нравится! Всегда к твоим услугам.");
            }
            "negative" => {
                response.push_str(
                    "\n\n😔 Постараюсь улучшить настроение вкусным блюдом. Позволь помочь!",
                );
            }
            _ => {}
        }

        // Реакция на эмоцию/состояние
        if let Some(emo) = emotion {
            match emo {
                "hungry" => {
                    response.push_str("\n\n🍽️ Похоже, ты проголодался! Хочешь, покажу что-то вкусное прямо сейчас?");
                }
                "tired" => {
                    response.push_str("\n\n☕ После тяжёлого дня рекомендую что-то лёгкое и вкусное — расслабься и наслаждайся!");
                }
                "celebrating" => {
                    response.push_str("\n\n🎉 Праздник — отличный повод попробовать что-то особенное! Покажу наши премиум-сеты?");
                }
                "alone" => {
                    response.push_str("\n\n🍴 Побалуй себя чем-то вкусным — ты это заслужил! Порционные блюда как раз для тебя.");
                }
                "with_company" => {
                    response.push_str("\n\n👥 Отлично! Для компании рекомендую большие порции и сеты — все будут в восторге!");
                }
                _ => {}
            }
        }

        response
    }

    /// 🔍 Извлекает ключевые слова для контекста
    #[allow(dead_code)] // Используется для улучшенных рекомендаций
    pub fn extract_keywords(message: &str) -> Vec<String> {
        let text = message.to_lowercase();
        let mut keywords = Vec::new();

        // Вкусовые предпочтения
        let flavor_words = vec![
            "острое",
            "сладкое",
            "солёное",
            "кислое",
            "пряное",
            "spicy",
            "sweet",
            "salty",
            "sour",
        ];

        for word in flavor_words {
            if text.contains(word) {
                keywords.push(word.to_string());
            }
        }

        // Типы блюд
        let dish_types = vec![
            "суп",
            "салат",
            "закуска",
            "десерт",
            "напиток",
            "soup",
            "salad",
            "appetizer",
            "dessert",
            "drink",
            "паэлья",
            "креветки",
            "лосось",
            "тунец",
            "paella",
            "shrimp",
            "salmon",
            "tuna",
        ];

        for word in dish_types {
            if text.contains(word) {
                keywords.push(word.to_string());
            }
        }

        keywords
    }

    /// 🔍 Извлекает ингредиент из запроса пользователя
    pub fn extract_ingredient(message: &str) -> Option<String> {
        let text = message.to_lowercase();

        // Список популярных ингредиентов
        let ingredients = [
            "лосось",
            "salmon",
            "креветки",
            "креветка",
            "shrimp",
            "prawns",
            "тунец",
            "tuna",
            "осьминог",
            "octopus",
            "мидии",
            "mussels",
            "кальмар",
            "кальмары",
            "squid",
            "рыба",
            "fish",
            "морепродукты",
            "seafood",
            "авокадо",
            "avocado",
            "овощи",
            "vegetables",
            "рис",
            "rice",
            "лапша",
            "noodles",
        ];

        for ingredient in ingredients {
            if text.contains(ingredient) {
                return Some(ingredient.to_string());
            }
        }

        None
    }

    /// 🍽️ Извлекает название продукта/блюда из запроса
    pub fn extract_product(message: &str) -> Option<String> {
        let text = message.to_lowercase();

        // Список типичных блюд (можно расширить)
        let products = [
            "паэлья",
            "paella",
            "креветки",
            "shrimp",
            "лосось",
            "salmon",
            "тунец",
            "tuna",
            "том-ям",
            "tom yam",
            "салат",
            "salad",
            "тартар",
            "tartar",
            "кальмар",
            "кальмары",
            "squid",
            "ролл",
            "роллы",
            "roll",
            "суши",
            "sushi",
            "футомаки",
            "futomaki",
            "урамаки",
            "uramaki",
            "нигири",
            "nigiri",
            "темпура",
            "tempura",
            "кока-кола",
            "coca-cola",
            "кола",
            "cola",
        ];

        for product in products {
            if text.contains(product) {
                return Some(product.to_string());
            }
        }

        None
    }

    /// 💬 Определяет тип диалога (вопрос, заказ, светская беседа)
    pub fn detect_conversation_type(message: &str) -> &'static str {
        let text = message.to_lowercase();

        // Вопросительные слова
        if text.contains("что")
            || text.contains("где")
            || text.contains("когда")
            || text.contains("сколько")
            || text.contains("how")
            || text.contains("what")
            || text.contains("where")
        {
            return "question";
        }

        // Заказ
        if text.contains("закажу")
            || text.contains("хочу заказать")
            || text.contains("оформить")
            || text.contains("i want to order")
            || text.contains("order")
        {
            return "order";
        }

        // Светская беседа
        if text.contains("как дела")
            || text.contains("что делаешь")
            || text.contains("привет")
            || text.contains("hello")
            || text.contains("how are you")
        {
            return "smalltalk";
        }

        // Жалоба
        if text.contains("не пришел")
            || text.contains("проблема")
            || text.contains("complaint")
            || text.contains("issue")
        {
            return "complaint";
        }

        "general"
    }

    /// 🧮 Анализирует "вес" запроса (простой/сложный)
    #[allow(dead_code)] // Используется для логирования сложности
    pub fn analyze_complexity(message: &str) -> &'static str {
        let words: Vec<&str> = message.split_whitespace().collect();

        if words.len() <= 3 {
            "simple" // "Покажи меню"
        } else if words.len() <= 10 {
            "medium" // "Хочу что-то острое с креветками"
        } else {
            "complex" // Длинный детальный запрос
        }
    }

    /// 🎯 Генерирует персонализированное обращение
    #[allow(dead_code)] // Зарезервировано для персонализированных приветствий
    pub fn generate_greeting(user_name: Option<&str>, message_count: usize) -> String {
        match (user_name, message_count) {
            (Some(name), 1) => format!("👋 Привет, {}! Рад познакомиться!", name),
            (Some(name), _) => format!("👋 Снова здравствуй, {}!", name),
            (None, 1) => "👋 Привет! Рад видеть нового гостя!".to_string(),
            (None, _) => "👋 Привет! Рад, что ты вернулся!".to_string(),
        }
    }

    // ================== GROQ AI INTEGRATION ==================

    /// 🧠 Advanced thinking using Groq Llama 3.3 70B
    /// 
    /// This is the main "consciousness" function - use it for complex reasoning
    /// All prompts and responses are logged to ai_activity.log
    /// 
    /// # Examples
    /// ```
    /// let answer = Thinker::think("Analyze customer data and suggest improvements").await?;
    /// ```
    pub async fn think(prompt: &str) -> Result<String> {
        tracing::info!("🧠 Thinking via Groq Llama 3.3 70B...");
        
        let system_prompt = "You are FodiFood AI - an intelligent restaurant assistant. \
                            Be helpful, concise, and friendly. Focus on food, orders, and business analysis.";
        
        match query_groq_with_system(system_prompt, prompt, &GroqConfig::default()).await {
            Ok(response) => {
                tracing::info!("✅ Groq response received ({} chars)", response.len());
                
                // Log activity to file
                Self::log_activity(prompt, &response);
                
                Ok(response)
            }
            Err(e) => {
                tracing::error!("❌ Groq thinking failed: {}", e);
                let fallback = "🤔 Обрабатываю запрос... (AI временно недоступен)";
                
                // Log failure too
                Self::log_activity(prompt, &format!("ERROR: {}", e));
                
                Ok(fallback.to_string())
            }
        }
    }

    /// 🚀 Fast thinking using Groq Llama 3.1 8B (instant)
    /// 
    /// Use this for simple, quick responses where speed matters more than depth
    /// Logged to ai_activity.log with [FAST] tag
    pub async fn think_fast(prompt: &str) -> Result<String> {
        tracing::debug!("⚡ Fast thinking via Groq Llama 3.1 8B...");
        
        let config = GroqConfig {
            model: GroqModel::Llama8B,
            temperature: 0.7,
            max_tokens: 1024,
            top_p: 0.9,
        };
        
        match query_groq_with_config(prompt, &config).await {
            Ok(response) => {
                // Log with FAST tag
                Self::log_activity(&format!("[FAST] {}", prompt), &response);
                Ok(response)
            }
            Err(e) => {
                tracing::warn!("⚠️ Fast thinking failed: {}", e);
                Self::log_activity(&format!("[FAST] {}", prompt), &format!("ERROR: {}", e));
                Ok("Обрабатываю...".to_string())
            }
        }
    }

    /// 💼 Business analysis using Groq
    /// 
    /// Analyzes business data and provides actionable insights
    /// Logged with [BUSINESS] tag
    pub async fn analyze_business(data_summary: &str) -> Result<String> {
        let prompt = format!(
            "Проанализируй бизнес-данные ресторана FodiFood:\n\n{}\n\n\
             Дай конкретные рекомендации по улучшению прибыли, оптимизации меню и управлению запасами.",
            data_summary
        );
        
        let config = GroqConfig {
            model: GroqModel::Llama70B, // Use most powerful for analysis
            temperature: 0.3, // Lower temperature for factual analysis
            max_tokens: 2048,
            top_p: 0.9,
        };
        
        match query_groq_with_system(
            "You are a business analyst specializing in restaurant analytics. \
             Provide data-driven insights and actionable recommendations.",
            &prompt,
            &config
        ).await {
            Ok(analysis) => {
                tracing::info!("📊 Business analysis completed");
                Self::log_activity(&format!("[BUSINESS] {}", prompt), &analysis);
                Ok(analysis)
            }
            Err(e) => {
                tracing::error!("❌ Business analysis failed: {}", e);
                Self::log_activity(&format!("[BUSINESS] {}", prompt), &format!("ERROR: {}", e));
                Ok("📊 Анализ временно недоступен. Попробуйте позже.".to_string())
            }
        }
    }

    /// 🎯 Generate personalized recommendations using AI
    /// 
    /// Takes user context and generates smart recommendations
    /// Logged with [RECOMMEND] tag
    pub async fn get_ai_recommendation(context: &str, user_preferences: Option<&str>) -> Result<String> {
        let prompt = if let Some(prefs) = user_preferences {
            format!(
                "Пользователь спрашивает: {}\n\
                 Его предпочтения: {}\n\n\
                 Порекомендуй блюда из меню FodiFood, учитывая контекст и предпочтения.",
                context, prefs
            )
        } else {
            format!(
                "Пользователь спрашивает: {}\n\n\
                 Порекомендуй подходящие блюда из меню FodiFood.",
                context
            )
        };
        
        let config = GroqConfig {
            model: GroqModel::Llama70B,
            temperature: 0.8, // Higher temperature for creative recommendations
            max_tokens: 1024,
            top_p: 0.95,
        };
        
        match query_groq_with_system(
            "You are a knowledgeable food consultant. Recommend dishes that match user preferences. \
             Be enthusiastic and descriptive about food.",
            &prompt,
            &config
        ).await {
            Ok(recommendation) => {
                Self::log_activity(&format!("[RECOMMEND] {}", prompt), &recommendation);
                Ok(recommendation)
            }
            Err(e) => {
                tracing::error!("❌ AI recommendation failed: {}", e);
                Self::log_activity(&format!("[RECOMMEND] {}", prompt), &format!("ERROR: {}", e));
                Ok("🍕 Рекомендую попробовать наши популярные блюда! Напиши 'меню' чтобы увидеть все.".to_string())
            }
        }
    }

    /// 🔍 Extract structured information using AI
    /// 
    /// Uses Groq to extract specific entities from unstructured text
    pub async fn extract_with_ai(text: &str, entity_type: &str) -> Result<Option<String>> {
        let prompt = format!(
            "Extract {} from this text: \"{}\"\n\
             Return ONLY the extracted value, nothing else. If not found, return 'NONE'.",
            entity_type, text
        );
        
        match Self::think_fast(&prompt).await {
            Ok(result) => {
                let trimmed = result.trim();
                if trimmed.eq_ignore_ascii_case("NONE") || trimmed.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(trimmed.to_string()))
                }
            }
            Err(_) => Ok(None),
        }
    }
    
    /// 🔍 Enhanced entity extraction - returns both ingredient and product
    /// 
    /// This is the advanced version for agent use
    pub fn extract_entities_advanced(text: &str) -> (Option<String>, Option<String>) {
        let ingredient = Self::extract_ingredient(text);
        let product = Self::extract_product(text);
        (ingredient, product)
    }
    
    /// 📊 List all public Thinker functions (for monitoring/debugging)
    pub fn list_public_functions() -> Vec<&'static str> {
        vec![
            "detect_mood",
            "extract_emotion",
            "personalize",
            "extract_keywords",
            "extract_ingredient",
            "extract_product",
            "detect_conversation_type",
            "analyze_complexity",
            "generate_greeting",
            "think",
            "think_fast",
            "analyze_business",
            "get_ai_recommendation",
            "extract_with_ai",
            "extract_entities_advanced",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mood_detection() {
        assert_eq!(Thinker::detect_mood("Спасибо большое!"), "positive");
        assert_eq!(Thinker::detect_mood("Это ужасно"), "negative");
        assert_eq!(Thinker::detect_mood("Покажи меню"), "neutral");
    }

    #[test]
    fn test_emotion_extraction() {
        assert_eq!(Thinker::extract_emotion("Очень голоден!"), Some("hungry"));
        assert_eq!(
            Thinker::extract_emotion("Устал после работы"),
            Some("tired")
        );
        assert_eq!(Thinker::extract_emotion("Просто хочу поесть"), None);
    }

    #[test]
    fn test_conversation_type() {
        assert_eq!(
            Thinker::detect_conversation_type("Что у вас есть?"),
            "question"
        );
        assert_eq!(
            Thinker::detect_conversation_type("Хочу заказать паэлью"),
            "order"
        );
        assert_eq!(Thinker::detect_conversation_type("Как дела?"), "smalltalk");
    }

    #[test]
    fn test_complexity() {
        assert_eq!(Thinker::analyze_complexity("Меню"), "simple");
        assert_eq!(
            Thinker::analyze_complexity("Покажи острые блюда с креветками"),
            "medium"
        );
    }
}

/// 📊 Thinker module statistics
#[derive(Debug, Clone)]
pub struct ThinkerStats {
    pub total_functions: usize,
    pub cognitive_functions: usize,
    pub ai_functions: usize,
    pub security_functions: usize,
}

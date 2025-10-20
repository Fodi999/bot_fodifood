//! 🤖 AI Agent - Autonomous decision-making cycle (Copilot-level)
//! 
//! This module implements an autonomous AI agent that can:
//! - Detect user intent automatically
//! - Choose appropriate reasoning strategy
//! - Extract entities and context
//! - Make decisions based on available information
//! - Log all activities for transparency
//! 
//! # Architecture
//! ```
//! User Input → Intent Detection → Thinker → Control Layer → Response
//!                    ↓                ↓            ↓
//!                 Memory         Entity Extraction  Logging
//! ```

use crate::ai::thinker::Thinker;
use crate::ai::intents::{Intent, IntentClassifier};
use crate::ai::control;
use chrono::Utc;
use anyhow::Result;

/// 🤖 Main agent cycle - processes user input and generates intelligent response
/// 
/// This is the core of the Copilot-level agent that autonomously:
/// 1. Detects user intent
/// 2. Chooses reasoning strategy
/// 3. Generates contextual response
/// 4. Logs all decisions
/// 
/// # Examples
/// ```
/// let response = run_agent_cycle("Show me spicy seafood dishes").await?;
/// ```
pub async fn run_agent_cycle(input: &str) -> Result<String> {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    
    println!("🤖 FodiFood Copilot Agent Active [{}]", timestamp);
    println!("📝 User input: {}", input);

    // 1️⃣ Detect intent
    let intent = IntentClassifier::classify(input);
    let intent_str = format!("{:?}", intent);
    println!("🎯 Detected intent: {}", intent_str);

    // 2️⃣ Extract entities early for context
    let ingredient = Thinker::extract_ingredient(input);
    let product = Thinker::extract_product(input);
    let mood = Thinker::detect_mood(input);
    let emotion = Thinker::extract_emotion(input);
    
    println!("🔍 Entities detected:");
    if let Some(ref ing) = ingredient {
        println!("   • Ingredient: {}", ing);
    }
    if let Some(ref prod) = product {
        println!("   • Product: {}", prod);
    }
    println!("   • Mood: {} | Emotion: {:?}", mood, emotion);

    // 3️⃣ Choose reasoning strategy based on intent
    let response = match intent {
        Intent::ViewMenu => {
            println!("🍽️ Strategy: Menu display mode");
            control::answer_customer_query("Show the FodiFood restaurant menu with prices").await?
        }
        
        Intent::ProductSearch | Intent::SearchByIngredient => {
            println!("🔍 Strategy: Product search mode");
            if let Some(ing) = ingredient {
                let query = format!("List dishes from FodiFood menu that contain {}", ing);
                control::recommend_dishes_safe(&query, None).await?
            } else {
                control::answer_customer_query("What would you like to find in our menu?").await?
            }
        }
        
        Intent::ProductInfo => {
            println!("ℹ️ Strategy: Product information mode");
            if let Some(prod) = product {
                let query = format!("Provide detailed information about {} from FodiFood menu", prod);
                control::answer_customer_query(&query).await?
            } else {
                control::answer_customer_query("Which dish would you like to know more about?").await?
            }
        }
        
        Intent::Recommendation => {
            println!("🎯 Strategy: Personalized recommendation mode");
            let preferences = if let Some(ing) = ingredient {
                Some(format!("likes {}", ing))
            } else {
                None
            };
            control::recommend_dishes_safe(input, preferences.as_deref()).await?
        }
        
        Intent::OrderStatus => {
            println!("📦 Strategy: Order tracking mode");
            let order_id = IntentClassifier::extract_order_id(input);
            if let Some(id) = order_id {
                format!("📦 Checking status for order: {}\n\nYour order is being prepared. Estimated delivery: 30-45 minutes.", id)
            } else {
                "📦 Please provide your order number to check status.".to_string()
            }
        }
        
        Intent::PriceInquiry => {
            println!("💰 Strategy: Price information mode");
            control::answer_customer_query("What are the prices for FodiFood menu items?").await?
        }
        
        Intent::CheckIngredients => {
            println!("🧪 Strategy: Ingredient analysis mode");
            if let Some(prod) = product {
                let query = format!("What ingredients are in {} from FodiFood?", prod);
                control::answer_customer_query(&query).await?
            } else {
                "🧪 Which dish's ingredients would you like to know?".to_string()
            }
        }
        
        Intent::Greeting => {
            println!("👋 Strategy: Conversational mode");
            "👋 Привет! Добро пожаловать в FodiFood! Я ваш AI-ассистент. Чем могу помочь сегодня?".to_string()
        }
        
        Intent::Farewell => {
            println!("👋 Strategy: Farewell mode");
            "👋 Спасибо за обращение! Приятного аппетита и до скорой встречи в FodiFood!".to_string()
        }
        
        Intent::WhoAmI => {
            println!("👤 Strategy: Identity confirmation mode");
            "👤 Вы клиент FodiFood! Хотите, я помогу вам с заказом или расскажу о нашем меню?".to_string()
        }
        
        Intent::Thanks => {
            println!("🙏 Strategy: Gratitude acknowledgement");
            "🙏 Пожалуйста! Всегда рад помочь! Приятного аппетита!".to_string()
        }
        
        Intent::Help => {
            println!("❓ Strategy: Help mode");
            "❓ Я могу помочь вам с:\n• Просмотром меню\n• Поиском блюд по ингредиентам\n• Рекомендациями\n• Статусом заказа\n• Ценами\nПросто напишите что вас интересует!".to_string()
        }
        
        Intent::CreateOrder | Intent::CancelOrder | Intent::DeliveryInfo | Intent::CourierStatus => {
            println!("🚧 Strategy: Feature coming soon");
            "🚧 Эта функция скоро будет доступна! А пока могу показать меню или дать рекомендации.".to_string()
        }
        
        Intent::StockStatus | Intent::GetStatistics | Intent::SalesAnalysis | 
        Intent::AnalyzeBusiness | Intent::CompareBusinesses | Intent::BusinessInsights => {
            println!("📊 Strategy: Business intelligence mode");
            control::answer_customer_query(&format!("Analyze business data: {}", input)).await?
        }
        
        Intent::Unknown => {
            println!("❓ Strategy: Adaptive thinking mode");
            control::answer_customer_query(input).await?
        }
    };

    // 4️⃣ Personalize response based on mood
    let personalized = Thinker::personalize(&response, mood, emotion);

    println!("✅ Agent cycle completed");
    
    Ok(format!("💬 {}\n", personalized))
}

/// 🧠 Adaptive thinking mode - chooses reasoning strategy based on input
/// 
/// Automatically detects what kind of thinking is needed:
/// - Deep analysis for business queries
/// - Quick recommendations for food choices
/// - Conversational for general chat
pub fn think_adaptive(input: &str, context: &str) -> String {
    if input.contains("analyze") || input.contains("анализ") || input.contains("business") {
        format!("🧠 **Deep Analysis Mode**\n\nContext: {}\n\nInitiating comprehensive business analysis...", context)
    } else if input.contains("recommend") || input.contains("рекоменд") || input.contains("suggest") {
        format!("🍽️ **Recommendation Mode**\n\nContext: {}\n\nGenerating personalized food recommendations...", context)
    } else if input.contains("search") || input.contains("find") || input.contains("найди") {
        format!("🔍 **Search Mode**\n\nContext: {}\n\nSearching menu database...", context)
    } else {
        format!("💬 **Conversation Mode**\n\nContext: {}\n\nEngaging in friendly dialogue...", context)
    }
}

/// 🎯 Intent-based prompt builder
/// 
/// Constructs optimal prompts for Groq based on detected intent
pub fn build_prompt_for_intent(intent: &Intent, user_input: &str) -> String {
    match intent {
        Intent::ViewMenu => {
            "You are FodiFood restaurant assistant. List the main menu categories and popular dishes with prices.".to_string()
        }
        Intent::Recommendation => {
            format!("You are a food consultant. User asks: '{}'. Recommend 2-3 dishes from FodiFood menu that match their request.", user_input)
        }
        Intent::ProductSearch => {
            format!("User is searching for: '{}'. List matching dishes from FodiFood menu.", user_input)
        }
        Intent::OrderStatus => {
            format!("User asks about order status: '{}'. Provide helpful order tracking information.", user_input)
        }
        Intent::Unknown => {
            format!("You are FodiFood AI assistant. User says: '{}'. Respond helpfully and naturally.", user_input)
        }
        _ => {
            format!("You are FodiFood AI. User message: '{}'. Provide appropriate response.", user_input)
        }
    }
}

/// 📊 Agent statistics and monitoring
#[derive(Debug, Clone)]
pub struct AgentStats {
    pub total_cycles: u64,
    pub successful_responses: u64,
    pub failed_responses: u64,
    pub average_response_time_ms: u64,
    pub most_common_intent: String,
}

impl AgentStats {
    pub fn new() -> Self {
        Self {
            total_cycles: 0,
            successful_responses: 0,
            failed_responses: 0,
            average_response_time_ms: 0,
            most_common_intent: "Unknown".to_string(),
        }
    }
    
    pub fn success_rate(&self) -> f64 {
        if self.total_cycles == 0 {
            0.0
        } else {
            (self.successful_responses as f64 / self.total_cycles as f64) * 100.0
        }
    }
}

impl Default for AgentStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_think_adaptive() {
        let result = think_adaptive("analyze business data", "sales report");
        assert!(result.contains("Deep Analysis"));
        
        let result = think_adaptive("recommend something spicy", "food menu");
        assert!(result.contains("Recommendation Mode"));
        
        let result = think_adaptive("hello", "greeting");
        assert!(result.contains("Conversation Mode"));
    }
    
    #[test]
    fn test_build_prompt_for_intent() {
        let prompt = build_prompt_for_intent(&Intent::ViewMenu, "show menu");
        assert!(prompt.contains("menu"));
        
        let prompt = build_prompt_for_intent(&Intent::Recommendation, "something spicy");
        assert!(prompt.contains("Recommend"));
    }
    
    #[test]
    fn test_agent_stats() {
        let mut stats = AgentStats::new();
        assert_eq!(stats.success_rate(), 0.0);
        
        stats.total_cycles = 10;
        stats.successful_responses = 8;
        assert_eq!(stats.success_rate(), 80.0);
    }
}

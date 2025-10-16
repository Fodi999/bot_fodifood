pub mod admin_assistant; // 🔧 Admin AI assistant
pub mod analysis; // 💡 AI-powered business analysis
pub mod intent_handler; // 🎯 Intent handler system
mod intents;
mod memory;
pub mod modules;
pub mod persistent_memory; // 💾 Persistent memory service
mod rules;
mod thinker; // 🧠 Новый когнитивный модуль // 📦 Modular intent handlers

use crate::api::go_backend::GoBackendClient;
use crate::config::Config;
use anyhow::Result;

pub use admin_assistant::AdminAssistant;
pub use intent_handler::{IntentHandler, IntentRegistry};
pub use intents::{Intent, IntentClassifier};
pub use memory::BotMemory;
pub use rules::ResponseGenerator;
pub use thinker::Thinker; // Экспортируем для внешнего использования

/// Главный AI движок бота
pub struct AIEngine {
    memory: BotMemory,
    backend: GoBackendClient,
    #[allow(dead_code)] // Used by process_with_plugins and process_with_insights
    intent_registry: IntentRegistry, // 🎯 Plugin system registry
}

impl AIEngine {
    /// Создать новый AI движок
    pub fn new(config: &Config) -> Self {
        // 🎯 Initialize plugin system registry
        let mut registry = IntentRegistry::new();
        modules::register_all_handlers(&mut registry);
        
        tracing::info!("🚀 AIEngine initialized with {} intent handlers", registry.count());
        
        Self {
            memory: BotMemory::new(),
            backend: GoBackendClient::new(config),
            intent_registry: registry,
        }
    }

    /// Получить доступ к памяти
    #[allow(dead_code)]
    pub fn memory(&self) -> &BotMemory {
        &self.memory
    }

    /// Получить доступ к backend клиенту
    #[allow(dead_code)]
    pub fn backend(&self) -> &GoBackendClient {
        &self.backend
    }

    /// 👤 Установить имя пользователя в память (вызывается при авторизации)
    pub async fn set_user_name(&self, user_id: &str, name: String) {
        self.memory.set_user_name(user_id, name).await;
    }

    /// 👤 Получить имя пользователя из памяти
    #[allow(dead_code)] // Используется через WhoAmI intent
    pub async fn get_user_name(&self, user_id: &str) -> Option<String> {
        self.memory.get_user_name(user_id).await
    }

    /// Обработать сообщение и сгенерировать ответ
    pub async fn process_message(&self, user_id: &str, message: &str) -> Result<String> {
        // 💬 ПРОВЕРКА: Светская беседа (smalltalk) — обрабатываем первыми
        if let Some(smalltalk_reply) = rules::smalltalk::respond(message) {
            self.memory.add_message(user_id, message.to_string()).await;
            return Ok(smalltalk_reply);
        }

        // 🧠 КОГНИТИВНЫЙ АНАЛИЗ: Определяем настроение и эмоции
        let mood = Thinker::detect_mood(message);
        let emotion = Thinker::extract_emotion(message);
        let conversation_type = Thinker::detect_conversation_type(message);
        let complexity = Thinker::analyze_complexity(message); // 🧮 Анализ сложности

        tracing::info!(
            "🧠 Cognitive: mood={}, emotion={:?}, type={}, complexity={}",
            mood,
            emotion,
            conversation_type,
            complexity
        );

        // ❤️ СОХРАНЯЕМ ЭМОЦИОНАЛЬНОЕ СОСТОЯНИЕ в память
        self.memory
            .set_emotional_state(user_id, mood, emotion)
            .await;

        // 📝 Извлекаем и сохраняем предпочтения автоматически
        self.memory
            .extract_and_save_preferences(user_id, message)
            .await;

        // Сохраняем сообщение в историю
        self.memory.add_message(user_id, message.to_string()).await;

        // Классифицируем намерение
        let intent = IntentClassifier::classify(message);

        // 🔍 Логируем интент для отладки
        tracing::info!("🧠 Detected Intent: {:?} for message: {}", intent, message);

        // Сохраняем намерение
        self.memory
            .set_last_intent(user_id, format!("{:?}", intent))
            .await;

        // 🔥 ИНТЕГРАЦИЯ С GO BACKEND - проверяем специальные интенты
        match intent {
            // 🍽️ ViewMenu - загружаем реальное меню
            Intent::ViewMenu => {
                tracing::info!("🍽️ AIEngine: ViewMenu detected - fetching real menu");
                match self.backend.get_products().await {
                    Ok(products) => {
                        let formatted = GoBackendClient::format_products_list(&products);
                        tracing::info!("✅ AIEngine: Loaded {} products", products.len());
                        return Ok(formatted);
                    }
                    Err(e) => {
                        tracing::error!("❌ AIEngine: Failed to load menu: {}", e);
                        // Продолжаем с fallback ответом
                    }
                }
            }

            // 🔍 ProductSearch - фильтруем продукты по ингредиенту
            Intent::ProductSearch => {
                if let Some(ingredient) = Thinker::extract_ingredient(message) {
                    tracing::info!("🔍 AIEngine: ProductSearch for: {}", ingredient);
                    match self.backend.get_products().await {
                        Ok(products) => {
                            let filtered =
                                GoBackendClient::filter_by_ingredient(&products, &ingredient);
                            if !filtered.is_empty() {
                                use crate::api::go_backend::Product;
                                let filtered_products: Vec<Product> =
                                    filtered.iter().map(|&p| p.clone()).collect();
                                let response = format!(
                                    "🔍 **Нашёл {} блюд с \"{}\":**\n\n{}",
                                    filtered_products.len(),
                                    ingredient,
                                    GoBackendClient::format_products_list(&filtered_products)
                                );
                                tracing::info!(
                                    "✅ AIEngine: Found {} products",
                                    filtered_products.len()
                                );
                                return Ok(response);
                            }
                        }
                        Err(e) => {
                            tracing::error!("❌ AIEngine: Failed to search products: {}", e);
                        }
                    }
                }
            }

            // ℹ️ ProductInfo - показываем детали продукта
            Intent::ProductInfo => {
                if let Some(product_name) = Thinker::extract_product(message) {
                    tracing::info!("ℹ️ AIEngine: ProductInfo for: {}", product_name);
                    match self.backend.get_products().await {
                        Ok(products) => {
                            if let Some(product) =
                                GoBackendClient::find_product_by_name(&products, &product_name)
                            {
                                let response = format!(
                                    "ℹ️ **{}**\n\n\
                                     💰 **Цена:** {}₽\n\
                                     📦 **Вес/Объём:** {}\n\
                                     📋 **Описание:** {}\n\
                                     🏷️ **Категория:** {}\n\n\
                                     💡 Хочешь заказать? Просто скажи \"беру\" или \"закажу {}\"!",
                                    product.name,
                                    product.price as i32,
                                    product.weight.as_deref().unwrap_or("—"),
                                    product
                                        .description
                                        .as_deref()
                                        .unwrap_or("Вкуснейшее блюдо из свежих ингредиентов"),
                                    product.category.as_deref().unwrap_or("Другое"),
                                    product.name
                                );
                                tracing::info!("✅ AIEngine: Found product: {}", product.name);
                                return Ok(response);
                            }
                        }
                        Err(e) => {
                            tracing::error!("❌ AIEngine: Failed to get product info: {}", e);
                        }
                    }
                }
            }

            // 💰 PriceInquiry - показываем цены
            Intent::PriceInquiry => {
                tracing::info!("💰 AIEngine: PriceInquiry detected");
                match self.backend.get_products().await {
                    Ok(products) => {
                        let response = format!(
                            "💰 **Актуальные цены:**\n\n{}",
                            GoBackendClient::format_products_list(&products)
                        );
                        tracing::info!(
                            "✅ AIEngine: Loaded prices for {} products",
                            products.len()
                        );
                        return Ok(response);
                    }
                    Err(e) => {
                        tracing::error!("❌ AIEngine: Failed to load prices: {}", e);
                    }
                }
            }

            // 🐟 SearchByIngredient - поиск блюд по конкретному ингредиенту
            Intent::SearchByIngredient => {
                tracing::info!("🐟 AIEngine: SearchByIngredient detected");

                // 🎯 Используем IntentClassifier для извлечения ингредиента
                let clean_ingredient = IntentClassifier::extract_ingredient(message);

                tracing::info!(
                    "🔍 Searching for ingredient: \"{}\" (extracted from: \"{}\")",
                    clean_ingredient,
                    message
                );

                match self.backend.get_products().await {
                    Ok(products) => {
                        tracing::info!("✅ Loaded {} products from backend", products.len());

                        let matched =
                            GoBackendClient::filter_by_ingredient(&products, &clean_ingredient);

                        if matched.is_empty() {
                            tracing::info!(
                                "😕 No products found with ingredient: {}",
                                clean_ingredient
                            );
                            return Ok(format!(
                                "😕 Не нашёл блюд с \"{}\". Может, попробуешь что-то другое? \
                                 Напиши \"меню\" чтобы увидеть весь ассортимент!",
                                clean_ingredient
                            ));
                        }

                        tracing::info!(
                            "🎉 Found {} products with ingredient: {}",
                            matched.len(),
                            clean_ingredient
                        );

                        let mut response = format!("🐟 **Блюда с \"{}\":**\n\n", clean_ingredient);
                        for product in &matched {
                            response.push_str(&format!(
                                "• **{}** — {}₽\n",
                                product.name, product.price as i32
                            ));

                            if let Some(desc) = &product.description {
                                if !desc.is_empty() && desc.len() < 100 {
                                    response.push_str(&format!("  _{}_\n", desc));
                                }
                            }

                            if let Some(weight) = &product.weight {
                                if !weight.is_empty() {
                                    response.push_str(&format!("  📦 {}\n", weight));
                                }
                            }

                            response.push_str("\n");
                        }

                        response.push_str("💡 Хочешь добавить что-то из этого в заказ?");

                        tracing::info!("✅ Found {} matching products", matched.len());
                        return Ok(response);
                    }
                    Err(e) => {
                        tracing::error!("❌ AIEngine: Failed to search by ingredient: {}", e);
                        return Ok(format!(
                            "😕 Произошла ошибка при поиске блюд. Попробуй позже или напиши \"меню\" \
                             чтобы увидеть весь ассортимент."
                        ));
                    }
                }
            }

            _ => {
                // Для остальных интентов используем стандартную логику
            }
        }

        // Извлекаем контекст в зависимости от намерения
        let context = match intent {
            Intent::OrderStatus => IntentClassifier::extract_order_id(message),
            Intent::CheckIngredients | Intent::ProductInfo => {
                IntentClassifier::extract_product_name(message)
            }
            Intent::Recommendation => {
                // 🧠 Извлекаем ключевые слова из сообщения
                let keywords = Thinker::extract_keywords(message);

                // 💡 Получаем сохранённые предпочтения
                let saved_context = self.memory.get_recommendation_context(user_id).await;

                // 🔀 Комбинируем: ключевые слова + предпочтения
                match (keywords.is_empty(), saved_context) {
                    (false, Some(prefs)) => Some(format!("{}, {}", keywords.join(", "), prefs)),
                    (false, None) => Some(keywords.join(", ")),
                    (true, Some(prefs)) => Some(prefs),
                    (true, None) => None,
                }
            }
            Intent::ProductSearch => {
                // 🔍 Извлекаем ингредиент из запроса
                Thinker::extract_ingredient(message)
            }
            Intent::SearchByIngredient => {
                // 🐟 Извлекаем ингредиент (возвращаем всё сообщение как ингредиент)
                Some(message.trim().to_lowercase())
            }
            Intent::WhoAmI => {
                // 👤 Получаем имя пользователя из памяти
                self.memory.get_user_name(user_id).await
            }
            _ => None,
        };

        // Генерируем базовый ответ
        let base_response = ResponseGenerator::generate(&intent, context.as_deref());

        // 🎨 ПЕРСОНАЛИЗАЦИЯ: Добавляем эмоциональный слой
        let personalized = Thinker::personalize(&base_response, mood, emotion);

        // ❤️ ПРОВЕРЯЕМ ИЗМЕНЕНИЕ НАСТРОЕНИЯ
        let prev_mood = self.memory.get_last_mood(user_id).await;
        let mood_context = if let Some(prev) = prev_mood {
            if prev == "negative" && mood == "positive" {
                Some("\n\n😊 Рад, что настроение улучшилось! Это заслуга хорошей еды?")
            } else if prev == "positive" && mood == "negative" {
                Some("\n\n😔 Вижу, что-то расстроило. Давай исправлю это вкусным предложением!")
            } else {
                None
            }
        } else {
            None
        };

        // Добавляем mood_context к ответу
        let with_mood = if let Some(mood_msg) = mood_context {
            format!("{}{}", personalized, mood_msg)
        } else {
            personalized
        };

        // 🎯 Дополнительная персонализация для новых пользователей
        let final_response = if self.memory.get_message_count(user_id).await == 1 {
            format!(
                "🎉 Добро пожаловать в FodiFood!\n\n{}\n\n\
                 💡 Я запомню ваши предпочтения для персонализированного сервиса!",
                with_mood
            )
        } else {
            with_mood
        };

        Ok(final_response)
    }

    /// 🎯 Process message using new plugin system
    /// This is the new recommended way to process messages
    #[allow(dead_code)] // Will be used when we migrate webhook handler
    pub async fn process_with_plugins(
        &self,
        user_id: &str,
        message: &str,
        state: &crate::state::AppState,
    ) -> Result<String> {
        // 💬 Smalltalk check first (highest priority)
        if let Some(smalltalk_reply) = rules::smalltalk::respond(message) {
            self.memory.add_message(user_id, message.to_string()).await;
            return Ok(smalltalk_reply);
        }

        // 🧠 Cognitive analysis
        let mood = Thinker::detect_mood(message);
        let emotion = Thinker::extract_emotion(message);
        
        tracing::info!(target: "ai", "🧠 Cognitive: mood={}, emotion={:?}", mood, emotion);

        // ❤️ Save emotional state
        self.memory.set_emotional_state(user_id, mood, emotion).await;

        // 📝 Extract and save preferences
        self.memory.extract_and_save_preferences(user_id, message).await;

        // Save message to history
        self.memory.add_message(user_id, message.to_string()).await;

        // 🎯 Classify intent
        let intent = IntentClassifier::classify(message);
        let intent_str = format!("{:?}", intent).to_lowercase();
        
        tracing::info!(target: "ai", "🎯 Classified intent: {} for message: {}", intent_str, message);

        // Save intent
        self.memory.set_last_intent(user_id, intent_str.clone()).await;

        // 🚀 Create context for plugin system
        let mut ctx = intent_handler::Context::new(
            user_id.to_string(),
            message.to_string(),
            intent_str,
        );

        // 📦 Extract entities (simple for now)
        if let Some(ingredient) = Thinker::extract_ingredient(message) {
            ctx = ctx.with_entities(vec![ingredient]);
        } else if let Some(product) = Thinker::extract_product(message) {
            ctx = ctx.with_entities(vec![product]);
        }

        // 🎯 Handle through plugin registry
        let response = self.intent_registry.handle(message, &mut ctx, state).await;

        Ok(response)
    }

    /// Get registry stats (for debugging/monitoring)
    #[allow(dead_code)] // Used for monitoring and debugging
    pub fn registry_stats(&self) -> (usize, Vec<String>) {
        (
            self.intent_registry.count(),
            self.intent_registry.registered_handlers(),
        )
    }

    /// Process message with plugin system AND broadcast insight events
    ///
    /// This is an enhanced version of process_with_plugins that also broadcasts
    /// real-time AI processing events via WebSocket for frontend visibility.
    ///
    /// # Arguments
    /// * `user_id` - User identifier
    /// * `message` - User message
    /// * `state` - Application state (includes insight broadcaster)
    ///
    /// # Returns
    /// * `Result<String>` - AI response
    #[allow(dead_code)] // Will be used when webhook handler is migrated to new system
    pub async fn process_with_insights(
        &self,
        user_id: &str,
        message: &str,
        state: &crate::state::AppState,
    ) -> Result<String> {
        use crate::handlers::{AIInsightEvent, ExtractedEntity};
        use std::collections::HashMap;

        let start_time = std::time::Instant::now();

        // 📡 Event: Classification started
        state.insight_broadcaster.broadcast(
            AIInsightEvent::classification_started(user_id.to_string(), message.to_string())
        );

        // 🎯 Classify intent
        let intent = IntentClassifier::classify(message);
        let intent_str = format!("{:?}", intent);

        // 📡 Event: Intent classified
        state.insight_broadcaster.broadcast(
            AIInsightEvent::classified(
                user_id.to_string(),
                intent_str.clone(),
                0.85, // TODO: Add real confidence scoring
                start_time.elapsed().as_millis() as u64,
            )
        );

        // 🧩 Extract entities
        let entities = match intent {
            Intent::SearchByIngredient => {
                vec![ExtractedEntity {
                    entity_type: "ingredient".to_string(),
                    value: IntentClassifier::extract_ingredient(message),
                    confidence: 0.9,
                }]
            }
            Intent::OrderStatus => {
                if let Some(order_id) = IntentClassifier::extract_order_id(message) {
                    vec![ExtractedEntity {
                        entity_type: "order_id".to_string(),
                        value: order_id,
                        confidence: 0.95,
                    }]
                } else {
                    vec![]
                }
            }
            Intent::ProductInfo => {
                if let Some(product) = IntentClassifier::extract_product_name(message) {
                    vec![ExtractedEntity {
                        entity_type: "product_name".to_string(),
                        value: product,
                        confidence: 0.85,
                    }]
                } else {
                    vec![]
                }
            }
            _ => vec![],
        };

        // 📡 Event: Entity extraction
        if !entities.is_empty() {
            state.insight_broadcaster.broadcast(
                AIInsightEvent::entity_extraction(user_id.to_string(), entities.clone())
            );
        }

        // Create context
        let entity_strings: Vec<String> = entities.iter().map(|e| e.value.clone()).collect();
        let mut ctx = intent_handler::Context::new(
            user_id.to_string(),
            message.to_string(),
            intent_str.clone(),
        ).with_entities(entity_strings);

        // 📡 Event: Context updated
        let mut metadata = HashMap::new();
        metadata.insert("intent".to_string(), intent_str.clone());
        metadata.insert("entity_count".to_string(), entities.len().to_string());
        
        state.insight_broadcaster.broadcast(
            AIInsightEvent::context_updated(
                user_id.to_string(),
                ctx.metadata.len(),
                metadata,
            )
        );

        // 📡 Event: Handler routing
        let handlers = self.intent_registry.registered_handlers();
        state.insight_broadcaster.broadcast(
            AIInsightEvent::handler_routing(
                user_id.to_string(),
                intent_str.clone(),
                handlers.clone(),
            )
        );

        // Route through registry
        let handler_start = std::time::Instant::now();
        
        // Find matching handler
        let handler_name = handlers.iter()
            .find(|h| h.to_lowercase().contains(&intent_str.to_lowercase()))
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());

        // 📡 Event: Handler execution started
        state.insight_broadcaster.broadcast(
            AIInsightEvent::handler_started(
                user_id.to_string(),
                handler_name.clone(),
                100, // TODO: Get real priority from handler
            )
        );

        let response = self.intent_registry.handle(message, &mut ctx, state).await;

        // 📡 Event: Handler execution completed
        state.insight_broadcaster.broadcast(
            AIInsightEvent::handler_completed(
                user_id.to_string(),
                handler_name,
                !response.contains("🤔"), // Success if not confused
                response.chars().count(),
                handler_start.elapsed().as_millis() as u64,
            )
        );

        // 📊 Record metrics
        state.metrics.record_intent(&intent_str);
        state.metrics.record_response_time(&intent_str, start_time.elapsed());
        state.metrics.record_success(&intent_str);

        // 📡 Event: Processing completed
        state.insight_broadcaster.broadcast(
            AIInsightEvent::processing_completed(
                user_id.to_string(),
                start_time.elapsed().as_millis() as u64,
                1, // handlers invoked
            )
        );

        Ok(response)
    }
}

impl Default for AIEngine {
    fn default() -> Self {
        // Создаём конфиг по умолчанию для Default
        let config = Config::default();
        Self::new(&config)
    }
}

/// Основная функция для совместимости с текущим API
#[allow(dead_code)] // Устаревшая функция, оставлена для совместимости
pub async fn generate_reply(config: &Config, prompt: &str) -> Result<String> {
    let engine = AIEngine::new(config);
    // Используем фиктивный user_id для совместимости
    engine.process_message("default_user", prompt).await
}

/// Анализ бизнес-данных (встроенный)
#[allow(dead_code)]
pub async fn analyze_data(
    _config: &Config,
    data_description: &str,
    question: &str,
) -> Result<String> {
    let analysis = format!(
        "📊 **Анализ данных:**\n\n\
         📋 Данные: {}\n\
         ❓ Вопрос: {}\n\n\
         💡 **Рекомендации:**\n\
         • Проверьте топ-5 продуктов по продажам\n\
         • Следите за остатками популярных ингредиентов\n\
         • Анализируйте пиковые часы заказов (обычно 12:00-14:00, 18:00-21:00)\n\
         • Планируйте закупки на основе трендов последних 7 дней\n\
         • Используйте сезонные акции для увеличения среднего чека\n\n\
         📈 Для детальной аналитики используйте команду `get_stats`",
        data_description, question
    );

    Ok(analysis)
}

/// Персонализированные рекомендации (встроенные)
#[allow(dead_code)]
pub async fn get_recommendation(_config: &Config, context: &str) -> Result<String> {
    let intent = Intent::Recommendation;
    let response = ResponseGenerator::generate(&intent, Some(context));
    Ok(response)
}

/// 🤖 Standalone order creation function
/// 
/// Создаёт заказ и отправляет уведомление на Go backend
///
/// Пример использования:
/// ```
/// create_order("ORD-12345", 2500.0).await?;
/// ```
#[allow(dead_code)]
pub async fn create_order(order_id: &str, total: f64) -> Result<String> {
    tracing::info!(
        "🤖 AI: Создаю заказ {} на сумму {:.2} руб.",
        order_id,
        total
    );

    // Отправляем заказ на Go backend
    use crate::api::go_backend;
    match go_backend::send_order_to_backend(order_id, total).await {
        Ok(_) => {
            tracing::info!("✅ Заказ {} успешно отправлен на backend", order_id);
            Ok(format!(
                "✅ **Заказ создан!**\n\n\
                 📦 Номер заказа: `{}`\n\
                 💰 Сумма: {:.2} руб.\n\n\
                 Уведомление отправлено на сервер. Вы получите обновления о статусе заказа.",
                order_id, total
            ))
        }
        Err(e) => {
            tracing::error!("❌ Ошибка отправки заказа на backend: {:?}", e);
            Ok(format!(
                "⚠️ Заказ {} создан локально, но не удалось связаться с сервером.\n\
                 Попробуйте позже или обратитесь в поддержку.",
                order_id
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_engine_greeting() {
        let config = Config::default();
        let engine = AIEngine::new(&config);
        let response = engine
            .process_message("test_user", "Привет!")
            .await
            .unwrap();
        assert!(response.contains("Привет") || response.contains("Добро пожаловать"));
    }

    #[tokio::test]
    async fn test_ai_engine_menu() {
        let config = Config::default();
        let engine = AIEngine::new(&config);
        let response = engine
            .process_message("test_user", "покажи меню")
            .await
            .unwrap();
        assert!(response.contains("меню") || response.contains("Меню"));
    }

    #[tokio::test]
    async fn test_intent_classification() {
        assert_eq!(IntentClassifier::classify("привет"), Intent::Greeting);
        assert_eq!(IntentClassifier::classify("покажи меню"), Intent::ViewMenu);
        assert_eq!(
            IntentClassifier::classify("статус заказа"),
            Intent::OrderStatus
        );
    }
}

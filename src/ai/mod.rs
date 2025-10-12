mod intents;
mod rules;
mod memory;
mod thinker;  // 🧠 Новый когнитивный модуль

use anyhow::Result;
use crate::config::Config;
use crate::api::go_backend;

pub use intents::{Intent, IntentClassifier};
pub use rules::ResponseGenerator;
pub use memory::BotMemory;
pub use thinker::Thinker;  // Экспортируем для внешнего использования

/// Главный AI движок бота
pub struct AIEngine {
    memory: BotMemory,
}

impl AIEngine {
    /// Создать новый AI движок
    pub fn new() -> Self {
        Self {
            memory: BotMemory::new(),
        }
    }
    
    /// Получить доступ к памяти
    #[allow(dead_code)]
    pub fn memory(&self) -> &BotMemory {
        &self.memory
    }
    
    /// 👤 Установить имя пользователя в память (вызывается при авторизации)
    pub async fn set_user_name(&self, user_id: &str, name: String) {
        self.memory.set_user_name(user_id, name).await;
    }
    
    /// 👤 Получить имя пользователя из памяти
    #[allow(dead_code)]  // Используется через WhoAmI intent
    pub async fn get_user_name(&self, user_id: &str) -> Option<String> {
        self.memory.get_user_name(user_id).await
    }
    
    /// Обработать сообщение и сгенерировать ответ
    pub async fn process_message(
        &self,
        user_id: &str,
        message: &str,
    ) -> Result<String> {
        // 💬 ПРОВЕРКА: Светская беседа (smalltalk) — обрабатываем первыми
        if let Some(smalltalk_reply) = rules::smalltalk::respond(message) {
            self.memory.add_message(user_id, message.to_string()).await;
            return Ok(smalltalk_reply);
        }
        
        // 🧠 КОГНИТИВНЫЙ АНАЛИЗ: Определяем настроение и эмоции
        let mood = Thinker::detect_mood(message);
        let emotion = Thinker::extract_emotion(message);
        let conversation_type = Thinker::detect_conversation_type(message);
        let complexity = Thinker::analyze_complexity(message);  // 🧮 Анализ сложности
        
        tracing::info!(
            "🧠 Cognitive: mood={}, emotion={:?}, type={}, complexity={}", 
            mood, emotion, conversation_type, complexity
        );
        
        // ❤️ СОХРАНЯЕМ ЭМОЦИОНАЛЬНОЕ СОСТОЯНИЕ в память
        self.memory.set_emotional_state(user_id, mood, emotion).await;
        
        // 📝 Извлекаем и сохраняем предпочтения автоматически
        self.memory.extract_and_save_preferences(user_id, message).await;
        
        // Сохраняем сообщение в историю
        self.memory.add_message(user_id, message.to_string()).await;
        
        // Классифицируем намерение
        let intent = IntentClassifier::classify(message);
        
        // Сохраняем намерение
        self.memory.set_last_intent(user_id, format!("{:?}", intent)).await;
        
        // Извлекаем контекст в зависимости от намерения
        let context = match intent {
            Intent::OrderStatus => IntentClassifier::extract_order_id(message),
            Intent::CheckIngredients | Intent::ProductInfo => {
                IntentClassifier::extract_product_name(message)
            },
            Intent::Recommendation => {
                // 🧠 Извлекаем ключевые слова из сообщения
                let keywords = Thinker::extract_keywords(message);
                
                // 💡 Получаем сохранённые предпочтения
                let saved_context = self.memory.get_recommendation_context(user_id).await;
                
                // 🔀 Комбинируем: ключевые слова + предпочтения
                match (keywords.is_empty(), saved_context) {
                    (false, Some(prefs)) => {
                        Some(format!("{}, {}", keywords.join(", "), prefs))
                    },
                    (false, None) => Some(keywords.join(", ")),
                    (true, Some(prefs)) => Some(prefs),
                    (true, None) => None,
                }
            },
            Intent::ProductSearch => {
                // 🔍 Извлекаем ингредиент из запроса
                Thinker::extract_ingredient(message)
            },
            Intent::WhoAmI => {
                // 👤 Получаем имя пользователя из памяти
                self.memory.get_user_name(user_id).await
            },
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
}

impl Default for AIEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Основная функция для совместимости с текущим API
#[allow(dead_code)]  // Устаревшая функция, оставлена для совместимости
pub async fn generate_reply(_config: &Config, prompt: &str) -> Result<String> {
    let engine = AIEngine::new();
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

/// Создать заказ и отправить уведомление на Go backend
/// 
/// Пример использования:
/// ```
/// create_order("ORD-12345", 2500.0).await?;
/// ```
#[allow(dead_code)]
pub async fn create_order(order_id: &str, total: f64) -> Result<String> {
    tracing::info!("🤖 AI: Создаю заказ {} на сумму {:.2} руб.", order_id, total);
    
    // Отправляем заказ на Go backend
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
        let engine = AIEngine::new();
        let response = engine.process_message("test_user", "Привет!").await.unwrap();
        assert!(response.contains("Привет") || response.contains("Добро пожаловать"));
    }

    #[tokio::test]
    async fn test_ai_engine_menu() {
        let engine = AIEngine::new();
        let response = engine.process_message("test_user", "покажи меню").await.unwrap();
        assert!(response.contains("меню") || response.contains("Меню"));
    }

    #[tokio::test]
    async fn test_intent_classification() {
        assert_eq!(IntentClassifier::classify("привет"), Intent::Greeting);
        assert_eq!(IntentClassifier::classify("покажи меню"), Intent::ViewMenu);
        assert_eq!(IntentClassifier::classify("статус заказа"), Intent::OrderStatus);
    }
}


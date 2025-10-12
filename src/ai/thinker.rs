/// Когнитивная прослойка — эмоциональный анализ и персонализация

/// Анализатор настроения и эмоций пользователя
pub struct Thinker;

impl Thinker {
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
            || text.contains("great") {
            return "positive";
        }
        
        // Негативные маркеры
        if text.contains("плохо") 
            || text.contains("ужас")
            || text.contains("не нравится")
            || text.contains("разочаров")
            || text.contains("terrible")
            || text.contains("bad") {
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
            || lower.contains("hungry") {
            return Some("hungry");
        }
        
        if lower.contains("устал") 
            || lower.contains("tired")
            || lower.contains("выматывающий день") {
            return Some("tired");
        }
        
        if lower.contains("праздник") 
            || lower.contains("отмечаю")
            || lower.contains("celebration") {
            return Some("celebrating");
        }
        
        if lower.contains("один") 
            || lower.contains("одиночест")
            || lower.contains("alone") {
            return Some("alone");
        }
        
        if lower.contains("компания") 
            || lower.contains("друзья")
            || lower.contains("гости")
            || lower.contains("with friends") {
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
            },
            "negative" => {
                response.push_str("\n\n😔 Постараюсь улучшить настроение вкусным блюдом. Позволь помочь!");
            },
            _ => {}
        }

        // Реакция на эмоцию/состояние
        if let Some(emo) = emotion {
            match emo {
                "hungry" => {
                    response.push_str("\n\n🍽️ Похоже, ты проголодался! Хочешь, покажу что-то вкусное прямо сейчас?");
                },
                "tired" => {
                    response.push_str("\n\n☕ После тяжёлого дня рекомендую что-то лёгкое и вкусное — расслабься и наслаждайся!");
                },
                "celebrating" => {
                    response.push_str("\n\n🎉 Праздник — отличный повод попробовать что-то особенное! Покажу наши премиум-сеты?");
                },
                "alone" => {
                    response.push_str("\n\n🍴 Побалуй себя чем-то вкусным — ты это заслужил! Порционные блюда как раз для тебя.");
                },
                "with_company" => {
                    response.push_str("\n\n👥 Отлично! Для компании рекомендую большие порции и сеты — все будут в восторге!");
                },
                _ => {}
            }
        }

        response
    }

    /// 🔍 Извлекает ключевые слова для контекста
    #[allow(dead_code)]  // Используется для улучшенных рекомендаций
    pub fn extract_keywords(message: &str) -> Vec<String> {
        let text = message.to_lowercase();
        let mut keywords = Vec::new();
        
        // Вкусовые предпочтения
        let flavor_words = vec![
            "острое", "сладкое", "солёное", "кислое", "пряное",
            "spicy", "sweet", "salty", "sour",
        ];
        
        for word in flavor_words {
            if text.contains(word) {
                keywords.push(word.to_string());
            }
        }
        
        // Типы блюд
        let dish_types = vec![
            "суп", "салат", "закуска", "десерт", "напиток",
            "soup", "salad", "appetizer", "dessert", "drink",
            "паэлья", "креветки", "лосось", "тунец",
            "paella", "shrimp", "salmon", "tuna",
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
            "лосось", "salmon",
            "креветки", "креветка", "shrimp", "prawns",
            "тунец", "tuna",
            "осьминог", "octopus",
            "мидии", "mussels",
            "кальмар", "кальмары", "squid",
            "рыба", "fish",
            "морепродукты", "seafood",
            "авокадо", "avocado",
            "овощи", "vegetables",
            "рис", "rice",
            "лапша", "noodles",
        ];
        
        for ingredient in ingredients {
            if text.contains(ingredient) {
                return Some(ingredient.to_string());
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
            || text.contains("where") {
            return "question";
        }
        
        // Заказ
        if text.contains("закажу") 
            || text.contains("хочу заказать")
            || text.contains("оформить")
            || text.contains("i want to order")
            || text.contains("order") {
            return "order";
        }
        
        // Светская беседа
        if text.contains("как дела") 
            || text.contains("что делаешь")
            || text.contains("привет")
            || text.contains("hello")
            || text.contains("how are you") {
            return "smalltalk";
        }
        
        // Жалоба
        if text.contains("не пришел") 
            || text.contains("проблема")
            || text.contains("complaint")
            || text.contains("issue") {
            return "complaint";
        }
        
        "general"
    }

    /// 🧮 Анализирует "вес" запроса (простой/сложный)
    #[allow(dead_code)]  // Используется для логирования сложности
    pub fn analyze_complexity(message: &str) -> &'static str {
        let words: Vec<&str> = message.split_whitespace().collect();
        
        if words.len() <= 3 {
            "simple"  // "Покажи меню"
        } else if words.len() <= 10 {
            "medium"  // "Хочу что-то острое с креветками"
        } else {
            "complex" // Длинный детальный запрос
        }
    }

    /// 🎯 Генерирует персонализированное обращение
    #[allow(dead_code)]  // Зарезервировано для персонализированных приветствий
    pub fn generate_greeting(user_name: Option<&str>, message_count: usize) -> String {
        match (user_name, message_count) {
            (Some(name), 1) => format!("👋 Привет, {}! Рад познакомиться!", name),
            (Some(name), _) => format!("👋 Снова здравствуй, {}!", name),
            (None, 1) => "👋 Привет! Рад видеть нового гостя!".to_string(),
            (None, _) => "👋 Привет! Рад, что ты вернулся!".to_string(),
        }
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
        assert_eq!(Thinker::extract_emotion("Устал после работы"), Some("tired"));
        assert_eq!(Thinker::extract_emotion("Просто хочу поесть"), None);
    }

    #[test]
    fn test_conversation_type() {
        assert_eq!(Thinker::detect_conversation_type("Что у вас есть?"), "question");
        assert_eq!(Thinker::detect_conversation_type("Хочу заказать паэлью"), "order");
        assert_eq!(Thinker::detect_conversation_type("Как дела?"), "smalltalk");
    }

    #[test]
    fn test_complexity() {
        assert_eq!(Thinker::analyze_complexity("Меню"), "simple");
        assert_eq!(Thinker::analyze_complexity("Покажи острые блюда с креветками"), "medium");
    }
}

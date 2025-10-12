/// Типы намерений пользователя
#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    // Приветствия и общение
    Greeting,
    Farewell,
    Thanks,
    Help,
    WhoAmI,  // 👤 Определение пользователя ("как меня зовут", "кто я")
    
    // Заказы
    OrderStatus,
    CreateOrder,
    CancelOrder,
    
    // Меню и продукты
    ViewMenu,
    ProductInfo,
    PriceInquiry,
    Recommendation,
    ProductSearch,  // 🔍 Поиск блюд по ингредиенту
    
    // Ингредиенты и склад
    CheckIngredients,
    StockStatus,
    
    // Аналитика (для менеджеров)
    GetStatistics,
    SalesAnalysis,
    
    // Доставка
    DeliveryInfo,
    CourierStatus,
    
    // Неизвестное намерение
    Unknown,
}

/// Приоритет намерения (для разрешения конфликтов)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum IntentPriority {
    Low = 1,
    Medium = 2,
    High = 3,
}

/// Кандидат намерения с приоритетом
#[derive(Debug, Clone)]
struct IntentCandidate {
    intent: Intent,
    priority: IntentPriority,
    score: usize, // количество совпавших ключевых слов
}

/// Словарь ключевых слов для определения намерений
pub struct IntentClassifier;

impl IntentClassifier {
    /// Определить намерение по тексту сообщения с учётом приоритетов и контекста
    pub fn classify(text: &str) -> Intent {
        Self::classify_with_context(text, None)
    }
    
    /// Определить намерение с учётом предыдущего контекста
    pub fn classify_with_context(text: &str, last_intent: Option<&Intent>) -> Intent {
        let text_lower = text.to_lowercase();
        let mut candidates: Vec<IntentCandidate> = Vec::new();
        
        // === Приветствия (высокий приоритет) ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            // Русский
            "привет", "здравствуй", "добрый день", "доброе утро", "добрый вечер", "приветик", "здорово",
            // English
            "hi", "hello", "hey", "good morning", "good afternoon", "good evening",
            // Polski
            "cześć", "dzień dobry", "witaj"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::Greeting,
                priority: IntentPriority::High,
                score,
            });
        }
        
        // === Прощания (высокий приоритет) ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            // Русский - более специфичные фразы
            " пока", "пока!", "пока,", "до свидания", "прощай", "увидимся", "всего доброго", "до встречи",
            // English
            "bye", "goodbye", "see you", "farewell",
            // Polski
            "do widzenia", "żegnaj"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::Farewell,
                priority: IntentPriority::High,
                score,
            });
        }
        
        // === Благодарности (высокий приоритет) ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            // Русский
            "спасибо", "благодарю", "спс", "благодарен", "огромное спасибо",
            // English
            "thanks", "thank you", "thx", "ty",
            // Polski
            "dziękuję", "dzięki"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::Thanks,
                priority: IntentPriority::High,
                score,
            });
        }
        
        // === Помощь (высокий приоритет) ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            // Русский
            "помощь", "помоги", "команды", "что умеешь", "как работать", "справка",
            // English
            "help", "commands", "what can you do", "assistance",
            // Polski
            "pomoc", "komendy", "co potrafisz"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::Help,
                priority: IntentPriority::High,
                score,
            });
        }
        
        // === 👤 Кто я? / Как меня зовут? (высокий приоритет) ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            // Русский
            "как меня зовут", "кто я", "моё имя", "мое имя", "меня зовут",
            "скажи моё имя", "напомни имя", "помнишь меня",
            // English
            "what is my name", "who am i", "my name", "do you know me",
            // Polski
            "jak się nazywam", "kim jestem"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::WhoAmI,
                priority: IntentPriority::High,
                score,
            });
        }
        
        // === Статус заказа (высокий приоритет с улучшенными синонимами) ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            // Русский - расширенные синонимы
            "статус заказа", "где заказ", "где мой заказ", "покажи заказ", "покажи мой заказ",
            "сколько ждать", "когда будет", "трек", "отследить", "проверить заказ",
            // English
            "order status", "where is my order", "track order", "check order",
            // Polski
            "status zamówienia", "gdzie jest zamówienie"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::OrderStatus,
                priority: IntentPriority::High,
                score,
            });
        }
        
        // Также проверяем наличие ORD- паттерна
        if text_lower.contains("ord-") {
            candidates.push(IntentCandidate {
                intent: Intent::OrderStatus,
                priority: IntentPriority::High,
                score: 3,
            });
        }
        
        // === Создание заказа (средний приоритет) ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            // Русский - естественные фразы
            "создать заказ", "оформить заказ", "заказать", "хочу заказать", "сделать заказ", "новый заказ",
            "закажу", "буду заказывать", "хочу купить", "возьму", "оформлю заказ",
            // English
            "create order", "make order", "place order", "new order",
            // Polski
            "złóż zamówienie", "nowe zamówienie"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::CreateOrder,
                priority: IntentPriority::Medium,
                score,
            });
        }
        
        // === Отмена заказа (высокий приоритет при наличии контекста) ===
        let cancel_priority = if matches!(last_intent, Some(Intent::OrderStatus) | Some(Intent::CreateOrder)) {
            IntentPriority::High
        } else {
            IntentPriority::Medium
        };
        
        if let Some(score) = Self::match_keywords(&text_lower, &[
            // Русский - с учётом контекста
            "отменить заказ", "отмена", "не нужен заказ", "его можно отменить", "можно отменить",
            "отменить", "отказаться", "удалить заказ",
            // English
            "cancel order", "cancel it", "remove order",
            // Polski
            "anuluj zamówienie", "usuń zamówienie"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::CancelOrder,
                priority: cancel_priority,
                score,
            });
        }
        
        // === Меню (средний приоритет) ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            // Русский - избегаем конфликта с "покажи"
            "меню", "блюда", "что есть", "что у вас есть", "ассортимент", 
            "покажи меню", "посмотреть меню", "показать меню",
            "какие блюда", "что можно заказать", "что готовите", "список блюд",
            "хочу посмотреть меню", "дайте меню",
            // English
            "menu", "dishes", "what do you have", "show menu", "products", "show me menu",
            // Polski
            "menu", "dania", "co macie", "pokaż menu"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::ViewMenu,
                priority: IntentPriority::Medium,
                score,
            });
        }
        
        // === Информация о продукте (средний приоритет) ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            // Русский
            "состав", "ингредиенты блюда", "что входит", "калории", "информация о блюде",
            // English
            "ingredients", "what's in", "calories", "product info",
            // Polski
            "składniki", "co zawiera", "kalorie"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::ProductInfo,
                priority: IntentPriority::Medium,
                score,
            });
        }
        
        // === Цены (средний приоритет) ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            // Русский
            "цена", "стоимость", "сколько стоит", "прайс", "цены", "сколько",
            // English
            "price", "cost", "how much", "pricing",
            // Polski
            "cena", "ile kosztuje", "cennik"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::PriceInquiry,
                priority: IntentPriority::Medium,
                score,
            });
        }
        
        // === Рекомендации (средний приоритет) ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            // Русский - естественные фразы
            "посоветуй", "что заказать", "рекомендация", "что вкусное", "что попробовать",
            "что посоветуешь", "что лучше", "что рекомендуешь", "порекомендуй",
            "не знаю что выбрать", "помоги выбрать", "что взять", "что вкусного",
            "хочу чего-то вкусного", "что-нибудь вкусное",
            // English
            "recommend", "what's good", "what should i order", "suggestions",
            // Polski
            "poleć", "co zamówić", "co dobre"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::Recommendation,
                priority: IntentPriority::Medium,
                score,
            });
        }
        
        // === 🔍 Поиск блюд по ингредиенту (высокий приоритет) ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            // Русский
            "что есть с", "покажи с", "блюда с", "есть с", "с чем есть",
            "покажи что есть с", "что у вас с", "хочу с",
            // English
            "dishes with", "what do you have with", "show me with",
            // Polski
            "dania z", "co macie z"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::ProductSearch,
                priority: IntentPriority::High,
                score,
            });
        }
        
        // === Проверка ингредиентов (низкий приоритет - для менеджеров) ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            "остатки", "ингредиенты", "склад", "check ingredients", "sprawdź składniki"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::CheckIngredients,
                priority: IntentPriority::Low,
                score,
            });
        }
        
        // === Статус склада ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            "наличие", "есть ли", "в наличии", "stock", "availability", "dostępność"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::StockStatus,
                priority: IntentPriority::Low,
                score,
            });
        }
        
        // === Статистика vs Анализ продаж (весовые приоритеты) ===
        // Если есть временной контекст - это скорее SalesAnalysis
        let has_time_context = Self::match_keywords(&text_lower, &[
            "за день", "за неделю", "за месяц", "сегодня", "вчера", "этот месяц",
            "today", "yesterday", "this week", "this month"
        ]).is_some();
        
        if let Some(score) = Self::match_keywords(&text_lower, &[
            "продажи", "sales", "выручка", "доход", "revenue", "sprzedaż", "przychód"
        ]) {
            let intent = if has_time_context {
                Intent::SalesAnalysis
            } else {
                Intent::GetStatistics
            };
            candidates.push(IntentCandidate {
                intent,
                priority: IntentPriority::Low,
                score: if has_time_context { score + 2 } else { score },
            });
        }
        
        if let Some(score) = Self::match_keywords(&text_lower, &[
            "статистика", "stats", "аналитика", "отчет", "analytics", "statystyki", "raport"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::GetStatistics,
                priority: IntentPriority::Low,
                score,
            });
        }
        
        // === Доставка ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            "доставка", "курьер", "delivery", "когда привезут", "dostawa", "kiedy dostarczycie",
            "сколько стоит доставка", "как доставляете", "доставляете ли", "время доставки",
            "доставка бесплатная", "зона доставки", "куда доставляете"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::DeliveryInfo,
                priority: IntentPriority::Medium,
                score,
            });
        }
        
        // === Статус курьера ===
        if let Some(score) = Self::match_keywords(&text_lower, &[
            "где курьер", "courier", "курьер едет", "delivery status", "gdzie kurier"
        ]) {
            candidates.push(IntentCandidate {
                intent: Intent::CourierStatus,
                priority: IntentPriority::Medium,
                score,
            });
        }
        
        // Выбираем лучшего кандидата
        Self::select_best_intent(candidates)
    }
    
    /// Подсчитать совпадения ключевых слов
    fn match_keywords(text: &str, keywords: &[&str]) -> Option<usize> {
        let score = keywords.iter()
            .filter(|&&kw| text.contains(kw))
            .count();
        
        if score > 0 {
            Some(score)
        } else {
            None
        }
    }
    
    /// Выбрать лучшее намерение из кандидатов
    fn select_best_intent(mut candidates: Vec<IntentCandidate>) -> Intent {
        if candidates.is_empty() {
            return Intent::Unknown;
        }
        
        // Сортируем по приоритету (убывание), затем по score (убывание)
        candidates.sort_by(|a, b| {
            match b.priority.cmp(&a.priority) {
                std::cmp::Ordering::Equal => b.score.cmp(&a.score),
                other => other,
            }
        });
        
        candidates[0].intent.clone()
    }
    
    /// Извлечь ID заказа из текста (если есть)
    pub fn extract_order_id(text: &str) -> Option<String> {
        let text_upper = text.to_uppercase();
        
        // Поиск паттерна ORD-XXXXX
        if let Some(start) = text_upper.find("ORD-") {
            let id_part = &text_upper[start..];
            if let Some(end) = id_part.find(|c: char| !c.is_alphanumeric() && c != '-') {
                Some(id_part[..end].to_string())
            } else {
                Some(id_part.to_string())
            }
        } else {
            None
        }
    }
    
    /// Извлечь название продукта/ингредиента из текста
    pub fn extract_product_name(text: &str) -> Option<String> {
        // Простая эвристика: берём слова после команды
        let words: Vec<&str> = text.split_whitespace().collect();
        
        if words.len() > 1 {
            // Пропускаем первое слово (команду) и берём остальное
            Some(words[1..].join(" "))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting_classification() {
        assert_eq!(IntentClassifier::classify("Привет!"), Intent::Greeting);
        assert_eq!(IntentClassifier::classify("hello"), Intent::Greeting);
        assert_eq!(IntentClassifier::classify("Добрый день!"), Intent::Greeting);
    }

    #[test]
    fn test_order_id_extraction() {
        assert_eq!(
            IntentClassifier::extract_order_id("Статус заказа ORD-12345"),
            Some("ORD-12345".to_string())
        );
        assert_eq!(
            IntentClassifier::extract_order_id("order_status ORD-ABC123"),
            Some("ORD-ABC123".to_string())
        );
    }

    #[test]
    fn test_menu_classification() {
        assert_eq!(IntentClassifier::classify("покажи меню"), Intent::ViewMenu);
        assert_eq!(IntentClassifier::classify("что у вас есть?"), Intent::ViewMenu);
    }
    
    #[test]
    fn test_improved_synonyms() {
        // Тест расширенных синонимов для заказов
        assert_eq!(IntentClassifier::classify("где мой заказ"), Intent::OrderStatus);
        assert_eq!(IntentClassifier::classify("покажи мой заказ"), Intent::OrderStatus);
        assert_eq!(IntentClassifier::classify("сколько ждать"), Intent::OrderStatus);
    }
    
    #[test]
    fn test_weighted_priorities() {
        // Продажи за день -> SalesAnalysis
        assert_eq!(IntentClassifier::classify("продажи за день"), Intent::SalesAnalysis);
        assert_eq!(IntentClassifier::classify("продажи за неделю"), Intent::SalesAnalysis);
        
        // Просто аналитика -> GetStatistics
        assert_eq!(IntentClassifier::classify("статистика"), Intent::GetStatistics);
        assert_eq!(IntentClassifier::classify("аналитика"), Intent::GetStatistics);
    }
    
    #[test]
    fn test_context_aware() {
        // Без контекста "отменить" -> CancelOrder (средний приоритет)
        assert_eq!(IntentClassifier::classify("можно отменить?"), Intent::CancelOrder);
        
        // С контекстом OrderStatus -> CancelOrder (высокий приоритет)
        assert_eq!(
            IntentClassifier::classify_with_context("его можно отменить?", Some(&Intent::OrderStatus)),
            Intent::CancelOrder
        );
    }
    
    #[test]
    fn test_multilang() {
        // English
        assert_eq!(IntentClassifier::classify("show me the menu"), Intent::ViewMenu);
        assert_eq!(IntentClassifier::classify("what's the price"), Intent::PriceInquiry);
        
        // Polski
        assert_eq!(IntentClassifier::classify("pokaż menu"), Intent::ViewMenu);
        assert_eq!(IntentClassifier::classify("dziękuję"), Intent::Thanks);
    }
}

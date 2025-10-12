# 🎯 Пример использования всех функций когнитивного слоя

## Код для интеграции в AIEngine

### 1. Используем `extract_keywords()` для улучшенных рекомендаций

```rust
// В src/ai/mod.rs, в функции process_message()

Intent::Recommendation => {
    // 🧠 Извлекаем ключевые слова из сообщения
    let keywords = Thinker::extract_keywords(message);
    
    // 💡 Получаем сохранённые предпочтения
    let saved_context = self.memory.get_recommendation_context(user_id).await;
    
    // 🔀 Комбинируем: ключевые слова + предпочтения
    let combined_context = match (keywords.is_empty(), saved_context) {
        (false, Some(prefs)) => {
            Some(format!("{}, {}", keywords.join(", "), prefs))
        },
        (false, None) => Some(keywords.join(", ")),
        (true, Some(prefs)) => Some(prefs),
        (true, None) => None,
    };
    
    combined_context
}
```

**Эффект:**
```
👤 "Посоветуй что-то острое"
🧠 Извлечены ключевые слова: ["острое"]
💾 Найдены предпочтения: favorite=salmon
🎯 Комбинированный контекст: "острое, salmon"
🤖 "🌶️ Для любителей острого (и с учётом твоей любви к лососю):
     • Острый лосось терияки
     • Лосось с чили-соусом"
```

---

### 2. Используем `get_last_mood()` для эмоциональной преемственности

```rust
// В src/ai/mod.rs, после когнитивного анализа

// ❤️ Проверяем изменение настроения
let prev_mood = self.memory.get_last_mood(user_id).await;

let mood_context = if let Some(prev) = prev_mood {
    if prev == "negative" && mood == "positive" {
        Some("\n\n😊 Рад, что настроение улучшилось! Это заслуга хорошей еды?")
    } else if prev == "positive" && mood == "negative" {
        Some("\n\n😔 Вижу, что-то расстроило. Давай исправлю это вкусным предложением!")
    } else if prev == mood {
        Some("\n\n🙂 Настроение стабильное — отлично!")
    } else {
        None
    }
} else {
    None
};

// Добавляем к финальному ответу
let final_response = if let Some(mood_msg) = mood_context {
    format!("{}{}", personalized, mood_msg)
} else {
    personalized
};
```

**Пример:**
```
Сессия 1:
👤 "Всё плохо 😔"
🧠 mood=negative
❤️ Сохранено: last_mood=negative

Сессия 2:
👤 "Спасибо, классно!"
🧠 mood=positive
❤️ Было: negative → Стало: positive
🤖 "Пожалуйста! 😊 Рад, что настроение улучшилось! Это заслуга хорошей еды?"
```

---

### 3. Используем `generate_greeting()` для персонализированных приветствий

```rust
// В src/ai/rules/common.rs

pub fn greeting_response() -> String {
    // Вместо статичного приветствия используем случайное
    let greeting = smalltalk::generate_random_greeting();
    
    // TODO: Можно улучшить, передав имя и счётчик:
    // let greeting = Thinker::generate_greeting(Some("Дима"), 5);
    
    format!(
        "{}\n\n\
         Чем могу помочь?...",
        greeting
    )
}
```

**Или ещё лучше — в AIEngine:**
```rust
// В process_message(), для Intent::Greeting
Intent::Greeting => {
    let message_count = self.memory.get_message_count(user_id).await;
    let user_name = self.memory.get_user_name(user_id).await;
    
    // 🎯 Персонализированное приветствие
    let personalized_greeting = Thinker::generate_greeting(
        user_name.as_deref(),
        message_count
    );
    
    Some(personalized_greeting)
}
```

**Результат:**
```
Первое сообщение:
👤 "Привет"
🤖 "👋 Привет, Дима! Рад познакомиться!"

10-е сообщение:
👤 "Привет"
🤖 "👋 Снова здравствуй, Дима!"
```

---

### 4. Используем `analyze_complexity()` для адаптации ответа

```rust
// В AIEngine::process_message()

let complexity = Thinker::analyze_complexity(message);

tracing::info!(
    "🧠 Cognitive: mood={}, emotion={:?}, type={}, complexity={}", 
    mood, emotion, conversation_type, complexity
);

// Адаптируем длину ответа
let response_style = match complexity {
    "simple" => "brief",    // Короткий ответ для "меню"
    "medium" => "standard", // Обычный для "хочу что-то острое"
    "complex" => "detailed", // Детальный для длинного запроса
    _ => "standard"
};

// Используем в генерации
let base_response = ResponseGenerator::generate_with_style(
    &intent, 
    context.as_deref(),
    response_style  // 🎯 Передаём стиль
);
```

**Эффект:**
```
👤 "Меню" (1 слово)
🧠 complexity=simple
🤖 "🍽️ Меню:" (короткий список)

👤 "Посоветуй что-нибудь вкусное и сытное для большой компании на праздник" (11 слов)
🧠 complexity=complex
🤖 [Развёрнутый ответ с деталями, ценами, порциями]
```

---

### 5. Используем `conversation_state` для follow-up диалогов

```rust
// В src/ai/mod.rs

// После ответа с вопросом
if base_response.contains("Хочешь") || base_response.contains("Показать") {
    // Сохраняем состояние ожидания
    self.memory.update_context(user_id, |ctx| {
        ctx.conversation_state = Some("awaiting_confirmation".to_string());
    }).await;
}

// При следующем сообщении
let context = self.memory.get_context(user_id).await;

if context.conversation_state == Some("awaiting_confirmation".to_string()) {
    let text_lower = message.to_lowercase();
    
    if text_lower.contains("да") || text_lower.contains("покажи") || text_lower.contains("хочу") {
        // Продолжаем без повторного вопроса
        tracing::info!("🔄 Follow-up: user confirmed, showing menu");
        
        // Сбрасываем состояние
        self.memory.update_context(user_id, |ctx| {
            ctx.conversation_state = None;
        }).await;
        
        // Сразу показываем меню
        return Ok(format_menu_response());
    }
}
```

**Пример:**
```
Сообщение 1:
👤 "Я голоден"
🧠 emotion=hungry
🤖 "🍽️ Хочешь, покажу меню?"
💾 conversation_state = "awaiting_confirmation"

Сообщение 2:
👤 "Да"
🔄 Состояние: awaiting_confirmation → проверка
✅ Подтверждение найдено!
🤖 [Показывает меню без повторного "хочешь?"]
💾 conversation_state = None (сброшено)
```

---

## 📊 Итоговая схема использования

```
Сообщение пользователя
    ↓
1. Thinker::detect_mood() → mood
2. Thinker::extract_emotion() → emotion
3. Thinker::detect_conversation_type() → type
4. Thinker::analyze_complexity() → complexity
5. Thinker::extract_keywords() → keywords
    ↓
6. memory.set_emotional_state(mood, emotion) → сохранение
7. memory.get_last_mood() → сравнение с предыдущим
8. memory.get_recommendation_context() → предпочтения
    ↓
9. IntentClassifier::classify() → intent
10. ResponseGenerator::generate() → базовый ответ
    ↓
11. Thinker::personalize(base, mood, emotion) → эмоциональный слой
12. Добавление mood_context (изменение настроения)
13. Проверка conversation_state (follow-up)
    ↓
Финальный ответ пользователю
```

---

## 🎯 Приоритет внедрения:

### Быстро (10 минут):
1. ✅ `extract_keywords()` → улучшение рекомендаций
2. ✅ `get_last_mood()` → эмоциональная преемственность
3. ✅ `analyze_complexity()` → логирование сложности

### Средне (30 минут):
4. ⏳ `generate_greeting()` → персонализированные приветствия
5. ⏳ Адаптация длины ответа по complexity

### Сложно (1-2 часа):
6. ⏳ `conversation_state` → follow-up диалоги
7. ⏳ Полная интеграция всех функций

---

**Все функции готовы к использованию!** 🚀
Выбери что внедрить первым, и я сразу применю!

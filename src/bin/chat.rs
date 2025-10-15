use fodifood_bot::ai::{AIEngine, IntentClassifier};
use fodifood_bot::config::Config;
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    // Проверяем наличие флага --debug
    let args: Vec<String> = env::args().collect();
    let debug_mode = args.contains(&"--debug".to_string());

    // Инициализация логирования
    if debug_mode {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
        println!("🐛 DEBUG MODE ENABLED");
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    println!("🔧 Инициализация AIEngine...");

    // Создаем конфигурацию с дефолтными значениями для тестирования
    let config = Config {
        openai_api_key: String::new(),
        go_backend_url: std::env::var("GO_BACKEND_URL")
            .unwrap_or_else(|_| "http://localhost:8080/api".to_string()),
        jwt_secret: "test_secret".to_string(),
        orchestrator_enabled: false,
        orchestrator_managed: false,
        go_backend_bin: String::new(),
    };

    let engine = AIEngine::new(&config);
    let user_id = "test_user";

    println!("✅ FodiFood Bot запущен!");
    if debug_mode {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("� РЕЖИМ ОТЛАДКИ ИНТЕНТОВ");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    } else {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
    println!("�💬 Пиши сообщения для тестирования интентов");
    println!("🧪 Примеры:");
    println!("   • лосось");
    println!("   • с креветками");
    println!("   • блюда с тунцом");
    println!("   • покажи меню");
    println!("   • привет");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    if debug_mode {
        println!("🐛 Режим отладки: показывает детали классификации");
    }
    println!("⌨️  Введи 'exit' для выхода\n");

    loop {
        print!("👤 > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "exit" || input == "выход" || input == "quit" {
            println!("👋 Пока! Спасибо за тестирование!");
            break;
        }

        // 🐛 DEBUG MODE: показываем процесс классификации
        if debug_mode {
            println!("\n━━━ 🐛 DEBUG INFO ━━━");
            let intent = IntentClassifier::classify(input);
            println!("🎯 Intent: {:?}", intent);

            // Если это SearchByIngredient - показываем извлеченный ингредиент
            if matches!(intent, fodifood_bot::ai::Intent::SearchByIngredient) {
                let ingredient = IntentClassifier::extract_ingredient(input);
                println!("🧩 Extracted: \"{}\"", ingredient);
                println!("💡 Will search products containing: {}", ingredient);
            }

            // Если есть order ID - показываем его
            if let Some(order_id) = IntentClassifier::extract_order_id(input) {
                println!("📦 Order ID: {}", order_id);
            }

            // Показываем название продукта если есть
            if let Some(product) = IntentClassifier::extract_product_name(input) {
                println!("🍱 Product name: {}", product);
            }

            println!("━━━━━━━━━━━━━━━━━━━━\n");
        }

        // Обработка сообщения
        match engine.process_message(user_id, input).await {
            Ok(response) => {
                println!("🤖 {}\n", response);
            }
            Err(e) => {
                println!("⚠️ Ошибка обработки: {}\n", e);
            }
        }
    }
}

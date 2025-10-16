/// 🧪 Демонстрация AI-аналитики бизнес-метрик
/// 
/// Запуск: cargo run --example business_analysis_demo

use anyhow::Result;
use fodifood_bot::ai::analysis::{
    analyze_metrics, compare_businesses, investment_recommendation, quick_summary,
};
use fodifood_bot::services::{fetch_businesses, fetch_business_metrics};

#[tokio::main]
async fn main() -> Result<()> {
    // Инициализация логирования
    tracing_subscriber::fmt::init();

    println!("🧠 AI Business Analysis Demo\n");

    // Проверяем, что Go backend запущен
    let base_url = std::env::var("GO_BACKEND_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080/api".to_string());

    println!("🌐 Connecting to: {}\n", base_url);

    // Получаем список бизнесов
    println!("📊 Fetching businesses...");
    match fetch_businesses().await {
        Ok(businesses) => {
            if businesses.is_empty() {
                println!("❌ No businesses found. Please create some businesses first.");
                return Ok(());
            }

            println!("✅ Found {} businesses\n", businesses.len());

            // Анализируем первые 3 бизнеса
            let mut metrics_list = Vec::new();

            for (i, business) in businesses.iter().take(3).enumerate() {
                println!("{}. Analyzing: {}", i + 1, business.name);

                match fetch_business_metrics(&business.id).await {
                    Ok(metrics) => {
                        println!("\n{}", "=".repeat(60));
                        println!("📈 Quick Summary:");
                        println!("{}\n", quick_summary(&metrics));

                        println!("{}", analyze_metrics(&metrics));
                        println!("{}", investment_recommendation(&metrics));
                        println!("{}", "=".repeat(60));
                        println!();

                        metrics_list.push((business.name.clone(), metrics));
                    }
                    Err(e) => {
                        println!("❌ Failed to fetch metrics: {}\n", e);
                    }
                }
            }

            // Сравнительный анализ, если есть несколько бизнесов
            if metrics_list.len() > 1 {
                println!("\n{}", "=".repeat(60));
                println!("🏆 COMPARATIVE ANALYSIS");
                println!("{}", "=".repeat(60));

                let comparison_data: Vec<(&str, &_)> = metrics_list
                    .iter()
                    .map(|(name, metrics)| (name.as_str(), metrics))
                    .collect();

                println!("{}", compare_businesses(comparison_data));
                println!("{}", "=".repeat(60));
            }

            // Демонстрация различных сценариев
            println!("\n{}", "=".repeat(60));
            println!("🎭 SCENARIO ANALYSIS");
            println!("{}", "=".repeat(60));

            // Сценарий 1: Быстрорастущий бизнес
            println!("\n1️⃣ Scenario: Fast Growing Business");
            let growing = create_test_metrics("GROW", 150.0, 45.0, 250, 5_000_000.0, 60.0, 75.0);
            println!("{}", quick_summary(&growing));
            println!("{}", investment_recommendation(&growing));

            // Сценарий 2: Стабильный бизнес
            println!("\n2️⃣ Scenario: Stable Business");
            let stable = create_test_metrics("STBL", 100.0, 3.0, 120, 2_000_000.0, 25.0, 30.0);
            println!("{}", quick_summary(&stable));
            println!("{}", investment_recommendation(&stable));

            // Сценарий 3: Падающий бизнес
            println!("\n3️⃣ Scenario: Declining Business");
            let declining = create_test_metrics("FALL", 50.0, -25.0, 30, 500_000.0, -10.0, -15.0);
            println!("{}", quick_summary(&declining));
            println!("{}", investment_recommendation(&declining));

            println!("\n{}", "=".repeat(60));
        }
        Err(e) => {
            println!("❌ Failed to fetch businesses: {}", e);
            println!("💡 Make sure Go backend is running on http://127.0.0.1:8080");
            println!("\n🎭 Running demo with test data instead...\n");

            // Демонстрация с тестовыми данными
            demo_with_test_data();
        }
    }

    println!("\n✅ Analysis completed!");
    Ok(())
}

/// Создать тестовые метрики
fn create_test_metrics(
    symbol: &str,
    price: f64,
    change: f64,
    investors: i64,
    cap: f64,
    roi: f64,
    avg_roi: f64,
) -> fodifood_bot::services::BusinessMetrics {
    use fodifood_bot::services::go_client::BusinessMetrics;

    BusinessMetrics {
        token_symbol: symbol.to_string(),
        current_price: price,
        price_change: change,
        total_investors: investors,
        market_cap: cap,
        roi,
        avg_investor_roi: avg_roi,
    }
}

/// Демонстрация с тестовыми данными
fn demo_with_test_data() {
    println!("{}", "=".repeat(60));
    println!("TEST DATA ANALYSIS");
    println!("{}", "=".repeat(60));

    let test_businesses = vec![
        (
            "Sushi Paradise",
            create_test_metrics("SUSHI", 125.0, 35.0, 180, 3_500_000.0, 45.0, 55.0),
        ),
        (
            "Pizza Factory",
            create_test_metrics("PIZZA", 80.0, -5.0, 90, 1_200_000.0, 15.0, 20.0),
        ),
        (
            "Coffee Roasters",
            create_test_metrics("COFFEE", 200.0, 60.0, 300, 8_000_000.0, 80.0, 95.0),
        ),
    ];

    // Анализ каждого
    for (name, metrics) in test_businesses.iter() {
        println!("\n🏢 {}", name);
        println!("{}", quick_summary(metrics));
    }

    // Сравнение
    println!("\n{}", "=".repeat(60));
    let comparison_data: Vec<(&str, &_)> = test_businesses
        .iter()
        .map(|(name, metrics)| (*name, metrics))
        .collect();
    println!("{}", compare_businesses(comparison_data));
}

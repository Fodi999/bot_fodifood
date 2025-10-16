/// 🧪 Примеры использования Go Client
/// 
/// Запуск: cargo run --example go_client_demo

use anyhow::Result;
use fodifood_bot::services::{fetch_business_metrics, fetch_businesses, GoClient};

#[tokio::main]
async fn main() -> Result<()> {
    // Инициализация логирования
    tracing_subscriber::fmt::init();

    println!("🚀 Go Backend Client Demo\n");

    // Получаем URL из переменной окружения
    let base_url = std::env::var("GO_BACKEND_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080/api".to_string());

    println!("🌐 Connecting to: {}\n", base_url);

    let client = GoClient::new(base_url);

    // 1️⃣ Получить список бизнесов
    println!("1️⃣ Fetching businesses list...");
    match fetch_businesses().await {
        Ok(businesses) => {
            println!("✅ Found {} businesses:", businesses.len());
            for business in businesses.iter().take(3) {
                println!(
                    "   • {} - {}",
                    business.name,
                    business.category.as_deref().unwrap_or("No category")
                );
            }
            println!();

            // 2️⃣ Получить метрики первого бизнеса (если есть)
            if let Some(first_business) = businesses.first() {
                println!("2️⃣ Fetching metrics for: {}", first_business.name);
                match fetch_business_metrics(&first_business.id).await {
                    Ok(metrics) => {
                        println!("✅ Metrics:");
                        println!("   💹 Token: {}", metrics.token_symbol);
                        println!("   💵 Current Price: ${:.2}", metrics.current_price);
                        println!("   📈 Price Change: {:.2}%", metrics.price_change);
                        println!("   👥 Total Investors: {}", metrics.total_investors);
                        println!("   💰 Market Cap: ${:.2}", metrics.market_cap);
                        println!("   📊 ROI: {:.2}%", metrics.roi);
                        println!("   📊 Avg Investor ROI: {:.2}%", metrics.avg_investor_roi);
                    }
                    Err(e) => {
                        println!("❌ Failed to fetch metrics: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to fetch businesses: {}", e);
            println!("💡 Make sure Go backend is running on http://127.0.0.1:8080");
        }
    }

    println!("\n3️⃣ Testing authentication...");
    match client.login("test@example.com", "password123").await {
        Ok(token_response) => {
            println!("✅ Login successful!");
            println!("   👤 User: {}", token_response.user.name);
            println!("   📧 Email: {}", token_response.user.email);
            println!("   🔑 Token: {}...", &token_response.token[..20]);
        }
        Err(e) => {
            println!("❌ Login failed: {}", e);
            println!("💡 This is expected if test user doesn't exist");
        }
    }

    println!("\n✅ Demo completed!");
    Ok(())
}

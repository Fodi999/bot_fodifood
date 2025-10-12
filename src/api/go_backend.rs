use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    config::Config,
    models::user::{VerifyTokenRequest, VerifyTokenResponse},
};

pub struct GoBackendClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    pub weight: Option<String>,
    pub category: Option<String>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: i64,
    pub user_id: String,
    pub status: String,
    pub total: f64,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItem {
    pub product_id: i64,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: i64,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub min_quantity: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub total_orders: i64,
    pub total_revenue: f64,
    pub today_orders: i64,
    pub today_revenue: f64,
    pub popular_products: Vec<Value>,
}

impl GoBackendClient {
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            base_url: config.go_backend_url.clone(),
        }
    }

    /// Verify JWT token with Go backend
    pub async fn verify_token(&self, token: &str) -> Result<VerifyTokenResponse> {
        let url = format!("{}/auth/verify", self.base_url);

        tracing::info!("🔍 Sending verify request to Go backend: {}", url);

        let response = self
            .client
            .post(&url)
            .json(&VerifyTokenRequest {
                token: token.to_string(),
            })
            .send()
            .await
            .context("Failed to send verify token request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read verify token body")?;

        tracing::info!("🧾 Raw verify_token response ({}): {}", status, text);

        if !status.is_success() {
            return Ok(VerifyTokenResponse {
                valid: false,
                user_id: None,
                role: None,
                name: None,
                email: None,
            });
        }

        // Попытка распарсить JSON
        let result: VerifyTokenResponse = serde_json::from_str(&text)
            .context("Failed to parse verify token JSON")?;

        tracing::info!(
            "✅ Parsed verify_token: valid={} user_id={:?} name={:?} email={:?} role={:?}",
            result.valid, result.user_id, result.name, result.email, result.role
        );

        Ok(result)
    }

    /// Get all products (menu)
    pub async fn get_products(&self) -> Result<Vec<Product>> {
        let url = format!("{}/products", self.base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch products")?;

        let products = response
            .json::<Vec<Product>>()
            .await
            .context("Failed to parse products response")?;

        Ok(products)
    }

    /// Get all orders (admin only)
    pub async fn get_orders(&self) -> Result<Vec<Order>> {
        let url = format!("{}/orders", self.base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch orders")?;

        let orders = response
            .json::<Vec<Order>>()
            .await
            .context("Failed to parse orders response")?;

        Ok(orders)
    }

    /// Create new order
    pub async fn create_order(&self, order_data: Value) -> Result<Order> {
        let url = format!("{}/orders", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&order_data)
            .send()
            .await
            .context("Failed to create order")?;

        let order = response
            .json::<Order>()
            .await
            .context("Failed to parse order response")?;

        Ok(order)
    }

    /// Get ingredients/inventory
    #[allow(dead_code)]  // Зарезервировано для команд инвентаря
    pub async fn get_ingredients(&self) -> Result<Vec<Ingredient>> {
        let url = format!("{}/ingredients", self.base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch ingredients")?;

        let ingredients = response
            .json::<Vec<Ingredient>>()
            .await
            .context("Failed to parse ingredients response")?;

        Ok(ingredients)
    }

    /// Get statistics
    #[allow(dead_code)]  // Зарезервировано для команд статистики
    pub async fn get_stats(&self) -> Result<Stats> {
        let url = format!("{}/stats", self.base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch stats")?;

        let stats = response
            .json::<Stats>()
            .await
            .context("Failed to parse stats response")?;

        Ok(stats)
    }
    
    /// 📋 Форматировать продукты в красивый список для показа пользователю
    pub fn format_products_list(products: &[Product]) -> String {
        if products.is_empty() {
            return "🤔 Меню временно пусто. Скоро добавим новые блюда!".to_string();
        }
        
        let mut result = String::from("🍽️ **Актуальное меню с реальными ценами:**\n\n");
        
        // Группируем по категориям
        let mut by_category: std::collections::HashMap<String, Vec<&Product>> = std::collections::HashMap::new();
        
        for product in products {
            if product.is_visible.unwrap_or(true) {
                let category = product.category.clone().unwrap_or_else(|| "Другое".to_string());
                by_category.entry(category).or_default().push(product);
            }
        }
        
        // Порядок категорий
        let category_order = vec!["Роллы", "Суши", "Закуски", "Напитки", "Другое"];
        
        for category_name in category_order {
            if let Some(items) = by_category.get(category_name) {
                result.push_str(&format!("📂 **{}:**\n", category_name));
                
                for product in items {
                    let price = format!("{}₽", product.price as i32);
                    let weight = product.weight.as_deref().unwrap_or("");
                    let desc = product.description.as_deref().unwrap_or("");
                    
                    result.push_str(&format!(
                        "• **{}** — {} {}\n",
                        product.name,
                        price,
                        if !weight.is_empty() { format!("({})", weight) } else { String::new() }
                    ));
                    
                    if !desc.is_empty() && desc.len() < 100 {
                        result.push_str(&format!("  _{}_\n", desc));
                    }
                }
                result.push('\n');
            }
        }
        
        result.push_str("💡 Все блюда готовятся из свежайших ингредиентов!\n");
        result.push_str("🚚 Доставка от 1500₽ — бесплатно!");
        
        result
    }

    /// Update order status
    #[allow(dead_code)]
    pub async fn update_order_status(&self, order_id: i64, status: &str) -> Result<Order> {
        let url = format!("{}/orders/{}", self.base_url, order_id);

        let response = self
            .client
            .patch(&url)
            .json(&serde_json::json!({ "status": status }))
            .send()
            .await
            .context("Failed to update order status")?;

        let order = response
            .json::<Order>()
            .await
            .context("Failed to parse order response")?;

        Ok(order)
    }
}

/// Отправить уведомление о заказе на Go backend
/// 
/// Используется для отправки заказов, созданных через AI-бота
#[allow(dead_code)]
pub async fn send_order_to_backend(order_id: &str, total: f64) -> Result<()> {
    let backend_url = std::env::var("GO_BACKEND_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080".into());

    let client = Client::new();
    let res = client
        .post(format!("{}/api/orders/notify", backend_url))
        .json(&serde_json::json!({
            "order_id": order_id,
            "total": total,
        }))
        .send()
        .await
        .context("Failed to send order notification to backend")?;

    tracing::info!("📦 Sent order to backend → Status: {:?}", res.status());
    Ok(())
}

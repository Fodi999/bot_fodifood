use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// 📊 Метрики бизнеса из Go backend
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BusinessMetrics {
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: String,
    #[serde(rename = "currentPrice")]
    pub current_price: f64,
    #[serde(rename = "priceChange")]
    pub price_change: f64,
    #[serde(rename = "totalInvestors")]
    pub total_investors: i64,
    #[serde(rename = "marketCap")]
    pub market_cap: f64,
    pub roi: f64,
    #[serde(rename = "avgInvestorROI")]
    pub avg_investor_roi: f64,
}

/// 🏢 Информация о бизнесе
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Business {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

///  Ответ с токеном
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub token: String,
    pub user: UserInfo,
}

/// 👤 Информация о пользователе
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
}

/// 🛒 Данные заказа для создания
#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderData {
    pub user_id: String,
    pub name: String,
    pub phone: String,
    pub address: String,
    pub items: Vec<OrderItem>,
}

/// 📦 Элемент заказа
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub product_id: String,
    pub name: String,
    pub quantity: i32,
    pub price: f64,
}

/// 📝 Ответ при создании заказа
#[derive(Debug, Clone, Deserialize)]
pub struct CreateOrderResponse {
    pub message: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    pub status: String,
    pub total: f64,
}

/// 🌐 Go Backend Client
pub struct GoClient {
    client: Client,
    base_url: String,
}

impl GoClient {
    /// Создать новый клиент
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, base_url }
    }

    /// 📊 Получить метрики бизнеса
    pub async fn fetch_business_metrics(&self, business_id: &str) -> Result<BusinessMetrics> {
        let url = format!("{}/metrics/{}", self.base_url, business_id);
        
        tracing::info!("📊 Fetching metrics for business: {}", business_id);
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch metrics: {}", response.status());
        }
        
        let json: serde_json::Value = response.json().await?;
        let metrics = serde_json::from_value(json["metrics"].clone())?;
        
        tracing::info!("✅ Metrics fetched successfully");
        Ok(metrics)
    }

    /// 🏢 Получить список всех бизнесов
    pub async fn fetch_businesses(&self) -> Result<Vec<Business>> {
        let url = format!("{}/businesses", self.base_url);
    
    tracing::info!("🏢 Fetching businesses list");
    
    let response = self.client.get(&url).send().await?;
    
    if !response.status().is_success() {
        anyhow::bail!("Failed to fetch businesses: {}", response.status());
    }
    
    // Go backend возвращает массив напрямую, не обёрнутый в объект
    let businesses = response.json::<Vec<Business>>().await?;
    
    tracing::info!("✅ Fetched {} businesses", businesses.len());
    Ok(businesses)
}    /// 🏢 Получить информацию о конкретном бизнесе
    pub async fn fetch_business(&self, business_id: &str) -> Result<Business> {
        let url = format!("{}/businesses/{}", self.base_url, business_id);
        
        tracing::info!("🏢 Fetching business: {}", business_id);
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch business: {}", response.status());
        }
        
        let business = response.json::<Business>().await?;
        
        tracing::info!("✅ Business fetched: {}", business.name);
        Ok(business)
    }

    /// 🔑 Аутентификация пользователя
    pub async fn login(&self, email: &str, password: &str) -> Result<TokenResponse> {
        let url = format!("{}/auth/login", self.base_url);
        
        tracing::info!("🔑 Logging in user: {}", email);
        
        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "email": email,
                "password": password,
            }))
            .send()
            .await?;
        
        if !response.status().is_success() {
            anyhow::bail!("Login failed: {}", response.status());
        }
        
        let token_response = response.json::<TokenResponse>().await?;
        
        tracing::info!("✅ User logged in: {}", token_response.user.name);
        Ok(token_response)
    }

    /// 🛒 Создать заказ
    pub async fn create_order(&self, order_data: CreateOrderData) -> Result<CreateOrderResponse> {
        let url = format!("{}/orders", self.base_url);
        
        tracing::info!("🛒 Creating order for user: {}", order_data.user_id);
        
        let response = self
            .client
            .post(&url)
            .json(&order_data)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to create order: {} - {}", status, error_text);
        }
        
        let order_response = response.json::<CreateOrderResponse>().await?;
        
        tracing::info!("✅ Order created: {}", order_response.order_id);
        Ok(order_response)
    }

    /// 🔍 Проверить токен
    pub async fn verify_token(&self, token: &str) -> Result<UserInfo> {
        let url = format!("{}/auth/verify", self.base_url);
        
        let response = self
            .client
            .post(&url)
            .bearer_auth(token)
            .send()
            .await?;
        
        if !response.status().is_success() {
            anyhow::bail!("Token verification failed: {}", response.status());
        }
        
        let user = response.json::<UserInfo>().await?;
        Ok(user)
    }
}

/// 🚀 Удобная функция для получения метрик
pub async fn fetch_business_metrics(business_id: &str) -> Result<BusinessMetrics> {
    let base_url = std::env::var("GO_BACKEND_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080/api".to_string());
    
    let client = GoClient::new(base_url);
    client.fetch_business_metrics(business_id).await
}

/// 🚀 Удобная функция для получения списка бизнесов
/// 🚀 Удобная функция для получения списка бизнесов
pub async fn fetch_businesses() -> Result<Vec<Business>> {
    let base_url = std::env::var("GO_BACKEND_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080/api".to_string());
    
    let client = GoClient::new(base_url);
    client.fetch_businesses().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_go_client_creation() {
        let client = GoClient::new("http://localhost:8080/api".to_string());
        assert_eq!(client.base_url, "http://localhost:8080/api");
    }
}

use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::Value;

use super::types::{Order, OrdersResponse};

/// 📦 Orders service
pub struct OrdersClient {
    client: Client,
    base_url: String,
}

impl OrdersClient {
    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
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

    /// Get recent orders (admin only)
    pub async fn get_recent_orders(&self, token: &str) -> Result<Vec<Order>> {
        let url = format!("{}/admin/orders/recent", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to fetch recent orders")?;

        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read recent orders body")?;

        tracing::info!("📦 Raw recent orders response ({}): {}", status, text);

        // Try as array first
        if let Ok(orders) = serde_json::from_str::<Vec<Order>>(&text) {
            return Ok(orders);
        }

        // Try as object with orders field
        let orders_response: OrdersResponse =
            serde_json::from_str(&text).context("Failed to parse recent orders JSON")?;

        Ok(orders_response.orders)
    }

    /// Get all orders (admin only)
    pub async fn get_all_orders_admin(&self, token: &str) -> Result<Vec<Order>> {
        let url = format!("{}/admin/orders", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to fetch admin orders")?;

        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read admin orders body")?;

        tracing::info!("📦 Raw all orders response ({}): {}", status, text);

        let orders_response: OrdersResponse =
            serde_json::from_str(&text).context("Failed to parse admin orders JSON")?;

        Ok(orders_response.orders)
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

        // Parse CreateOrderResponse first
        let create_response = response
            .json::<super::types::CreateOrderResponse>()
            .await
            .context("Failed to parse order response")?;

        // Convert to Order struct for compatibility
        Ok(Order {
            id: create_response.order_id,
            user_id: None,
            status: create_response.status,
            total: create_response.total,
            address: None,
            phone: None,
            comment: None,
            created_at: None,
            items: vec![],
            user: None,
        })
    }

    /// Update order status (admin only)
    pub async fn update_order_status_admin(
        &self,
        token: &str,
        id: i64,
        status: &str,
    ) -> Result<Order> {
        let url = format!("{}/admin/orders/{}/status", self.base_url, id);

        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&serde_json::json!({ "status": status }))
            .send()
            .await
            .context("Failed to update order status")?;

        let order = response
            .json::<Order>()
            .await
            .context("Failed to parse updated order response")?;

        Ok(order)
    }

    /// Update order status (legacy method)
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

    /// Send order notification to backend
    #[allow(dead_code)]
    pub async fn send_order_notification(order_id: &str, total: f64) -> Result<()> {
        let backend_url =
            std::env::var("GO_BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".into());

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
}

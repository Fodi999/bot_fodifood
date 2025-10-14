mod admin;
mod auth;
mod orders;
mod products;
pub mod types;

pub use admin::AdminClient;
pub use auth::AuthClient;
pub use orders::OrdersClient;
pub use products::ProductsClient;
pub use types::*;

use crate::config::Config;
use reqwest::Client;

/// 🌐 Go Backend Client - Unified facade for all services
pub struct GoBackendClient {
    pub auth: AuthClient,
    pub products: ProductsClient,
    pub orders: OrdersClient,
    pub admin: AdminClient,
}

impl GoBackendClient {
    pub fn new(config: &Config) -> Self {
        let client = Client::new();
        let base_url = config.go_backend_url.clone();

        Self {
            auth: AuthClient::new(client.clone(), base_url.clone()),
            products: ProductsClient::new(client.clone(), base_url.clone()),
            orders: OrdersClient::new(client.clone(), base_url.clone()),
            admin: AdminClient::new(client, base_url),
        }
    }

    // ============================================================================
    // Convenience methods (delegates to underlying services)
    // ============================================================================

    /// Login user (delegates to auth service)
    pub async fn login(&self, email: &str, password: &str) -> anyhow::Result<LoginResponse> {
        self.auth.login(email, password).await
    }

    /// Register user (delegates to auth service)
    pub async fn register(
        &self,
        email: &str,
        password: &str,
        name: &str,
    ) -> anyhow::Result<LoginResponse> {
        self.auth.register(email, password, name).await
    }

    /// Verify token (delegates to auth service)
    pub async fn verify_token(
        &self,
        token: &str,
    ) -> anyhow::Result<crate::models::user::VerifyTokenResponse> {
        self.auth.verify_token(token).await
    }

    /// Get products (delegates to products service)
    pub async fn get_products(&self) -> anyhow::Result<Vec<Product>> {
        self.products.get_products().await
    }

    /// Get user profile (delegates to auth service)
    pub async fn get_user_profile(&self, token: &str) -> anyhow::Result<UserProfile> {
        self.auth.get_user_profile(token).await
    }

    /// Get all orders (delegates to orders service)
    pub async fn get_orders(&self) -> anyhow::Result<Vec<Order>> {
        self.orders.get_orders().await
    }

    /// Get recent orders (delegates to orders service)
    pub async fn get_recent_orders(&self, token: &str) -> anyhow::Result<Vec<Order>> {
        self.orders.get_recent_orders(token).await
    }

    /// Create order (delegates to orders service)
    pub async fn create_order(&self, order_data: serde_json::Value) -> anyhow::Result<Order> {
        self.orders.create_order(order_data).await
    }

    /// Get ingredients (delegates to admin service)
    pub async fn get_ingredients(&self, token: &str) -> anyhow::Result<Vec<Ingredient>> {
        self.admin.get_ingredients(token).await
    }

    /// Create ingredient (delegates to admin service)
    pub async fn create_ingredient(
        &self,
        token: &str,
        data: serde_json::Value,
    ) -> anyhow::Result<Ingredient> {
        self.admin.create_ingredient(token, data).await
    }

    /// Update ingredient (delegates to admin service)
    pub async fn update_ingredient(
        &self,
        token: &str,
        id: i64,
        data: serde_json::Value,
    ) -> anyhow::Result<Ingredient> {
        self.admin.update_ingredient(token, id, data).await
    }

    /// Delete ingredient (delegates to admin service)
    pub async fn delete_ingredient(&self, token: &str, id: i64) -> anyhow::Result<()> {
        self.admin.delete_ingredient(token, id).await
    }

    /// Get ingredient movements (delegates to admin service)
    pub async fn get_ingredient_movements(
        &self,
        token: &str,
        id: i64,
    ) -> anyhow::Result<Vec<IngredientMovement>> {
        self.admin.get_ingredient_movements(token, id).await
    }

    /// Get users (delegates to auth service)
    pub async fn get_users(&self, token: &str) -> anyhow::Result<Vec<UserProfile>> {
        self.auth.get_users(token).await
    }

    /// Update user (delegates to auth service)
    pub async fn update_user(
        &self,
        token: &str,
        id: &str,
        data: serde_json::Value,
    ) -> anyhow::Result<UserProfile> {
        self.auth.update_user(token, id, data).await
    }

    /// Delete user (delegates to auth service)
    pub async fn delete_user(&self, token: &str, id: &str) -> anyhow::Result<()> {
        self.auth.delete_user(token, id).await
    }

    /// Get all orders admin (delegates to orders service)
    pub async fn get_all_orders_admin(&self, token: &str) -> anyhow::Result<Vec<Order>> {
        self.orders.get_all_orders_admin(token).await
    }

    /// Update order status admin (delegates to orders service)
    pub async fn update_order_status_admin(
        &self,
        token: &str,
        id: i64,
        status: &str,
    ) -> anyhow::Result<Order> {
        self.orders
            .update_order_status_admin(token, id, status)
            .await
    }

    /// Get stats (delegates to admin service)
    pub async fn get_stats(&self, token: &str) -> anyhow::Result<Stats> {
        self.admin.get_stats(token).await
    }

    // ============================================================================
    // Static utility methods (product formatting and search)
    // ============================================================================

    /// Format products list for display
    pub fn format_products_list(products: &[Product]) -> String {
        ProductsClient::format_products_list(products)
    }

    /// Find product by name
    pub fn find_product_by_name<'a>(products: &'a [Product], query: &str) -> Option<&'a Product> {
        ProductsClient::find_product_by_name(products, query)
    }

    /// Filter products by ingredient
    pub fn filter_by_ingredient<'a>(products: &'a [Product], ingredient: &str) -> Vec<&'a Product> {
        ProductsClient::filter_by_ingredient(products, ingredient)
    }
}

// ============================================================================
// Standalone utility functions
// ============================================================================

/// Send order notification to backend
pub async fn send_order_to_backend(order_id: &str, total: f64) -> anyhow::Result<()> {
    OrdersClient::send_order_notification(order_id, total).await
}

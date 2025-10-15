use serde::{Deserialize, Serialize};
use serde_json::Value;

// ============================================================================
// Authentication Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
}

// ============================================================================
// Product Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
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

// ============================================================================
// Order Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    pub status: String,
    pub total: f64,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub comment: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(default)]
    pub items: Vec<OrderItem>,
    #[serde(default)]
    pub user: Option<OrderUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Option<String>,
    #[serde(rename = "productId")]
    pub product_id: Option<i64>,
    pub quantity: i32,
    pub price: f64,
    #[serde(default)]
    pub product: Option<OrderProduct>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderUser {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderProduct {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrdersResponse {
    pub orders: Vec<Order>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub message: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    pub status: String,
    pub total: f64,
}

// ============================================================================
// Inventory Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: i64,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub min_quantity: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientMovement {
    pub id: i64,
    pub ingredient_id: i64,
    pub movement_type: String,
    pub quantity: f64,
    pub reason: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemiFinished {
    pub id: i64,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub min_quantity: Option<f64>,
}

// ============================================================================
// Statistics Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    #[serde(rename = "totalUsers")]
    pub total_users: Option<i64>,
    #[serde(rename = "totalOrders")]
    pub total_orders: i64,
    #[serde(rename = "totalProducts")]
    pub total_products: Option<i64>,
    pub revenue: f64,
    #[serde(default)]
    pub today_orders: i64,
    #[serde(default)]
    pub today_revenue: f64,
    #[serde(default)]
    pub popular_products: Vec<Value>,
}

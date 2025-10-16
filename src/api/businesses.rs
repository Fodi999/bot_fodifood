use axum::{extract::State, http::HeaderMap, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Business {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBusinessPayload {
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub city: Option<String>,
}

// Вложенные структуры для ответа от Go backend
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateBusinessResponse {
    pub message: String,
    pub business: BusinessFull,
    pub token: TokenFull,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BusinessFull {
    pub id: String,
    #[serde(rename = "ownerId")]
    pub owner_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenFull {
    pub id: String,
    #[serde(rename = "businessId")]
    pub business_id: String,
    pub symbol: String,
    #[serde(rename = "totalSupply")]
    pub total_supply: i32,
    pub price: f64,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub business: Option<NestedBusiness>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NestedBusiness {
    pub id: String,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub city: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/businesses", get(get_businesses).post(create_business))
        .route("/businesses", get(get_businesses).post(create_business)) // 🔗 Прямой маршрут для Frontend
}

async fn get_businesses(
    State(state): State<AppState>,
) -> Result<Json<Vec<Business>>, (axum::http::StatusCode, String)> {
    let go_api = &state.config.go_backend_url;
    
    // Убираем /api если оно уже есть в URL
    let base_url = go_api.trim_end_matches("/api");
    let url = format!("{}/api/businesses", base_url);

    tracing::info!("📡 Proxying businesses request to: {}", url);

    let client = Client::new();
    let res = client.get(&url).send().await.map_err(|e| {
        tracing::error!("❌ Failed to reach Go backend: {}", e);
        (
            axum::http::StatusCode::BAD_GATEWAY,
            format!("Failed to reach Go backend: {}", e),
        )
    })?;

    if !res.status().is_success() {
        let status = res.status();
        tracing::error!("❌ Go backend returned status: {}", status);
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            format!("Go backend returned {}", status),
        ));
    }

    let businesses: Vec<Business> = res.json().await.map_err(|e| {
        tracing::error!("❌ Invalid JSON from Go backend: {}", e);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Invalid JSON from Go: {}", e),
        )
    })?;

    tracing::info!("✅ Successfully proxied {} businesses", businesses.len());
    Ok(Json(businesses))
}

/// POST /businesses - Создание нового бизнеса
async fn create_business(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateBusinessPayload>,
) -> Result<Json<CreateBusinessResponse>, (axum::http::StatusCode, String)> {
    tracing::info!("📝 Creating new business: {}", payload.name);

    // Извлекаем токен из заголовка Authorization
    let token = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .trim_start_matches("Bearer ")
        .trim();

    if token.is_empty() {
        tracing::warn!("❌ No authorization token provided");
        return Err((
            axum::http::StatusCode::UNAUTHORIZED,
            "Authorization token required".to_string(),
        ));
    }

    // Верифицируем токен
    tracing::info!("🔍 Verifying token for business creation...");
    let verify_response = match state.backend.verify_token(token).await {
        Ok(response) if response.valid => response,
        Ok(_) => {
            tracing::warn!("❌ Token verification failed: invalid token");
            return Err((
                axum::http::StatusCode::UNAUTHORIZED,
                "Invalid token".to_string(),
            ));
        }
        Err(e) => {
            tracing::error!("❌ Token verification error: {}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Token verification failed: {}", e),
            ));
        }
    };

    // Проверяем, что пользователь имеет права на создание бизнеса
    let user_role = verify_response.role.as_deref().unwrap_or("client");
    if user_role != "admin" && user_role != "business_owner" {
        tracing::warn!("❌ User role {} not allowed to create business", user_role);
        return Err((
            axum::http::StatusCode::FORBIDDEN,
            "Only admin or business_owner can create businesses".to_string(),
        ));
    }

    tracing::info!("✅ Token verified for user with role: {}", user_role);

    // Отправляем запрос в Go backend
    let go_api = &state.config.go_backend_url;
    let base_url = go_api.trim_end_matches("/api");
    let url = format!("{}/api/businesses", base_url);

    tracing::info!("📡 Sending create business request to Go backend: {}", url);

    let client = Client::new();
    let response = client
        .post(&url)
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("❌ Failed to reach Go backend: {}", e);
            (
                axum::http::StatusCode::BAD_GATEWAY,
                format!("Failed to reach Go backend: {}", e),
            )
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();
        tracing::error!("❌ Go backend returned error: {} - {}", status, error_body);
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            format!("Go backend error: {}", error_body),
        ));
    }

    // Логируем сырой ответ для отладки
    let response_text = response.text().await.map_err(|e| {
        tracing::error!("❌ Failed to read response body: {}", e);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to read response".to_string(),
        )
    })?;
    
    tracing::info!("🧾 Raw create business response: {}", response_text);

    // Парсим ответ от Go backend с вложенными объектами
    let create_response: CreateBusinessResponse = serde_json::from_str(&response_text).map_err(|e| {
        tracing::error!("❌ Invalid JSON response from Go backend: {} | Raw: {}", e, response_text);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Invalid response from Go backend: {}", e),
        )
    })?;

    tracing::info!(
        "✅ Business created successfully: {} (ID: {}), Token: {} @ ${}", 
        create_response.business.name, 
        create_response.business.id,
        create_response.token.symbol,
        create_response.token.price
    );

    Ok(Json(create_response))
}

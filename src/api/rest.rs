use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::ai::{Intent, IntentClassifier};
use crate::state::AppState;

/// 🤖 Запрос к AI боту
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub user_id: String,
    pub message: String,
}

/// 🤖 Ответ от AI бота
#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub intent: String,
    pub response: String,
    pub suggestions: Option<Vec<String>>,
    pub products: Option<Vec<ProductInfo>>,
}

/// 📦 Информация о продукте
#[derive(Debug, Serialize)]
pub struct ProductInfo {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    pub category: Option<String>,
}

/// 🔍 Поиск по ингредиентам
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub ingredient: String,
}

/// 🎯 Рекомендации
#[derive(Debug, Deserialize)]
pub struct RecommendationRequest {
    pub user_id: String,
    #[allow(dead_code)] // Will be used for filtering recommendations
    pub preferences: Option<Vec<String>>,
}

/// 🔐 Запрос на логин
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// 📝 Запрос на регистрацию
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

/// 🔑 Ответ с токеном
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserData,
}

#[derive(Debug, Serialize)]
pub struct UserData {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
}

// ============================================================================
// REST API Handlers
// ============================================================================

/// POST /api/v1/auth/login - Авторизация пользователя
pub async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    tracing::info!("🔐 Login attempt for: {}", req.email);

    let login_response = state
        .backend
        .login(&req.email, &req.password)
        .await
        .map_err(|e| {
            tracing::error!("❌ Login error: {}", e);
            (StatusCode::UNAUTHORIZED, format!("Login failed: {}", e))
        })?;

    Ok(Json(AuthResponse {
        token: login_response.token,
        user: UserData {
            id: login_response.user.id,
            email: login_response.user.email,
            name: login_response.user.name,
            role: login_response.user.role,
        },
    }))
}

/// POST /api/v1/auth/register - Регистрация нового пользователя
pub async fn register_handler(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    tracing::info!("📝 Registration attempt for: {}", req.email);

    let register_response = state
        .backend
        .register(&req.email, &req.password, &req.name)
        .await
        .map_err(|e| {
            tracing::error!("❌ Registration error: {}", e);
            (
                StatusCode::BAD_REQUEST,
                format!("Registration failed: {}", e),
            )
        })?;

    Ok(Json(AuthResponse {
        token: register_response.token,
        user: UserData {
            id: register_response.user.id,
            email: register_response.user.email,
            name: register_response.user.name,
            role: register_response.user.role,
        },
    }))
}

/// GET /api/v1/user/profile - Get authenticated user profile
pub async fn get_user_profile(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<UserData>, (StatusCode, String)> {
    // Извлекаем токен из заголовка Authorization
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            tracing::warn!("❌ Missing Authorization header");
            (
                StatusCode::UNAUTHORIZED,
                "Missing Authorization header".to_string(),
            )
        })?;

    // Проверяем формат "Bearer <token>"
    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        tracing::warn!("❌ Invalid Authorization header format");
        (
            StatusCode::UNAUTHORIZED,
            "Invalid Authorization header format".to_string(),
        )
    })?;

    tracing::info!("� Verifying token for profile request");

    // Верифицируем токен через Go backend
    let verify_response = state.backend.verify_token(token).await.map_err(|e| {
        tracing::error!("❌ Token verification failed: {}", e);
        (StatusCode::UNAUTHORIZED, "Invalid token".to_string())
    })?;

    if !verify_response.valid {
        tracing::warn!("❌ Token is not valid");
        return Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()));
    }

    // Получаем профиль пользователя
    let profile = state.backend.get_user_profile(token).await.map_err(|e| {
        tracing::error!("❌ Failed to get user profile: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get profile: {}", e),
        )
    })?;

    tracing::info!("✅ Profile retrieved for user: {}", profile.email);

    Ok(Json(UserData {
        id: profile.id,
        email: profile.email,
        name: profile.name,
        role: profile.role,
    }))
}

// ============================================================================
// Admin Endpoints
// ============================================================================

/// 📊 Статистика (admin only)
#[derive(Debug, Serialize)]
pub struct StatsResponse {
    #[serde(rename = "totalUsers")]
    pub total_users: Option<i64>,
    #[serde(rename = "totalOrders")]
    pub total_orders: i64,
    #[serde(rename = "totalProducts")]
    pub total_products: Option<i64>,
    pub revenue: f64,
}

/// 📦 Заказ для админа
#[derive(Debug, Serialize)]
pub struct OrderResponse {
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
    pub items: Vec<OrderItemResponse>,
}

#[derive(Debug, Serialize)]
pub struct OrderItemResponse {
    pub id: Option<String>,
    #[serde(rename = "productId")]
    pub product_id: Option<i64>,
    pub quantity: i32,
    pub price: f64,
    pub product: Option<OrderProductResponse>,
}

#[derive(Debug, Serialize)]
pub struct OrderProductResponse {
    pub id: String,
    pub name: String,
}

/// 👤 User для админа
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
}

/// GET /api/v1/admin/stats - Получить статистику (admin only)
pub async fn get_admin_stats(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<StatsResponse>, (StatusCode, String)> {
    // Извлекаем и проверяем токен
    let token = extract_bearer_token(&headers)?;

    tracing::info!("📊 Getting admin stats");

    // Верифицируем токен
    let verify_response = state.backend.verify_token(token).await.map_err(|e| {
        tracing::error!("❌ Token verification failed: {}", e);
        (StatusCode::UNAUTHORIZED, "Invalid token".to_string())
    })?;

    if !verify_response.valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()));
    }

    // Проверяем роль пользователя
    if verify_response.role != Some(crate::models::user::UserRole::Admin) {
        tracing::warn!("❌ User is not admin: {:?}", verify_response.role);
        return Err((StatusCode::FORBIDDEN, "Admin access required".to_string()));
    }

    // Получаем статистику из Go backend
    let stats = state.backend.get_stats(token).await.map_err(|e| {
        tracing::error!("❌ Failed to get stats: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get stats: {}", e),
        )
    })?;

    Ok(Json(StatsResponse {
        total_users: stats.total_users,
        total_orders: stats.total_orders,
        total_products: stats.total_products,
        revenue: stats.revenue,
    }))
}

/// GET /api/v1/admin/orders/recent - Получить последние заказы (admin only)
pub async fn get_recent_orders(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<OrderResponse>>, (StatusCode, String)> {
    // Извлекаем и проверяем токен
    let token = extract_bearer_token(&headers)?;

    tracing::info!("📦 Getting recent orders");

    // Верифицируем токен
    let verify_response = state.backend.verify_token(token).await.map_err(|e| {
        tracing::error!("❌ Token verification failed: {}", e);
        (StatusCode::UNAUTHORIZED, "Invalid token".to_string())
    })?;

    if !verify_response.valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()));
    }

    // Проверяем роль пользователя
    if verify_response.role != Some(crate::models::user::UserRole::Admin) {
        tracing::warn!("❌ User is not admin: {:?}", verify_response.role);
        return Err((StatusCode::FORBIDDEN, "Admin access required".to_string()));
    }

    // Получаем заказы из Go backend
    let orders = state.backend.get_recent_orders(token).await.map_err(|e| {
        tracing::error!("❌ Failed to get recent orders: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get orders: {}", e),
        )
    })?;

    let order_responses: Vec<OrderResponse> = orders
        .iter()
        .map(|o| OrderResponse {
            id: o.id.clone(),
            user_id: o.user_id.clone(),
            status: o.status.clone(),
            total: o.total,
            address: o.address.clone(),
            phone: o.phone.clone(),
            comment: o.comment.clone(),
            created_at: o.created_at.clone(),
            items: o
                .items
                .iter()
                .map(|item| OrderItemResponse {
                    id: item.id.clone(),
                    product_id: item.product_id,
                    quantity: item.quantity,
                    price: item.price,
                    product: item.product.as_ref().map(|p| OrderProductResponse {
                        id: p.id.clone(),
                        name: p.name.clone(),
                    }),
                })
                .collect(),
        })
        .collect();

    Ok(Json(order_responses))
}

/// GET /api/v1/admin/users - Получить всех пользователей (admin only)
pub async fn get_admin_users(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<UserResponse>>, (StatusCode, String)> {
    // Извлекаем и проверяем токен
    let token = extract_bearer_token(&headers)?;

    tracing::info!("👥 Getting admin users");

    // Верифицируем токен
    let verify_response = state.backend.verify_token(token).await.map_err(|e| {
        tracing::error!("❌ Token verification failed: {}", e);
        (StatusCode::UNAUTHORIZED, "Invalid token".to_string())
    })?;

    if !verify_response.valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()));
    }

    // Проверяем роль пользователя
    if verify_response.role != Some(crate::models::user::UserRole::Admin) {
        tracing::warn!("❌ User is not admin: {:?}", verify_response.role);
        return Err((StatusCode::FORBIDDEN, "Admin access required".to_string()));
    }

    // Получаем пользователей из Go backend
    let users = state.backend.get_users(token).await.map_err(|e| {
        tracing::error!("❌ Failed to get users: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get users: {}", e),
        )
    })?;

    let user_responses: Vec<UserResponse> = users
        .iter()
        .map(|u| UserResponse {
            id: u.id.clone(),
            email: u.email.clone(),
            name: u.name.clone(),
            role: u.role.clone(),
            created_at: u.created_at.clone(),
        })
        .collect();

    Ok(Json(user_responses))
}

/// GET /api/v1/admin/orders - Получить все заказы (admin only)
pub async fn get_admin_orders(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<OrderResponse>>, (StatusCode, String)> {
    // Извлекаем и проверяем токен
    let token = extract_bearer_token(&headers)?;

    tracing::info!("📦 Getting all admin orders");

    // Верифицируем токен
    let verify_response = state.backend.verify_token(token).await.map_err(|e| {
        tracing::error!("❌ Token verification failed: {}", e);
        (StatusCode::UNAUTHORIZED, "Invalid token".to_string())
    })?;

    if !verify_response.valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()));
    }

    // Проверяем роль пользователя
    if verify_response.role != Some(crate::models::user::UserRole::Admin) {
        tracing::warn!("❌ User is not admin: {:?}", verify_response.role);
        return Err((StatusCode::FORBIDDEN, "Admin access required".to_string()));
    }

    // Получаем все заказы из Go backend
    let orders = state
        .backend
        .get_all_orders_admin(token)
        .await
        .map_err(|e| {
            tracing::error!("❌ Failed to get all orders: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get orders: {}", e),
            )
        })?;

    let order_responses: Vec<OrderResponse> = orders
        .iter()
        .map(|o| OrderResponse {
            id: o.id.clone(),
            user_id: o.user_id.clone(),
            status: o.status.clone(),
            total: o.total,
            address: o.address.clone(),
            phone: o.phone.clone(),
            comment: o.comment.clone(),
            created_at: o.created_at.clone(),
            items: o
                .items
                .iter()
                .map(|item| OrderItemResponse {
                    id: item.id.clone(),
                    product_id: item.product_id,
                    quantity: item.quantity,
                    price: item.price,
                    product: item.product.as_ref().map(|p| OrderProductResponse {
                        id: p.id.clone(),
                        name: p.name.clone(),
                    }),
                })
                .collect(),
        })
        .collect();

    Ok(Json(order_responses))
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Извлечь Bearer токен из заголовков
fn extract_bearer_token(headers: &axum::http::HeaderMap) -> Result<&str, (StatusCode, String)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            tracing::warn!("❌ Missing Authorization header");
            (
                StatusCode::UNAUTHORIZED,
                "Missing Authorization header".to_string(),
            )
        })?;

    auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        tracing::warn!("❌ Invalid Authorization header format");
        (
            StatusCode::UNAUTHORIZED,
            "Invalid Authorization header format".to_string(),
        )
    })
}

// ============================================================================
// REST API Handlers
// ============================================================================

/// POST /api/v1/chat - Отправить сообщение боту
pub async fn chat_handler(
    State(state): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, String)> {
    tracing::info!("💬 Chat request from user {}: {}", req.user_id, req.message);

    // Определяем интент
    let intent = IntentClassifier::classify(&req.message);
    tracing::info!("🎯 Detected intent: {:?}", intent);

    // Обрабатываем сообщение через AI
    let response = state
        .ai
        .process_message(&req.user_id, &req.message)
        .await
        .map_err(|e| {
            tracing::error!("❌ AI processing error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("AI error: {}", e),
            )
        })?;

    // Формируем ответ в зависимости от интента
    let chat_response = match intent {
        Intent::SearchByIngredient => {
            // Извлекаем ингредиент
            let ingredient = IntentClassifier::extract_ingredient(&req.message);

            // Получаем продукты из бэкенда
            let products = state.backend.get_products().await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Backend error: {}", e),
                )
            })?;

            // Фильтруем по ингредиенту
            let matched = crate::api::go_backend::GoBackendClient::filter_by_ingredient(
                &products,
                &ingredient,
            );

            let product_infos: Vec<ProductInfo> = matched
                .iter()
                .map(|p| ProductInfo {
                    id: p.id.clone(),
                    name: p.name.clone(),
                    price: p.price,
                    description: p.description.clone(),
                    image_url: p.image_url.clone(),
                    category: p.category.clone(),
                })
                .collect();

            ChatResponse {
                intent: format!("{:?}", intent),
                response,
                suggestions: Some(vec![
                    "Добавить в корзину".to_string(),
                    "Узнать больше".to_string(),
                    "Показать похожие".to_string(),
                ]),
                products: Some(product_infos),
            }
        }
        Intent::ViewMenu => {
            let products = state.backend.get_products().await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Backend error: {}", e),
                )
            })?;

            let product_infos: Vec<ProductInfo> = products
                .iter()
                .map(|p| ProductInfo {
                    id: p.id.clone(),
                    name: p.name.clone(),
                    price: p.price,
                    description: p.description.clone(),
                    image_url: p.image_url.clone(),
                    category: p.category.clone(),
                })
                .collect();

            ChatResponse {
                intent: format!("{:?}", intent),
                response,
                suggestions: Some(vec![
                    "Показать роллы".to_string(),
                    "Показать суши".to_string(),
                    "Что посоветуешь?".to_string(),
                ]),
                products: Some(product_infos),
            }
        }
        _ => ChatResponse {
            intent: format!("{:?}", intent),
            response,
            suggestions: None,
            products: None,
        },
    };

    Ok(Json(chat_response))
}

/// GET /api/v1/search?ingredient=лосось - Поиск по ингредиенту
pub async fn search_by_ingredient(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<ProductInfo>>, (StatusCode, String)> {
    tracing::info!("🔍 Searching for ingredient: {}", query.ingredient);

    let products = state.backend.get_products().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Backend error: {}", e),
        )
    })?;

    let matched =
        crate::api::go_backend::GoBackendClient::filter_by_ingredient(&products, &query.ingredient);

    let result: Vec<ProductInfo> = matched
        .iter()
        .map(|p| ProductInfo {
            id: p.id.clone(),
            name: p.name.clone(),
            price: p.price,
            description: p.description.clone(),
            image_url: p.image_url.clone(),
            category: p.category.clone(),
        })
        .collect();

    Ok(Json(result))
}

/// POST /api/v1/recommendations - Получить рекомендации
pub async fn get_recommendations(
    State(state): State<AppState>,
    Json(req): Json<RecommendationRequest>,
) -> Result<Json<Vec<ProductInfo>>, (StatusCode, String)> {
    tracing::info!("🌟 Getting recommendations for user: {}", req.user_id);

    // Получаем все продукты
    let products = state.backend.get_products().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Backend error: {}", e),
        )
    })?;

    // TODO: Реализовать умные рекомендации на основе истории пользователя
    // Пока возвращаем топ-3 самых популярных
    let top_products: Vec<ProductInfo> = products
        .iter()
        .take(3)
        .map(|p| ProductInfo {
            id: p.id.clone(),
            name: p.name.clone(),
            price: p.price,
            description: p.description.clone(),
            image_url: p.image_url.clone(),
            category: p.category.clone(),
        })
        .collect();

    Ok(Json(top_products))
}

/// GET /api/v1/intents/{text} - Определить интент текста
pub async fn detect_intent(
    Path(text): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let intent = IntentClassifier::classify(&text);

    let response = json!({
        "text": text,
        "intent": format!("{:?}", intent),
        "extracted_ingredient": match intent {
            Intent::SearchByIngredient => Some(IntentClassifier::extract_ingredient(&text)),
            _ => None,
        },
        "extracted_order_id": IntentClassifier::extract_order_id(&text),
    });

    Ok(Json(response))
}

/// GET /api/v1/health - Health check
pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "FodiFood AI Bot",
        "version": "1.0.0"
    }))
}

/// GET /api/v1/products - Получить все продукты из меню
pub async fn get_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProductInfo>>, (StatusCode, String)> {
    let products = state.backend.get_products().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Backend error: {}", e),
        )
    })?;

    let product_list: Vec<ProductInfo> = products
        .iter()
        .filter(|p| p.is_visible.unwrap_or(true)) // Только видимые продукты
        .map(|p| ProductInfo {
            id: p.id.clone(),
            name: p.name.clone(),
            price: p.price,
            description: p.description.clone(),
            image_url: p.image_url.clone(),
            category: p.category.clone(),
        })
        .collect();

    Ok(Json(product_list))
}

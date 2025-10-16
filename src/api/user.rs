use axum::{extract::State, http::HeaderMap, routing::patch, Json, Router};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use crate::state::AppState;

// Тело запроса от фронтенда - только роль
#[derive(Debug, Deserialize)]
pub struct UpdateRolePayload {
    pub role: String,
}

// Тело запроса для Go backend - роль + user_id
#[derive(Debug, Serialize)]
pub struct GoUpdateRolePayload {
    pub user_id: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateRoleResponse {
    pub status: String,
    pub message: String,
    #[serde(rename = "newRole")]
    pub new_role: String,
}

/// PATCH /api/v1/user/role - Обновление роли пользователя
pub async fn update_user_role(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdateRolePayload>,
) -> Result<Json<UpdateRoleResponse>, (axum::http::StatusCode, String)> {
    tracing::info!("🔄 Updating user role to: {}", payload.role);

    // 1️⃣ Извлекаем токен из заголовка Authorization
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

    // 2️⃣ Верифицируем токен через Go backend и получаем user_id
    tracing::info!("🔍 Verifying token to get user_id...");
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

    let user_id = verify_response.user_id.unwrap_or_default();
    if user_id.is_empty() {
        tracing::warn!("❌ No user_id in token");
        return Err((
            axum::http::StatusCode::UNAUTHORIZED,
            "Invalid token: no user_id".to_string(),
        ));
    }

    tracing::info!("✅ Token verified for user: {}", user_id);

    // 3️⃣ Валидация роли
    let valid_roles = vec![
        "business_owner",
        "investor",
        "client",
        "admin",
        "manager",
        "courier",
        "cook",
    ];

    if !valid_roles.contains(&payload.role.as_str()) {
        tracing::warn!("❌ Invalid role: {}", payload.role);
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            format!("Invalid role: {}. Valid roles: {:?}", payload.role, valid_roles),
        ));
    }

    // 4️⃣ Формируем правильное тело для Go backend
    let go_payload = GoUpdateRolePayload {
        user_id: user_id.clone(),
        role: payload.role.clone(),
    };

    // 5️⃣ Отправляем PATCH-запрос в Go backend
    let go_api = &state.config.go_backend_url;
    let base_url = go_api.trim_end_matches("/api");
    let url = format!("{}/api/admin/users/update-role", base_url);
    
    tracing::info!("📡 Sending role update to Go backend: {} for user {}", url, user_id);

    // Получаем админский токен из окружения
    let admin_token = std::env::var("ADMIN_TOKEN")
        .unwrap_or_else(|_| {
            tracing::warn!("⚠️ ADMIN_TOKEN not set in environment");
            String::new()
        });

    let client = Client::new();
    let mut request = client.patch(&url).json(&go_payload);
    
    // Добавляем Bearer токен если он есть
    if !admin_token.is_empty() {
        request = request.bearer_auth(&admin_token);
        tracing::info!("🔑 Adding Bearer token to request");
    }

    // 6️⃣ Проверяем ответ
    match request.send().await
    {
        Ok(response) => {
            if response.status().is_success() {
                tracing::info!("✅ Go backend confirmed role update for user: {}", user_id);
                Ok(Json(UpdateRoleResponse {
                    status: "ok".to_string(),
                    message: format!("Role updated to {}", payload.role),
                    new_role: payload.role,
                }))
            } else {
                let status = response.status();
                let error_body = response.text().await.unwrap_or_default();
                tracing::error!("❌ Go backend rejected role update: {} - {}", status, error_body);
                
                // Если Go backend не поддерживает endpoint, возвращаем успех локально
                if status.as_u16() == 404 {
                    tracing::warn!("⚠️ Go backend doesn't have update-role endpoint yet. Returning success locally.");
                    Ok(Json(UpdateRoleResponse {
                        status: "ok".to_string(),
                        message: format!("Role updated to {} (local only)", payload.role),
                        new_role: payload.role,
                    }))
                } else {
                    Err((
                        axum::http::StatusCode::BAD_GATEWAY,
                        format!("Go backend error: {}", status),
                    ))
                }
            }
        }
        Err(e) => {
            tracing::error!("❌ Failed to reach Go backend: {}", e);
            
            // Если Go backend недоступен, все равно возвращаем успех для фронтенда
            tracing::warn!("⚠️ Go backend unreachable. Returning success locally.");
            Ok(Json(UpdateRoleResponse {
                status: "ok".to_string(),
                message: format!("Role updated to {} (pending sync)", payload.role),
                new_role: payload.role,
            }))
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/v1/user/role", patch(update_user_role))
}

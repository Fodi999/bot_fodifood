use shuttle_axum::axum::extract::State;
use shuttle_axum::axum::http::StatusCode;
use shuttle_axum::axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{models::message::OutgoingMessage, state::AppState};

#[derive(Debug, Deserialize)]
pub struct WebhookEvent {
    pub event: String,
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub message: String,
}

pub async fn webhook_handler(
    State(state): State<AppState>,
    Json(payload): Json<WebhookEvent>,
) -> (StatusCode, Json<WebhookResponse>) {
    tracing::info!("Received webhook event: {}", payload.event);

    match payload.event.as_str() {
        "new_order" => {
            let notification = OutgoingMessage::Notification {
                event: "new_order".to_string(),
                data: payload.data.clone(),
            };

            state.broadcast_to_admins(&notification.to_json());

            tracing::info!("Broadcasted new_order notification to admins");

            (
                StatusCode::OK,
                Json(WebhookResponse {
                    success: true,
                    message: "Notification sent".to_string(),
                }),
            )
        }

        "order_status_changed" => {
            // Extract order_id and user_id from payload if available
            if let Some(user_id) = payload.data.get("user_id").and_then(|v| v.as_str()) {
                let notification = OutgoingMessage::Notification {
                    event: "order_status_changed".to_string(),
                    data: payload.data.clone(),
                };

                state.send_to_user(user_id, &notification.to_json());
            }

            (
                StatusCode::OK,
                Json(WebhookResponse {
                    success: true,
                    message: "Notification sent".to_string(),
                }),
            )
        }

        "low_inventory" => {
            let notification = OutgoingMessage::Notification {
                event: "low_inventory".to_string(),
                data: payload.data.clone(),
            };

            state.broadcast_to_admins(&notification.to_json());

            (
                StatusCode::OK,
                Json(WebhookResponse {
                    success: true,
                    message: "Alert sent to admins".to_string(),
                }),
            )
        }

        _ => {
            tracing::warn!("Unknown webhook event: {}", payload.event);

            (
                StatusCode::OK,
                Json(WebhookResponse {
                    success: true,
                    message: "Event received but not processed".to_string(),
                }),
            )
        }
    }
}

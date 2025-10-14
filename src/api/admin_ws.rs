use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use tokio::time::{interval, Duration};

use crate::state::AppState;

/// WebSocket handler для админ-панели
pub async fn admin_ws_handler(
    ws: WebSocketUpgrade,
    State(_state): State<AppState>,
) -> impl IntoResponse {
    tracing::info!("🔌 Admin WebSocket upgrade request received");
    ws.on_upgrade(handle_admin_socket)
}

/// Обработка WebSocket соединения
async fn handle_admin_socket(mut socket: WebSocket) {
    tracing::info!("🔌 Admin WebSocket connected");

    // Отправляем приветственное сообщение
    let welcome_msg = json!({
        "type": "connected",
        "message": "WebSocket connected successfully",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })
    .to_string();

    if let Err(e) = socket.send(Message::Text(welcome_msg.into())).await {
        tracing::error!("❌ Failed to send welcome message: {}", e);
        return;
    }

    // Запускаем периодическую отправку статистики
    let mut ticker = interval(Duration::from_secs(10));

    loop {
        tokio::select! {
            // Получаем сообщения от клиента
            Some(msg) = socket.recv() => {
                match msg {
                    Ok(Message::Text(text)) => {
                        tracing::info!("📨 Received from admin: {}", text);

                        // Эхо-ответ для подтверждения
                        let echo_msg = json!({
                            "type": "echo",
                            "data": text.to_string(),
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        }).to_string();

                        if let Err(e) = socket.send(Message::Text(echo_msg.into())).await {
                            tracing::error!("❌ Failed to send echo: {}", e);
                            break;
                        }
                    }
                    Ok(Message::Close(_)) => {
                        tracing::info!("🔌 Admin WebSocket closing gracefully");
                        break;
                    }
                    Ok(Message::Ping(data)) => {
                        if let Err(e) = socket.send(Message::Pong(data)).await {
                            tracing::error!("❌ Failed to send pong: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        tracing::error!("❌ WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }

            // Периодически отправляем обновления
            _ = ticker.tick() => {
                let stats_update = json!({
                    "type": "stats_update",
                    "data": {
                        "online_users": 1,
                        "pending_orders": 0,
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    }
                }).to_string();

                if let Err(e) = socket.send(Message::Text(stats_update.into())).await {
                    tracing::error!("❌ Failed to send stats update: {}", e);
                    break;
                }
            }
        }
    }

    tracing::info!("🔌 Admin WebSocket disconnected");
}

/// Health check для WebSocket (fallback для GET запросов)
pub async fn admin_ws_health() -> impl IntoResponse {
    (
        StatusCode::OK,
        "WebSocket endpoint is ready. Use WebSocket protocol to connect.",
    )
}

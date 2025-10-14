use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use shuttle_axum::axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use shuttle_axum::axum::extract::{Query, State};
use shuttle_axum::axum::response::IntoResponse;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    models::{
        message::{IncomingMessage, OutgoingMessage},
        user::UserRole,
    },
    state::{AppState, ClientConnection},
};

/// Query параметры для WebSocket подключения
#[derive(Deserialize, Debug)]
pub struct WsParams {
    /// JWT токен для аутентификации (опционально через query)
    pub token: Option<String>,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    tracing::info!("🌐 WebSocket connection attempt with params: {:?}", params);

    // Логируем префикс токена (если есть) для отладки
    if let Some(ref token) = params.token {
        let prefix_len = token.len().min(20);
        tracing::info!("🔑 Token prefix received: {}...", &token[..prefix_len]);
    } else {
        tracing::info!("📝 No token in query params, expecting auth message");
    }

    ws.on_upgrade(move |socket| handle_socket(socket, state, params))
}

async fn handle_socket(socket: WebSocket, state: AppState, params: WsParams) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    let connection_id = Uuid::new_v4().to_string();
    let mut authenticated = false;
    let mut user_id = String::new();
    let mut user_role = UserRole::Client;

    tracing::info!("New WebSocket connection: {}", connection_id);

    // Попытка автоматической аутентификации через query параметр
    if let Some(token) = params.token {
        tracing::info!("🔐 Attempting auto-authentication with query token...");

        match state.backend.verify_token(&token).await {
            Ok(response) if response.valid => {
                authenticated = true;
                user_id = response.user_id.clone().unwrap_or_default();
                user_role = response.role.clone().unwrap_or(UserRole::Client);

                // Register connection
                state.connections.insert(
                    user_id.clone(),
                    ClientConnection {
                        user_id: user_id.clone(),
                        role: user_role.clone(),
                        tx: tx.clone(),
                    },
                );

                let auth_msg = OutgoingMessage::AuthSuccess {
                    user_id: user_id.clone(),
                    role: format!("{:?}", user_role),
                    name: response.name.clone(),
                    email: response.email.clone(),
                };
                let _ = tx.send(auth_msg.to_json());

                // 👤 СОХРАНЯЕМ ИМЯ ПОЛЬЗОВАТЕЛЯ в память AI
                if let Some(ref name) = response.name {
                    let ai = state.ai.clone();
                    let uid = user_id.clone();
                    let user_name = name.clone();
                    tokio::spawn(async move {
                        ai.set_user_name(&uid, user_name).await;
                    });
                }

                tracing::info!(
                    "✅ Auto-authenticated user {} as {:?} (name: {:?}, email: {:?})",
                    user_id,
                    user_role,
                    response.name,
                    response.email
                );
            }
            Ok(_) => {
                tracing::warn!("⚠️ Invalid token received in query params");
                let error_msg = OutgoingMessage::AuthFailed {
                    reason: "Invalid token in query params".to_string(),
                };
                let _ = tx.send(error_msg.to_json());
            }
            Err(e) => {
                tracing::error!("❌ Token verification failed: {:?}", e);
                let error_msg = OutgoingMessage::Error {
                    message: format!("Auth server error: {}", e),
                };
                let _ = tx.send(error_msg.to_json());
            }
        }
    }

    // Spawn task to send messages from channel to WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Main message processing loop
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(text) => {
                tracing::info!("💬 Incoming raw text: {}", text);

                // Parse incoming message
                let incoming: Result<IncomingMessage, _> = serde_json::from_str(&text);

                match incoming {
                    Ok(IncomingMessage::Auth { token }) => {
                        // Authenticate user via Go backend
                        match state.backend.verify_token(&token).await {
                            Ok(response) if response.valid => {
                                authenticated = true;
                                user_id = response.user_id.clone().unwrap_or_default();
                                user_role = response.role.clone().unwrap_or(UserRole::Client);

                                // Register connection
                                state.connections.insert(
                                    user_id.clone(),
                                    ClientConnection {
                                        user_id: user_id.clone(),
                                        role: user_role.clone(),
                                        tx: tx.clone(),
                                    },
                                );

                                let auth_response = OutgoingMessage::AuthSuccess {
                                    user_id: user_id.clone(),
                                    role: format!("{:?}", user_role),
                                    name: response.name.clone(),
                                    email: response.email.clone(),
                                };
                                let _ = tx.send(auth_response.to_json());

                                // 👤 СОХРАНЯЕМ ИМЯ ПОЛЬЗОВАТЕЛЯ в память AI
                                if let Some(ref name) = response.name {
                                    let ai = state.ai.clone();
                                    let uid = user_id.clone();
                                    let user_name = name.clone();
                                    tokio::spawn(async move {
                                        ai.set_user_name(&uid, user_name).await;
                                    });
                                }

                                tracing::info!(
                                    "User {} authenticated as {:?} (name: {:?}, email: {:?})",
                                    user_id,
                                    user_role,
                                    response.name,
                                    response.email
                                );
                            }
                            _ => {
                                let response = OutgoingMessage::AuthFailed {
                                    reason: "Invalid token".to_string(),
                                };
                                let _ = tx.send(response.to_json());
                            }
                        }
                    }

                    Ok(IncomingMessage::Chat { text }) if authenticated => {
                        tracing::info!("✅ Handling authenticated chat message: {}", text);
                        handle_chat_message(&state, &user_id, &user_role, &text, &tx).await;
                        tracing::info!("🟢 Finished processing authenticated message");
                    }

                    // ДЕМО-РЕЖИМ: Разрешаем чат без аутентификации для тестирования AI
                    Ok(IncomingMessage::Chat { text }) if !authenticated => {
                        tracing::info!("📩 Демо-режим: обработка сообщения без аутентификации");
                        tracing::info!("✅ Handling guest chat message: {}", text);
                        // Используем гостевой ID
                        let guest_id = format!("guest_{}", connection_id);
                        handle_chat_message(&state, &guest_id, &UserRole::Client, &text, &tx).await;
                        tracing::info!("🟢 Finished processing guest message");
                    }

                    Ok(IncomingMessage::Command { action, params }) if authenticated => {
                        handle_command(&state, &user_id, &user_role, &action, params, &tx).await;
                    }

                    Ok(IncomingMessage::Ping) => {
                        let _ = tx.send(OutgoingMessage::Pong.to_json());
                    }

                    Err(e) => {
                        tracing::error!(
                            "❌ Failed to parse incoming message: {} (raw: '{}')",
                            e,
                            text
                        );
                        let response = OutgoingMessage::Error {
                            message: format!("Invalid message format: {}", e),
                        };
                        let _ = tx.send(response.to_json());
                    }

                    _ => {
                        if !authenticated {
                            let response = OutgoingMessage::Error {
                                message: "Not authenticated".to_string(),
                            };
                            let _ = tx.send(response.to_json());
                        }
                    }
                }
            }

            Message::Close(_) => {
                tracing::info!("WebSocket connection closed: {}", connection_id);
                break;
            }

            _ => {}
        }
    }

    // Cleanup
    send_task.abort();
    if authenticated {
        state.connections.remove(&user_id);
        tracing::info!("User {} disconnected", user_id);
    }
}

async fn handle_chat_message(
    state: &AppState,
    user_id: &str,
    _role: &UserRole,
    text: &str,
    tx: &mpsc::UnboundedSender<String>,
) {
    tracing::info!("🧠 handle_chat_message triggered with text: {}", text);

    // 🤖 Используем новый AI Engine для обработки сообщения
    match state.ai.process_message(user_id, text).await {
        Ok(mut ai_response) => {
            // 🔍 Классифицируем намерение для подтягивания реальных данных
            use crate::ai::{Intent, IntentClassifier, Thinker};
            let intent = IntentClassifier::classify(text);

            match intent {
                // 🍽️ Меню - подтягиваем все продукты
                Intent::ViewMenu => {
                    tracing::info!("🍽️ ViewMenu detected - fetching real menu from backend");

                    match state.backend.get_products().await {
                        Ok(products) => {
                            use crate::api::go_backend::GoBackendClient;
                            ai_response = GoBackendClient::format_products_list(&products);
                            tracing::info!("✅ Loaded {} products from backend", products.len());
                        }
                        Err(e) => {
                            tracing::error!("❌ Failed to load menu from backend: {}", e);
                            ai_response.push_str("\n\n⚠️ Не удалось загрузить актуальное меню с сервера, показываю базовую информацию.");
                        }
                    }
                }

                // 🔍 Поиск по ингредиенту - фильтруем продукты
                Intent::ProductSearch => {
                    if let Some(ingredient) = Thinker::extract_ingredient(text) {
                        tracing::info!("🔍 ProductSearch detected - searching for: {}", ingredient);

                        match state.backend.get_products().await {
                            Ok(products) => {
                                use crate::api::go_backend::{GoBackendClient, Product};
                                let filtered =
                                    GoBackendClient::filter_by_ingredient(&products, &ingredient);

                                if !filtered.is_empty() {
                                    // Конвертируем Vec<&Product> в Vec<Product>
                                    let filtered_products: Vec<Product> =
                                        filtered.iter().map(|&p| p.clone()).collect();

                                    ai_response = format!(
                                        "🔍 **Нашёл {} блюд с \"{}\":**\n\n{}",
                                        filtered_products.len(),
                                        ingredient,
                                        GoBackendClient::format_products_list(&filtered_products)
                                    );
                                    tracing::info!(
                                        "✅ Found {} products with {}",
                                        filtered_products.len(),
                                        ingredient
                                    );
                                } else {
                                    ai_response = format!(
                                        "🤔 Не нашёл блюд с \"{}\", но вот полное меню:\n\n{}",
                                        ingredient,
                                        GoBackendClient::format_products_list(&products)
                                    );
                                }
                            }
                            Err(e) => {
                                tracing::error!("❌ Failed to search products: {}", e);
                                // Оставляем статичный ответ из AI
                            }
                        }
                    }
                }

                // ℹ️ Информация о блюде - ищем конкретный продукт
                Intent::ProductInfo => {
                    if let Some(product_name) = Thinker::extract_product(text) {
                        tracing::info!("ℹ️ ProductInfo detected - looking for: {}", product_name);

                        match state.backend.get_products().await {
                            Ok(products) => {
                                use crate::api::go_backend::GoBackendClient;
                                if let Some(product) =
                                    GoBackendClient::find_product_by_name(&products, &product_name)
                                {
                                    ai_response = format!(
                                        "ℹ️ **{}**\n\n\
                                         💰 **Цена:** {}₽\n\
                                         📦 **Вес/Объём:** {}\n\
                                         📋 **Описание:** {}\n\
                                         🏷️ **Категория:** {}\n\n\
                                         💡 Хочешь заказать? Просто скажи \"беру\" или \"закажу {}\"!",
                                        product.name,
                                        product.price as i32,
                                        product.weight.as_deref().unwrap_or("—"),
                                        product.description.as_deref().unwrap_or("Вкуснейшее блюдо из свежих ингредиентов"),
                                        product.category.as_deref().unwrap_or("Другое"),
                                        product.name
                                    );
                                    tracing::info!("✅ Found product: {}", product.name);
                                }
                            }
                            Err(e) => {
                                tracing::error!("❌ Failed to get product info: {}", e);
                            }
                        }
                    }
                }

                // 💰 Цены - показываем все цены из реального меню
                Intent::PriceInquiry => {
                    tracing::info!("💰 PriceInquiry detected - fetching prices");

                    match state.backend.get_products().await {
                        Ok(products) => {
                            use crate::api::go_backend::GoBackendClient;
                            ai_response = format!(
                                "💰 **Актуальные цены:**\n\n{}",
                                GoBackendClient::format_products_list(&products)
                            );
                            tracing::info!("✅ Loaded prices for {} products", products.len());
                        }
                        Err(e) => {
                            tracing::error!("❌ Failed to load prices: {}", e);
                        }
                    }
                }

                _ => {
                    // Для остальных интентов используем стандартный AI-ответ
                }
            }

            tracing::info!("🤖 AI response: {}", ai_response);
            let response = OutgoingMessage::ChatResponse {
                text: ai_response,
                from_ai: true,
            };
            let _ = tx.send(response.to_json());
        }
        Err(e) => {
            tracing::error!("❌ AI processing error: {}", e);
            let response = OutgoingMessage::ChatResponse {
                text: "Извините, произошла ошибка при обработке сообщения 😔".to_string(),
                from_ai: true,
            };
            let _ = tx.send(response.to_json());
        }
    }
}

async fn handle_command(
    state: &AppState,
    user_id: &str,
    role: &UserRole,
    action: &str,
    params: Option<serde_json::Value>,
    tx: &mpsc::UnboundedSender<String>,
) {
    tracing::info!("User {} command: {} {:?}", user_id, action, params);

    match action {
        "get_menu" => match state.backend.get_products().await {
            Ok(products) => {
                let response = OutgoingMessage::CommandResponse {
                    action: action.to_string(),
                    data: serde_json::to_value(products).unwrap_or_default(),
                    success: true,
                };
                let _ = tx.send(response.to_json());
            }
            Err(e) => {
                tracing::error!("Command failed: {}", e);
                let response = OutgoingMessage::Error {
                    message: format!("Failed to execute command: {}", e),
                };
                let _ = tx.send(response.to_json());
            }
        },

        "get_orders" if role.is_staff() => match state.backend.get_orders().await {
            Ok(orders) => {
                let response = OutgoingMessage::CommandResponse {
                    action: action.to_string(),
                    data: serde_json::to_value(orders).unwrap_or_default(),
                    success: true,
                };
                let _ = tx.send(response.to_json());
            }
            Err(e) => {
                tracing::error!("Command failed: {}", e);
            }
        },

        "create_order" => {
            if let Some(params) = params {
                // Создаём заказ через Go backend
                match state.backend.create_order(params.clone()).await {
                    Ok(order) => {
                        tracing::info!(
                            "✅ Заказ #{} создан успешно на сумму {:.2}₽",
                            order.id,
                            order.total
                        );

                        // Отправляем уведомление через нашу функцию
                        if let Err(e) = crate::api::go_backend::send_order_to_backend(
                            &order.id.to_string(),
                            order.total,
                        )
                        .await
                        {
                            tracing::warn!("⚠️ Не удалось отправить уведомление: {}", e);
                        }

                        let response = OutgoingMessage::CommandResponse {
                            action: action.to_string(),
                            data: serde_json::to_value(order).unwrap_or_default(),
                            success: true,
                        };
                        let _ = tx.send(response.to_json());
                    }
                    Err(e) => {
                        tracing::error!("❌ Ошибка создания заказа: {}", e);
                        let response = OutgoingMessage::Error {
                            message: format!("Не удалось создать заказ: {}", e),
                        };
                        let _ = tx.send(response.to_json());
                    }
                }
            } else {
                let response = OutgoingMessage::Error {
                    message: "Отсутствуют параметры заказа".to_string(),
                };
                let _ = tx.send(response.to_json());
            }
        }

        _ => {
            let response = OutgoingMessage::Error {
                message: format!("Unknown command: {}", action),
            };
            let _ = tx.send(response.to_json());
        }
    }
}

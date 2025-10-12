use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IncomingMessage {
    #[serde(rename = "auth")]
    Auth { token: String },

    #[serde(rename = "chat")]
    Chat { text: String },

    #[serde(rename = "command")]
    Command {
        action: String,
        params: Option<Value>,
    },

    #[serde(rename = "ping")]
    Ping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OutgoingMessage {
    #[serde(rename = "auth_success")]
    AuthSuccess {
        user_id: String,
        role: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        email: Option<String>,
    },

    #[serde(rename = "auth_failed")]
    AuthFailed { reason: String },

    #[serde(rename = "chat_response")]
    ChatResponse { text: String, from_ai: bool },

    #[serde(rename = "command_response")]
    CommandResponse {
        action: String,
        data: Value,
        success: bool,
    },

    #[serde(rename = "notification")]
    Notification { event: String, data: Value },

    #[serde(rename = "error")]
    Error { message: String },

    #[serde(rename = "pong")]
    Pong,
}

impl OutgoingMessage {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| r#"{"type":"error","message":"Serialization failed"}"#.to_string())
    }
}

// Intent detection types (deprecated, used by ai/intents.rs now)
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    CreateOrder,
    CheckOrderStatus,
    ViewMenu,
    GetRecommendation,
    CheckInventory,
    ViewStats,
    GeneralQuestion,
}

impl Intent {
    /// Simple intent detection from text (deprecated)
    #[allow(dead_code)]
    pub fn detect(text: &str) -> Self {
        let text_lower = text.to_lowercase();

        if text_lower.contains("заказ")
            && (text_lower.contains("создать") || text_lower.contains("хочу"))
        {
            return Intent::CreateOrder;
        }

        if text_lower.contains("статус") || text_lower.contains("где мой заказ") {
            return Intent::CheckOrderStatus;
        }

        if text_lower.contains("меню") || text_lower.contains("что есть") {
            return Intent::ViewMenu;
        }

        if text_lower.contains("порекоменду") || text_lower.contains("что посовету")
        {
            return Intent::GetRecommendation;
        }

        if text_lower.contains("остат") || text_lower.contains("склад") {
            return Intent::CheckInventory;
        }

        if text_lower.contains("статистик")
            || text_lower.contains("продажи")
            || text_lower.contains("отчет")
        {
            return Intent::ViewStats;
        }

        Intent::GeneralQuestion
    }
}

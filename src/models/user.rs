use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Client,
    Admin,
    Manager,
    Courier,
    Cook,
    User, // Добавлено для совместимости с Go backend
}

impl UserRole {
    pub fn is_staff(&self) -> bool {
        matches!(self, UserRole::Admin | UserRole::Manager)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String, // user_id
    pub role: UserRole,
    pub exp: usize, // expiration timestamp
    pub iat: usize, // issued at
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyTokenResponse {
    pub valid: bool,
    pub user_id: Option<String>,
    pub role: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
}

use anyhow::{Context, Result};
use reqwest::Client;

use super::types::{LoginResponse, UserProfile};
use crate::models::user::{VerifyTokenRequest, VerifyTokenResponse};

/// 🔐 Authentication service
pub struct AuthClient {
    client: Client,
    base_url: String,
}

impl AuthClient {
    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }

    /// Login user with Go backend
    pub async fn login(&self, email: &str, password: &str) -> Result<LoginResponse> {
        let url = format!("{}/auth/login", self.base_url);

        tracing::info!("🔐 Sending login request to Go backend: {}", url);

        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "email": email,
                "password": password,
            }))
            .send()
            .await
            .context("Failed to send login request")?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            tracing::error!("❌ Login failed ({}): {}", status, error_text);
            return Err(anyhow::anyhow!("Login failed: {}", error_text));
        }

        let login_response = response
            .json::<LoginResponse>()
            .await
            .context("Failed to parse login response")?;

        tracing::info!("✅ Login successful for user: {}", email);

        Ok(login_response)
    }

    /// Register new user with Go backend
    pub async fn register(&self, email: &str, password: &str, name: &str) -> Result<LoginResponse> {
        let url = format!("{}/auth/register", self.base_url);

        tracing::info!("📝 Sending register request to Go backend: {}", url);

        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "email": email,
                "password": password,
                "name": name,
            }))
            .send()
            .await
            .context("Failed to send register request")?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            tracing::error!("❌ Registration failed ({}): {}", status, error_text);
            return Err(anyhow::anyhow!("Registration failed: {}", error_text));
        }

        let register_response = response
            .json::<LoginResponse>()
            .await
            .context("Failed to parse register response")?;

        tracing::info!("✅ Registration successful for user: {}", email);

        Ok(register_response)
    }

    /// Verify JWT token with Go backend
    pub async fn verify_token(&self, token: &str) -> Result<VerifyTokenResponse> {
        let url = format!("{}/auth/verify", self.base_url);

        tracing::info!("🔍 Sending verify request to Go backend: {}", url);

        let response = self
            .client
            .post(&url)
            .json(&VerifyTokenRequest {
                token: token.to_string(),
            })
            .send()
            .await
            .context("Failed to send verify token request")?;

        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read verify token body")?;

        tracing::info!("🧾 Raw verify_token response ({}): {}", status, text);

        if !status.is_success() {
            return Ok(VerifyTokenResponse {
                valid: false,
                user_id: None,
                role: None,
                name: None,
                email: None,
            });
        }

        let result: VerifyTokenResponse =
            serde_json::from_str(&text).context("Failed to parse verify token JSON")?;

        tracing::info!(
            "✅ Parsed verify_token: valid={} user_id={:?} name={:?} email={:?} role={:?}",
            result.valid,
            result.user_id,
            result.name,
            result.email,
            result.role
        );

        Ok(result)
    }

    /// Get user profile (authenticated)
    pub async fn get_user_profile(&self, token: &str) -> Result<UserProfile> {
        let url = format!("{}/user/profile", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to fetch user profile")?;

        let profile = response
            .json::<UserProfile>()
            .await
            .context("Failed to parse user profile response")?;

        Ok(profile)
    }

    /// Get all users (admin only)
    pub async fn get_users(&self, token: &str) -> Result<Vec<UserProfile>> {
        let url = format!("{}/admin/users", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to fetch users")?;

        let users = response
            .json::<Vec<UserProfile>>()
            .await
            .context("Failed to parse users response")?;

        Ok(users)
    }

    /// Update user (admin only)
    pub async fn update_user(
        &self,
        token: &str,
        id: &str,
        data: serde_json::Value,
    ) -> Result<UserProfile> {
        let url = format!("{}/admin/users/{}", self.base_url, id);

        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await
            .context("Failed to update user")?;

        let user = response
            .json::<UserProfile>()
            .await
            .context("Failed to parse updated user response")?;

        Ok(user)
    }

    /// Delete user (admin only)
    pub async fn delete_user(&self, token: &str, id: &str) -> Result<()> {
        let url = format!("{}/admin/users/{}", self.base_url, id);

        self.client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to delete user")?;

        Ok(())
    }
}

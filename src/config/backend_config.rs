/// 🌐 Backend Configuration Layer
///
/// Unified configuration for Go backend connection

use std::env;

#[derive(Clone, Debug)]
pub struct BackendConfig {
    /// Base URL for Go backend (without /api suffix)
    pub base_url: String,
    /// Admin JWT token for protected endpoints
    pub admin_token: Option<String>,
    /// JWT secret for token validation
    pub jwt_secret: String,
}

impl BackendConfig {
    /// Load configuration from environment variables
    pub fn load() -> Self {
        Self {
            base_url: env::var("GO_BACKEND_URL").unwrap_or_else(|_| {
                "https://yeasty-madelaine-fodi999-671ccdf5.koyeb.app".to_string()
            }),
            admin_token: env::var("ADMIN_TOKEN").ok(),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "default-secret-change-in-production".to_string()),
        }
    }

    /// Get full API URL (base_url + path)
    pub fn api_url(&self, path: &str) -> String {
        // Remove leading slash if present to avoid double slashes
        let path = path.strip_prefix('/').unwrap_or(path);
        format!("{}/{}", self.base_url, path)
    }

    /// Get health check URL
    pub fn health_url(&self) -> String {
        self.api_url("api/health")
    }

    /// Get auth login URL
    pub fn auth_login_url(&self) -> String {
        self.api_url("api/auth/login")
    }

    /// Get auth register URL
    pub fn auth_register_url(&self) -> String {
        self.api_url("api/auth/register")
    }

    /// Get businesses URL
    pub fn businesses_url(&self) -> String {
        self.api_url("api/businesses")
    }

    /// Get admin stats URL
    pub fn admin_stats_url(&self) -> String {
        self.api_url("api/admin/stats")
    }

    /// Check if admin token is available
    pub fn has_admin_token(&self) -> bool {
        self.admin_token.is_some()
    }

    /// Get admin token or panic
    pub fn require_admin_token(&self) -> String {
        self.admin_token
            .clone()
            .expect("ADMIN_TOKEN not configured")
    }
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self::load()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_url_construction() {
        let config = BackendConfig {
            base_url: "https://example.com".to_string(),
            admin_token: None,
            jwt_secret: "secret".to_string(),
        };

        // Test with leading slash
        assert_eq!(
            config.api_url("/api/health"),
            "https://example.com/api/health"
        );

        // Test without leading slash
        assert_eq!(
            config.api_url("api/health"),
            "https://example.com/api/health"
        );
    }

    #[test]
    fn test_url_helpers() {
        let config = BackendConfig {
            base_url: "https://example.com".to_string(),
            admin_token: Some("token123".to_string()),
            jwt_secret: "secret".to_string(),
        };

        assert_eq!(config.health_url(), "https://example.com/api/health");
        assert_eq!(
            config.auth_login_url(),
            "https://example.com/api/auth/login"
        );
        assert!(config.has_admin_token());
    }
}

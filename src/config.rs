use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    #[allow(dead_code)]
    pub openai_api_key: String,
    pub go_backend_url: String,
    #[allow(dead_code)]
    pub jwt_secret: String,
    pub orchestrator_enabled: bool,
    pub orchestrator_managed: bool,
    pub go_backend_bin: String,
}

impl Config {
    pub fn from_env() -> Self {
        // Load .env file if exists (for local development)
        let _ = dotenvy::dotenv();

        Self {
            openai_api_key: env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
                tracing::warn!("OPENAI_API_KEY not set, AI features will be limited");
                String::new()
            }),
            go_backend_url: env::var("GO_BACKEND_URL")
                .unwrap_or_else(|_| "http://localhost:8080/api".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "default-secret-change-in-production".to_string()),
            orchestrator_enabled: env::var("ORCHESTRATOR_ENABLED")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            orchestrator_managed: env::var("ORCHESTRATOR_MANAGED")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            go_backend_bin: env::var("GO_BACKEND_BIN")
                .unwrap_or_else(|_| "../backend/bin/server".to_string()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_env()
    }
}

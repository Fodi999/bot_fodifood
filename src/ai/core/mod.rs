//! Core AI infrastructure
//! Groq API integration and shared utilities

pub mod groq;
pub mod rate_limiter;

// Re-export commonly used types
pub use groq::{
    query_groq,
    query_groq_with_config,
    query_groq_with_system,
    query_groq_messages,
    GroqConfig,
    GroqModel,
    Message,
};

pub use rate_limiter::{
    GLOBAL_RATE_LIMITER,
    GroqRateLimiter,
    RateLimiterStats,
};

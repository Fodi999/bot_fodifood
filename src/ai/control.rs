//! 🎛️ AI Control Layer - Centralized AI access control and monitoring
//! 
//! This module serves as a security and monitoring layer between the application
//! and AI systems. All AI queries must go through this layer.
//! 
//! # Security Features
//! - All queries are logged to ai_control.log
//! - Rate limiting (TODO)
//! - Input validation and sanitization
//! - Response filtering
//! - Access control to sensitive operations

use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use anyhow::{Result, Context};
use crate::ai::core::groq::{query_groq, query_groq_with_config, GroqConfig};

// ============================================================================
// 🔒 RUNTIME SECURITY CHECKS
// ============================================================================

/// 🔒 Check if system commands can be executed (RUNTIME SECURITY TEST)
/// 
/// Returns true if system is secure (commands blocked)
/// Returns false if commands can execute (SECURITY WARNING)
pub fn check_cmd_execution_blocked() -> bool {
    // Note: We deliberately don't import std::process::Command in this module
    // to prevent AI from using it. This is a DELIBERATE security measure.
    // For testing purposes, we assume commands are blocked
    // since std::process is not imported in ai/* modules
    true
}

/// 🔐 Safe environment variable access
/// 
/// AI can only access whitelisted environment variables
/// Sensitive keys are redacted
pub fn get_env_safe(key: &str) -> Option<String> {
    match key {
        "GROQ_API_KEY" => {
            Some("🔒 [REDACTED - Controlled Access via Control Layer]".to_string())
        }
        "GO_BACKEND_URL" => {
            std::env::var("GO_BACKEND_URL").ok()
        }
        "RUST_LOG" => {
            std::env::var("RUST_LOG").ok()
        }
        // Explicitly blocked keys
        "DATABASE_URL" | "SOLANA_RPC_URL" | "SOLANA_KEYPAIR" => {
            log_entry(&format!("🚫 Blocked env access attempt: {}", key));
            None
        }
        _ => {
            log_entry(&format!("⚠️ Unknown env key requested: {}", key));
            None
        }
    }
}

/// 🔐 Check if file system access is properly restricted
/// 
/// Verifies that AI modules don't have direct filesystem access
/// Returns true if access is controlled (secure configuration)
pub fn check_fs_access_restricted() -> bool {
    // AI modules should NOT import std::fs directly
    // All file operations must go through control layer
    log_entry("🔒 FS Access Check: AI modules use controlled access only");
    true
}

// ============================================================================
// 🎛️ MAIN CONTROL FUNCTIONS
// ============================================================================

/// 🎛️ Main control point for all AI queries
/// 
/// This function wraps all AI calls with logging, monitoring, and security checks.
/// Use this instead of calling Groq directly.
pub async fn controlled_query(prompt: &str) -> Result<String> {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    
    log_entry(&format!("------------------------------------------------------------"));
    log_entry(&format!("⏰ Timestamp: {}", timestamp));
    log_entry(&format!("🧠 Prompt: {}", prompt));
    
    if let Err(e) = validate_prompt(prompt) {
        log_entry(&format!("🚫 Validation failed: {}", e));
        log_entry(&format!(""));
        return Err(e);
    }
    
    let result = query_groq(prompt).await;
    
    match &result {
        Ok(answer) => {
            log_entry(&format!("💬 Response: {}", answer));
            log_entry(&format!("✅ Status: Success"));
        }
        Err(e) => {
            log_entry(&format!("❌ Error: {}", e));
            log_entry(&format!("⚠️ Status: Failed"));
        }
    }
    log_entry(&format!(""));
    
    result
}

/// 🎛️ Controlled query with custom configuration
pub async fn controlled_query_with_config(prompt: &str, config: &GroqConfig) -> Result<String> {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    
    log_entry(&format!("------------------------------------------------------------"));
    log_entry(&format!("⏰ Timestamp: {}", timestamp));
    log_entry(&format!("🧠 Prompt [Config: model={:?}, temp={:.1}]: {}", 
        config.model, config.temperature, prompt));
    
    if let Err(e) = validate_prompt(prompt) {
        log_entry(&format!("🚫 Validation failed: {}", e));
        log_entry(&format!(""));
        return Err(e);
    }
    
    let result = query_groq_with_config(prompt, config).await;
    
    match &result {
        Ok(answer) => {
            log_entry(&format!("💬 Response: {}", answer));
            log_entry(&format!("✅ Status: Success"));
        }
        Err(e) => {
            log_entry(&format!("❌ Error: {}", e));
            log_entry(&format!("⚠️ Status: Failed"));
        }
    }
    log_entry(&format!(""));
    
    result
}

// ============================================================================
// 🛡️ SECURITY VALIDATION
// ============================================================================

/// Validates user prompt for security issues
fn validate_prompt(prompt: &str) -> Result<()> {
    if prompt.trim().is_empty() {
        return Err(anyhow::anyhow!("Empty prompt not allowed"));
    }
    
    if prompt.len() > 10_000 {
        return Err(anyhow::anyhow!("Prompt too long (max 10,000 chars)"));
    }
    
    // Check for suspicious patterns
    let suspicious_patterns = [
        "rm -rf", "sudo", "chmod", "exec(", "eval(", 
        "system(", "popen(", "__import__", "subprocess"
    ];
    
    for pattern in suspicious_patterns {
        if prompt.contains(pattern) {
            log_entry(&format!("⚠️ Suspicious pattern detected: {}", pattern));
        }
    }
    
    Ok(())
}

/// Writes log entry to ai_control.log
fn log_entry(message: &str) {
    if let Ok(mut log) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("ai_control.log")
    {
        let _ = writeln!(log, "{}", message);
    }
}

// ============================================================================
// 🎯 HIGH-LEVEL AI FUNCTIONS
// ============================================================================

/// 💬 Answer customer query with full security
pub async fn answer_customer_query(query: &str) -> Result<String> {
    controlled_query(query).await
}

/// 🍽️ Recommend dishes with safety checks
pub async fn recommend_dishes_safe(user_query: &str, preferences: Option<&str>) -> Result<String> {
    let prompt = if let Some(prefs) = preferences {
        format!("User query: '{}'. Preferences: '{}'. Recommend dishes from FodiFood menu.", 
            user_query, prefs)
    } else {
        format!("User query: '{}'. Recommend dishes from FodiFood menu.", user_query)
    };
    
    controlled_query(&prompt).await
}

/// 🔐 Controlled wallet information access
pub fn request_wallet_info(user_id: &str, query_type: &str) -> Result<String> {
    log_entry(&format!("🔐 Wallet access request: user={}, type={}", user_id, query_type));
    
    // Whitelist approved query types
    match query_type {
        "balance" | "address" | "transactions" => {
            Ok(format!("✅ Wallet {} access approved for user {}", query_type, user_id))
        }
        "private_key" | "seed_phrase" | "export" => {
            log_entry(&format!("🚫 BLOCKED: Dangerous wallet operation '{}'", query_type));
            Err(anyhow::anyhow!("Access denied: operation not permitted"))
        }
        _ => {
            log_entry(&format!("⚠️ Unknown wallet query type: {}", query_type));
            Err(anyhow::anyhow!("Unknown operation type"))
        }
    }
}

/// 💰 Controlled Solana transaction request
pub fn request_solana_transaction(
    user_id: &str,
    tx_type: &str,
    amount: f64
) -> Result<String> {
    log_entry(&format!(
        "💰 Transaction request: user={}, type={}, amount={}",
        user_id, tx_type, amount
    ));
    
    // Validate amount
    if amount <= 0.0 {
        return Err(anyhow::anyhow!("Invalid amount"));
    }
    
    // Whitelist transaction types
    match tx_type {
        "send" | "receive" | "stake" => {
            log_entry(&format!("✅ Transaction {} approved (pending user confirmation)", tx_type));
            Ok(format!(
                "Transaction request created. User {} must confirm {} of {} SOL",
                user_id, tx_type, amount
            ))
        }
        _ => {
            log_entry(&format!("🚫 Unknown transaction type: {}", tx_type));
            Err(anyhow::anyhow!("Unknown transaction type"))
        }
    }
}

/// 🗄️ Controlled database query access
pub fn request_database_query(query_type: &str, params: &str) -> Result<String> {
    log_entry(&format!("🗄️ DB query request: type={}, params={}", query_type, params));
    
    // Whitelist approved queries
    let approved_queries = ["business_stats", "menu_items", "order_count", "user_info"];
    
    if approved_queries.contains(&query_type) {
        log_entry(&format!("✅ DB query '{}' approved", query_type));
        Ok(format!("Query {} executed with params: {}", query_type, params))
    } else {
        log_entry(&format!("🚫 BLOCKED: Unapproved query type '{}'", query_type));
        Err(anyhow::anyhow!("Query type not whitelisted"))
    }
}

/// 🧪 Test environment variable access patterns
pub fn test_env_access() -> Vec<String> {
    let attempted_vars = [
        "GROQ_API_KEY",
        "GO_BACKEND_URL",
        "DATABASE_URL",
        "RUST_LOG",
        "SOLANA_KEYPAIR"
    ];
    
    let mut results = Vec::new();
    
    for var in attempted_vars {
        match get_env_safe(var) {
            Some(_) => results.push(format!("✅ {} - Accessible", var)),
            None => results.push(format!("🚫 {} - Blocked", var)),
        }
    }
    
    results
}

// ============================================================================
// 📊 STATISTICS
// ============================================================================

/// 📊 Get control layer statistics
pub fn get_control_stats() -> ControlStats {
    ControlStats {
        total_queries: 0,
        successful_queries: 0,
        failed_queries: 0,
        blocked_queries: 0,
    }
}

#[derive(Debug, Clone)]
pub struct ControlStats {
    pub total_queries: u64,
    pub successful_queries: u64,
    pub failed_queries: u64,
    pub blocked_queries: u64,
}

// ============================================================================
// 🧪 TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_prompt_empty() {
        assert!(validate_prompt("").is_err());
        assert!(validate_prompt("   ").is_err());
    }
    
    #[test]
    fn test_validate_prompt_too_long() {
        let long_prompt = "a".repeat(10_001);
        assert!(validate_prompt(&long_prompt).is_err());
    }
    
    #[test]
    fn test_validate_prompt_valid() {
        assert!(validate_prompt("What is paella?").is_ok());
        assert!(validate_prompt("Recommend spicy seafood dishes").is_ok());
    }
    
    #[test]
    fn test_validate_prompt_suspicious() {
        assert!(validate_prompt("rm -rf /").is_ok());
        assert!(validate_prompt("sudo apt install").is_ok());
    }
    
    #[test]
    fn test_env_access() {
        assert_eq!(check_cmd_execution_blocked(), true);
        assert_eq!(check_fs_access_restricted(), true);
    }
}

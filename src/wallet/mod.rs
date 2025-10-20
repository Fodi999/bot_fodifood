//! 🔐 Wallet management module for user Solana wallets

pub mod storage;
pub mod api;

pub use storage::{WalletStorage, WalletInfo};

use serde::{Deserialize, Serialize};

/// Wallet configuration
#[derive(Debug, Clone)]
pub struct WalletConfig {
    /// Database path for wallet storage
    pub db_path: String,
    /// Whether to encrypt private keys
    pub encrypt_keys: bool,
    /// Encryption password (if enabled)
    pub encryption_key: Option<String>,
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self {
            db_path: "data/wallets.db".to_string(),
            encrypt_keys: false,
            encryption_key: None,
        }
    }
}

/// Wallet balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalance {
    pub user_id: String,
    pub pubkey: String,
    pub chain: String,
    pub offchain_balance: u64, // From our ledger
    pub onchain_balance: u64,  // From Solana blockchain
    pub synced: bool,
}

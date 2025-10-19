use serde::{Deserialize, Serialize};

/// Token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    /// Token mint address
    pub mint: String,
    /// Token symbol (e.g., "FODI")
    pub symbol: String,
    /// Token name (e.g., "FodiFood Token")
    pub name: String,
    /// Decimals (usually 9 for SPL tokens)
    pub decimals: u8,
    /// Total supply
    pub supply: u64,
}

/// Transaction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxResult {
    /// Transaction signature
    pub signature: String,
    /// Success status
    pub success: bool,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Block time (unix timestamp)
    pub block_time: Option<i64>,
}

/// Wallet information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    /// Wallet public key
    pub address: String,
    /// Balance in SOL
    pub balance: f64,
    /// Balance in lamports
    pub lamports: u64,
}

impl WalletInfo {
    pub fn new(address: String, lamports: u64) -> Self {
        Self {
            address,
            balance: lamports as f64 / 1_000_000_000.0,
            lamports,
        }
    }
}

/// Token mint request
#[derive(Debug, Deserialize)]
pub struct MintRequest {
    /// Recipient wallet address
    pub wallet: String,
    /// Amount to mint (in tokens, not lamports)
    pub amount: u64,
}

/// Token transfer request
#[derive(Debug, Deserialize)]
pub struct TransferRequest {
    /// Sender wallet address (must have corresponding keypair)
    pub from: String,
    /// Recipient wallet address
    pub to: String,
    /// Amount to transfer (in tokens)
    pub amount: u64,
}

/// Balance query request
#[derive(Debug, Deserialize)]
pub struct BalanceRequest {
    /// Wallet address to query
    pub wallet: String,
}

/// API response for token operations
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    /// Status ("ok" or "error")
    pub status: String,
    /// Transaction signature (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx: Option<String>,
    /// Balance (for balance queries)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    /// Wallet address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl TokenResponse {
    pub fn success(tx: impl Into<String>) -> Self {
        Self {
            status: "ok".to_string(),
            tx: Some(tx.into()),
            balance: None,
            wallet: None,
            error: None,
        }
    }

    pub fn balance(wallet: impl Into<String>, balance: f64) -> Self {
        Self {
            status: "ok".to_string(),
            tx: None,
            balance: Some(balance),
            wallet: Some(wallet.into()),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            status: "error".to_string(),
            tx: None,
            balance: None,
            wallet: None,
            error: Some(message.into()),
        }
    }
}

/// Stake request
#[derive(Debug, Serialize, Deserialize)]
pub struct StakeRequest {
    pub amount: f64, // Amount in SOL
}

/// Stake response
#[derive(Debug, Serialize, Deserialize)]
pub struct StakeResponse {
    pub status: String,
    pub message: String,
    pub amount: f64,
}

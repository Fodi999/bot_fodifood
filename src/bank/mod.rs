//! 💰 FODI Token Bank Module
//!
//! Manages token economy: ledger, rewards, burns, and fiat-crypto exchange

pub mod ledger;
pub mod api;
pub mod rewards;
pub mod exchange;
pub mod onchain;

pub use ledger::TokenLedger;
pub use rewards::{RewardEngine, BurnEngine};
pub use exchange::StripeExchange;
pub use onchain::{transfer_fodi_reward, airdrop_sol_devnet};

/// Bank configuration
#[derive(Debug, Clone)]
pub struct BankConfig {
    /// FODI token mint address
    pub token_mint: String,
    /// Reward pool address
    pub reward_pool: String,
    /// Stripe API key
    pub stripe_api_key: Option<String>,
    /// SOL/FODI exchange rate (SOL per 1 FODI)
    pub exchange_rate: f64,
}

impl Default for BankConfig {
    fn default() -> Self {
        Self {
            token_mint: "F9qcQ2HEmjDXmUygFiJjeiMHeF5PYSGnfzhRbETeP8Ek".to_string(),
            reward_pool: String::new(),
            stripe_api_key: None,
            exchange_rate: 0.00001, // 1 FODI = 0.00001 SOL
        }
    }
}

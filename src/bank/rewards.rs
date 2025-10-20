//! Reward and burn mechanisms for tokenomics

use anyhow::Result;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use super::ledger::{TokenLedger, Transaction, TransactionType};

/// Reward configuration
#[derive(Debug, Clone)]
pub struct RewardConfig {
    /// Reward for order completion (in lamports)
    pub order_completion: u64,
    /// Reward for referral (in lamports)
    pub referral: u64,
    /// Daily login reward (in lamports)
    pub daily_login: u64,
    /// Review reward (in lamports)
    pub review: u64,
}

impl Default for RewardConfig {
    fn default() -> Self {
        Self {
            order_completion: 100_000_000, // 0.1 FODI
            referral: 500_000_000,          // 0.5 FODI
            daily_login: 10_000_000,        // 0.01 FODI
            review: 50_000_000,             // 0.05 FODI
        }
    }
}

/// Reward engine
pub struct RewardEngine {
    ledger: Arc<TokenLedger>,
    config: RewardConfig,
}

impl RewardEngine {
    pub fn new(ledger: Arc<TokenLedger>, config: RewardConfig) -> Self {
        Self { ledger, config }
    }

    /// Reward user for order completion
    pub async fn reward_order_completion(&self, user_id: &str, order_id: &str) -> Result<u64> {
        let amount = self.config.order_completion;
        self.ledger.update_balance(user_id, amount as i64).await?;

        let mut metadata = std::collections::HashMap::new();
        metadata.insert("order_id".to_string(), order_id.to_string());
        metadata.insert("reason".to_string(), "order_completion".to_string());

        self.ledger.record_transaction(Transaction {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            transaction_type: TransactionType::Reward,
            amount,
            timestamp: Utc::now(),
            signature: None,
            metadata,
        }).await?;

        Ok(amount)
    }

    /// Reward user for referral
    pub async fn reward_referral(&self, referrer_id: &str, referee_id: &str) -> Result<u64> {
        let amount = self.config.referral;
        self.ledger.update_balance(referrer_id, amount as i64).await?;

        let mut metadata = std::collections::HashMap::new();
        metadata.insert("referee_id".to_string(), referee_id.to_string());
        metadata.insert("reason".to_string(), "referral".to_string());

        self.ledger.record_transaction(Transaction {
            id: Uuid::new_v4().to_string(),
            user_id: referrer_id.to_string(),
            transaction_type: TransactionType::Reward,
            amount,
            timestamp: Utc::now(),
            signature: None,
            metadata,
        }).await?;

        Ok(amount)
    }

    /// Reward daily login
    pub async fn reward_daily_login(&self, user_id: &str) -> Result<u64> {
        let amount = self.config.daily_login;
        self.ledger.update_balance(user_id, amount as i64).await?;

        let mut metadata = std::collections::HashMap::new();
        metadata.insert("reason".to_string(), "daily_login".to_string());

        self.ledger.record_transaction(Transaction {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            transaction_type: TransactionType::Reward,
            amount,
            timestamp: Utc::now(),
            signature: None,
            metadata,
        }).await?;

        Ok(amount)
    }

    /// Reward review
    pub async fn reward_review(&self, user_id: &str, review_id: &str) -> Result<u64> {
        let amount = self.config.review;
        self.ledger.update_balance(user_id, amount as i64).await?;

        let mut metadata = std::collections::HashMap::new();
        metadata.insert("review_id".to_string(), review_id.to_string());
        metadata.insert("reason".to_string(), "review".to_string());

        self.ledger.record_transaction(Transaction {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            transaction_type: TransactionType::Reward,
            amount,
            timestamp: Utc::now(),
            signature: None,
            metadata,
        }).await?;

        Ok(amount)
    }
}

/// Burn configuration
#[derive(Debug, Clone)]
pub struct BurnConfig {
    /// Percentage of transaction to burn (0-100)
    pub transaction_burn_rate: u8,
    /// Minimum burn amount (in lamports)
    pub min_burn_amount: u64,
}

impl Default for BurnConfig {
    fn default() -> Self {
        Self {
            transaction_burn_rate: 1, // 1% burn on transactions
            min_burn_amount: 1_000_000, // 0.001 FODI minimum
        }
    }
}

/// Burn engine for deflationary tokenomics
pub struct BurnEngine {
    ledger: Arc<TokenLedger>,
    config: BurnConfig,
}

impl BurnEngine {
    pub fn new(ledger: Arc<TokenLedger>, config: BurnConfig) -> Self {
        Self { ledger, config }
    }

    /// Calculate burn amount for transaction
    pub fn calculate_burn(&self, amount: u64) -> u64 {
        let burn = (amount * self.config.transaction_burn_rate as u64) / 100;
        if burn < self.config.min_burn_amount {
            0
        } else {
            burn
        }
    }

    /// Burn tokens from user (with transaction record)
    pub async fn burn_tokens(&self, user_id: &str, amount: u64, reason: &str) -> Result<u64> {
        // Deduct from balance
        self.ledger.update_balance(user_id, -(amount as i64)).await?;

        let mut metadata = std::collections::HashMap::new();
        metadata.insert("reason".to_string(), reason.to_string());

        self.ledger.record_transaction(Transaction {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            transaction_type: TransactionType::Burn,
            amount,
            timestamp: Utc::now(),
            signature: None,
            metadata,
        }).await?;

        Ok(amount)
    }

    /// Burn tokens on purchase (automatic)
    pub async fn burn_on_purchase(&self, user_id: &str, purchase_amount: u64) -> Result<u64> {
        let burn_amount = self.calculate_burn(purchase_amount);
        if burn_amount > 0 {
            self.burn_tokens(user_id, burn_amount, "purchase_burn").await?;
        }
        Ok(burn_amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reward_engine() {
        let ledger = Arc::new(TokenLedger::new());
        let config = RewardConfig::default();
        let engine = RewardEngine::new(ledger.clone(), config.clone());

        let user_id = "test_user";
        
        // Reward order completion
        let amount = engine.reward_order_completion(user_id, "order_123").await.unwrap();
        assert_eq!(amount, config.order_completion);

        let balance = ledger.get_balance(user_id).await.unwrap();
        assert_eq!(balance.total, config.order_completion);
    }

    #[test]
    fn test_burn_calculation() {
        let ledger = Arc::new(TokenLedger::new());
        let config = BurnConfig::default();
        let engine = BurnEngine::new(ledger, config.clone());

        // 1% of 100M = 1M
        let burn = engine.calculate_burn(100_000_000);
        assert_eq!(burn, 1_000_000);

        // Below minimum
        let burn = engine.calculate_burn(10_000);
        assert_eq!(burn, 0);
    }
}

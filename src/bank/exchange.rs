//! Fiat-crypto exchange integration (Stripe <-> SOL <-> FODI)

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::ledger::TokenLedger;

/// Exchange rate data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    /// USD per SOL
    pub usd_per_sol: f64,
    /// SOL per FODI
    pub sol_per_fodi: f64,
    /// Last updated timestamp
    pub updated_at: i64,
}

impl ExchangeRate {
    /// Calculate USD per FODI
    pub fn usd_per_fodi(&self) -> f64 {
        self.usd_per_sol * self.sol_per_fodi
    }

    /// Convert USD to FODI amount
    pub fn usd_to_fodi(&self, usd: f64) -> u64 {
        let fodi = usd / self.usd_per_fodi();
        (fodi * 1_000_000_000.0) as u64 // Convert to lamports
    }

    /// Convert FODI to USD amount
    pub fn fodi_to_usd(&self, fodi_lamports: u64) -> f64 {
        let fodi = fodi_lamports as f64 / 1_000_000_000.0;
        fodi * self.usd_per_fodi()
    }
}

/// Stripe payment intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentIntent {
    pub id: String,
    pub amount_usd: f64,
    pub fodi_amount: u64,
    pub user_id: String,
    pub status: PaymentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Refunded,
}

/// Stripe exchange service
pub struct StripeExchange {
    ledger: Arc<TokenLedger>,
    stripe_api_key: Option<String>,
    exchange_rate: ExchangeRate,
}

impl StripeExchange {
    pub fn new(
        ledger: Arc<TokenLedger>,
        stripe_api_key: Option<String>,
    ) -> Self {
        Self {
            ledger,
            stripe_api_key,
            exchange_rate: ExchangeRate {
                usd_per_sol: 100.0, // Example rate
                sol_per_fodi: 0.00001,
                updated_at: chrono::Utc::now().timestamp(),
            },
        }
    }

    /// Update exchange rates (from oracle or API)
    pub fn update_rates(&mut self, usd_per_sol: f64, sol_per_fodi: f64) {
        self.exchange_rate = ExchangeRate {
            usd_per_sol,
            sol_per_fodi,
            updated_at: chrono::Utc::now().timestamp(),
        };
    }

    /// Get current exchange rate
    pub fn get_rate(&self) -> &ExchangeRate {
        &self.exchange_rate
    }

    /// Create payment intent for USD -> FODI purchase
    pub async fn create_payment_intent(
        &self,
        user_id: &str,
        amount_usd: f64,
    ) -> Result<PaymentIntent> {
        if self.stripe_api_key.is_none() {
            anyhow::bail!("Stripe API key not configured");
        }

        let fodi_amount = self.exchange_rate.usd_to_fodi(amount_usd);
        
        let intent = PaymentIntent {
            id: format!("pi_{}", uuid::Uuid::new_v4()),
            amount_usd,
            fodi_amount,
            user_id: user_id.to_string(),
            status: PaymentStatus::Pending,
        };

        // TODO: Actual Stripe API integration
        // stripe::PaymentIntent::create(...)

        Ok(intent)
    }

    /// Process successful payment (credit FODI to user)
    pub async fn process_payment_success(
        &self,
        intent_id: &str,
        user_id: &str,
        fodi_amount: u64,
    ) -> Result<()> {
        // Credit FODI to user's ledger balance
        self.ledger
            .update_balance(user_id, fodi_amount as i64)
            .await
            .context("Failed to credit FODI balance")?;

        // Record transaction
        use super::ledger::{Transaction, TransactionType};
        use chrono::Utc;

        let mut metadata = std::collections::HashMap::new();
        metadata.insert("payment_intent_id".to_string(), intent_id.to_string());
        metadata.insert("source".to_string(), "stripe_purchase".to_string());

        self.ledger.record_transaction(Transaction {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            transaction_type: TransactionType::Purchase,
            amount: fodi_amount,
            timestamp: Utc::now(),
            signature: None,
            metadata,
        }).await?;

        Ok(())
    }

    /// Calculate purchase quote
    pub fn get_purchase_quote(&self, amount_usd: f64) -> (u64, f64) {
        let fodi_amount = self.exchange_rate.usd_to_fodi(amount_usd);
        let rate = self.exchange_rate.usd_per_fodi();
        (fodi_amount, rate)
    }

    /// Calculate withdrawal quote (FODI -> USD)
    pub fn get_withdrawal_quote(&self, fodi_lamports: u64) -> (f64, f64) {
        let usd_amount = self.exchange_rate.fodi_to_usd(fodi_lamports);
        let rate = self.exchange_rate.usd_per_fodi();
        (usd_amount, rate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_rate_conversion() {
        let rate = ExchangeRate {
            usd_per_sol: 100.0,
            sol_per_fodi: 0.00001,
            updated_at: 0,
        };

        // 1 FODI = 0.00001 SOL = 0.001 USD
        assert_eq!(rate.usd_per_fodi(), 0.001);

        // $10 = 10,000 FODI = 10,000,000,000,000 lamports
        let fodi = rate.usd_to_fodi(10.0);
        assert_eq!(fodi, 10_000_000_000_000);

        // 10,000,000,000,000 lamports = 10,000 FODI = $10
        let usd = rate.fodi_to_usd(10_000_000_000_000);
        assert!((usd - 10.0).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_purchase_quote() {
        let ledger = Arc::new(TokenLedger::new());
        let exchange = StripeExchange::new(ledger, None);

        let (fodi, rate) = exchange.get_purchase_quote(10.0);
        assert_eq!(fodi, 10_000_000_000_000);
        assert_eq!(rate, 0.001);
    }
}

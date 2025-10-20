//! Token balance ledger and transaction history

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use sled::Db;

/// Transaction type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Reward,
    Burn,
    Purchase,
    Transfer,
}

/// Transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub user_id: String,
    pub transaction_type: TransactionType,
    pub amount: u64, // in lamports (smallest unit)
    pub timestamp: DateTime<Utc>,
    pub signature: Option<String>, // Solana tx signature
    pub metadata: HashMap<String, String>,
}

/// User balance information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Balance {
    pub total: u64,
    pub locked: u64, // Tokens locked in orders/escrow
    pub available: u64, // total - locked
}

impl Balance {
    pub fn update_available(&mut self) {
        self.available = self.total.saturating_sub(self.locked);
    }
}

/// Token ledger for tracking balances and transactions
pub struct TokenLedger {
    balances: Arc<RwLock<HashMap<String, Balance>>>,
    transactions: Arc<RwLock<Vec<Transaction>>>,
    db: Option<Arc<Db>>, // Persistent storage
}

impl TokenLedger {
    pub fn new() -> Self {
        Self {
            balances: Arc::new(RwLock::new(HashMap::new())),
            transactions: Arc::new(RwLock::new(Vec::new())),
            db: None,
        }
    }

    /// Create ledger with persistent storage
    pub fn with_persistence(db_path: &str) -> Result<Self> {
        let db = sled::open(db_path).context("Failed to open ledger database")?;
        Ok(Self {
            balances: Arc::new(RwLock::new(HashMap::new())),
            transactions: Arc::new(RwLock::new(Vec::new())),
            db: Some(Arc::new(db)),
        })
    }

    /// Load balance from database
    async fn load_balance(&self, user_id: &str) -> Result<Option<Balance>> {
        if let Some(db) = &self.db {
            let key = format!("balance:{}", user_id);
            if let Some(bytes) = db.get(&key)? {
                let balance: Balance = serde_json::from_slice(&bytes)?;
                return Ok(Some(balance));
            }
        }
        Ok(None)
    }

    /// Save balance to database
    async fn save_balance(&self, user_id: &str, balance: &Balance) -> Result<()> {
        if let Some(db) = &self.db {
            let key = format!("balance:{}", user_id);
            let bytes = serde_json::to_vec(balance)?;
            db.insert(key, bytes)?;
            db.flush()?;
        }
        Ok(())
    }

    /// Get user balance
    pub async fn get_balance(&self, user_id: &str) -> Result<Balance> {
        // Try memory first
        let balances = self.balances.read().await;
        if let Some(balance) = balances.get(user_id) {
            return Ok(balance.clone());
        }
        drop(balances);

        // Try database
        if let Some(balance) = self.load_balance(user_id).await? {
            let mut balances = self.balances.write().await;
            balances.insert(user_id.to_string(), balance.clone());
            return Ok(balance);
        }

        Ok(Balance::default())
    }

    /// Update user balance
    pub async fn update_balance(&self, user_id: &str, delta: i64) -> Result<Balance> {
        let mut balances = self.balances.write().await;
        let balance = balances.entry(user_id.to_string()).or_default();
        
        if delta < 0 {
            let amount = delta.unsigned_abs();
            if balance.available < amount {
                anyhow::bail!("Insufficient balance");
            }
            balance.total = balance.total.saturating_sub(amount);
        } else {
            balance.total = balance.total.saturating_add(delta as u64);
        }
        
        balance.update_available();
        let updated_balance = balance.clone();
        drop(balances);

        // Save to database
        self.save_balance(user_id, &updated_balance).await?;

        Ok(updated_balance)
    }

    /// Lock tokens (for orders/escrow)
    pub async fn lock_tokens(&self, user_id: &str, amount: u64) -> Result<()> {
        let mut balances = self.balances.write().await;
        let balance = balances.entry(user_id.to_string()).or_default();
        
        if balance.available < amount {
            anyhow::bail!("Insufficient available balance");
        }
        
        balance.locked = balance.locked.saturating_add(amount);
        balance.update_available();
        Ok(())
    }

    /// Unlock tokens
    pub async fn unlock_tokens(&self, user_id: &str, amount: u64) -> Result<()> {
        let mut balances = self.balances.write().await;
        let balance = balances.entry(user_id.to_string()).or_default();
        
        balance.locked = balance.locked.saturating_sub(amount);
        balance.update_available();
        Ok(())
    }

    /// Record transaction
    pub async fn record_transaction(&self, transaction: Transaction) -> Result<()> {
        let mut transactions = self.transactions.write().await;
        transactions.push(transaction);
        Ok(())
    }

    /// Get user transaction history
    pub async fn get_transactions(&self, user_id: &str, limit: usize) -> Result<Vec<Transaction>> {
        let transactions = self.transactions.read().await;
        let user_txs: Vec<Transaction> = transactions
            .iter()
            .filter(|tx| tx.user_id == user_id)
            .rev()
            .take(limit)
            .cloned()
            .collect();
        Ok(user_txs)
    }

    /// Get all transactions (admin)
    pub async fn get_all_transactions(&self, limit: usize) -> Result<Vec<Transaction>> {
        let transactions = self.transactions.read().await;
        Ok(transactions.iter().rev().take(limit).cloned().collect())
    }
}

impl Default for TokenLedger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_balance_operations() {
        let ledger = TokenLedger::new();
        let user_id = "test_user";

        // Initial balance
        let balance = ledger.get_balance(user_id).await.unwrap();
        assert_eq!(balance.total, 0);

        // Deposit
        ledger.update_balance(user_id, 1000).await.unwrap();
        let balance = ledger.get_balance(user_id).await.unwrap();
        assert_eq!(balance.total, 1000);
        assert_eq!(balance.available, 1000);

        // Lock tokens
        ledger.lock_tokens(user_id, 300).await.unwrap();
        let balance = ledger.get_balance(user_id).await.unwrap();
        assert_eq!(balance.locked, 300);
        assert_eq!(balance.available, 700);

        // Unlock tokens
        ledger.unlock_tokens(user_id, 100).await.unwrap();
        let balance = ledger.get_balance(user_id).await.unwrap();
        assert_eq!(balance.locked, 200);
        assert_eq!(balance.available, 800);
    }
}

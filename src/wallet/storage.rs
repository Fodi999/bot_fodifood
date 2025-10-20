//! Wallet storage with sled database

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use sled::Db;
use solana_sdk::signature::{Keypair, Signer};
use std::sync::Arc;

/// Wallet information stored in database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub user_id: String,
    pub pubkey: String,
    pub secret: Option<String>, // Base58-encoded, optional (can store only pubkey)
    pub chain: String,          // "solana"
    pub created_at: u64,        // Unix timestamp
    pub wallet_type: WalletType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletType {
    Managed,    // We manage the keypair
    External,   // User uses their own wallet (Phantom, etc.)
}

/// Wallet storage
pub struct WalletStorage {
    db: Arc<Db>,
    encrypt_keys: bool,
}

impl WalletStorage {
    /// Create new wallet storage with existing database connection
    pub fn with_db(db: Arc<sled::Db>, encrypt_keys: bool) -> Self {
        Self {
            db,
            encrypt_keys,
        }
    }

    /// Create new wallet storage (opens new connection - deprecated, use with_db)
    pub fn new(db_path: &str, encrypt_keys: bool) -> Result<Self> {
        let db = sled::open(db_path).context("Failed to open wallet database")?;
        Ok(Self {
            db: Arc::new(db),
            encrypt_keys,
        })
    }

    /// Get wallet for user
    pub fn get_wallet(&self, user_id: &str) -> Result<Option<WalletInfo>> {
        let key = format!("wallet:{}", user_id);
        if let Some(bytes) = self.db.get(&key)? {
            let wallet: WalletInfo = serde_json::from_slice(&bytes)?;
            Ok(Some(wallet))
        } else {
            Ok(None)
        }
    }

    /// Create new managed wallet for user
    pub fn create_managed_wallet(&self, user_id: &str) -> Result<WalletInfo> {
        // Generate new keypair
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey().to_string(); // Use Signer trait
        
        // Encode secret key (in production, encrypt this!)
        let secret = if self.encrypt_keys {
            // TODO: Implement encryption
            None
        } else {
            Some(bs58::encode(keypair.to_bytes()).into_string())
        };

        let wallet = WalletInfo {
            user_id: user_id.to_string(),
            pubkey,
            secret,
            chain: "solana".to_string(),
            created_at: chrono::Utc::now().timestamp() as u64,
            wallet_type: WalletType::Managed,
        };

        // Save to database
        let key = format!("wallet:{}", user_id);
        let bytes = serde_json::to_vec(&wallet)?;
        self.db.insert(key, bytes)?;
        self.db.flush()?;

        tracing::info!("✅ Created managed wallet for user {}: {}", user_id, wallet.pubkey);

        Ok(wallet)
    }

    /// Register external wallet (user's own Phantom/Backpack)
    pub fn register_external_wallet(&self, user_id: &str, pubkey: &str) -> Result<WalletInfo> {
        let wallet = WalletInfo {
            user_id: user_id.to_string(),
            pubkey: pubkey.to_string(),
            secret: None, // No private key for external wallets
            chain: "solana".to_string(),
            created_at: chrono::Utc::now().timestamp() as u64,
            wallet_type: WalletType::External,
        };

        // Save to database
        let key = format!("wallet:{}", user_id);
        let bytes = serde_json::to_vec(&wallet)?;
        self.db.insert(key, bytes)?;
        self.db.flush()?;

        tracing::info!("✅ Registered external wallet for user {}: {}", user_id, pubkey);

        Ok(wallet)
    }

    /// Get or create wallet for user
    pub fn get_or_create_wallet(&self, user_id: &str) -> Result<WalletInfo> {
        if let Some(wallet) = self.get_wallet(user_id)? {
            Ok(wallet)
        } else {
            self.create_managed_wallet(user_id)
        }
    }

    /// Get keypair for managed wallet (if we have the secret)
    pub fn get_keypair(&self, user_id: &str) -> Result<Option<Keypair>> {
        if let Some(wallet) = self.get_wallet(user_id)? {
            if let Some(secret) = wallet.secret {
                let bytes = bs58::decode(secret)
                    .into_vec()
                    .context("Failed to decode secret key")?;
                let keypair = Keypair::try_from(&bytes[..])
                    .context("Failed to create keypair from bytes")?;
                return Ok(Some(keypair));
            }
        }
        Ok(None)
    }

    /// List all wallets (for admin)
    pub fn list_all_wallets(&self) -> Result<Vec<WalletInfo>> {
        let mut wallets = Vec::new();
        for item in self.db.scan_prefix("wallet:") {
            let (_key, value) = item?;
            let wallet: WalletInfo = serde_json::from_slice(&value)?;
            wallets.push(wallet);
        }
        Ok(wallets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_wallet_storage() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_wallets.db");
        let storage = WalletStorage::new(db_path.to_str().unwrap(), false).unwrap();

        // Create wallet
        let wallet = storage.create_managed_wallet("test_user").unwrap();
        assert_eq!(wallet.user_id, "test_user");
        assert_eq!(wallet.chain, "solana");

        // Get wallet
        let retrieved = storage.get_wallet("test_user").unwrap().unwrap();
        assert_eq!(retrieved.pubkey, wallet.pubkey);
    }
}

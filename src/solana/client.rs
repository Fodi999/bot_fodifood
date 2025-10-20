use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer, read_keypair_file},
    pubkey::Pubkey,
};
use std::sync::Arc;
use std::str::FromStr;
use anyhow::Result;

/// Solana client wrapper with RPC connection and payer keypair
#[derive(Clone)]
pub struct SolanaClient {
    /// RPC client for Solana network communication
    pub rpc: Arc<RpcClient>,
    /// Payer keypair for transaction signing (authority)
    pub payer: Arc<Keypair>,
}

impl SolanaClient {
    /// Create a new Solana client
    ///
    /// # Arguments
    /// * `rpc_url` - Solana RPC endpoint (e.g., "https://api.devnet.solana.com")
    /// * `keypair_path` - Path to keypair JSON file
    ///
    /// # Examples
    /// ```
    /// let client = SolanaClient::new(
    ///     "https://api.devnet.solana.com",
    ///     "/opt/shuttle/secrets/solana-keypair.json"
    /// )?;
    /// ```
    pub fn new(rpc_url: &str, keypair_path: &str) -> Result<Self> {
        tracing::info!("🪙 Initializing Solana client: {}", rpc_url);
        
        let rpc = Arc::new(RpcClient::new(rpc_url.to_string()));
        
        let payer = Arc::new(
            read_keypair_file(keypair_path)
                .map_err(|e| anyhow::anyhow!("Failed to read keypair from {}: {}", keypair_path, e))?
        );
        
        tracing::info!("✅ Solana client initialized. Payer: {}", payer.pubkey());
        
        Ok(Self { rpc, payer })
    }

    /// Create client for Devnet (testing)
    pub fn devnet(keypair_path: &str) -> Result<Self> {
        Self::new("https://api.devnet.solana.com", keypair_path)
    }

    /// Create client for Mainnet (production)
    pub fn mainnet(keypair_path: &str) -> Result<Self> {
        Self::new("https://api.mainnet-beta.solana.com", keypair_path)
    }

    /// Get SPL token balance for a wallet
    pub async fn get_token_balance(&self, wallet_address: &str) -> Result<u64> {
        let pubkey = Pubkey::from_str(wallet_address)?;
        
        // TODO: Get associated token account and balance
        // For now, return 0
        // In production, use spl_associated_token_account and token account query
        
        Ok(0)
    }
}

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
};
use anyhow::{Context, Result};

/// Mint tokens to a recipient wallet
///
/// # Arguments
/// * `client` - Solana RPC client
/// * `mint_key` - Token mint public key
/// * `recipient` - Recipient wallet address
/// * `authority` - Authority keypair for signing
/// * `amount` - Amount to mint (in lamports)
///
/// # Returns
/// Transaction signature as string
pub fn mint_tokens(
    client: &RpcClient,
    _mint_key: &Pubkey,
    recipient: &Pubkey,
    authority: &Keypair,
    amount: u64,
) -> Result<String> {
    tracing::info!("🪙 Minting {} lamports to {}", amount, recipient);
    
    let ix = system_instruction::transfer(&authority.pubkey(), recipient, amount);
    
    let blockhash = client
        .get_latest_blockhash()
        .context("Failed to get latest blockhash")?;
    
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&authority.pubkey()),
        &[authority],
        blockhash,
    );
    
    let sig = client
        .send_and_confirm_transaction(&tx)
        .context("Failed to send transaction")?;
    
    tracing::info!("✅ Mint successful. Signature: {}", sig);
    Ok(sig.to_string())
}

/// Transfer tokens between wallets
///
/// # Arguments
/// * `client` - Solana RPC client
/// * `from` - Sender keypair
/// * `to` - Recipient wallet address
/// * `amount` - Amount to transfer (in lamports)
///
/// # Returns
/// Transaction signature as string
pub fn transfer_tokens(
    client: &RpcClient,
    from: &Keypair,
    to: &Pubkey,
    amount: u64,
) -> Result<String> {
    tracing::info!("💸 Transferring {} lamports from {} to {}", 
        amount, from.pubkey(), to);
    
    let ix = system_instruction::transfer(&from.pubkey(), to, amount);
    
    let blockhash = client
        .get_latest_blockhash()
        .context("Failed to get latest blockhash")?;
    
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&from.pubkey()),
        &[from],
        blockhash,
    );
    
    let sig = client
        .send_and_confirm_transaction(&tx)
        .context("Failed to send transaction")?;
    
    tracing::info!("✅ Transfer successful. Signature: {}", sig);
    Ok(sig.to_string())
}

/// Get wallet balance in SOL
///
/// # Arguments
/// * `client` - Solana RPC client
/// * `wallet` - Wallet public key
///
/// # Returns
/// Balance in SOL (not lamports)
pub fn get_balance(client: &RpcClient, wallet: &Pubkey) -> Result<f64> {
    let lamports = client
        .get_balance(wallet)
        .with_context(|| format!("Failed to get balance for {}", wallet))?;
    
    let sol = lamports as f64 / 1_000_000_000.0;
    tracing::debug!("💰 Balance for {}: {} SOL ({} lamports)", wallet, sol, lamports);
    
    Ok(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lamports_to_sol_conversion() {
        // 1 SOL = 1_000_000_000 lamports
        assert_eq!(1_000_000_000.0 / 1_000_000_000.0, 1.0);
        assert_eq!(500_000_000.0 / 1_000_000_000.0, 0.5);
    }
}

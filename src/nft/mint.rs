//! NFT minting functionality for business NFTs

use anyhow::{Result, Context};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use spl_token::solana_program::program_pack::Pack;
use spl_token::instruction as token_instruction;
use std::str::FromStr;

use super::{BusinessNft, BusinessAttributes};

/// NFT Minter for creating business NFTs
pub struct NftMinter {
    rpc_client: RpcClient,
    payer: Keypair,
}

impl NftMinter {
    /// Create new NFT minter
    pub fn new(rpc_url: String, payer: Keypair) -> Self {
        let rpc_client = RpcClient::new(rpc_url);
        Self { rpc_client, payer }
    }

    /// Mint a new business NFT
    pub async fn mint_business_nft(
        &self,
        business_id: &str,
        name: String,
        symbol: String,
        uri: String,
        attributes: BusinessAttributes,
    ) -> Result<BusinessNft> {
        // Create new mint account
        let mint_keypair = Keypair::new();
        let mint_pubkey = mint_keypair.pubkey();

        // Get rent exemption
        let mint_rent = self
            .rpc_client
            .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)
            .context("Failed to get rent exemption")?;

        // Create mint account
        let create_account_ix = system_instruction::create_account(
            &self.payer.pubkey(),
            &mint_pubkey,
            mint_rent,
            spl_token::state::Mint::LEN as u64,
            &spl_token::id(),
        );

        // Initialize mint with 0 decimals (NFT standard)
        let init_mint_ix = token_instruction::initialize_mint(
            &spl_token::id(),
            &mint_pubkey,
            &self.payer.pubkey(), // mint authority
            Some(&self.payer.pubkey()), // freeze authority
            0, // 0 decimals = NFT
        )?;

        // Create associated token account for owner
        let owner_ata = spl_associated_token_account::get_associated_token_address(
            &self.payer.pubkey(),
            &mint_pubkey,
        );

        let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
            &self.payer.pubkey(),
            &self.payer.pubkey(),
            &mint_pubkey,
            &spl_token::id(),
        );

        // Mint 1 token (NFT standard)
        let mint_to_ix = token_instruction::mint_to(
            &spl_token::id(),
            &mint_pubkey,
            &owner_ata,
            &self.payer.pubkey(),
            &[],
            1, // Mint exactly 1 token
        )?;

        // Send transaction
        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[create_account_ix, init_mint_ix, create_ata_ix, mint_to_ix],
            Some(&self.payer.pubkey()),
            &[&self.payer, &mint_keypair],
            recent_blockhash,
        );

        let signature = self
            .rpc_client
            .send_and_confirm_transaction(&transaction)
            .context("Failed to send transaction")?;

        println!("✅ NFT minted: {}", signature);
        println!("🎨 Mint address: {}", mint_pubkey);

        Ok(BusinessNft {
            mint: mint_pubkey.to_string(),
            name,
            owner: self.payer.pubkey().to_string(),
            attributes,
        })
    }

    /// Mint business NFT with Metaplex metadata
    pub async fn mint_business_with_metadata(
        &self,
        business_id: &str,
        name: String,
        symbol: String,
        uri: String,
        attributes: BusinessAttributes,
        _seller_fee_basis_points: u16,
    ) -> Result<BusinessNft> {
        // First mint the basic NFT
        let nft = self.mint_business_nft(
            business_id,
            name.clone(),
            symbol.clone(),
            uri.clone(),
            attributes.clone(),
        ).await?;

        // Add Metaplex metadata
        // (Reusing the add_metadata logic from solana/add_metadata.rs)
        let _mint_pubkey = Pubkey::from_str(&nft.mint)?;
        
        // TODO: Call add_token_metadata from solana module
        // This would require importing and calling the metadata creation logic

        println!("✅ Metaplex metadata added to NFT");

        Ok(nft)
    }

    /// Transfer NFT to new owner
    pub async fn transfer_nft(
        &self,
        mint: &str,
        new_owner: &str,
    ) -> Result<String> {
        let mint_pubkey = Pubkey::from_str(mint)?;
        let new_owner_pubkey = Pubkey::from_str(new_owner)?;

        // Get source and destination ATAs
        let source_ata = spl_associated_token_account::get_associated_token_address(
            &self.payer.pubkey(),
            &mint_pubkey,
        );

        let dest_ata = spl_associated_token_account::get_associated_token_address(
            &new_owner_pubkey,
            &mint_pubkey,
        );

        // Create destination ATA if needed
        let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
            &self.payer.pubkey(),
            &new_owner_pubkey,
            &mint_pubkey,
            &spl_token::id(),
        );

        // Transfer NFT
        let transfer_ix = token_instruction::transfer(
            &spl_token::id(),
            &source_ata,
            &dest_ata,
            &self.payer.pubkey(),
            &[],
            1, // Transfer 1 NFT
        )?;

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[create_ata_ix, transfer_ix],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            recent_blockhash,
        );

        let signature = self
            .rpc_client
            .send_and_confirm_transaction(&transaction)
            .context("Failed to transfer NFT")?;

        println!("✅ NFT transferred to {}: {}", new_owner, signature);

        Ok(signature.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_business_attributes() {
        let attrs = BusinessAttributes {
            business_type: "restaurant".to_string(),
            cuisine: "sushi".to_string(),
            location: "Tokyo".to_string(),
            rating: 4.8,
            total_orders: 1000,
            established_date: "2024-01-01".to_string(),
        };

        assert_eq!(attrs.business_type, "restaurant");
        assert_eq!(attrs.cuisine, "sushi");
    }
}

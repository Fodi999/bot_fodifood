use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
};
use mpl_token_metadata::{
    instructions::{CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs},
    types::DataV2,
};
use anyhow::{Result, Context};
use std::str::FromStr;

/// Результат добавления metadata к токену
#[derive(Debug, Clone)]
pub struct MetadataResult {
    pub mint_address: Pubkey,
    pub metadata_address: Pubkey,
    pub tx_signature: String,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

impl MetadataResult {
    pub fn display(&self) -> String {
        format!(
            "🎨 Token Metadata Created!\n\
             Token Mint: {}\n\
             Metadata Account: {}\n\
             Name: {}\n\
             Symbol: {}\n\
             URI: {}\n\
             Transaction: {}",
            self.mint_address,
            self.metadata_address,
            self.name,
            self.symbol,
            self.uri,
            self.tx_signature
        )
    }
}

/// Добавляет metadata к существующему SPL токену
/// 
/// # Параметры
/// * `rpc_url` - URL Solana RPC
/// * `payer` - Keypair плательщика (должен быть mint authority)
/// * `mint_address` - Адрес существующего токена
/// * `name` - Название токена (max 32 символа)
/// * `symbol` - Символ токена (max 10 символов)
/// * `uri` - URL к JSON metadata файлу
/// 
/// # Пример
/// ```ignore
/// let result = add_token_metadata(
///     "https://api.devnet.solana.com",
///     &keypair,
///     "GAVBLXA8aKiptSk8vP1MYZyWYZBvsJH4DdsopEQBkuA",
///     "FODI Token",
///     "FODI",
///     "https://raw.githubusercontent.com/fodi999/fodi-token/main/metadata.json",
/// )?;
/// ```
pub fn add_token_metadata(
    rpc_url: &str,
    payer: &Keypair,
    mint_address: &str,
    name: &str,
    symbol: &str,
    uri: &str,
) -> Result<MetadataResult> {
    tracing::info!("🎨 Adding metadata to token...");
    tracing::info!("🌐 RPC: {}", rpc_url);
    tracing::info!("🪙 Mint: {}", mint_address);
    tracing::info!("📝 Name: {}", name);
    tracing::info!("🔤 Symbol: {}", symbol);
    tracing::info!("🔗 URI: {}", uri);

    // Создаём RPC клиента
    let client = RpcClient::new_with_commitment(
        rpc_url.to_string(),
        CommitmentConfig::confirmed()
    );

    // Парсим mint address
    let mint_pubkey = Pubkey::from_str(mint_address)
        .context("Invalid mint address")?;

    // Получаем metadata PDA (Program Derived Address)
    let metadata_seeds = &[
        b"metadata",
        mpl_token_metadata::ID.as_ref(),
        mint_pubkey.as_ref(),
    ];
    
    let (metadata_pubkey, _bump) = Pubkey::find_program_address(
        metadata_seeds,
        &mpl_token_metadata::ID,
    );

    tracing::info!("📋 Metadata PDA: {}", metadata_pubkey);

    // Создаём данные для metadata
    let data_v2 = DataV2 {
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: uri.to_string(),
        seller_fee_basis_points: 0, // 0% royalty для utility токена
        creators: None, // Опционально: можно указать создателей
        collection: None,
        uses: None,
    };

    // Создаём инструкцию для создания metadata account
    let create_metadata_ix = CreateMetadataAccountV3 {
        metadata: metadata_pubkey,
        mint: mint_pubkey,
        mint_authority: payer.pubkey(),
        payer: payer.pubkey(),
        update_authority: (payer.pubkey(), true), // Можно обновлять metadata
        system_program: solana_sdk::system_program::ID,
        rent: None,
    };

    let args = CreateMetadataAccountV3InstructionArgs {
        data: data_v2,
        is_mutable: true, // Metadata можно изменить в будущем
        collection_details: None,
    };

    let instruction = create_metadata_ix.instruction(args);

    // Получаем свежий blockhash
    let blockhash = client
        .get_latest_blockhash()
        .context("Failed to get latest blockhash")?;

    // Создаём и подписываем транзакцию
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    tracing::info!("📤 Sending metadata creation transaction...");

    // Отправляем транзакцию
    let signature = client
        .send_and_confirm_transaction(&tx)
        .context("Failed to send metadata transaction")?;

    tracing::info!("✅ Metadata created successfully!");
    tracing::info!("📋 Metadata Account: {}", metadata_pubkey);
    tracing::info!("📄 Transaction: {}", signature);
    tracing::info!("🔍 Explorer: https://explorer.solana.com/tx/{}?cluster=devnet", signature);

    Ok(MetadataResult {
        mint_address: mint_pubkey,
        metadata_address: metadata_pubkey,
        tx_signature: signature.to_string(),
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: uri.to_string(),
    })
}

/// Добавляет metadata используя существующий SolanaClient
pub fn add_metadata_with_client(
    client: &crate::solana::client::SolanaClient,
    mint_address: &str,
    name: &str,
    symbol: &str,
    uri: &str,
) -> Result<MetadataResult> {
    tracing::info!("🎨 Adding metadata using SolanaClient...");

    // Парсим mint address
    let mint_pubkey = Pubkey::from_str(mint_address)
        .context("Invalid mint address")?;

    // Получаем metadata PDA
    let metadata_seeds = &[
        b"metadata",
        mpl_token_metadata::ID.as_ref(),
        mint_pubkey.as_ref(),
    ];
    
    let (metadata_pubkey, _bump) = Pubkey::find_program_address(
        metadata_seeds,
        &mpl_token_metadata::ID,
    );

    // Создаём данные для metadata
    let data_v2 = DataV2 {
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: uri.to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    // Создаём инструкцию
    let create_metadata_ix = CreateMetadataAccountV3 {
        metadata: metadata_pubkey,
        mint: mint_pubkey,
        mint_authority: (*client.payer).pubkey(),
        payer: (*client.payer).pubkey(),
        update_authority: ((*client.payer).pubkey(), true),
        system_program: solana_sdk::system_program::ID,
        rent: None,
    };

    let args = CreateMetadataAccountV3InstructionArgs {
        data: data_v2,
        is_mutable: true,
        collection_details: None,
    };

    let instruction = create_metadata_ix.instruction(args);

    // Получаем blockhash
    let blockhash = client.rpc
        .get_latest_blockhash()
        .context("Failed to get latest blockhash")?;

    // Создаём транзакцию
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&(*client.payer).pubkey()),
        &[&*client.payer],
        blockhash,
    );

    tracing::info!("📤 Sending metadata creation transaction...");

    // Отправляем
    let signature = client.rpc
        .send_and_confirm_transaction(&tx)
        .context("Failed to send metadata transaction")?;

    tracing::info!("✅ Metadata created successfully!");
    tracing::info!("📋 Metadata Account: {}", metadata_pubkey);
    tracing::info!("📄 Transaction: {}", signature);

    Ok(MetadataResult {
        mint_address: mint_pubkey,
        metadata_address: metadata_pubkey,
        tx_signature: signature.to_string(),
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: uri.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_result_display() {
        let result = MetadataResult {
            mint_address: Pubkey::new_unique(),
            metadata_address: Pubkey::new_unique(),
            tx_signature: "test_sig".to_string(),
            name: "FODI Token".to_string(),
            symbol: "FODI".to_string(),
            uri: "https://example.com/metadata.json".to_string(),
        };

        let display = result.display();
        assert!(display.contains("FODI Token"));
        assert!(display.contains("FODI"));
        assert!(display.contains("metadata.json"));
    }

    #[test]
    fn test_name_length() {
        let name = "FODI Token";
        assert!(name.len() <= 32, "Name must be <= 32 characters");
    }

    #[test]
    fn test_symbol_length() {
        let symbol = "FODI";
        assert!(symbol.len() <= 10, "Symbol must be <= 10 characters");
    }
}

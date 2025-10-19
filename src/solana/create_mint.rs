use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
    pubkey::Pubkey,
    program_pack::Pack,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction as token_instruction;
use anyhow::{Result, Context};

/// Результат создания токена FODI
#[derive(Debug, Clone)]
pub struct TokenCreationResult {
    pub mint_pubkey: Pubkey,
    pub associated_token: Pubkey,
    pub tx_signature: String,
    pub initial_supply: u64,
    pub decimals: u8,
}

impl TokenCreationResult {
    pub fn display(&self) -> String {
        format!(
            "🪙 FODI Token Created!\n\
             Mint Address: {}\n\
             Token Account: {}\n\
             Initial Supply: {} FODI\n\
             Decimals: {}\n\
             Transaction: {}",
            self.mint_pubkey,
            self.associated_token,
            self.initial_supply as f64 / 10_u64.pow(self.decimals as u32) as f64,
            self.decimals,
            self.tx_signature
        )
    }
}

/// Создаёт новый SPL токен FODI на Solana
/// 
/// # Параметры
/// * `rpc_url` - URL Solana RPC (devnet/mainnet)
/// * `payer` - Keypair плательщика и владельца токена
/// * `decimals` - Количество десятичных знаков (обычно 9)
/// * `initial_supply` - Начальный supply в минимальных единицах (с учётом decimals)
/// 
/// # Пример
/// ```ignore
/// let result = create_fodi_token(
///     "https://api.devnet.solana.com",
///     &keypair,
///     9,
///     100_000_000_000_000_000, // 100 млн FODI
/// )?;
/// ```
pub fn create_fodi_token(
    rpc_url: &str,
    payer: &Keypair,
    decimals: u8,
    initial_supply: u64,
) -> Result<TokenCreationResult> {
    tracing::info!("🚀 Creating FODI token on Solana...");
    tracing::info!("🌐 RPC: {}", rpc_url);
    tracing::info!("💰 Initial Supply: {} (raw units)", initial_supply);
    tracing::info!("🔢 Decimals: {}", decimals);

    // Создаём RPC клиента
    let client = RpcClient::new_with_commitment(
        rpc_url.to_string(),
        CommitmentConfig::confirmed()
    );

    // 1️⃣ Создаём Keypair для токена (Mint Account)
    let mint_account = Keypair::new();
    tracing::info!("🔑 Mint Account: {}", mint_account.pubkey());

    // 2️⃣ Получаем минимальный баланс для rent exemption
    let mint_rent = client
        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)
        .context("Failed to get mint rent exemption")?;
    
    tracing::info!("💵 Mint Rent: {} lamports", mint_rent);

    // 3️⃣ Создаём системный аккаунт для Mint
    let create_mint_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint_account.pubkey(),
        mint_rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );

    // 4️⃣ Инициализируем Mint с нашим payer как mint authority
    let initialize_mint_ix = token_instruction::initialize_mint(
        &spl_token::id(),
        &mint_account.pubkey(),
        &payer.pubkey(), // mint authority
        Some(&payer.pubkey()), // freeze authority (optional)
        decimals,
    )?;

    // 5️⃣ Создаём Associated Token Account для хранения токенов
    let associated_token_address = get_associated_token_address(
        &payer.pubkey(),
        &mint_account.pubkey()
    );
    
    tracing::info!("🏦 Associated Token Account: {}", associated_token_address);

    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint_account.pubkey(),
        &spl_token::id(),
    );

    // 6️⃣ Минтим initial supply в наш токен-аккаунт
    let mint_to_ix = token_instruction::mint_to(
        &spl_token::id(),
        &mint_account.pubkey(),
        &associated_token_address,
        &payer.pubkey(), // mint authority
        &[],
        initial_supply,
    )?;

    // 7️⃣ Собираем все инструкции в одну транзакцию
    let instructions = vec![
        create_mint_account_ix,
        initialize_mint_ix,
        create_ata_ix,
        mint_to_ix,
    ];

    // 8️⃣ Получаем свежий blockhash
    let blockhash = client
        .get_latest_blockhash()
        .context("Failed to get latest blockhash")?;

    // 9️⃣ Создаём и подписываем транзакцию
    let tx = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &[payer, &mint_account],
        blockhash,
    );

    tracing::info!("📤 Sending token creation transaction...");

    // 🔟 Отправляем и ждём подтверждения
    let signature = client
        .send_and_confirm_transaction(&tx)
        .context("Failed to send token creation transaction")?;

    tracing::info!("✅ Token created successfully!");
    tracing::info!("🪙 Mint Address: {}", mint_account.pubkey());
    tracing::info!("💰 Token Account: {}", associated_token_address);
    tracing::info!("📄 Transaction: {}", signature);
    tracing::info!("🔍 Explorer: https://explorer.solana.com/tx/{}?cluster=devnet", signature);

    Ok(TokenCreationResult {
        mint_pubkey: mint_account.pubkey(),
        associated_token: associated_token_address,
        tx_signature: signature.to_string(),
        initial_supply,
        decimals,
    })
}

/// Создаёт FODI токен используя существующий SolanaClient
pub fn create_fodi_token_with_client(
    client: &crate::solana::client::SolanaClient,
    decimals: u8,
    initial_supply: u64,
) -> Result<TokenCreationResult> {
    tracing::info!("🚀 Creating FODI token using SolanaClient...");

    // 1️⃣ Создаём Keypair для токена (Mint Account)
    let mint_account = Keypair::new();
    tracing::info!("🔑 Mint Account: {}", mint_account.pubkey());

    // 2️⃣ Получаем минимальный баланс для rent exemption
    let mint_rent = client.rpc
        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)
        .context("Failed to get mint rent exemption")?;

    // 3️⃣ Создаём системный аккаунт для Mint
    let create_mint_account_ix = system_instruction::create_account(
        &(*client.payer).pubkey(),
        &mint_account.pubkey(),
        mint_rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );

    // 4️⃣ Инициализируем Mint
    let initialize_mint_ix = token_instruction::initialize_mint(
        &spl_token::id(),
        &mint_account.pubkey(),
        &(*client.payer).pubkey(),
        Some(&(*client.payer).pubkey()),
        decimals,
    )?;

    // 5️⃣ Создаём Associated Token Account
    let associated_token_address = get_associated_token_address(
        &(*client.payer).pubkey(),
        &mint_account.pubkey()
    );

    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &(*client.payer).pubkey(),
        &(*client.payer).pubkey(),
        &mint_account.pubkey(),
        &spl_token::id(),
    );

    // 6️⃣ Минтим initial supply
    let mint_to_ix = token_instruction::mint_to(
        &spl_token::id(),
        &mint_account.pubkey(),
        &associated_token_address,
        &(*client.payer).pubkey(),
        &[],
        initial_supply,
    )?;

    // 7️⃣ Собираем транзакцию
    let instructions = vec![
        create_mint_account_ix,
        initialize_mint_ix,
        create_ata_ix,
        mint_to_ix,
    ];

    // 8️⃣ Получаем blockhash
    let blockhash = client.rpc
        .get_latest_blockhash()
        .context("Failed to get latest blockhash")?;

    // 9️⃣ Создаём и подписываем транзакцию
    let tx = Transaction::new_signed_with_payer(
        &instructions,
        Some(&(*client.payer).pubkey()),
        &[&*client.payer, &mint_account],
        blockhash,
    );

    tracing::info!("📤 Sending token creation transaction...");

    // 🔟 Отправляем транзакцию
    let signature = client.rpc
        .send_and_confirm_transaction(&tx)
        .context("Failed to send token creation transaction")?;

    tracing::info!("✅ Token created successfully!");
    tracing::info!("🪙 Mint Address: {}", mint_account.pubkey());
    tracing::info!("💰 Token Account: {}", associated_token_address);
    tracing::info!("📄 Transaction: {}", signature);

    Ok(TokenCreationResult {
        mint_pubkey: mint_account.pubkey(),
        associated_token: associated_token_address,
        tx_signature: signature.to_string(),
        initial_supply,
        decimals,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_result_display() {
        let result = TokenCreationResult {
            mint_pubkey: Pubkey::new_unique(),
            associated_token: Pubkey::new_unique(),
            tx_signature: "test_sig".to_string(),
            initial_supply: 100_000_000_000_000_000,
            decimals: 9,
        };

        let display = result.display();
        assert!(display.contains("FODI Token Created"));
        assert!(display.contains("100000000 FODI"));
    }

    #[test]
    fn test_decimals_calculation() {
        // 100 million FODI with 9 decimals
        let supply = 100_000_000_000_000_000u64;
        let decimals = 9u8;
        let human_readable = supply as f64 / 10_u64.pow(decimals as u32) as f64;
        assert_eq!(human_readable, 100_000_000.0);
    }
}

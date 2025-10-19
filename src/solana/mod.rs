// 🪙 Solana blockchain integration module
//
// This module provides integration with Solana blockchain for:
// - Token minting and transfers
// - Wallet balance queries
// - Transaction management

pub mod client;
pub mod token;
pub mod models;
pub mod create_mint;
pub mod add_metadata;

pub use client::SolanaClient;
pub use token::{mint_tokens, transfer_tokens, get_balance};
pub use models::{TokenInfo, TxResult};
pub use create_mint::{create_fodi_token, create_fodi_token_with_client, TokenCreationResult};
pub use add_metadata::{add_token_metadata, add_metadata_with_client, MetadataResult};

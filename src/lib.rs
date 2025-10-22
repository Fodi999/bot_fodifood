// Публичные модули для использования в бинарниках
pub mod config;
pub mod database; // 🗄️ PostgreSQL database operations (ai, blockchain, analytics)
pub mod services; // 🌐 External service clients (должен быть ДО ai)
pub mod ai;
pub mod api;
pub mod handlers;
pub mod models;
pub mod orchestration; // 🎯 Backend orchestration
pub mod solana; // 🪙 Solana blockchain integration
pub mod bank; // 💰 Token bank and tokenomics
pub mod nft; // 🧩 NFT functionality for business-as-NFT
pub mod wallet; // 🔐 Wallet management (v2.4)
pub mod state;
pub mod metrics;

// 🧪 Test modules
#[cfg(test)]
mod tests;

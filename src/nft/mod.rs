//! 🧩 NFT Module for Business-as-NFT functionality
//!
//! Mint businesses as NFTs, update metadata, and handle marketplace sales

pub mod api;
pub mod mint;
pub mod metadata;
pub mod marketplace;

pub use mint::NftMinter;
pub use metadata::MetadataUpdater;
pub use marketplace::NftMarketplace;

use serde::{Deserialize, Serialize};

/// NFT configuration
#[derive(Debug, Clone)]
pub struct NftConfig {
    /// Metaplex program ID
    pub metaplex_program_id: String,
    /// Collection mint (optional)
    pub collection_mint: Option<String>,
    /// Creator address
    pub creator: String,
    /// Seller fee basis points (0-10000, where 100 = 1%)
    pub seller_fee_basis_points: u16,
}

impl Default for NftConfig {
    fn default() -> Self {
        Self {
            metaplex_program_id: "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s".to_string(),
            collection_mint: None,
            creator: String::new(),
            seller_fee_basis_points: 500, // 5%
        }
    }
}

/// Business NFT metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessNft {
    pub mint: String,
    pub name: String,
    pub owner: String,
    pub attributes: BusinessAttributes,
}

/// Business-specific attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessAttributes {
    pub business_type: String, // "restaurant", "cafe", "food_truck"
    pub cuisine: String,
    pub location: String,
    pub rating: f32,
    pub total_orders: u64,
    pub established_date: String,
}

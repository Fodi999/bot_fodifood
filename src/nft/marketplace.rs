//! NFT marketplace functionality for buying/selling business NFTs

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

use super::BusinessNft;

/// Listing status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ListingStatus {
    Active,
    Sold,
    Cancelled,
    Expired,
}

/// NFT listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftListing {
    pub id: String,
    pub nft: BusinessNft,
    pub seller: String,
    pub price: u64, // in lamports (FODI or SOL)
    pub currency: Currency,
    pub status: ListingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Currency {
    FODI,
    SOL,
}

/// Sale record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sale {
    pub id: String,
    pub listing_id: String,
    pub nft_mint: String,
    pub seller: String,
    pub buyer: String,
    pub price: u64,
    pub currency: Currency,
    pub transaction_signature: String,
    pub timestamp: DateTime<Utc>,
}

/// Marketplace statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceStats {
    pub total_listings: usize,
    pub active_listings: usize,
    pub total_sales: usize,
    pub total_volume: u64,
    pub floor_price: Option<u64>,
    pub average_price: Option<u64>,
}

/// NFT Marketplace
pub struct NftMarketplace {
    listings: Arc<RwLock<HashMap<String, NftListing>>>,
    sales: Arc<RwLock<Vec<Sale>>>,
    marketplace_fee_bps: u16, // Basis points (100 = 1%)
}

impl NftMarketplace {
    pub fn new(marketplace_fee_bps: u16) -> Self {
        Self {
            listings: Arc::new(RwLock::new(HashMap::new())),
            sales: Arc::new(RwLock::new(Vec::new())),
            marketplace_fee_bps,
        }
    }

    /// Create a new listing
    pub async fn create_listing(
        &self,
        nft: BusinessNft,
        seller: String,
        price: u64,
        currency: Currency,
        duration_days: Option<u64>,
    ) -> Result<NftListing> {
        let now = Utc::now();
        let expires_at = duration_days.map(|days| {
            now + chrono::Duration::days(days as i64)
        });

        let listing = NftListing {
            id: uuid::Uuid::new_v4().to_string(),
            nft,
            seller,
            price,
            currency,
            status: ListingStatus::Active,
            created_at: now,
            updated_at: now,
            expires_at,
        };

        let mut listings = self.listings.write().await;
        listings.insert(listing.id.clone(), listing.clone());

        println!("✅ Listing created: {} for {} lamports", listing.id, price);

        Ok(listing)
    }

    /// Get listing by ID
    pub async fn get_listing(&self, listing_id: &str) -> Result<NftListing> {
        let listings = self.listings.read().await;
        listings
            .get(listing_id)
            .cloned()
            .context("Listing not found")
    }

    /// Get all active listings
    pub async fn get_active_listings(&self) -> Result<Vec<NftListing>> {
        let listings = self.listings.read().await;
        let now = Utc::now();

        let active: Vec<NftListing> = listings
            .values()
            .filter(|l| {
                l.status == ListingStatus::Active
                    && l.expires_at.map_or(true, |exp| exp > now)
            })
            .cloned()
            .collect();

        Ok(active)
    }

    /// Cancel listing
    pub async fn cancel_listing(&self, listing_id: &str, seller: &str) -> Result<()> {
        let mut listings = self.listings.write().await;
        let listing = listings
            .get_mut(listing_id)
            .context("Listing not found")?;

        if listing.seller != seller {
            anyhow::bail!("Only seller can cancel listing");
        }

        if listing.status != ListingStatus::Active {
            anyhow::bail!("Listing is not active");
        }

        listing.status = ListingStatus::Cancelled;
        listing.updated_at = Utc::now();

        println!("✅ Listing cancelled: {}", listing_id);

        Ok(())
    }

    /// Execute sale
    pub async fn execute_sale(
        &self,
        listing_id: &str,
        buyer: String,
        transaction_signature: String,
    ) -> Result<Sale> {
        let mut listings = self.listings.write().await;
        let listing = listings
            .get_mut(listing_id)
            .context("Listing not found")?;

        if listing.status != ListingStatus::Active {
            anyhow::bail!("Listing is not active");
        }

        let now = Utc::now();
        if let Some(expires_at) = listing.expires_at {
            if expires_at <= now {
                listing.status = ListingStatus::Expired;
                anyhow::bail!("Listing has expired");
            }
        }

        // Mark as sold
        listing.status = ListingStatus::Sold;
        listing.updated_at = now;

        // Create sale record
        let sale = Sale {
            id: uuid::Uuid::new_v4().to_string(),
            listing_id: listing_id.to_string(),
            nft_mint: listing.nft.mint.clone(),
            seller: listing.seller.clone(),
            buyer: buyer.clone(),
            price: listing.price,
            currency: listing.currency.clone(),
            transaction_signature,
            timestamp: now,
        };

        let mut sales = self.sales.write().await;
        sales.push(sale.clone());

        println!("✅ Sale executed: {} bought by {}", sale.id, buyer);

        Ok(sale)
    }

    /// Calculate marketplace fee
    pub fn calculate_fee(&self, price: u64) -> u64 {
        (price * self.marketplace_fee_bps as u64) / 10000
    }

    /// Get marketplace statistics
    pub async fn get_stats(&self) -> Result<MarketplaceStats> {
        let listings = self.listings.read().await;
        let sales = self.sales.read().await;

        let total_listings = listings.len();
        let active_listings = listings
            .values()
            .filter(|l| l.status == ListingStatus::Active)
            .count();

        let total_sales = sales.len();
        let total_volume: u64 = sales.iter().map(|s| s.price).sum();

        // Calculate floor price (lowest active listing)
        let active_prices: Vec<u64> = listings
            .values()
            .filter(|l| l.status == ListingStatus::Active)
            .map(|l| l.price)
            .collect();

        let floor_price = active_prices.iter().min().copied();
        let average_price = if !active_prices.is_empty() {
            Some(active_prices.iter().sum::<u64>() / active_prices.len() as u64)
        } else {
            None
        };

        Ok(MarketplaceStats {
            total_listings,
            active_listings,
            total_sales,
            total_volume,
            floor_price,
            average_price,
        })
    }

    /// Get sales history
    pub async fn get_sales_history(&self, limit: usize) -> Result<Vec<Sale>> {
        let sales = self.sales.read().await;
        Ok(sales.iter().rev().take(limit).cloned().collect())
    }

    /// Search listings by business type
    pub async fn search_by_business_type(&self, business_type: &str) -> Result<Vec<NftListing>> {
        let listings = self.listings.read().await;
        let results: Vec<NftListing> = listings
            .values()
            .filter(|l| {
                l.status == ListingStatus::Active
                    && l.nft.attributes.business_type == business_type
            })
            .cloned()
            .collect();

        Ok(results)
    }

    /// Search listings by cuisine
    pub async fn search_by_cuisine(&self, cuisine: &str) -> Result<Vec<NftListing>> {
        let listings = self.listings.read().await;
        let results: Vec<NftListing> = listings
            .values()
            .filter(|l| {
                l.status == ListingStatus::Active
                    && l.nft.attributes.cuisine == cuisine
            })
            .cloned()
            .collect();

        Ok(results)
    }
}

impl Default for NftMarketplace {
    fn default() -> Self {
        Self::new(250) // 2.5% default fee
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nft::BusinessAttributes;

    #[tokio::test]
    async fn test_create_and_get_listing() {
        let marketplace = NftMarketplace::new(250);

        let nft = BusinessNft {
            mint: "mint123".to_string(),
            name: "Sushi Bar".to_string(),
            owner: "owner123".to_string(),
            attributes: BusinessAttributes {
                business_type: "restaurant".to_string(),
                cuisine: "sushi".to_string(),
                location: "Tokyo".to_string(),
                rating: 4.8,
                total_orders: 1000,
                established_date: "2024-01-01".to_string(),
            },
        };

        let listing = marketplace
            .create_listing(
                nft,
                "seller123".to_string(),
                1_000_000_000, // 1 FODI
                Currency::FODI,
                Some(30),
            )
            .await
            .unwrap();

        assert_eq!(listing.price, 1_000_000_000);
        assert_eq!(listing.status, ListingStatus::Active);

        let retrieved = marketplace.get_listing(&listing.id).await.unwrap();
        assert_eq!(retrieved.id, listing.id);
    }

    #[tokio::test]
    async fn test_marketplace_fee() {
        let marketplace = NftMarketplace::new(250); // 2.5%
        
        let fee = marketplace.calculate_fee(1_000_000_000);
        assert_eq!(fee, 25_000_000); // 2.5% of 1B
    }
}

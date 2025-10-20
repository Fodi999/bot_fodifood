//! NFT metadata update functionality

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Keypair,
};
use std::str::FromStr;

use super::BusinessAttributes;

/// On-chain metadata structure (simplified Metaplex format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Vec<Creator>,
    pub update_authority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
    pub address: String,
    pub verified: bool,
    pub share: u8, // Percentage share (0-100)
}

/// Off-chain metadata (JSON at URI)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffChainMetadata {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub image: String,
    pub external_url: Option<String>,
    pub attributes: Vec<Attribute>,
    pub properties: Properties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Properties {
    pub files: Vec<File>,
    pub category: String,
    pub creators: Vec<Creator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub uri: String,
    pub r#type: String, // "image/png", "video/mp4", etc.
}

/// Metadata updater
pub struct MetadataUpdater {
    rpc_client: RpcClient,
    update_authority: Keypair,
}

impl MetadataUpdater {
    pub fn new(rpc_url: String, update_authority: Keypair) -> Self {
        let rpc_client = RpcClient::new(rpc_url);
        Self {
            rpc_client,
            update_authority,
        }
    }

    /// Create off-chain metadata JSON for business NFT
    pub fn create_business_metadata(
        &self,
        name: String,
        description: String,
        image_url: String,
        attributes: BusinessAttributes,
        creator: String,
    ) -> OffChainMetadata {
        let nft_attributes = vec![
            Attribute {
                trait_type: "Business Type".to_string(),
                value: attributes.business_type.clone(),
            },
            Attribute {
                trait_type: "Cuisine".to_string(),
                value: attributes.cuisine.clone(),
            },
            Attribute {
                trait_type: "Location".to_string(),
                value: attributes.location.clone(),
            },
            Attribute {
                trait_type: "Rating".to_string(),
                value: attributes.rating.to_string(),
            },
            Attribute {
                trait_type: "Total Orders".to_string(),
                value: attributes.total_orders.to_string(),
            },
            Attribute {
                trait_type: "Established".to_string(),
                value: attributes.established_date.clone(),
            },
        ];

        OffChainMetadata {
            name: name.clone(),
            symbol: "BZNFT".to_string(), // Business NFT
            description,
            image: image_url.clone(),
            external_url: Some(format!("https://fodifood.com/business/{}", name)),
            attributes: nft_attributes,
            properties: Properties {
                files: vec![File {
                    uri: image_url,
                    r#type: "image/png".to_string(),
                }],
                category: "image".to_string(),
                creators: vec![Creator {
                    address: creator,
                    verified: true,
                    share: 100,
                }],
            },
        }
    }

    /// Update on-chain metadata URI
    pub async fn update_metadata_uri(
        &self,
        mint: &str,
        new_uri: String,
    ) -> Result<String> {
        let mint_pubkey = Pubkey::from_str(mint)?;

        // Derive metadata PDA
        let metadata_seeds: &[&[u8]] = &[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            mint_pubkey.as_ref(),
        ];
        let (metadata_pda, _) = Pubkey::find_program_address(
            metadata_seeds,
            &mpl_token_metadata::ID,
        );

        // Create update metadata instruction
        // Note: This requires mpl-token-metadata crate
        // let update_ix = mpl_token_metadata::instruction::update_metadata_accounts_v2(
        //     mpl_token_metadata::ID,
        //     metadata_pda,
        //     self.update_authority.pubkey(),
        //     None, // new update authority
        //     Some(mpl_token_metadata::state::DataV2 {
        //         name: "...".to_string(),
        //         symbol: "...".to_string(),
        //         uri: new_uri,
        //         seller_fee_basis_points: 500,
        //         creators: Some(vec![...]),
        //         collection: None,
        //         uses: None,
        //     }),
        //     None, // primary sale happened
        //     Some(true), // is mutable
        // );

        // For now, return placeholder
        // TODO: Implement full Metaplex metadata update
        
        println!("⚠️  Metadata update not yet implemented");
        println!("🔗 Mint: {}", mint);
        println!("📝 New URI: {}", new_uri);
        println!("📍 Metadata PDA: {}", metadata_pda);

        Ok("pending_implementation".to_string())
    }

    /// Update business attributes (regenerate off-chain JSON)
    pub fn update_business_attributes(
        &self,
        metadata: &mut OffChainMetadata,
        new_attributes: BusinessAttributes,
    ) {
        // Clear old business attributes
        metadata.attributes.retain(|attr| {
            !matches!(
                attr.trait_type.as_str(),
                "Business Type" | "Cuisine" | "Location" | "Rating" | "Total Orders" | "Established"
            )
        });

        // Add new attributes
        metadata.attributes.extend(vec![
            Attribute {
                trait_type: "Business Type".to_string(),
                value: new_attributes.business_type,
            },
            Attribute {
                trait_type: "Cuisine".to_string(),
                value: new_attributes.cuisine,
            },
            Attribute {
                trait_type: "Location".to_string(),
                value: new_attributes.location,
            },
            Attribute {
                trait_type: "Rating".to_string(),
                value: new_attributes.rating.to_string(),
            },
            Attribute {
                trait_type: "Total Orders".to_string(),
                value: new_attributes.total_orders.to_string(),
            },
            Attribute {
                trait_type: "Established".to_string(),
                value: new_attributes.established_date,
            },
        ]);
    }

    /// Export metadata to JSON string
    pub fn export_metadata_json(&self, metadata: &OffChainMetadata) -> Result<String> {
        serde_json::to_string_pretty(metadata)
            .context("Failed to serialize metadata")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_business_metadata() {
        let keypair = Keypair::new();
        let updater = MetadataUpdater::new(
            "https://api.devnet.solana.com".to_string(),
            keypair,
        );

        let attributes = BusinessAttributes {
            business_type: "restaurant".to_string(),
            cuisine: "sushi".to_string(),
            location: "Tokyo".to_string(),
            rating: 4.8,
            total_orders: 1000,
            established_date: "2024-01-01".to_string(),
        };

        let metadata = updater.create_business_metadata(
            "Sushi Paradise".to_string(),
            "Premium sushi restaurant in Tokyo".to_string(),
            "https://example.com/image.png".to_string(),
            attributes,
            "creator123".to_string(),
        );

        assert_eq!(metadata.name, "Sushi Paradise");
        assert_eq!(metadata.attributes.len(), 6);
        
        // Check specific attribute
        let cuisine_attr = metadata.attributes.iter()
            .find(|a| a.trait_type == "Cuisine")
            .unwrap();
        assert_eq!(cuisine_attr.value, "sushi");
    }

    #[test]
    fn test_metadata_json_export() {
        let keypair = Keypair::new();
        let updater = MetadataUpdater::new(
            "https://api.devnet.solana.com".to_string(),
            keypair,
        );

        let attributes = BusinessAttributes {
            business_type: "cafe".to_string(),
            cuisine: "coffee".to_string(),
            location: "Seattle".to_string(),
            rating: 4.5,
            total_orders: 500,
            established_date: "2024-01-01".to_string(),
        };

        let metadata = updater.create_business_metadata(
            "Cafe Latte".to_string(),
            "Best coffee in Seattle".to_string(),
            "https://example.com/cafe.png".to_string(),
            attributes,
            "creator456".to_string(),
        );

        let json = updater.export_metadata_json(&metadata).unwrap();
        assert!(json.contains("Cafe Latte"));
        assert!(json.contains("coffee"));
    }
}

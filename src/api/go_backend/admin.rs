use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::Value;

use super::types::{Ingredient, IngredientMovement, Stats};

/// 📊 Admin service
pub struct AdminClient {
    client: Client,
    base_url: String,
}

impl AdminClient {
    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }

    /// Get statistics (admin only)
    pub async fn get_stats(&self, token: &str) -> Result<Stats> {
        let url = format!("{}/admin/stats", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to fetch stats")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read stats body")?;

        tracing::info!("📊 Raw stats response ({}): {}", status, text);

        let stats: Stats = serde_json::from_str(&text).context("Failed to parse stats JSON")?;

        Ok(stats)
    }

    /// Get ingredients/inventory
    pub async fn get_ingredients(&self, token: &str) -> Result<Vec<Ingredient>> {
        let url = format!("{}/admin/ingredients", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to fetch ingredients")?;

        let ingredients = response
            .json::<Vec<Ingredient>>()
            .await
            .context("Failed to parse ingredients response")?;

        Ok(ingredients)
    }

    /// Create new ingredient (admin only)
    #[allow(dead_code)] // Will be used in v2.2 Step 5 Admin AI Assistant
    pub async fn create_ingredient(&self, token: &str, data: Value) -> Result<Ingredient> {
        let url = format!("{}/admin/ingredients", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await
            .context("Failed to create ingredient")?;

        let ingredient = response
            .json::<Ingredient>()
            .await
            .context("Failed to parse ingredient response")?;

        Ok(ingredient)
    }

    /// Update ingredient (admin only)
    #[allow(dead_code)]
    pub async fn update_ingredient(&self, token: &str, id: i64, data: Value) -> Result<Ingredient> {
        let url = format!("{}/admin/ingredients/{}", self.base_url, id);

        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&data)
            .send()
            .await
            .context("Failed to update ingredient")?;

        let ingredient = response
            .json::<Ingredient>()
            .await
            .context("Failed to parse updated ingredient response")?;

        Ok(ingredient)
    }

    /// Delete ingredient (admin only)
    #[allow(dead_code)]
    pub async fn delete_ingredient(&self, token: &str, id: i64) -> Result<()> {
        let url = format!("{}/admin/ingredients/{}", self.base_url, id);

        self.client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to delete ingredient")?;

        Ok(())
    }

    /// Get ingredient movements (admin only)
    #[allow(dead_code)]
    pub async fn get_ingredient_movements(
        &self,
        token: &str,
        id: i64,
    ) -> Result<Vec<IngredientMovement>> {
        let url = format!("{}/admin/ingredients/{}/movements", self.base_url, id);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to fetch ingredient movements")?;

        let movements = response
            .json::<Vec<IngredientMovement>>()
            .await
            .context("Failed to parse movements response")?;

        Ok(movements)
    }
}

use anyhow::{Context, Result};
use reqwest::Client;

use super::types::Product;

/// 🍽️ Products service
pub struct ProductsClient {
    client: Client,
    base_url: String,
}

impl ProductsClient {
    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }

    /// Get all visible products (public menu)
    pub async fn get_products(&self) -> Result<Vec<Product>> {
        let url = format!("{}/products", self.base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch products")?;

        let products = response
            .json::<Vec<Product>>()
            .await
            .context("Failed to parse products response")?;

        Ok(products)
    }

    /// 📋 Format products list for display
    pub fn format_products_list(products: &[Product]) -> String {
        if products.is_empty() {
            return "🤔 Меню временно пусто. Скоро добавим новые блюда!".to_string();
        }

        let mut result = String::from("🍽️ **Актуальное меню с реальными ценами:**\n\n");

        // Группируем по категориям
        let mut by_category: std::collections::HashMap<String, Vec<&Product>> =
            std::collections::HashMap::new();

        for product in products {
            if product.is_visible.unwrap_or(true) {
                let category = product
                    .category
                    .clone()
                    .unwrap_or_else(|| "Другое".to_string());
                by_category.entry(category).or_default().push(product);
            }
        }

        // Порядок категорий
        let category_order = vec!["Роллы", "Суши", "Закуски", "Напитки", "Другое"];

        for category_name in category_order {
            if let Some(items) = by_category.get(category_name) {
                result.push_str(&format!("📂 **{}:**\n", category_name));

                for product in items {
                    let price = format!("{}₽", product.price as i32);
                    let weight = product.weight.as_deref().unwrap_or("");
                    let desc = product.description.as_deref().unwrap_or("");

                    result.push_str(&format!(
                        "• **{}** — {} {}\n",
                        product.name,
                        price,
                        if !weight.is_empty() {
                            format!("({})", weight)
                        } else {
                            String::new()
                        }
                    ));

                    if !desc.is_empty() && desc.len() < 100 {
                        result.push_str(&format!("  _{}_\n", desc));
                    }
                }
                result.push('\n');
            }
        }

        result.push_str("💡 Все блюда готовятся из свежайших ингредиентов!\n");
        result.push_str("🚚 Доставка от 1500₽ — бесплатно!");

        result
    }

    /// 🔍 Search product by name
    pub fn find_product_by_name<'a>(products: &'a [Product], query: &str) -> Option<&'a Product> {
        let query_lower = query.to_lowercase();

        // Точное совпадение
        if let Some(p) = products
            .iter()
            .find(|p| p.name.to_lowercase() == query_lower)
        {
            return Some(p);
        }

        // Частичное совпадение
        products
            .iter()
            .find(|p| p.name.to_lowercase().contains(&query_lower))
    }

    /// 🐟 Filter products by ingredient
    pub fn filter_by_ingredient<'a>(products: &'a [Product], ingredient: &str) -> Vec<&'a Product> {
        let ing_lower = ingredient.to_lowercase();
        let ingredient_forms = normalize_ingredient(&ing_lower);

        products
            .iter()
            .filter(|p| {
                let name_lower = p.name.to_lowercase();
                let desc_lower = p.description.as_deref().unwrap_or("").to_lowercase();

                ingredient_forms
                    .iter()
                    .any(|form| name_lower.contains(form) || desc_lower.contains(form))
            })
            .collect()
    }
}

/// 🔄 Normalize ingredient for search (generates word forms)
fn normalize_ingredient(ingredient: &str) -> Vec<String> {
    let mut forms = vec![ingredient.to_string()];

    let ingredient_map = vec![
        (
            "лосось",
            vec!["лосось", "лосося", "лососем", "лососю", "salmon"],
        ),
        (
            "лосося",
            vec!["лосось", "лосося", "лососем", "лососю", "salmon"],
        ),
        (
            "лососем",
            vec!["лосось", "лосося", "лососем", "лососю", "salmon"],
        ),
        (
            "salmon",
            vec!["лосось", "лосося", "лососем", "лососю", "salmon"],
        ),
        (
            "креветки",
            vec!["креветки", "креветок", "креветками", "креветке", "shrimp"],
        ),
        (
            "креветок",
            vec!["креветки", "креветок", "креветками", "креветке", "shrimp"],
        ),
        (
            "креветками",
            vec!["креветки", "креветок", "креветками", "креветке", "shrimp"],
        ),
        (
            "shrimp",
            vec!["креветки", "креветок", "креветками", "креветке", "shrimp"],
        ),
        ("тунец", vec!["тунец", "тунца", "тунцом", "тунцу", "tuna"]),
        ("тунца", vec!["тунец", "тунца", "тунцом", "тунцу", "tuna"]),
        ("тунцом", vec!["тунец", "тунца", "тунцом", "тунцу", "tuna"]),
        ("tuna", vec!["тунец", "тунца", "тунцом", "тунцу", "tuna"]),
        ("угорь", vec!["угорь", "угря", "угрём", "угрю", "eel"]),
        ("угря", vec!["угорь", "угря", "угрём", "угрю", "eel"]),
        ("угрём", vec!["угорь", "угря", "угрём", "угрю", "eel"]),
        ("eel", vec!["угорь", "угря", "угрём", "угрю", "eel"]),
        ("авокадо", vec!["авокадо", "avocado"]),
        ("avocado", vec!["авокадо", "avocado"]),
        ("огурец", vec!["огурец", "огурца", "огурцом", "cucumber"]),
        ("огурца", vec!["огурец", "огурца", "огурцом", "cucumber"]),
        ("огурцом", vec!["огурец", "огурца", "огурцом", "cucumber"]),
        ("cucumber", vec!["огурец", "огурца", "огурцом", "cucumber"]),
        ("сыр", vec!["сыр", "сыра", "сыром", "сыру", "cheese"]),
        ("сыра", vec!["сыр", "сыра", "сыром", "сыру", "cheese"]),
        ("сыром", vec!["сыр", "сыра", "сыром", "сыру", "cheese"]),
        ("cheese", vec!["сыр", "сыра", "сыром", "сыру", "cheese"]),
        ("икра", vec!["икра", "икры", "икрой", "икре", "caviar"]),
        ("икры", vec!["икра", "икры", "икрой", "икре", "caviar"]),
        ("икрой", vec!["икра", "икры", "икрой", "икре", "caviar"]),
        ("caviar", vec!["икра", "икры", "икрой", "икре", "caviar"]),
    ];

    for (key, variants) in ingredient_map {
        if ingredient.contains(key) {
            for variant in variants {
                if !forms.contains(&variant.to_string()) {
                    forms.push(variant.to_string());
                }
            }
            break;
        }
    }

    forms
}

use async_trait::async_trait;

use super::super::intent_handler::{Context, IntentHandler};
use crate::state::AppState;

/// 📋 Menu Intent Handler
pub struct MenuHandler;

impl MenuHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for MenuHandler {
    fn name(&self) -> &'static str {
        "showmenu"  // Match lowercase intent
    }

    fn priority(&self) -> u8 {
        90
    }

    async fn handle(&self, _input: &str, ctx: &mut Context, state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "📋 Handling menu request for user: {}", ctx.user_id);

        match state.backend.products.get_products().await {
            Ok(products) => {
                if products.is_empty() {
                    Some("🤔 Меню временно пусто. Скоро добавим новые блюда!".to_string())
                } else {
                    let formatted =
                        crate::api::go_backend::ProductsClient::format_products_list(&products);
                    Some(formatted)
                }
            }
            Err(e) => {
                tracing::error!(target: "ai", "❌ Failed to fetch menu: {}", e);
                Some("Извините, не могу загрузить меню. Попробуйте позже 😞".to_string())
            }
        }
    }
}

/// 🔍 Search Menu Intent Handler
pub struct SearchMenuHandler;

impl SearchMenuHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for SearchMenuHandler {
    fn name(&self) -> &'static str {
        "searchmenu"  // Match lowercase intent
    }

    fn priority(&self) -> u8 {
        95
    }

    async fn handle(&self, input: &str, ctx: &mut Context, state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "🔍 Handling search menu request for user: {}", ctx.user_id);

        let query = ctx.entities.first().unwrap_or(&input.to_string()).clone();

        match state.backend.products.get_products().await {
            Ok(products) => {
                if let Some(product) =
                    crate::api::go_backend::ProductsClient::find_product_by_name(&products, &query)
                {
                    Some(format!(
                        "🍽️ **{}**\n💰 Цена: {}₽\n📏 Вес: {}\n\n_{}_",
                        product.name,
                        product.price as i32,
                        product.weight.as_deref().unwrap_or("не указан"),
                        product
                            .description
                            .as_deref()
                            .unwrap_or("Описание отсутствует")
                    ))
                } else {
                    Some(format!(
                        "😔 Не нашел блюдо '{}' в меню. Попробуйте другое название.",
                        query
                    ))
                }
            }
            Err(e) => {
                tracing::error!(target: "ai", "❌ Failed to search menu: {}", e);
                Some("Извините, не могу выполнить поиск. Попробуйте позже 😞".to_string())
            }
        }
    }
}

/// 🐟 Filter by Ingredient Handler
pub struct FilterByIngredientHandler;

impl FilterByIngredientHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IntentHandler for FilterByIngredientHandler {
    fn name(&self) -> &'static str {
        "searchbyingredient"  // Match lowercase intent (это SearchByIngredient в enum)
    }

    fn priority(&self) -> u8 {
        85
    }

    async fn handle(&self, input: &str, ctx: &mut Context, state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "🐟 Handling filter by ingredient request for user: {}", ctx.user_id);

        let ingredient = ctx.entities.first().unwrap_or(&input.to_string()).clone();

        match state.backend.products.get_products().await {
            Ok(products) => {
                let filtered = crate::api::go_backend::ProductsClient::filter_by_ingredient(
                    &products,
                    &ingredient,
                );

                if filtered.is_empty() {
                    Some(format!(
                        "😔 Не нашел блюда с ингредиентом '{}'. Попробуйте другой.",
                        ingredient
                    ))
                } else {
                    let mut result = format!("🐟 Блюда с **{}**:\n\n", ingredient);
                    for product in filtered {
                        result.push_str(&format!(
                            "• **{}** — {}₽\n",
                            product.name, product.price as i32
                        ));
                    }
                    Some(result)
                }
            }
            Err(e) => {
                tracing::error!(target: "ai", "❌ Failed to filter by ingredient: {}", e);
                Some("Извините, не могу выполнить фильтрацию. Попробуйте позже 😞".to_string())
            }
        }
    }
}

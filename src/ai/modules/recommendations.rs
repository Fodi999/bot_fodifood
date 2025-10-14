use async_trait::async_trait;

use crate::state::AppState;
use super::super::intent_handler::{IntentHandler, Context};

/// 🎯 Personalized Recommendations Handler
pub struct RecommendationHandler;

impl RecommendationHandler {
    pub fn new() -> Self {
        Self
    }

    /// Check if context contains spicy-related keywords
    fn is_spicy_request(context: &str) -> bool {
        let lower = context.to_lowercase();
        lower.contains("остр") || lower.contains("пикант") || lower.contains("спайс")
    }

    /// Check if context contains diet/healthy keywords
    fn is_diet_request(context: &str) -> bool {
        let lower = context.to_lowercase();
        lower.contains("диет") || lower.contains("здоров") || 
        lower.contains("правильн") || lower.contains("фитнес")
    }

    /// Check if context contains party/company keywords
    fn is_party_request(context: &str) -> bool {
        let lower = context.to_lowercase();
        lower.contains("компан") || lower.contains("вечерин") || 
        lower.contains("праздник") || lower.contains("друз")
    }

    /// Check if context contains seafood keywords
    fn is_seafood_request(context: &str) -> bool {
        let lower = context.to_lowercase();
        lower.contains("море") || lower.contains("рыб") || 
        lower.contains("креветк") || lower.contains("морепродукт")
    }
}

#[async_trait]
impl IntentHandler for RecommendationHandler {
    fn name(&self) -> &'static str {
        "recommendations"
    }

    fn priority(&self) -> u8 {
        70
    }

    async fn handle(&self, input: &str, ctx: &mut Context, state: &AppState) -> Option<String> {
        tracing::info!(target: "ai", "🎯 Handling recommendations request for user: {}", ctx.user_id);

        // Try to get actual products from backend
        let products = match state.backend.products.get_products().await {
            Ok(prods) => prods,
            Err(e) => {
                tracing::error!(target: "ai", "❌ Failed to get products for recommendations: {}", e);
                vec![]
            }
        };

        // Build context-aware recommendations
        let context = input.to_lowercase();

        if Self::is_spicy_request(&context) {
            Some(self.spicy_recommendations(&products))
        } else if Self::is_diet_request(&context) {
            Some(self.diet_recommendations(&products))
        } else if Self::is_party_request(&context) {
            Some(self.party_recommendations(&products))
        } else if Self::is_seafood_request(&context) {
            Some(self.seafood_recommendations(&products))
        } else {
            Some(self.general_recommendations(&products))
        }
    }
}

impl RecommendationHandler {
    fn spicy_recommendations(&self, products: &[crate::api::go_backend::types::Product]) -> String {
        let mut response = "🌶️ **Острые рекомендации:**\n\n".to_string();

        let spicy_products: Vec<_> = products.iter()
            .filter(|p| {
                let name = p.name.to_lowercase();
                name.contains("остр") || name.contains("спайси") || 
                name.contains("чили") || name.contains("халапеньо")
            })
            .take(3)
            .collect();

        if !spicy_products.is_empty() {
            for product in spicy_products {
                response.push_str(&format!(
                    "🔥 {} — {}₽\n",
                    product.name,
                    product.price as i32
                ));
            }
        } else {
            response.push_str(
                "🔥 Острые роллы с халапеньо\n\
                 🌶️ Пикантный тунец спайси\n\
                 🔥 Креветка темпура острая\n\n\
                 Для любителей ярких вкусов!"
            );
        }

        response.push_str("\n\n💡 Совет: Попробуйте с соевым соусом!");
        response
    }

    fn diet_recommendations(&self, products: &[crate::api::go_backend::types::Product]) -> String {
        let mut response = "💪 **Полезные рекомендации:**\n\n".to_string();

        let healthy_products: Vec<_> = products.iter()
            .filter(|p| {
                let name = p.name.to_lowercase();
                name.contains("салат") || name.contains("овощ") || 
                name.contains("рис") || name.contains("лосос")
            })
            .take(3)
            .collect();

        if !healthy_products.is_empty() {
            for product in healthy_products {
                response.push_str(&format!(
                    "🥗 {} — {}₽\n",
                    product.name,
                    product.price as i32
                ));
            }
        } else {
            response.push_str(
                "🥗 Салат из авокадо и лосося\n\
                 🍱 Роллы с огурцом и креветкой\n\
                 🐟 Сашими: лосось, тунец\n\n\
                 Для тех, кто следит за фигурой!"
            );
        }

        response.push_str("\n\n💡 Совет: Меньше калорий, больше пользы!");
        response
    }

    fn party_recommendations(&self, products: &[crate::api::go_backend::types::Product]) -> String {
        let mut response = "🎉 **Для компании:**\n\n".to_string();

        let party_products: Vec<_> = products.iter()
            .filter(|p| {
                let name = p.name.to_lowercase();
                name.contains("сет") || name.contains("набор") || 
                name.contains("микс") || name.contains("ассорти")
            })
            .take(3)
            .collect();

        if !party_products.is_empty() {
            for product in party_products {
                response.push_str(&format!(
                    "🍱 {} — {}₽\n",
                    product.name,
                    product.price as i32
                ));
            }
        } else {
            response.push_str(
                "🍱 Большой сет 'Мега вечеринка'\n\
                 🎉 Ассорти роллов (60 шт)\n\
                 🍣 Микс суши и роллов\n\n\
                 Идеально для вечеринки!"
            );
        }

        response.push_str("\n\n💡 Совет: Берите сеты — выгоднее!");
        response
    }

    fn seafood_recommendations(&self, products: &[crate::api::go_backend::types::Product]) -> String {
        let mut response = "🦐 **Морепродукты:**\n\n".to_string();

        let seafood_products: Vec<_> = products.iter()
            .filter(|p| {
                let name = p.name.to_lowercase();
                name.contains("креветк") || name.contains("лосос") || 
                name.contains("тунец") || name.contains("угор") ||
                name.contains("краб") || name.contains("икр")
            })
            .take(3)
            .collect();

        if !seafood_products.is_empty() {
            for product in seafood_products {
                response.push_str(&format!(
                    "🐟 {} — {}₽\n",
                    product.name,
                    product.price as i32
                ));
            }
        } else {
            response.push_str(
                "🦐 Тигровые креветки темпура\n\
                 🐟 Лосось с авокадо\n\
                 🦀 Краб спайси\n\n\
                 Свежие морепродукты каждый день!"
            );
        }

        response.push_str("\n\n💡 Совет: У нас свежий лосось из Норвегии!");
        response
    }

    fn general_recommendations(&self, products: &[crate::api::go_backend::types::Product]) -> String {
        let mut response = "🎯 **Популярные рекомендации:**\n\n".to_string();

        if products.is_empty() {
            response.push_str(
                "🏆 Топ-3 хита:\n\
                 1️⃣ Филадельфия классик\n\
                 2️⃣ Калифорния с лососем\n\
                 3️⃣ Дракон с угрём\n\n\
                 💡 Попробуйте наши бестселлеры!"
            );
        } else {
            // Take top 3 products or popular ones
            for (i, product) in products.iter().take(3).enumerate() {
                response.push_str(&format!(
                    "{}️⃣ {} — {}₽\n",
                    i + 1,
                    product.name,
                    product.price as i32
                ));
            }
            
            response.push_str("\n\n💡 Самые популярные позиции нашего меню!");
        }

        response.push_str(
            "\n\n🔍 Уточните предпочтения:\n\
             • 'хочу острое' 🌶️\n\
             • 'что-то полезное' 💪\n\
             • 'для компании' 🎉\n\
             • 'с морепродуктами' 🦐"
        );

        response
    }
}

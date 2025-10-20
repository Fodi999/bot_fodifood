// 🎁 AI Airdrop Agent - умный дистрибьютор токенов для маркетинговых кампаний
//
// Этот модуль управляет airdrop-кампаниями, распределяя токены между пользователями
// с учётом лимитов проекта, стратегии распределения и ROI-анализа.

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 🎯 Стратегия airdrop-кампании
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AirdropStrategy {
    /// Равномерное распределение всем пользователям
    Equal,
    /// Пропорционально активности пользователя
    ActivityBased { min_activity: f64 },
    /// Случайная лотерея среди участников
    Lottery { winners_count: usize },
    /// Градация по уровням (VIP, Premium, Regular)
    Tiered { vip: f64, premium: f64, regular: f64 },
}

/// 📊 Данные о распределении токенов одному пользователю
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirdropDistribution {
    pub user_id: String,
    pub amount: f64,
    pub timestamp: DateTime<Utc>,
    pub campaign_id: String,
    pub status: DistributionStatus,
}

/// ✅ Статус распределения токенов
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DistributionStatus {
    Pending,      // Ожидает отправки
    Sent,         // Отправлено (off-chain)
    OnChain,      // Подтверждено on-chain
    Failed,       // Ошибка отправки
}

/// 🎁 Airdrop кампания
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirdropCampaign {
    pub id: String,
    pub project_name: String,
    pub project_symbol: String,
    pub total_budget: f64,
    pub distributed: f64,
    pub participants: usize,
    pub strategy: AirdropStrategy,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub distributions: Vec<AirdropDistribution>,
}

impl AirdropCampaign {
    /// Создать новую airdrop-кампанию
    pub fn new(
        project_name: &str,
        project_symbol: &str,
        total_budget: f64,
        strategy: AirdropStrategy,
    ) -> Self {
        let campaign_id = format!("AIRDROP-{}-{}", 
            project_symbol, 
            Utc::now().timestamp()
        );

        Self {
            id: campaign_id,
            project_name: project_name.to_string(),
            project_symbol: project_symbol.to_string(),
            total_budget,
            distributed: 0.0,
            participants: 0,
            strategy,
            start_date: Utc::now(),
            end_date: None,
            distributions: Vec::new(),
        }
    }

    /// Получить процент распределения
    pub fn progress_percentage(&self) -> f64 {
        if self.total_budget == 0.0 {
            return 0.0;
        }
        (self.distributed / self.total_budget) * 100.0
    }

    /// Проверить, завершена ли кампания
    pub fn is_completed(&self) -> bool {
        self.end_date.is_some() && self.distributed >= self.total_budget
    }

    /// Получить оставшийся бюджет
    pub fn remaining_budget(&self) -> f64 {
        self.total_budget - self.distributed
    }
}

/// 🤖 AI Airdrop Agent - умный дистрибьютор токенов
pub struct AirdropAgent {
    /// История всех распределений по пользователям
    pub distributed_tokens: HashMap<String, f64>,
    /// Активные airdrop-кампании
    pub campaigns: Vec<AirdropCampaign>,
    /// Общее количество распределённых токенов
    pub total_distributed: f64,
}

impl AirdropAgent {
    /// Создать нового AI Airdrop агента
    pub fn new() -> Self {
        Self {
            distributed_tokens: HashMap::new(),
            campaigns: Vec::new(),
            total_distributed: 0.0,
        }
    }

    /// 🎁 Запустить простой airdrop с равномерным распределением
    ///
    /// # Arguments
    /// * `project_name` - Название проекта
    /// * `project_symbol` - Символ токена
    /// * `available_tokens` - Доступное количество токенов в пуле
    /// * `users` - Список пользователей для airdrop
    /// * `tokens_per_user` - Количество токенов на пользователя
    ///
    /// # Returns
    /// Количество успешно распределённых токенов
    pub fn launch_simple_airdrop(
        &mut self,
        project_name: &str,
        project_symbol: &str,
        available_tokens: f64,
        users: Vec<&str>,
        tokens_per_user: f64,
    ) -> f64 {
        let total_needed = tokens_per_user * users.len() as f64;

        if total_needed > available_tokens {
            println!("🚫 Недостаточно токенов в пуле проекта {}", project_symbol);
            println!("   Требуется: {:.2}, Доступно: {:.2}", total_needed, available_tokens);
            return 0.0;
        }

        println!("\n🎁 AI Copilot запускает Airdrop кампанию для {}!", project_name);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📊 Параметры:");
        println!("   • Токенов на пользователя: {:.2} {}", tokens_per_user, project_symbol);
        println!("   • Участников: {}", users.len());
        println!("   • Общий бюджет: {:.2} {}", total_needed, project_symbol);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        let mut campaign = AirdropCampaign::new(
            project_name,
            project_symbol,
            total_needed,
            AirdropStrategy::Equal,
        );

        let users_count = users.len();
        
        for user in users {
            println!("💸 Отправлено {:.2} {} → {}", tokens_per_user, project_symbol, user);
            
            // Обновить баланс пользователя
            *self.distributed_tokens.entry(user.to_string()).or_insert(0.0) += tokens_per_user;
            
            // Добавить запись о распределении
            campaign.distributions.push(AirdropDistribution {
                user_id: user.to_string(),
                amount: tokens_per_user,
                timestamp: Utc::now(),
                campaign_id: campaign.id.clone(),
                status: DistributionStatus::Sent,
            });
        }

        campaign.distributed = total_needed;
        campaign.participants = users_count;
        campaign.end_date = Some(Utc::now());

        self.campaigns.push(campaign);
        self.total_distributed += total_needed;

        println!("\n✅ Успешно распределено {:.2} {} среди {} участников", 
            total_needed, project_symbol, users_count);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        total_needed
    }

    /// 🎯 Запустить airdrop со стратегией
    ///
    /// # Arguments
    /// * `project_name` - Название проекта
    /// * `project_symbol` - Символ токена
    /// * `available_tokens` - Доступное количество токенов
    /// * `users` - Список пользователей с метаданными (user_id, activity_score)
    /// * `strategy` - Стратегия распределения
    ///
    /// # Returns
    /// Количество распределённых токенов
    pub fn launch_strategic_airdrop(
        &mut self,
        project_name: &str,
        project_symbol: &str,
        available_tokens: f64,
        users: Vec<(String, f64)>, // (user_id, activity_score/tier)
        strategy: AirdropStrategy,
    ) -> f64 {
        println!("\n🎯 AI Copilot запускает стратегический Airdrop для {}!", project_name);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📊 Стратегия: {:?}", strategy);
        println!("💰 Доступно токенов: {:.2} {}", available_tokens, project_symbol);
        println!("👥 Потенциальных участников: {}", users.len());
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        let mut campaign = AirdropCampaign::new(
            project_name,
            project_symbol,
            available_tokens,
            strategy.clone(),
        );

        let distributions = match strategy {
            AirdropStrategy::Equal => {
                self.calculate_equal_distribution(&users, available_tokens)
            }
            AirdropStrategy::ActivityBased { min_activity } => {
                self.calculate_activity_based_distribution(&users, available_tokens, min_activity)
            }
            AirdropStrategy::Lottery { winners_count } => {
                self.calculate_lottery_distribution(&users, available_tokens, winners_count)
            }
            AirdropStrategy::Tiered { vip, premium, regular } => {
                self.calculate_tiered_distribution(&users, available_tokens, vip, premium, regular)
            }
        };

        let mut total_distributed = 0.0;

        for (user_id, amount) in distributions {
            if total_distributed + amount > available_tokens {
                println!("⚠️  Достигнут лимит бюджета, остановка распределения");
                break;
            }

            println!("💸 {} → {:.2} {}", user_id, amount, project_symbol);

            *self.distributed_tokens.entry(user_id.clone()).or_insert(0.0) += amount;
            
            campaign.distributions.push(AirdropDistribution {
                user_id: user_id.clone(),
                amount,
                timestamp: Utc::now(),
                campaign_id: campaign.id.clone(),
                status: DistributionStatus::Sent,
            });

            total_distributed += amount;
        }

        campaign.distributed = total_distributed;
        let participants_count = campaign.distributions.len();
        campaign.participants = participants_count;
        campaign.end_date = Some(Utc::now());

        self.campaigns.push(campaign);
        self.total_distributed += total_distributed;

        println!("\n✅ Успешно распределено {:.2} {} среди {} участников", 
            total_distributed, project_symbol, participants_count);
        println!("📈 Прогресс: {:.1}% от бюджета", 
            (total_distributed / available_tokens) * 100.0);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        total_distributed
    }

    /// Рассчитать равномерное распределение
    fn calculate_equal_distribution(
        &self,
        users: &[(String, f64)],
        available_tokens: f64,
    ) -> Vec<(String, f64)> {
        let tokens_per_user = available_tokens / users.len() as f64;
        users.iter()
            .map(|(user_id, _)| (user_id.clone(), tokens_per_user))
            .collect()
    }

    /// Рассчитать распределение по активности
    fn calculate_activity_based_distribution(
        &self,
        users: &[(String, f64)],
        available_tokens: f64,
        min_activity: f64,
    ) -> Vec<(String, f64)> {
        // Фильтруем пользователей с активностью выше минимальной
        let eligible_users: Vec<_> = users.iter()
            .filter(|(_, activity)| *activity >= min_activity)
            .collect();

        if eligible_users.is_empty() {
            return Vec::new();
        }

        // Сумма всех активностей
        let total_activity: f64 = eligible_users.iter()
            .map(|(_, activity)| *activity)
            .sum();

        // Распределяем пропорционально активности
        eligible_users.iter()
            .map(|(user_id, activity)| {
                let share = *activity / total_activity;
                let amount = available_tokens * share;
                (user_id.to_string(), amount)
            })
            .collect()
    }

    /// Рассчитать лотерейное распределение
    fn calculate_lottery_distribution(
        &self,
        users: &[(String, f64)],
        available_tokens: f64,
        winners_count: usize,
    ) -> Vec<(String, f64)> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        let actual_winners = winners_count.min(users.len());
        
        // Случайно выбираем победителей
        let mut user_ids: Vec<_> = users.iter().map(|(id, _)| id.clone()).collect();
        user_ids.shuffle(&mut rng);
        
        let winners: Vec<String> = user_ids.into_iter().take(actual_winners).collect();
        let tokens_per_winner = available_tokens / actual_winners as f64;

        winners.into_iter()
            .map(|user_id| (user_id, tokens_per_winner))
            .collect()
    }

    /// Рассчитать градуированное распределение (VIP/Premium/Regular)
    fn calculate_tiered_distribution(
        &self,
        users: &[(String, f64)],
        available_tokens: f64,
        vip_multiplier: f64,
        premium_multiplier: f64,
        regular_multiplier: f64,
    ) -> Vec<(String, f64)> {
        // tier: 3.0 = VIP, 2.0 = Premium, 1.0 = Regular
        let total_multiplier: f64 = users.iter()
            .map(|(_, tier)| {
                if *tier >= 3.0 { vip_multiplier }
                else if *tier >= 2.0 { premium_multiplier }
                else { regular_multiplier }
            })
            .sum();

        users.iter()
            .map(|(user_id, tier)| {
                let multiplier = if *tier >= 3.0 { vip_multiplier }
                    else if *tier >= 2.0 { premium_multiplier }
                    else { regular_multiplier };
                
                let share = multiplier / total_multiplier;
                let amount = available_tokens * share;
                (user_id.clone(), amount)
            })
            .collect()
    }

    /// 📊 Получить баланс пользователя по всем airdrop'ам
    pub fn get_user_balance(&self, user_id: &str) -> f64 {
        *self.distributed_tokens.get(user_id).unwrap_or(&0.0)
    }

    /// 📈 Получить статистику по всем кампаниям
    pub fn get_statistics(&self) -> AirdropStatistics {
        let total_campaigns = self.campaigns.len();
        let active_campaigns = self.campaigns.iter()
            .filter(|c| !c.is_completed())
            .count();
        
        let total_participants: usize = self.campaigns.iter()
            .map(|c| c.participants)
            .sum();

        let average_tokens_per_user = if total_participants > 0 {
            self.total_distributed / total_participants as f64
        } else {
            0.0
        };

        AirdropStatistics {
            total_campaigns,
            active_campaigns,
            total_distributed: self.total_distributed,
            total_participants,
            average_tokens_per_user,
            unique_recipients: self.distributed_tokens.len(),
        }
    }

    /// 📋 Показать отчёт о всех кампаниях
    pub fn print_report(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  🎁 AI Airdrop Agent - Отчёт о распределении токенов       ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        let stats = self.get_statistics();

        println!("📊 **Общая статистика:**");
        println!("   • Всего кампаний: {}", stats.total_campaigns);
        println!("   • Активных кампаний: {}", stats.active_campaigns);
        println!("   • Распределено токенов: {:.2}", stats.total_distributed);
        println!("   • Всего участников: {}", stats.total_participants);
        println!("   • Уникальных получателей: {}", stats.unique_recipients);
        println!("   • Среднее на пользователя: {:.2}", stats.average_tokens_per_user);

        if !self.campaigns.is_empty() {
            println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("🗂️  **История кампаний:**\n");

            for (i, campaign) in self.campaigns.iter().enumerate() {
                let status = if campaign.is_completed() { "✅" } else { "🔄" };
                println!("{}. {} {} ({})", i + 1, status, campaign.project_name, campaign.project_symbol);
                println!("   Campaign ID: {}", campaign.id);
                println!("   Бюджет: {:.2} | Распределено: {:.2} ({:.1}%)",
                    campaign.total_budget,
                    campaign.distributed,
                    campaign.progress_percentage()
                );
                println!("   Участников: {} | Стратегия: {:?}",
                    campaign.participants,
                    campaign.strategy
                );
                println!("   Дата: {}", campaign.start_date.format("%Y-%m-%d %H:%M:%S"));
                println!();
            }
        }

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
}

impl Default for AirdropAgent {
    fn default() -> Self {
        Self::new()
    }
}

/// 📊 Статистика airdrop-агента
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirdropStatistics {
    pub total_campaigns: usize,
    pub active_campaigns: usize,
    pub total_distributed: f64,
    pub total_participants: usize,
    pub average_tokens_per_user: f64,
    pub unique_recipients: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_airdrop() {
        let mut agent = AirdropAgent::new();
        let users = vec!["@alice", "@bob", "@charlie"];
        
        let distributed = agent.launch_simple_airdrop(
            "Test Project",
            "TEST",
            1000.0,
            users.clone(),
            100.0,
        );

        assert_eq!(distributed, 300.0);
        assert_eq!(agent.get_user_balance("@alice"), 100.0);
        assert_eq!(agent.campaigns.len(), 1);
    }

    #[test]
    fn test_insufficient_tokens() {
        let mut agent = AirdropAgent::new();
        let users = vec!["@alice", "@bob"];
        
        let distributed = agent.launch_simple_airdrop(
            "Test Project",
            "TEST",
            100.0, // Недостаточно
            users,
            100.0, // Нужно 200
        );

        assert_eq!(distributed, 0.0);
        assert_eq!(agent.campaigns.len(), 0);
    }

    #[test]
    fn test_activity_based_strategy() {
        let mut agent = AirdropAgent::new();
        let users = vec![
            ("@alice".to_string(), 10.0),
            ("@bob".to_string(), 5.0),
            ("@charlie".to_string(), 15.0),
        ];

        let distributed = agent.launch_strategic_airdrop(
            "Activity Project",
            "ACT",
            1000.0,
            users,
            AirdropStrategy::ActivityBased { min_activity: 4.0 },
        );

        assert!(distributed > 0.0);
        assert!(distributed <= 1000.0);
        // Alice должна получить меньше, чем Charlie (меньше активности)
        assert!(agent.get_user_balance("@alice") < agent.get_user_balance("@charlie"));
    }
}

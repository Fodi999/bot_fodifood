// 🌱 AI Growth Campaign Engine - Автономная система управления ростом
//
// Этот модуль объединяет все AI компоненты в единую систему,
// которая автоматически создаёт, запускает и оптимизирует маркетинговые кампании.

use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

use crate::ai::social_tasks::{TaskManager, TaskPlatform, TaskType};
use crate::ai::airdrop_agent::AirdropAgent;

/// 🎯 Цель роста кампании
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GrowthGoal {
    /// Увеличение трафика (%)
    IncreaseTraffic { target_percent: f64 },
    /// Привлечение новых пользователей
    AcquireUsers { target_count: usize },
    /// Увеличение вовлечённости
    BoostEngagement { target_actions: usize },
    /// Увеличение продаж
    IncreaseSales { target_revenue: f64 },
    /// Повышение узнаваемости бренда
    BrandAwareness { target_reach: usize },
    /// Вирусный рост
    GoViral { target_shares: usize },
}

impl GrowthGoal {
    /// Получить описание цели
    pub fn description(&self) -> String {
        match self {
            GrowthGoal::IncreaseTraffic { target_percent } => {
                format!("Увеличить трафик на {:.1}%", target_percent)
            }
            GrowthGoal::AcquireUsers { target_count } => {
                format!("Привлечь {} новых пользователей", target_count)
            }
            GrowthGoal::BoostEngagement { target_actions } => {
                format!("Получить {} взаимодействий", target_actions)
            }
            GrowthGoal::IncreaseSales { target_revenue } => {
                format!("Увеличить выручку на {:.2}₽", target_revenue)
            }
            GrowthGoal::BrandAwareness { target_reach } => {
                format!("Охват {} пользователей", target_reach)
            }
            GrowthGoal::GoViral { target_shares } => {
                format!("Получить {} репостов", target_shares)
            }
        }
    }
}

/// 📊 Стратегия распределения бюджета
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BudgetStrategy {
    /// Равномерное распределение между платформами
    Equal,
    /// По эффективности прошлых кампаний
    PerformanceBased,
    /// Фокус на одной платформе
    Focused { platform: TaskPlatform },
    /// Адаптивное (AI оптимизирует в процессе)
    Adaptive,
}

/// 🎯 Growth Campaign - маркетинговая кампания роста
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthCampaign {
    /// ID кампании
    pub id: String,
    /// Название
    pub name: String,
    /// Бизнес-владелец
    pub business: String,
    /// Бюджет в токенах
    pub budget_tokens: f64,
    /// Цель кампании
    pub goal: GrowthGoal,
    /// Целевая аудитория (количество пользователей)
    pub users_target: usize,
    /// Целевая конверсия (%)
    pub conversion_goal: f64,
    /// Стратегия бюджета
    pub budget_strategy: BudgetStrategy,
    /// Дата запуска
    pub launched_at: Option<DateTime<Utc>>,
    /// Дата завершения
    pub ended_at: Option<DateTime<Utc>>,
    /// Продолжительность (часы)
    pub duration_hours: i64,
    /// Платформы для использования
    pub platforms: Vec<TaskPlatform>,
    /// Результаты кампании
    pub results: Option<CampaignResults>,
    /// Статус кампании
    pub status: CampaignStatus,
}

/// ✅ Статус кампании
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CampaignStatus {
    /// Готова к запуску
    Ready,
    /// Запущена и активна
    Running,
    /// Завершена успешно
    Completed,
    /// Приостановлена
    Paused,
    /// Отменена
    Cancelled,
}

/// 📊 Результаты кампании
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignResults {
    /// Всего вовлечённых пользователей
    pub total_engagement: usize,
    /// Всего потрачено токенов
    pub total_spent: f64,
    /// ROI (Return on Investment) %
    pub roi: f64,
    /// Конверсия %
    pub conversion_rate: f64,
    /// Результаты по платформам
    pub platform_results: HashMap<String, PlatformMetrics>,
    /// Достигнута ли цель
    pub goal_achieved: bool,
}

/// 📈 Метрики по платформе
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatformMetrics {
    /// Количество задач
    pub tasks_created: usize,
    /// Выполнено задач
    pub tasks_completed: usize,
    /// Токенов распределено
    pub tokens_spent: f64,
    /// Вовлечённость
    pub engagement: usize,
    /// CTR (Click-Through Rate) %
    pub ctr: f64,
    /// CPA (Cost Per Action)
    pub cpa: f64,
}

impl GrowthCampaign {
    /// Создать новую growth campaign
    pub fn new(
        name: &str,
        business: &str,
        budget_tokens: f64,
        goal: GrowthGoal,
        users_target: usize,
        conversion_goal: f64,
        duration_hours: i64,
    ) -> Self {
        let id = format!(
            "CAMPAIGN-{}-{}",
            business.replace(" ", "-"),
            Utc::now().timestamp()
        );

        Self {
            id,
            name: name.to_string(),
            business: business.to_string(),
            budget_tokens,
            goal,
            users_target,
            conversion_goal,
            budget_strategy: BudgetStrategy::Equal,
            launched_at: None,
            ended_at: None,
            duration_hours,
            platforms: vec![
                TaskPlatform::Instagram,
                TaskPlatform::TikTok,
                TaskPlatform::Twitter,
                TaskPlatform::Telegram,
                TaskPlatform::VK,
            ],
            results: None,
            status: CampaignStatus::Ready,
        }
    }

    /// Установить стратегию бюджета
    pub fn with_budget_strategy(mut self, strategy: BudgetStrategy) -> Self {
        self.budget_strategy = strategy;
        self
    }

    /// Установить платформы
    pub fn with_platforms(mut self, platforms: Vec<TaskPlatform>) -> Self {
        self.platforms = platforms;
        self
    }

    /// 🚀 Запустить кампанию
    pub fn launch(
        &mut self,
        task_manager: &mut TaskManager,
        airdrop: &mut AirdropAgent,
    ) -> Vec<String> {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  🚀 AI Growth Campaign Engine                              ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        println!("🌱 **{}**", self.name);
        println!("🏢 Бизнес: {}", self.business);
        println!("🎯 Цель: {}", self.goal.description());
        println!("💰 Бюджет: {:.2} токенов", self.budget_tokens);
        println!("👥 Целевая аудитория: {} пользователей", self.users_target);
        println!("📈 Целевая конверсия: {:.1}%", self.conversion_goal);
        println!("⏰ Продолжительность: {} часов", self.duration_hours);
        println!("🎬 Платформы: {}", self.platforms.len());
        println!();

        self.launched_at = Some(Utc::now());
        self.status = CampaignStatus::Running;

        let task_ids = self.create_missions(task_manager);

        println!("\n✅ Growth Campaign запущена [{}]", Utc::now().format("%Y-%m-%d %H:%M:%S"));
        println!("📊 Создано {} маркетинговых миссий", task_ids.len());
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        task_ids
    }

    /// Создать маркетинговые миссии
    fn create_missions(&self, task_manager: &mut TaskManager) -> Vec<String> {
        let mut task_ids = Vec::new();

        // Рассчитать бюджет на платформу
        let budget_per_platform = match &self.budget_strategy {
            BudgetStrategy::Equal => {
                self.budget_tokens / self.platforms.len() as f64
            }
            BudgetStrategy::PerformanceBased => {
                // TODO: Использовать данные прошлых кампаний
                self.budget_tokens / self.platforms.len() as f64
            }
            BudgetStrategy::Focused { .. } => {
                self.budget_tokens // Весь бюджет на одну платформу
            }
            BudgetStrategy::Adaptive => {
                self.budget_tokens / self.platforms.len() as f64
            }
        };

        println!("💸 **Создание маркетинговых миссий:**\n");

        for platform in &self.platforms {
            // Определить тип задачи на основе платформы
            let task_type = match platform {
                TaskPlatform::Instagram => TaskType::CreatePost,
                TaskPlatform::TikTok => TaskType::CreateVideo,
                TaskPlatform::Twitter => TaskType::Share,
                TaskPlatform::YouTube => TaskType::CreateVideo,
                TaskPlatform::Telegram => TaskType::Referral,
                TaskPlatform::VK => TaskType::Review,
                TaskPlatform::Threads => TaskType::CreatePost,
                TaskPlatform::Reddit => TaskType::CreatePost,
                TaskPlatform::Medium => TaskType::CreatePost,
            };

            let description = self.generate_task_description(platform, &task_type);
            let reward = budget_per_platform / (self.users_target as f64 / self.platforms.len() as f64);

            let task_id = task_manager.create_task(
                &self.business,
                platform.clone(),
                task_type,
                &description,
                reward,
                self.duration_hours,
            );

            println!("   {} {} - {:.2} токенов за действие", 
                platform.emoji(), platform.name(), reward);

            task_ids.push(task_id);
        }

        task_ids
    }

    /// Сгенерировать описание задачи
    fn generate_task_description(&self, platform: &TaskPlatform, task_type: &TaskType) -> String {
        match (platform, task_type) {
            (TaskPlatform::Instagram, TaskType::CreatePost) => {
                format!("📸 Создай пост о {} с хэштегом #FodiFood и отметь наш аккаунт!", self.name)
            }
            (TaskPlatform::TikTok, TaskType::CreateVideo) => {
                format!("🎥 Сними видео о {} с хэштегом #FodiFood!", self.name)
            }
            (TaskPlatform::Twitter, TaskType::Share) => {
                format!("🐦 Сделай ретвит нашего поста о {}!", self.name)
            }
            (TaskPlatform::Telegram, TaskType::Referral) => {
                format!("💬 Поделись ботом {} в 3 чата!", self.name)
            }
            (TaskPlatform::VK, TaskType::Review) => {
                format!("⭐ Напиши отзыв о {}!", self.name)
            }
            _ => {
                format!("Поддержи {} в соцсетях!", self.name)
            }
        }
    }

    /// 📊 Оценить результаты кампании
    pub fn evaluate_results(
        &mut self,
        total_engagement: usize,
        total_spent: f64,
        platform_data: Option<HashMap<String, PlatformMetrics>>,
    ) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  📊 Growth Campaign Results                                ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        println!("🌱 **{}**", self.name);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Рассчитать метрики
        let roi = if self.users_target > 0 {
            (total_engagement as f64 / self.users_target as f64) * 100.0
        } else {
            0.0
        };

        let conversion_rate = if total_engagement > 0 {
            (total_engagement as f64 / self.users_target as f64) * 100.0
        } else {
            0.0
        };

        let goal_achieved = roi >= self.conversion_goal;

        println!("\n💰 **Финансовые показатели:**");
        println!("   • Бюджет: {:.2} токенов", self.budget_tokens);
        println!("   • Потрачено: {:.2} токенов ({:.1}%)", 
            total_spent, 
            (total_spent / self.budget_tokens) * 100.0
        );
        println!("   • Осталось: {:.2} токенов", self.budget_tokens - total_spent);

        println!("\n📈 **Результаты:**");
        println!("   • Вовлечённость: {} действий", total_engagement);
        println!("   • ROI: {:.1}% (цель: {:.1}%)", roi, self.conversion_goal);
        println!("   • Конверсия: {:.1}%", conversion_rate);

        if goal_achieved {
            println!("\n   ✅ **Цель достигнута!** ROI = {:.1}%", roi);
        } else {
            println!("\n   ⚠️  **ROI ниже ожидаемого** ({:.1}% vs {:.1}%)", 
                roi, self.conversion_goal);
        }

        // Результаты по платформам
        if let Some(ref platform_metrics) = platform_data {
            println!("\n🌐 **Результаты по платформам:**\n");

            let mut sorted_platforms: Vec<_> = platform_metrics.iter().collect();
            sorted_platforms.sort_by(|a, b| {
                b.1.engagement.cmp(&a.1.engagement)
            });

            for (platform, metrics) in sorted_platforms {
                println!("   {} **{}**", 
                    self.get_platform_emoji(platform),
                    platform
                );
                println!("      • Выполнено задач: {}", metrics.tasks_completed);
                println!("      • Вовлечённость: {}", metrics.engagement);
                println!("      • Токенов: {:.2}", metrics.tokens_spent);
                if metrics.tasks_completed > 0 {
                    println!("      • CPA: {:.2} токенов/действие", metrics.cpa);
                }
                println!();
            }
        }

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Сохранить результаты
        self.results = Some(CampaignResults {
            total_engagement,
            total_spent,
            roi,
            conversion_rate,
            platform_results: platform_data.unwrap_or_default(),
            goal_achieved,
        });

        self.status = CampaignStatus::Completed;
        self.ended_at = Some(Utc::now());

        // AI рекомендации
        self.generate_ai_recommendations();
    }

    /// 🧠 Генерация AI рекомендаций
    fn generate_ai_recommendations(&self) {
        println!("\n💡 **AI Рекомендации для следующей кампании:**\n");

        if let Some(results) = &self.results {
            if results.roi >= self.conversion_goal * 1.5 {
                println!("   ✨ Отличный результат! Увеличьте бюджет на 30%");
            } else if results.roi >= self.conversion_goal {
                println!("   ✅ Хороший результат! Сохраняйте текущую стратегию");
            } else {
                println!("   ⚠️  Оптимизируйте: уменьшите слабые платформы, усильте эффективные");
            }

            // Анализ по платформам
            if !results.platform_results.is_empty() {
                let best_platform = results.platform_results.iter()
                    .max_by_key(|(_, m)| m.engagement)
                    .map(|(name, _)| name);

                if let Some(best) = best_platform {
                    println!("   🏆 Лучшая платформа: {} - увеличьте бюджет на неё", best);
                }
            }

            println!("   🎯 Рекомендуемый бюджет следующей кампании: {:.2} токенов",
                if results.goal_achieved {
                    self.budget_tokens * 1.2
                } else {
                    self.budget_tokens * 0.9
                }
            );
        }

        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }

    /// Получить эмодзи платформы
    fn get_platform_emoji(&self, platform_name: &str) -> &str {
        match platform_name {
            "Instagram" => "📸",
            "TikTok" => "🎥",
            "Twitter" | "Twitter/X" => "🐦",
            "YouTube" => "▶️",
            "Telegram" => "💬",
            "VK" => "🔵",
            "Threads" => "🧵",
            "Reddit" => "🔴",
            "Medium" => "📝",
            _ => "🌐",
        }
    }

    /// Приостановить кампанию
    pub fn pause(&mut self) {
        self.status = CampaignStatus::Paused;
        println!("⏸️  Кампания {} приостановлена", self.name);
    }

    /// Возобновить кампанию
    pub fn resume(&mut self) {
        self.status = CampaignStatus::Running;
        println!("▶️  Кампания {} возобновлена", self.name);
    }

    /// Отменить кампанию
    pub fn cancel(&mut self) {
        self.status = CampaignStatus::Cancelled;
        self.ended_at = Some(Utc::now());
        println!("❌ Кампания {} отменена", self.name);
    }
}

/// 🎯 Growth Campaign Manager - управление всеми кампаниями
pub struct GrowthCampaignManager {
    /// Активные кампании
    pub campaigns: HashMap<String, GrowthCampaign>,
    /// История кампаний
    pub history: Vec<GrowthCampaign>,
}

impl GrowthCampaignManager {
    /// Создать новый менеджер
    pub fn new() -> Self {
        Self {
            campaigns: HashMap::new(),
            history: Vec::new(),
        }
    }

    /// Добавить кампанию
    pub fn add_campaign(&mut self, campaign: GrowthCampaign) {
        let id = campaign.id.clone();
        self.campaigns.insert(id, campaign);
    }

    /// Получить активные кампании
    pub fn get_active_campaigns(&self) -> Vec<&GrowthCampaign> {
        self.campaigns.values()
            .filter(|c| c.status == CampaignStatus::Running)
            .collect()
    }

    /// Архивировать завершённую кампанию
    pub fn archive_campaign(&mut self, campaign_id: &str) {
        if let Some(campaign) = self.campaigns.remove(campaign_id) {
            self.history.push(campaign);
        }
    }

    /// Получить статистику всех кампаний
    pub fn get_total_stats(&self) -> (usize, f64, f64) {
        let total_campaigns = self.history.len();
        let total_budget: f64 = self.history.iter()
            .map(|c| c.budget_tokens)
            .sum();
        let avg_roi: f64 = if total_campaigns > 0 {
            self.history.iter()
                .filter_map(|c| c.results.as_ref().map(|r| r.roi))
                .sum::<f64>() / total_campaigns as f64
        } else {
            0.0
        };

        (total_campaigns, total_budget, avg_roi)
    }

    /// Показать сводку
    pub fn summary(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  📊 Growth Campaign Manager - Сводка                       ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        let (total, budget, avg_roi) = self.get_total_stats();

        println!("📈 **Всего кампаний:** {}", total);
        println!("💰 **Общий бюджет:** {:.2} токенов", budget);
        println!("📊 **Средний ROI:** {:.1}%", avg_roi);
        println!("🚀 **Активных:** {}", self.get_active_campaigns().len());
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
}

impl Default for GrowthCampaignManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_campaign() {
        let campaign = GrowthCampaign::new(
            "Test Campaign",
            "Test Business",
            10000.0,
            GrowthGoal::AcquireUsers { target_count: 200 },
            200,
            20.0,
            72,
        );

        assert_eq!(campaign.name, "Test Campaign");
        assert_eq!(campaign.budget_tokens, 10000.0);
        assert_eq!(campaign.status, CampaignStatus::Ready);
    }

    #[test]
    fn test_campaign_manager() {
        let mut manager = GrowthCampaignManager::new();
        let campaign = GrowthCampaign::new(
            "Test",
            "Business",
            5000.0,
            GrowthGoal::BoostEngagement { target_actions: 100 },
            100,
            15.0,
            48,
        );

        manager.add_campaign(campaign);
        assert_eq!(manager.campaigns.len(), 1);
    }
}

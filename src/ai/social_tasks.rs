// 🌐 Social Tasks Module - AI-управляемые маркетинговые миссии в соцсетях
//
// Этот модуль позволяет бизнесу создавать задачи для пользователей в социальных сетях,
// автоматически начислять токены за выполнение и отслеживать эффективность кампаний.

use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

/// 📱 Социальные платформы для маркетинговых миссий
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaskPlatform {
    /// Instagram (посты, stories, reels)
    Instagram,
    /// TikTok (короткие видео)
    TikTok,
    /// Twitter/X (твиты, ретвиты)
    Twitter,
    /// YouTube (shorts, видео)
    YouTube,
    /// Telegram (каналы, группы)
    Telegram,
    /// VK (посты, репосты)
    VK,
    /// Threads (Meta's text platform)
    Threads,
    /// Reddit (посты в сабреддитах)
    Reddit,
    /// Medium (статьи)
    Medium,
}

impl TaskPlatform {
    /// Получить эмодзи платформы
    pub fn emoji(&self) -> &str {
        match self {
            TaskPlatform::Instagram => "📸",
            TaskPlatform::TikTok => "🎥",
            TaskPlatform::Twitter => "🐦",
            TaskPlatform::YouTube => "▶️",
            TaskPlatform::Telegram => "💬",
            TaskPlatform::VK => "🔵",
            TaskPlatform::Threads => "🧵",
            TaskPlatform::Reddit => "🔴",
            TaskPlatform::Medium => "📝",
        }
    }

    /// Получить название платформы
    pub fn name(&self) -> &str {
        match self {
            TaskPlatform::Instagram => "Instagram",
            TaskPlatform::TikTok => "TikTok",
            TaskPlatform::Twitter => "Twitter/X",
            TaskPlatform::YouTube => "YouTube",
            TaskPlatform::Telegram => "Telegram",
            TaskPlatform::VK => "VK",
            TaskPlatform::Threads => "Threads",
            TaskPlatform::Reddit => "Reddit",
            TaskPlatform::Medium => "Medium",
        }
    }
}

/// 📋 Тип маркетинговой миссии
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskType {
    /// Создать пост с упоминанием бренда
    CreatePost,
    /// Сделать репост/шер
    Share,
    /// Написать отзыв
    Review,
    /// Снять видео
    CreateVideo,
    /// Подписаться на аккаунт
    Follow,
    /// Поставить лайк
    Like,
    /// Оставить комментарий
    Comment,
    /// Пригласить друзей (реферальная ссылка)
    Referral,
    /// Участвовать в конкурсе
    Contest,
}

/// ✅ Статус выполнения задачи
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    /// Активна и доступна для выполнения
    Active,
    /// Завершена (достигнут лимит выполнений или дедлайн)
    Completed,
    /// Отменена создателем
    Cancelled,
    /// Истёк срок
    Expired,
}

/// 🎯 Социальная маркетинговая миссия
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialTask {
    /// Уникальный ID задачи
    pub id: String,
    /// Название бизнеса-создателя
    pub business: String,
    /// Платформа для выполнения
    pub platform: TaskPlatform,
    /// Тип задачи
    pub task_type: TaskType,
    /// Описание задачи
    pub description: String,
    /// Вознаграждение в токенах
    pub reward_tokens: f64,
    /// Дедлайн выполнения
    pub deadline: DateTime<Utc>,
    /// Максимальное количество выполнений (None = без лимита)
    pub max_completions: Option<usize>,
    /// Список пользователей, выполнивших задачу
    pub completed_by: Vec<String>,
    /// Статус задачи
    pub status: TaskStatus,
    /// Дата создания
    pub created_at: DateTime<Utc>,
    /// URL для подтверждения (опционально)
    pub verification_url: Option<String>,
    /// Хэштеги для задачи
    pub hashtags: Vec<String>,
    /// Минимальный охват для вознаграждения (опционально)
    pub min_reach: Option<usize>,
}

impl SocialTask {
    /// Создать новую задачу
    pub fn new(
        business: &str,
        platform: TaskPlatform,
        task_type: TaskType,
        description: &str,
        reward_tokens: f64,
        deadline_hours: i64,
    ) -> Self {
        let id = format!(
            "TASK-{}-{}-{}",
            business.replace(" ", "-"),
            platform.name().replace("/", "-"),
            Utc::now().timestamp()
        );

        Self {
            id,
            business: business.to_string(),
            platform,
            task_type,
            description: description.to_string(),
            reward_tokens,
            deadline: Utc::now() + Duration::hours(deadline_hours),
            max_completions: None,
            completed_by: Vec::new(),
            status: TaskStatus::Active,
            created_at: Utc::now(),
            verification_url: None,
            hashtags: Vec::new(),
            min_reach: None,
        }
    }

    /// Установить максимальное количество выполнений
    pub fn with_max_completions(mut self, max: usize) -> Self {
        self.max_completions = Some(max);
        self
    }

    /// Установить URL для верификации
    pub fn with_verification_url(mut self, url: &str) -> Self {
        self.verification_url = Some(url.to_string());
        self
    }

    /// Добавить хэштеги
    pub fn with_hashtags(mut self, hashtags: Vec<String>) -> Self {
        self.hashtags = hashtags;
        self
    }

    /// Установить минимальный охват
    pub fn with_min_reach(mut self, reach: usize) -> Self {
        self.min_reach = Some(reach);
        self
    }

    /// Проверить, можно ли выполнить задачу
    pub fn can_be_completed(&self) -> bool {
        if self.status != TaskStatus::Active {
            return false;
        }

        // Проверить дедлайн
        if Utc::now() > self.deadline {
            return false;
        }

        // Проверить лимит выполнений
        if let Some(max) = self.max_completions {
            if self.completed_by.len() >= max {
                return false;
            }
        }

        true
    }

    /// Проверить, выполнил ли пользователь задачу
    pub fn is_completed_by(&self, user: &str) -> bool {
        self.completed_by.contains(&user.to_string())
    }

    /// Получить процент выполнения (если есть лимит)
    pub fn completion_percentage(&self) -> Option<f64> {
        self.max_completions.map(|max| {
            (self.completed_by.len() as f64 / max as f64) * 100.0
        })
    }

    /// Получить оставшееся время до дедлайна
    pub fn time_remaining(&self) -> Duration {
        self.deadline.signed_duration_since(Utc::now())
    }

    /// Обновить статус на основе текущего состояния
    pub fn update_status(&mut self) {
        if Utc::now() > self.deadline {
            self.status = TaskStatus::Expired;
        } else if let Some(max) = self.max_completions {
            if self.completed_by.len() >= max {
                self.status = TaskStatus::Completed;
            }
        }
    }
}

/// 🎯 Менеджер социальных маркетинговых миссий
pub struct TaskManager {
    /// Все задачи по ID
    pub tasks: HashMap<String, SocialTask>,
    /// Всего токенов распределено
    pub total_rewards_distributed: f64,
    /// Статистика по платформам
    pub platform_stats: HashMap<TaskPlatform, PlatformStats>,
}

/// 📊 Статистика по платформе
#[derive(Debug, Clone, Default)]
pub struct PlatformStats {
    pub tasks_created: usize,
    pub tasks_completed: usize,
    pub tokens_distributed: f64,
    pub unique_participants: usize,
}

impl TaskManager {
    /// Создать новый менеджер задач
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            total_rewards_distributed: 0.0,
            platform_stats: HashMap::new(),
        }
    }

    /// 🎯 Создать новую маркетинговую задачу
    pub fn create_task(
        &mut self,
        business: &str,
        platform: TaskPlatform,
        task_type: TaskType,
        description: &str,
        reward: f64,
        deadline_hours: i64,
    ) -> String {
        let task = SocialTask::new(
            business,
            platform.clone(),
            task_type,
            description,
            reward,
            deadline_hours,
        );

        let task_id = task.id.clone();

        // Обновить статистику платформы
        let stats = self.platform_stats.entry(platform).or_insert_with(PlatformStats::default);
        stats.tasks_created += 1;

        self.tasks.insert(task_id.clone(), task);

        println!("✅ Создана маркетинговая миссия: {}", task_id);

        task_id
    }

    /// ✅ Отметить задачу как выполненную пользователем
    ///
    /// # Returns
    /// Количество начисленных токенов (или None если задача не может быть выполнена)
    pub fn mark_completed(&mut self, task_id: &str, user: &str) -> Option<f64> {
        if let Some(task) = self.tasks.get_mut(task_id) {
            // Обновить статус задачи
            task.update_status();

            // Проверить, можно ли выполнить
            if !task.can_be_completed() {
                println!("🚫 Задача {} недоступна для выполнения", task_id);
                return None;
            }

            // Проверить, не выполнял ли пользователь уже эту задачу
            if task.is_completed_by(user) {
                println!("⚠️  {} уже выполнил задачу {}", user, task_id);
                return None;
            }

            // Отметить выполнение
            task.completed_by.push(user.to_string());

            // Обновить статистику
            self.total_rewards_distributed += task.reward_tokens;
            
            if let Some(stats) = self.platform_stats.get_mut(&task.platform) {
                stats.tasks_completed += 1;
                stats.tokens_distributed += task.reward_tokens;
            }

            // Обновить статус задачи
            task.update_status();

            println!(
                "✅ {} завершил задачу {} → +{:.2} токенов",
                user, task_id, task.reward_tokens
            );

            Some(task.reward_tokens)
        } else {
            println!("❌ Задача {} не найдена", task_id);
            None
        }
    }

    /// 📋 Получить все активные задачи
    pub fn get_active_tasks(&self) -> Vec<&SocialTask> {
        self.tasks
            .values()
            .filter(|t| t.status == TaskStatus::Active && t.can_be_completed())
            .collect()
    }

    /// 📋 Получить задачи по платформе
    pub fn get_tasks_by_platform(&self, platform: &TaskPlatform) -> Vec<&SocialTask> {
        self.tasks
            .values()
            .filter(|t| &t.platform == platform && t.status == TaskStatus::Active)
            .collect()
    }

    /// 📋 Получить задачи по бизнесу
    pub fn get_tasks_by_business(&self, business: &str) -> Vec<&SocialTask> {
        self.tasks
            .values()
            .filter(|t| t.business == business && t.status == TaskStatus::Active)
            .collect()
    }

    /// 👤 Получить выполненные задачи пользователя
    pub fn get_user_completed_tasks(&self, user: &str) -> Vec<&SocialTask> {
        self.tasks
            .values()
            .filter(|t| t.is_completed_by(user))
            .collect()
    }

    /// 💰 Получить общее вознаграждение пользователя
    pub fn get_user_total_rewards(&self, user: &str) -> f64 {
        self.get_user_completed_tasks(user)
            .iter()
            .map(|t| t.reward_tokens)
            .sum()
    }

    /// 🗑️ Удалить истёкшие задачи
    pub fn cleanup_expired_tasks(&mut self) -> usize {
        let expired: Vec<String> = self
            .tasks
            .iter_mut()
            .filter_map(|(id, task)| {
                task.update_status();
                if task.status == TaskStatus::Expired {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect();

        let count = expired.len();
        for id in expired {
            self.tasks.remove(&id);
        }

        if count > 0 {
            println!("🗑️  Удалено {} истёкших задач", count);
        }

        count
    }

    /// 📊 Показать статистику по всем задачам
    pub fn summary(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  📣 Активные маркетинговые миссии                          ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        let active_tasks = self.get_active_tasks();

        if active_tasks.is_empty() {
            println!("😕 Нет активных задач");
            return;
        }

        println!("📋 Всего активных миссий: {}\n", active_tasks.len());

        for task in active_tasks {
            let remaining = task.time_remaining();
            let hours_left = remaining.num_hours();
            
            println!("🎯 [{}]", task.id);
            println!("   {} {} | {} {}", 
                task.platform.emoji(),
                task.platform.name(),
                task.business,
                match &task.task_type {
                    TaskType::CreatePost => "✍️ Создать пост",
                    TaskType::Share => "🔄 Поделиться",
                    TaskType::Review => "⭐ Отзыв",
                    TaskType::CreateVideo => "🎬 Снять видео",
                    TaskType::Follow => "➕ Подписаться",
                    TaskType::Like => "❤️ Лайк",
                    TaskType::Comment => "💬 Комментарий",
                    TaskType::Referral => "👥 Реферал",
                    TaskType::Contest => "🏆 Конкурс",
                }
            );
            println!("   📝 {}", task.description);
            println!("   💰 Вознаграждение: {:.2} токенов", task.reward_tokens);
            println!("   ⏰ Осталось: {} часов", hours_left);
            
            if let Some(max) = task.max_completions {
                println!("   👥 Выполнено: {}/{} ({:.1}%)", 
                    task.completed_by.len(), 
                    max,
                    task.completion_percentage().unwrap_or(0.0)
                );
            } else {
                println!("   👥 Выполнено: {} пользователей", task.completed_by.len());
            }

            if !task.hashtags.is_empty() {
                println!("   🏷️  Хэштеги: {}", task.hashtags.join(" "));
            }

            println!();
        }

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("💰 Всего распределено: {:.2} токенов\n", self.total_rewards_distributed);
    }

    /// 📊 Показать статистику по платформам
    pub fn platform_report(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  📊 Статистика по платформам                               ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        if self.platform_stats.is_empty() {
            println!("📭 Нет данных для отображения");
            return;
        }

        for (platform, stats) in &self.platform_stats {
            println!("{} **{}**", platform.emoji(), platform.name());
            println!("   📋 Задач создано: {}", stats.tasks_created);
            println!("   ✅ Задач выполнено: {}", stats.tasks_completed);
            println!("   💰 Токенов распределено: {:.2}", stats.tokens_distributed);
            println!();
        }

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }

    /// 🔗 Генерация LinkHub для бизнеса
    ///
    /// Создаёт единый линк-хаб, куда собирается весь трафик
    pub fn generate_linkhub(&self, business: &str, base_url: &str) -> String {
        let safe_business = business.replace(" ", "-").to_lowercase();
        let linkhub_url = format!("{}/go/{}", base_url, safe_business);

        println!("\n🔗 **LinkHub для {}**", business);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🌐 {}", linkhub_url);
        println!();
        println!("📱 Куда направляется трафик:");
        println!("   • 🍽️  Меню и заказ");
        println!("   • 🤖 Telegram Bot");
        println!("   • 📸 Instagram");
        println!("   • 🎁 Бонусная программа");
        println!("   • ⭐ Отзывы");
        println!("   • 💳 Подписка на обновления");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        linkhub_url
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task() {
        let mut manager = TaskManager::new();
        let task_id = manager.create_task(
            "Test Restaurant",
            TaskPlatform::Instagram,
            TaskType::CreatePost,
            "Test task",
            50.0,
            24,
        );

        assert!(manager.tasks.contains_key(&task_id));
        assert_eq!(manager.tasks[&task_id].reward_tokens, 50.0);
    }

    #[test]
    fn test_mark_completed() {
        let mut manager = TaskManager::new();
        let task_id = manager.create_task(
            "Test Restaurant",
            TaskPlatform::Instagram,
            TaskType::CreatePost,
            "Test task",
            50.0,
            24,
        );

        let reward = manager.mark_completed(&task_id, "@testuser");
        assert_eq!(reward, Some(50.0));
        assert_eq!(manager.total_rewards_distributed, 50.0);

        // Попытка выполнить дважды
        let reward2 = manager.mark_completed(&task_id, "@testuser");
        assert_eq!(reward2, None);
    }

    #[test]
    fn test_task_expiration() {
        let mut task = SocialTask::new(
            "Test",
            TaskPlatform::Instagram,
            TaskType::CreatePost,
            "Test",
            10.0,
            -1, // Истекла час назад
        );

        task.update_status();
        assert_eq!(task.status, TaskStatus::Expired);
        assert!(!task.can_be_completed());
    }

    #[test]
    fn test_max_completions() {
        let mut manager = TaskManager::new();
        let task_id = manager.create_task(
            "Test",
            TaskPlatform::Instagram,
            TaskType::CreatePost,
            "Test",
            10.0,
            24,
        );

        // Установить лимит 2 выполнения
        if let Some(task) = manager.tasks.get_mut(&task_id) {
            task.max_completions = Some(2);
        }

        manager.mark_completed(&task_id, "@user1");
        manager.mark_completed(&task_id, "@user2");
        
        // Третье выполнение должно быть заблокировано
        let reward3 = manager.mark_completed(&task_id, "@user3");
        assert_eq!(reward3, None);
    }
}

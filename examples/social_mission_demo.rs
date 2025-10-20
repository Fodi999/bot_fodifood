// 🌐 Social Mission Demo - AI-управляемые маркетинговые кампании в соцсетях
//
// Этот пример демонстрирует:
// 1. Создание маркетинговых миссий для разных платформ
// 2. Выполнение задач пользователями
// 3. Автоматическое начисление токенов
// 4. Статистику по платформам
// 5. Генерацию LinkHub для сбора трафика
// 6. Интеграцию с Airdrop Agent

use fodifood_bot::ai::social_tasks::{TaskManager, TaskPlatform, TaskType};
use fodifood_bot::ai::airdrop_agent::AirdropAgent;

fn main() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║  🌐 FodiFood AI Social Missions Demo                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 📊 Demo 1: Создание маркетинговых миссий
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎯 Demo 1: Creating Marketing Missions\n");

    let mut manager = TaskManager::new();

    // 📸 Instagram миссия - создать пост
    let t1 = manager.create_task(
        "FodiFood Sushi Spot",
        TaskPlatform::Instagram,
        TaskType::CreatePost,
        "📸 Сделай пост о FodiFood и отметь наш аккаунт @FodiFoodOfficial",
        50.0,
        48, // 2 дня
    );

    // Добавим хэштеги и лимит
    if let Some(task) = manager.tasks.get_mut(&t1) {
        *task = task.clone()
            .with_hashtags(vec![
                "#FodiFood".to_string(),
                "#SushiLovers".to_string(),
                "#FoodieLife".to_string(),
            ])
            .with_max_completions(100); // Максимум 100 участников
    }

    // 🎥 TikTok миссия - снять видео
    let t2 = manager.create_task(
        "FodiFood Sushi Spot",
        TaskPlatform::TikTok,
        TaskType::CreateVideo,
        "🎥 Сними видео с хэштегом #FodiFood и расскажи о своём любимом блюде!",
        100.0,
        72, // 3 дня
    );

    if let Some(task) = manager.tasks.get_mut(&t2) {
        *task = task.clone()
            .with_hashtags(vec!["#FodiFood".to_string(), "#Foodie".to_string()])
            .with_min_reach(1000) // Минимум 1000 просмотров
            .with_max_completions(50);
    }

    // 💬 Telegram миссия - поделиться ботом
    let t3 = manager.create_task(
        "FodiFood Burger Club",
        TaskPlatform::Telegram,
        TaskType::Referral,
        "💬 Поделись ссылкой на наш бот в 3 чата и получи бонус!",
        30.0,
        24, // 1 день
    );

    // 🐦 Twitter миссия - ретвит
    let t4 = manager.create_task(
        "FodiFood Burger Club",
        TaskPlatform::Twitter,
        TaskType::Share,
        "🐦 Сделай ретвит нашего поста о новом меню!",
        25.0,
        48,
    );

    // ⭐ Отзыв на VK
    let t5 = manager.create_task(
        "FodiFood Sushi Spot",
        TaskPlatform::VK,
        TaskType::Review,
        "⭐ Напиши отзыв о нашем ресторане в VK!",
        75.0,
        168, // Неделя
    );

    // 🧵 Threads пост
    let t6 = manager.create_task(
        "FodiFood Burger Club",
        TaskPlatform::Threads,
        TaskType::CreatePost,
        "🧵 Создай тред о том, почему FodiFood — лучший выбор для обеда!",
        40.0,
        24,
    );

    println!("\n✅ Создано {} маркетинговых миссий\n", manager.tasks.len());

    // Показать все активные миссии
    manager.summary();

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 📊 Demo 2: Пользователи выполняют миссии
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("👥 Demo 2: Users Completing Missions\n");

    let completions = vec![
        ("@anna", &t1),      // Instagram пост
        ("@mark", &t2),      // TikTok видео
        ("@yuri", &t3),      // Telegram реферал
        ("@polina", &t1),    // Instagram пост
        ("@alex", &t4),      // Twitter ретвит
        ("@kate", &t5),      // VK отзыв
        ("@john", &t6),      // Threads пост
        ("@sara", &t1),      // Instagram пост
        ("@mike", &t2),      // TikTok видео
        ("@lucy", &t3),      // Telegram реферал
    ];

    println!("🚀 Пользователи начинают выполнять миссии...\n");

    let mut user_rewards: std::collections::HashMap<String, f64> = std::collections::HashMap::new();

    for (user, task_id) in completions {
        if let Some(amount) = manager.mark_completed(task_id, user) {
            *user_rewards.entry(user.to_string()).or_insert(0.0) += amount;
        }
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 📊 Demo 3: Статистика по пользователям
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("💰 Demo 3: User Rewards Summary\n");

    println!("📊 **Топ участников:**\n");
    
    let mut sorted_users: Vec<_> = user_rewards.iter().collect();
    sorted_users.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    for (i, (user, tokens)) in sorted_users.iter().enumerate() {
        let medal = match i {
            0 => "🥇",
            1 => "🥈",
            2 => "🥉",
            _ => "  ",
        };
        println!("{}  {} → {:.2} токенов", medal, user, tokens);
    }

    println!("\n💎 Всего распределено токенов: {:.2}", manager.total_rewards_distributed);

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 📊 Demo 4: Статистика по платформам
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    manager.platform_report();

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 📊 Demo 5: LinkHub генерация
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔗 Demo 5: LinkHub Generation\n");

    let sushi_hub = manager.generate_linkhub("FodiFood Sushi Spot", "https://fodifood.ai");
    let burger_hub = manager.generate_linkhub("FodiFood Burger Club", "https://fodifood.ai");

    println!("💡 **Как использовать LinkHub:**");
    println!("   • Разместите {} во всех соцсетях", sushi_hub);
    println!("   • Трафик автоматически распределяется по каналам");
    println!("   • AI отслеживает конверсию и оптимизирует редиректы");
    println!("   • Каждый переход = потенциальный заказ + подписчик\n");

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 📊 Demo 6: Интеграция с Airdrop Agent
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎁 Demo 6: Integration with Airdrop Agent\n");

    let mut airdrop = AirdropAgent::new();

    println!("💸 AI Copilot автоматически начисляет токены за выполненные миссии:\n");

    // Конвертируем награды в airdrop
    for (user, tokens) in &user_rewards {
        let user_str = user.as_str();
        airdrop.launch_simple_airdrop(
            "FodiFood Social Rewards",
            "FDF-SOCIAL",
            10_000.0, // Общий пул
            vec![user_str],
            *tokens,
        );
    }

    println!("\n📊 **Балансы после интеграции с Airdrop Agent:**\n");
    for (user, expected_tokens) in sorted_users.iter().take(5) {
        let actual_balance = airdrop.get_user_balance(user);
        println!("   {} → {:.2} FDF-SOCIAL (ожидалось {:.2})", 
            user, actual_balance, expected_tokens);
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 📊 Demo 7: Real Business Case
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("💼 Demo 7: Real Business Marketing Campaign\n");

    println!("🎯 **Сценарий:** Ресторан запускает вирусную маркетинговую кампанию");
    println!("📊 **Бюджет:** 10,000 FDF-SOCIAL tokens");
    println!("🎬 **Цель:** +5000 новых подписчиков за неделю\n");

    let mut viral_manager = TaskManager::new();

    // Создаём комплексную кампанию
    let viral_tasks = vec![
        (TaskPlatform::Instagram, TaskType::CreatePost, "Пост с фото блюда", 30.0, 168),
        (TaskPlatform::TikTok, TaskType::CreateVideo, "Видео о кухне", 100.0, 168),
        (TaskPlatform::Instagram, TaskType::Share, "Stories с локацией", 20.0, 168),
        (TaskPlatform::Telegram, TaskType::Referral, "Пригласи 5 друзей", 50.0, 168),
        (TaskPlatform::Twitter, TaskType::Share, "Ретвит акции", 15.0, 168),
        (TaskPlatform::VK, TaskType::Review, "Развёрнутый отзыв", 60.0, 168),
        (TaskPlatform::Threads, TaskType::CreatePost, "Тред о любимом блюде", 25.0, 168),
        (TaskPlatform::YouTube, TaskType::CreateVideo, "Обзор ресторана", 150.0, 168),
    ];

    for (platform, task_type, desc, reward, hours) in viral_tasks {
        viral_manager.create_task(
            "FodiFood Premium",
            platform,
            task_type,
            desc,
            reward,
            hours,
        );
    }

    println!("📋 Запущено {} маркетинговых каналов", viral_manager.tasks.len());
    println!();

    viral_manager.summary();

    println!("\n💡 **Ожидаемые результаты за неделю:**");
    println!("   • Instagram reach: +10,000 impressions");
    println!("   • TikTok views: +50,000 views");
    println!("   • Telegram referrals: +500 new users");
    println!("   • Total engagement: +15,000 interactions");
    println!("   • New orders: +300 (конверсия 2%)");
    println!("   • ROI: 300% (10,000 токенов → 30,000₽ выручки)");

    println!("\n🎯 **AI Copilot автоматически:**");
    println!("   ✅ Создаёт задачи на основе целей бизнеса");
    println!("   ✅ Оптимизирует вознаграждения по CTR");
    println!("   ✅ Отслеживает выполнение в реальном времени");
    println!("   ✅ Начисляет токены через Airdrop Agent");
    println!("   ✅ Анализирует ROI через AI CFO");
    println!("   ✅ Генерирует отчёты для Business Brain");

    // Финальный отчёт
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n✨ Social Missions Demo Complete!\n");
    println!("🌐 Demonstrated features:");
    println!("   ✅ Multi-platform task creation (9 platforms)");
    println!("   ✅ Automated token rewards");
    println!("   ✅ User leaderboards");
    println!("   ✅ Platform analytics");
    println!("   ✅ LinkHub generation");
    println!("   ✅ Airdrop Agent integration");
    println!("   ✅ Real business campaign simulation");
    println!("\n🚀 Ready to launch viral marketing campaigns!");
    println!("💡 Next: Connect social media APIs, add proof verification, automate campaigns\n");
}

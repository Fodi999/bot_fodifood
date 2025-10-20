// 🎁 AI Airdrop Agent Demo - Демонстрация токен-дистрибьюции
//
// Этот пример показывает возможности AI Airdrop Agent:
// 1. Простой равномерный airdrop
// 2. Airdrop на основе активности пользователей
// 3. Лотерейный airdrop (случайные победители)
// 4. Градуированный airdrop (VIP/Premium/Regular)
// 5. Мульти-кампании с отчётностью

use fodifood_bot::ai::airdrop_agent::{AirdropAgent, AirdropStrategy};

fn main() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║  🎁 FodiFood AI Airdrop Agent - Token Distribution Demo    ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 📊 Demo 1: Простой равномерный Airdrop
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎯 Demo 1: Simple Equal Airdrop for New Users\n");

    let mut agent = AirdropAgent::new();
    
    // Список новых пользователей для welcome airdrop
    let new_users = vec!["@anna", "@mark", "@yuri", "@sasha", "@polina"];
    
    // Запускаем airdrop: 50 токенов каждому
    agent.launch_simple_airdrop(
        "FodiFood Burger Club",
        "FDF-BURGER",
        10_000.0,  // Доступно в пуле
        new_users.clone(),
        50.0,      // Токенов на пользователя
    );

    // Проверяем балансы
    println!("📊 Балансы после airdrop:");
    let user_list = vec!["@anna", "@mark", "@yuri", "@sasha", "@polina"];
    for user in &user_list {
        println!("   {} → {:.2} FDF-BURGER", user, agent.get_user_balance(user));
    }
    println!();

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 📊 Demo 2: Activity-Based Airdrop
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🏃 Demo 2: Activity-Based Airdrop (Rewards for Active Users)\n");

    // Пользователи с разным уровнем активности
    let active_users = vec![
        ("@alice".to_string(), 10.0),    // Очень активная
        ("@bob".to_string(), 5.0),       // Средняя активность
        ("@charlie".to_string(), 15.0),  // Супер-активный
        ("@diana".to_string(), 3.0),     // Низкая активность (отсеется)
        ("@eve".to_string(), 8.0),       // Хорошая активность
    ];

    agent.launch_strategic_airdrop(
        "FodiFood Activity Rewards",
        "FDF-ACT",
        5_000.0,
        active_users,
        AirdropStrategy::ActivityBased { min_activity: 4.0 }, // Минимум 4 балла
    );

    println!("📊 Балансы после activity-based airdrop:");
    println!("   @alice → {:.2} FDF-ACT", agent.get_user_balance("@alice"));
    println!("   @bob → {:.2} FDF-ACT", agent.get_user_balance("@bob"));
    println!("   @charlie → {:.2} FDF-ACT (highest activity!)", agent.get_user_balance("@charlie"));
    println!("   @diana → {:.2} FDF-ACT (filtered out - low activity)", agent.get_user_balance("@diana"));
    println!("   @eve → {:.2} FDF-ACT", agent.get_user_balance("@eve"));
    println!();

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 📊 Demo 3: Lottery Airdrop
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎰 Demo 3: Lottery Airdrop (Random Winners)\n");

    let lottery_participants = vec![
        ("@user1".to_string(), 0.0),
        ("@user2".to_string(), 0.0),
        ("@user3".to_string(), 0.0),
        ("@user4".to_string(), 0.0),
        ("@user5".to_string(), 0.0),
        ("@user6".to_string(), 0.0),
        ("@user7".to_string(), 0.0),
        ("@user8".to_string(), 0.0),
    ];

    agent.launch_strategic_airdrop(
        "FodiFood Lucky Draw",
        "FDF-LUCKY",
        10_000.0,
        lottery_participants.clone(),
        AirdropStrategy::Lottery { winners_count: 3 }, // Только 3 победителя
    );

    println!("📊 Результаты лотереи:");
    for (user, _) in &lottery_participants {
        let balance = agent.get_user_balance(user);
        if balance > 0.0 {
            println!("   🎉 {} → {:.2} FDF-LUCKY (WINNER!)", user, balance);
        } else {
            println!("   {} → {:.2} FDF-LUCKY", user, balance);
        }
    }
    println!();

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 📊 Demo 4: Tiered Airdrop (VIP/Premium/Regular)
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("👑 Demo 4: Tiered Airdrop (VIP gets more!)\n");

    let tiered_users = vec![
        ("@vip_alex".to_string(), 3.0),      // VIP tier
        ("@vip_kate".to_string(), 3.0),      // VIP tier
        ("@premium_john".to_string(), 2.0),  // Premium tier
        ("@premium_sara".to_string(), 2.0),  // Premium tier
        ("@regular_mike".to_string(), 1.0),  // Regular tier
        ("@regular_lucy".to_string(), 1.0),  // Regular tier
    ];

    agent.launch_strategic_airdrop(
        "FodiFood VIP Club",
        "FDF-VIP",
        6_000.0,
        tiered_users,
        AirdropStrategy::Tiered {
            vip: 3.0,       // VIP получает в 3 раза больше
            premium: 2.0,   // Premium в 2 раза больше
            regular: 1.0,   // Regular базовое количество
        },
    );

    println!("📊 Балансы после tiered airdrop:");
    println!("   👑 @vip_alex → {:.2} FDF-VIP", agent.get_user_balance("@vip_alex"));
    println!("   👑 @vip_kate → {:.2} FDF-VIP", agent.get_user_balance("@vip_kate"));
    println!("   💎 @premium_john → {:.2} FDF-VIP", agent.get_user_balance("@premium_john"));
    println!("   💎 @premium_sara → {:.2} FDF-VIP", agent.get_user_balance("@premium_sara"));
    println!("   👤 @regular_mike → {:.2} FDF-VIP", agent.get_user_balance("@regular_mike"));
    println!("   👤 @regular_lucy → {:.2} FDF-VIP", agent.get_user_balance("@regular_lucy"));
    println!();

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 📊 Demo 5: Multiple Campaigns + Full Report
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Demo 5: Complete Report & Statistics\n");

    // Ещё одна кампания для разнообразия
    let referral_users = vec!["@referrer1", "@referrer2", "@referrer3"];
    agent.launch_simple_airdrop(
        "FodiFood Referral Bonus",
        "FDF-REF",
        3_000.0,
        referral_users,
        100.0,
    );

    // Показываем полный отчёт
    agent.print_report();

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 📊 Demo 6: Business Case Example
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("💼 Demo 6: Real Business Case - Restaurant Marketing\n");

    println!("🎯 **Scenario:** Ресторан запускает токен-экономику для лояльности");
    println!("📊 **Budget:** 50,000 FDF-LOYALTY tokens");
    println!("👥 **Target:** 200 постоянных клиентов\n");

    let loyalty_campaign = vec![
        ("@customer_gold1".to_string(), 3.0),   // Золотой клиент
        ("@customer_gold2".to_string(), 3.0),   // Золотой клиент
        ("@customer_silver1".to_string(), 2.0), // Серебро
        ("@customer_silver2".to_string(), 2.0), // Серебро
        ("@customer_silver3".to_string(), 2.0), // Серебро
        ("@customer_bronze1".to_string(), 1.0), // Бронза
        ("@customer_bronze2".to_string(), 1.0), // Бронза
        ("@customer_bronze3".to_string(), 1.0), // Бронза
        ("@customer_bronze4".to_string(), 1.0), // Бронза
        ("@customer_bronze5".to_string(), 1.0), // Бронза
    ];

    let mut loyalty_agent = AirdropAgent::new();
    
    loyalty_agent.launch_strategic_airdrop(
        "FodiFood Loyalty Program",
        "FDF-LOYALTY",
        50_000.0,
        loyalty_campaign.clone(),
        AirdropStrategy::Tiered {
            vip: 5.0,      // Gold: 5x multiplier
            premium: 3.0,  // Silver: 3x multiplier
            regular: 1.0,  // Bronze: 1x multiplier
        },
    );

    let campaign_size = 10; // loyalty_campaign.len()
    println!("\n💡 **Marketing ROI Analysis:**");
    println!("   • Total distributed: {:.2} tokens", loyalty_agent.total_distributed);
    println!("   • Cost per customer: {:.2} tokens", 
        loyalty_agent.total_distributed / campaign_size as f64);
    println!("   • Gold customers received: {:.2} tokens each", 
        loyalty_agent.get_user_balance("@customer_gold1"));
    println!("   • Silver customers received: {:.2} tokens each", 
        loyalty_agent.get_user_balance("@customer_silver1"));
    println!("   • Bronze customers received: {:.2} tokens each", 
        loyalty_agent.get_user_balance("@customer_bronze1"));
    
    println!("\n🎯 **Expected Results:**");
    println!("   • Customer retention: +30%");
    println!("   • Average order value: +20%");
    println!("   • Token redemption rate: 70%");
    println!("   • Marketing cost per acquisition: -40%");

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n✨ AI Airdrop Agent Demo Complete!\n");
    println!("🎁 Demonstrated features:");
    println!("   ✅ Simple equal distribution");
    println!("   ✅ Activity-based rewards");
    println!("   ✅ Lottery system (random winners)");
    println!("   ✅ Tiered distribution (VIP/Premium/Regular)");
    println!("   ✅ Multi-campaign management");
    println!("   ✅ Real business case analysis");
    println!("\n🚀 Ready to launch on-chain airdrops with Solana/TON integration!");
    println!("💡 Next steps: Connect to blockchain, add wallet support, automate campaigns\n");
}

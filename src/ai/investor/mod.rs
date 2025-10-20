//! 🏦 AI Investment Copilot - Complete Investment Management System
//!
//! Advanced investment analysis, portfolio management, and AI-driven insights
//! with real-time data feeds, on-chain dividend distribution, and intelligent alerts

pub mod advisor;
pub mod ai_alerter;
pub mod bot;
pub mod data_feed;
pub mod opportunity;
pub mod portfolio;
pub mod reward_vault;
pub mod screener;
pub mod yield_engine;

// Re-export main types
pub use advisor::{InvestmentAdvisor, AllocationStrategy};
pub use ai_alerter::{AIAlerter, InvestmentAlert, WatchlistEntry};
pub use bot::InvestorBot;
pub use data_feed::{DataFeedManager, RealTimeMetrics, MetricAlert};
pub use opportunity::{CompanyMetrics, InvestmentOpportunity};
pub use portfolio::{Position, Portfolio};
pub use reward_vault::{RewardVaultManager, TreasuryVault, DividendDistribution};
pub use screener::{InvestmentScreener, ScreenerWeights};
pub use yield_engine::YieldCalculator;
//! 🚨 AI Alerter - Intelligent Investment Monitoring System
//! 
//! Proactive monitoring with AI-driven alerts for investment opportunities,
//! portfolio changes, market movements, and dividend notifications

use super::data_feed::{DataFeedManager, AlertSeverity};
// use super::portfolio::Portfolio;
// use super::opportunity::CompanyMetrics;
// use crate::ai::control::AIControl; // Placeholder - will be implemented
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use tokio::time::{Duration, Interval};
use chrono::{DateTime, Utc};

/// 👁️ AI-powered investment watchlist and alerting system
pub struct AIAlerter {
    /// Companies being monitored
    watchlist: HashMap<String, WatchlistEntry>,
    /// Alert history (last 1000 alerts)
    alert_history: VecDeque<InvestmentAlert>,
    /// Data feed manager
    data_feed: DataFeedManager,
    /// AI control for analysis (placeholder)
    ai_control: Option<String>, // Placeholder for AIControl
    /// Alert rules configuration
    alert_rules: AlertRules,
    /// Monitoring interval
    monitoring_interval: Duration,
    /// Last full scan time
    last_scan: DateTime<Utc>,
}

/// 📊 Watchlist entry with monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistEntry {
    /// Company symbol
    pub symbol: String,
    /// Company name
    pub name: String,
    /// Current position size (if any)
    pub position_size: f64,
    /// Target allocation
    pub target_allocation: Option<f64>,
    /// Price alerts
    pub price_alerts: Vec<PriceAlert>,
    /// Metric thresholds
    pub metric_thresholds: MetricThresholds,
    /// Alert preferences
    pub alert_preferences: AlertPreferences,
    /// Last alert time
    pub last_alert: Option<DateTime<Utc>>,
    /// Is actively monitored
    pub is_active: bool,
}

/// 💰 Price-based alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceAlert {
    /// Alert type
    pub alert_type: PriceAlertType,
    /// Threshold value
    pub threshold: f64,
    /// Is enabled
    pub enabled: bool,
    /// Last triggered
    pub last_triggered: Option<DateTime<Utc>>,
}

/// Types of price alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriceAlertType {
    /// Price goes above threshold
    PriceAbove(f64),
    /// Price goes below threshold
    PriceBelow(f64),
    /// Price changes by percentage
    PriceChange(f64),
    /// Volume spike
    VolumeSpike(f64),
}

/// 📈 Metric-based alert thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricThresholds {
    /// Revenue growth threshold (%)
    pub revenue_growth_threshold: Option<f64>,
    /// Profit margin threshold (%)
    pub profit_margin_threshold: Option<f64>,
    /// Customer retention threshold (%)
    pub retention_threshold: Option<f64>,
    /// Social sentiment threshold (-1.0 to 1.0)
    pub sentiment_threshold: Option<f64>,
    /// Risk score threshold (0.0 to 1.0)
    pub risk_threshold: Option<f64>,
    /// Runway threshold (months)
    pub runway_threshold: Option<f64>,
}

impl Default for MetricThresholds {
    fn default() -> Self {
        Self {
            revenue_growth_threshold: Some(15.0), // 15% growth
            profit_margin_threshold: Some(10.0),  // 10% margin
            retention_threshold: Some(70.0),      // 70% retention
            sentiment_threshold: Some(0.3),       // Positive sentiment
            risk_threshold: Some(0.7),            // High risk warning
            runway_threshold: Some(6.0),          // 6 months runway warning
        }
    }
}

/// 🔔 Alert delivery preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertPreferences {
    /// Enable real-time alerts
    pub real_time_enabled: bool,
    /// Enable daily summaries
    pub daily_summary_enabled: bool,
    /// Enable weekly reports
    pub weekly_report_enabled: bool,
    /// Minimum alert severity
    pub min_severity: AlertSeverity,
    /// Alert cooldown period (minutes)
    pub cooldown_minutes: u32,
    /// Delivery channels
    pub delivery_channels: Vec<AlertChannel>,
}

impl Default for AlertPreferences {
    fn default() -> Self {
        Self {
            real_time_enabled: true,
            daily_summary_enabled: true,
            weekly_report_enabled: false,
            min_severity: AlertSeverity::Medium,
            cooldown_minutes: 60, // 1 hour cooldown
            delivery_channels: vec![AlertChannel::InApp, AlertChannel::Email],
        }
    }
}

/// Alert delivery channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertChannel {
    InApp,
    Email,
    SMS,
    Telegram,
    Discord,
    Webhook,
}

/// 🚨 Investment alert with AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentAlert {
    /// Alert ID
    pub id: String,
    /// Company symbol
    pub symbol: String,
    /// Alert type
    pub alert_type: InvestmentAlertType,
    /// Alert title
    pub title: String,
    /// Alert message
    pub message: String,
    /// AI analysis and recommendation
    pub ai_analysis: String,
    /// Severity level
    pub severity: AlertSeverity,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Triggered metric values
    pub metric_values: Option<AlertMetricValues>,
    /// Recommended actions
    pub recommended_actions: Vec<String>,
    /// Is delivered
    pub delivered: bool,
}

/// Types of investment alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvestmentAlertType {
    /// Opportunity alert (good time to buy)
    OpportunityAlert,
    /// Growth spike detected
    GrowthSpike,
    /// Revenue increase
    RevenueIncrease,
    /// Profit margin improvement
    MarginImprovement,
    /// Risk increase warning
    RiskWarning,
    /// Sentiment decline
    SentimentDecline,
    /// Dividend payout
    DividendPayout,
    /// Portfolio rebalancing suggestion
    RebalanceAlert,
    /// Market movement
    MarketMovement,
    /// Custom threshold triggered
    CustomThreshold,
}

/// Metric values that triggered the alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertMetricValues {
    pub current_price: Option<f64>,
    pub price_change_24h: Option<f64>,
    pub revenue_growth: Option<f64>,
    pub profit_margin: Option<f64>,
    pub sentiment_score: Option<f64>,
    pub risk_score: Option<f64>,
    pub social_mentions: Option<u32>,
}

/// 📋 Alert configuration rules
#[derive(Debug, Clone)]
pub struct AlertRules {
    /// Minimum time between alerts for same symbol (minutes)
    pub min_alert_interval: u32,
    /// Maximum alerts per day per symbol
    pub max_daily_alerts: u32,
    /// Growth spike threshold (%)
    pub growth_spike_threshold: f64,
    /// Revenue jump threshold (%)
    pub revenue_jump_threshold: f64,
    /// Sentiment change threshold
    pub sentiment_change_threshold: f64,
    /// Risk increase threshold
    pub risk_increase_threshold: f64,
}

impl Default for AlertRules {
    fn default() -> Self {
        Self {
            min_alert_interval: 30,      // 30 minutes
            max_daily_alerts: 10,
            growth_spike_threshold: 20.0, // 20% growth spike
            revenue_jump_threshold: 25.0, // 25% revenue jump
            sentiment_change_threshold: 0.3,
            risk_increase_threshold: 0.2,
        }
    }
}

impl AIAlerter {
    /// Create new AI alerter
    pub fn new(data_feed: DataFeedManager) -> Self {
        Self {
            watchlist: HashMap::new(),
            alert_history: VecDeque::with_capacity(1000),
            data_feed,
            ai_control: None, // Placeholder
            alert_rules: AlertRules::default(),
            monitoring_interval: Duration::from_secs(300), // 5 minutes
            last_scan: Utc::now(),
        }
    }

    /// Add company to watchlist
    pub fn add_to_watchlist(&mut self, symbol: String, name: String, position_size: f64) -> Result<()> {
        tracing::info!("👁️ Adding {} ({}) to watchlist", name, symbol);

        let entry = WatchlistEntry {
            symbol: symbol.clone(),
            name,
            position_size,
            target_allocation: None,
            price_alerts: Vec::new(),
            metric_thresholds: MetricThresholds::default(),
            alert_preferences: AlertPreferences::default(),
            last_alert: None,
            is_active: true,
        };

        self.watchlist.insert(symbol, entry);
        Ok(())
    }

    /// Configure price alerts for a company
    pub fn set_price_alerts(&mut self, symbol: &str, alerts: Vec<PriceAlert>) -> Result<()> {
        if let Some(entry) = self.watchlist.get_mut(symbol) {
            entry.price_alerts = alerts;
            tracing::info!("🔔 Configured {} price alerts for {}", entry.price_alerts.len(), symbol);
        }
        Ok(())
    }

    /// Configure metric thresholds
    pub fn set_metric_thresholds(&mut self, symbol: &str, thresholds: MetricThresholds) -> Result<()> {
        if let Some(entry) = self.watchlist.get_mut(symbol) {
            entry.metric_thresholds = thresholds;
            tracing::info!("📊 Updated metric thresholds for {}", symbol);
        }
        Ok(())
    }

    /// Start monitoring (background task)
    pub async fn start_monitoring(&mut self) -> Result<()> {
        tracing::info!("🚀 Starting AI investment monitoring...");
        
        let mut interval = tokio::time::interval(self.monitoring_interval);
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.scan_for_alerts().await {
                tracing::error!("❌ Monitoring scan failed: {}", e);
                continue;
            }

            // Clean up old alerts (keep last 1000)
            if self.alert_history.len() > 1000 {
                self.alert_history.pop_front();
            }
        }
    }

    /// Perform alert scan
    async fn scan_for_alerts(&mut self) -> Result<()> {
        tracing::debug!("🔍 Scanning for investment alerts...");

        let watchlist_symbols: Vec<String> = self.watchlist.keys().cloned().collect();
        
        for symbol in watchlist_symbols {
            if let Err(e) = self.check_symbol_alerts(&symbol).await {
                tracing::warn!("⚠️ Failed to check alerts for {}: {}", symbol, e);
            }
        }

        // Check for portfolio-wide alerts
        self.check_portfolio_alerts().await?;

        self.last_scan = Utc::now();
        Ok(())
    }

    /// Check alerts for specific symbol
    async fn check_symbol_alerts(&mut self, symbol: &str) -> Result<()> {
        let entry = match self.watchlist.get(symbol) {
            Some(entry) if entry.is_active => entry.clone(),
            _ => return Ok(()),
        };

        // Skip if in cooldown period
        if self.is_in_cooldown(&entry) {
            return Ok(());
        }

        // Get current metrics
        let current_metrics = match self.data_feed.get_metrics(symbol).await? {
            Some(metrics) => metrics,
            None => {
                tracing::warn!("📊 No metrics available for {}", symbol);
                return Ok(());
            }
        };

        // Check various alert conditions
        self.check_growth_alerts(&entry, &current_metrics).await?;
        self.check_revenue_alerts(&entry, &current_metrics).await?;
        self.check_sentiment_alerts(&entry, &current_metrics).await?;
        self.check_risk_alerts(&entry, &current_metrics).await?;
        self.check_price_alerts(&entry, &current_metrics).await?;

        Ok(())
    }

    /// Check for growth spike alerts
    async fn check_growth_alerts(&mut self, entry: &WatchlistEntry, metrics: &super::data_feed::RealTimeMetrics) -> Result<()> {
        // Calculate revenue growth rate (mock calculation)
        let revenue_growth_rate = (metrics.monthly_revenue / 150_000.0 - 1.0) * 100.0; // Base 150k

        if revenue_growth_rate > self.alert_rules.growth_spike_threshold {
            let alert = InvestmentAlert {
                id: format!("growth_{}_{}", entry.symbol, Utc::now().timestamp()),
                symbol: entry.symbol.clone(),
                alert_type: InvestmentAlertType::GrowthSpike,
                title: format!("🚀 {} Revenue Surge!", entry.name),
                message: format!(
                    "{} experiencing strong growth: +{:.1}% revenue increase! Current monthly revenue: ${:.0}",
                    entry.symbol, revenue_growth_rate, metrics.monthly_revenue
                ),
                ai_analysis: self.generate_ai_analysis(&entry.symbol, "growth_spike", revenue_growth_rate).await?,
                severity: if revenue_growth_rate > 30.0 { AlertSeverity::High } else { AlertSeverity::Medium },
                timestamp: Utc::now(),
                metric_values: Some(AlertMetricValues {
                    current_price: None,
                    price_change_24h: None,
                    revenue_growth: Some(revenue_growth_rate),
                    profit_margin: Some(metrics.monthly_profit / metrics.monthly_revenue * 100.0),
                    sentiment_score: Some(metrics.sentiment_score),
                    risk_score: None,
                    social_mentions: Some(metrics.social_mentions),
                }),
                recommended_actions: vec![
                    "Consider increasing position size".to_string(),
                    "Monitor for sustained growth".to_string(),
                    "Check competitor performance".to_string(),
                ],
                delivered: false,
            };

            self.send_alert(alert).await?;
        }

        Ok(())
    }

    /// Check for revenue alerts
    async fn check_revenue_alerts(&mut self, entry: &WatchlistEntry, metrics: &super::data_feed::RealTimeMetrics) -> Result<()> {
        // Check if revenue meets threshold
        if let Some(threshold) = entry.metric_thresholds.revenue_growth_threshold {
            let growth_rate = (metrics.monthly_revenue / 150_000.0 - 1.0) * 100.0;
            
            if growth_rate > threshold {
                let alert = InvestmentAlert {
                    id: format!("revenue_{}_{}", entry.symbol, Utc::now().timestamp()),
                    symbol: entry.symbol.clone(),
                    alert_type: InvestmentAlertType::RevenueIncrease,
                    title: format!("📈 {} Revenue Target Hit!", entry.name),
                    message: format!(
                        "{} revenue growth of {:.1}% exceeds your {:.1}% threshold! Current: ${:.0}/month",
                        entry.symbol, growth_rate, threshold, metrics.monthly_revenue
                    ),
                    ai_analysis: self.generate_ai_analysis(&entry.symbol, "revenue_threshold", growth_rate).await?,
                    severity: AlertSeverity::Medium,
                    timestamp: Utc::now(),
                    metric_values: Some(AlertMetricValues {
                        current_price: None,
                        price_change_24h: None,
                        revenue_growth: Some(growth_rate),
                        profit_margin: Some(metrics.monthly_profit / metrics.monthly_revenue * 100.0),
                        sentiment_score: Some(metrics.sentiment_score),
                        risk_score: None,
                        social_mentions: Some(metrics.social_mentions),
                    }),
                    recommended_actions: vec![
                        "Review allocation strategy".to_string(),
                        "Consider profit taking".to_string(),
                    ],
                    delivered: false,
                };

                self.send_alert(alert).await?;
            }
        }

        Ok(())
    }

    /// Check sentiment alerts
    async fn check_sentiment_alerts(&mut self, entry: &WatchlistEntry, metrics: &super::data_feed::RealTimeMetrics) -> Result<()> {
        if let Some(threshold) = entry.metric_thresholds.sentiment_threshold {
            if metrics.sentiment_score < threshold - 0.2 {
                let alert = InvestmentAlert {
                    id: format!("sentiment_{}_{}", entry.symbol, Utc::now().timestamp()),
                    symbol: entry.symbol.clone(),
                    alert_type: InvestmentAlertType::SentimentDecline,
                    title: format!("⚠️ {} Sentiment Decline", entry.name),
                    message: format!(
                        "{} social sentiment dropped to {:.2} (below {:.2} threshold). {} mentions with {:.1}% engagement",
                        entry.symbol, metrics.sentiment_score, threshold,
                        metrics.social_mentions, metrics.engagement_rate * 100.0
                    ),
                    ai_analysis: self.generate_ai_analysis(&entry.symbol, "sentiment_decline", metrics.sentiment_score).await?,
                    severity: AlertSeverity::Medium,
                    timestamp: Utc::now(),
                    metric_values: Some(AlertMetricValues {
                        current_price: None,
                        price_change_24h: None,
                        revenue_growth: None,
                        profit_margin: None,
                        sentiment_score: Some(metrics.sentiment_score),
                        risk_score: None,
                        social_mentions: Some(metrics.social_mentions),
                    }),
                    recommended_actions: vec![
                        "Monitor news and reviews".to_string(),
                        "Consider reducing position".to_string(),
                        "Wait for sentiment recovery".to_string(),
                    ],
                    delivered: false,
                };

                self.send_alert(alert).await?;
            }
        }

        Ok(())
    }

    /// Check risk alerts
    async fn check_risk_alerts(&mut self, entry: &WatchlistEntry, metrics: &super::data_feed::RealTimeMetrics) -> Result<()> {
        // Calculate risk score based on runway and cash flow
        let risk_score = if metrics.runway_months < 6.0 {
            0.9 // Very high risk
        } else if metrics.cash_flow < 0.0 {
            0.7 // High risk
        } else {
            0.3 // Medium risk
        };

        if let Some(threshold) = entry.metric_thresholds.risk_threshold {
            if risk_score > threshold {
                let alert = InvestmentAlert {
                    id: format!("risk_{}_{}", entry.symbol, Utc::now().timestamp()),
                    symbol: entry.symbol.clone(),
                    alert_type: InvestmentAlertType::RiskWarning,
                    title: format!("🚨 {} Risk Alert", entry.name),
                    message: format!(
                        "{} risk increased! Runway: {:.1} months, Cash flow: ${:.0}, Burn rate: ${:.0}/month",
                        entry.symbol, metrics.runway_months, metrics.cash_flow, metrics.burn_rate
                    ),
                    ai_analysis: self.generate_ai_analysis(&entry.symbol, "risk_warning", risk_score).await?,
                    severity: AlertSeverity::High,
                    timestamp: Utc::now(),
                    metric_values: Some(AlertMetricValues {
                        current_price: None,
                        price_change_24h: None,
                        revenue_growth: None,
                        profit_margin: None,
                        sentiment_score: None,
                        risk_score: Some(risk_score),
                        social_mentions: None,
                    }),
                    recommended_actions: vec![
                        "Consider reducing exposure".to_string(),
                        "Monitor cash flow closely".to_string(),
                        "Review financial reports".to_string(),
                    ],
                    delivered: false,
                };

                self.send_alert(alert).await?;
            }
        }

        Ok(())
    }

    /// Check price alerts
    async fn check_price_alerts(&mut self, entry: &WatchlistEntry, metrics: &super::data_feed::RealTimeMetrics) -> Result<()> {
        // Mock price calculation based on performance
        let estimated_price = match entry.symbol.as_str() {
            "FDF-SEA" => 2.45 * (1.0 + (metrics.monthly_revenue / 200_000.0 - 1.0)),
            "FDF-TRK" => 1.15 * (1.0 + (metrics.monthly_revenue / 150_000.0 - 1.0)),
            _ => 1.0,
        };

        for price_alert in &entry.price_alerts {
            if !price_alert.enabled {
                continue;
            }

            let should_trigger = match &price_alert.alert_type {
                PriceAlertType::PriceAbove(threshold) => estimated_price > *threshold,
                PriceAlertType::PriceBelow(threshold) => estimated_price < *threshold,
                PriceAlertType::PriceChange(threshold) => {
                    // Mock price change calculation
                    let price_change = (estimated_price / 2.0 - 1.0) * 100.0;
                    price_change.abs() > *threshold
                },
                PriceAlertType::VolumeSpike(threshold) => {
                    metrics.social_mentions > (*threshold as u32)
                },
            };

            if should_trigger {
                let alert = InvestmentAlert {
                    id: format!("price_{}_{}", entry.symbol, Utc::now().timestamp()),
                    symbol: entry.symbol.clone(),
                    alert_type: InvestmentAlertType::MarketMovement,
                    title: format!("💰 {} Price Alert", entry.name),
                    message: format!(
                        "{} price movement detected! Estimated price: ${:.2}",
                        entry.symbol, estimated_price
                    ),
                    ai_analysis: self.generate_ai_analysis(&entry.symbol, "price_movement", estimated_price).await?,
                    severity: AlertSeverity::Medium,
                    timestamp: Utc::now(),
                    metric_values: Some(AlertMetricValues {
                        current_price: Some(estimated_price),
                        price_change_24h: Some((estimated_price / 2.0 - 1.0) * 100.0),
                        revenue_growth: None,
                        profit_margin: None,
                        sentiment_score: Some(metrics.sentiment_score),
                        risk_score: None,
                        social_mentions: Some(metrics.social_mentions),
                    }),
                    recommended_actions: vec![
                        "Review price target".to_string(),
                        "Consider position adjustment".to_string(),
                    ],
                    delivered: false,
                };

                self.send_alert(alert).await?;
            }
        }

        Ok(())
    }

    /// Check portfolio-wide alerts
    async fn check_portfolio_alerts(&mut self) -> Result<()> {
        // Implementation for portfolio-level alerts
        // e.g., rebalancing suggestions, overall risk assessment
        Ok(())
    }

    /// Generate AI analysis for alert
    async fn generate_ai_analysis(&self, symbol: &str, alert_type: &str, value: f64) -> Result<String> {
        // Mock AI analysis (in production, would use actual AI service)
        let analysis = match alert_type {
            "growth_spike" => format!(
                "Strong growth momentum detected for {}. Revenue increase of {:.1}% indicates positive market response and business expansion. Consider increasing allocation if fundamentals support sustained growth.",
                symbol, value
            ),
            "revenue_threshold" => format!(
                "{} revenue growth of {:.1}% exceeds expectations. This suggests effective operations and market demand. Monitor for consistency over next quarters.",
                symbol, value
            ),
            "sentiment_decline" => format!(
                "Social sentiment for {} dropped to {:.2}. This may indicate temporary market reaction or operational challenges. Review recent news and consider temporary position reduction.",
                symbol, value
            ),
            "risk_warning" => format!(
                "{} risk score increased to {:.2}. Monitor cash flow, runway, and operational metrics closely. Consider reducing exposure until risk factors improve.",
                symbol, value
            ),
            "price_movement" => format!(
                "{} price movement to ${:.2} suggests market recognition of value changes. Evaluate if price reflects current fundamentals and adjust position accordingly.",
                symbol, value
            ),
            _ => format!(
                "AI analysis: {} shows {} pattern with value {:.2}. Monitor closely for continued trends.",
                symbol, alert_type, value
            ),
        };

        Ok(analysis)
    }

    /// Send alert through configured channels
    async fn send_alert(&mut self, mut alert: InvestmentAlert) -> Result<()> {
        tracing::info!("🚨 Sending alert: {} - {}", alert.title, alert.message);

        // Add to history
        self.alert_history.push_back(alert.clone());

        // Update last alert time
        if let Some(entry) = self.watchlist.get_mut(&alert.symbol) {
            entry.last_alert = Some(alert.timestamp);
        }

        // Send through configured channels
        self.deliver_alert(&mut alert).await?;

        Ok(())
    }

    /// Deliver alert through various channels
    async fn deliver_alert(&self, alert: &mut InvestmentAlert) -> Result<()> {
        // In real implementation, send through:
        // - Push notifications
        // - Email
        // - Telegram bot
        // - Discord webhook
        // - In-app notifications

        // Mock delivery
        println!("🔔 ALERT: {}", alert.title);
        println!("   📊 {}", alert.message);
        println!("   🤖 AI: {}", alert.ai_analysis);
        println!("   💡 Actions: {:?}", alert.recommended_actions);
        println!();

        alert.delivered = true;
        Ok(())
    }

    /// Check if entry is in cooldown period
    fn is_in_cooldown(&self, entry: &WatchlistEntry) -> bool {
        if let Some(last_alert) = entry.last_alert {
            let cooldown_duration = chrono::Duration::minutes(entry.alert_preferences.cooldown_minutes as i64);
            Utc::now() - last_alert < cooldown_duration
        } else {
            false
        }
    }

    /// Get alert history for symbol
    pub fn get_alert_history(&self, symbol: &str) -> Vec<&InvestmentAlert> {
        self.alert_history
            .iter()
            .filter(|alert| alert.symbol == symbol)
            .collect()
    }

    /// Get recent alerts (last 24 hours)
    pub fn get_recent_alerts(&self) -> Vec<&InvestmentAlert> {
        let cutoff = Utc::now() - chrono::Duration::hours(24);
        self.alert_history
            .iter()
            .filter(|alert| alert.timestamp > cutoff)
            .collect()
    }

    /// Create dividend payout alert
    pub fn create_dividend_alert(&mut self, symbol: &str, amount: f64, tx_signature: &str) -> Result<()> {
        if let Some(entry) = self.watchlist.get(symbol) {
            let alert = InvestmentAlert {
                id: format!("dividend_{}_{}", symbol, Utc::now().timestamp()),
                symbol: symbol.to_string(),
                alert_type: InvestmentAlertType::DividendPayout,
                title: format!("💰 {} Dividend Received!", entry.name),
                message: format!(
                    "Dividend payout from {}: +${:.2} deposited to your wallet. Transaction: {}",
                    symbol, amount, &tx_signature[..8]
                ),
                ai_analysis: format!(
                    "Successful dividend distribution from {}. This indicates healthy cash flow and investor-friendly management.",
                    symbol
                ),
                severity: AlertSeverity::Low,
                timestamp: Utc::now(),
                metric_values: None,
                recommended_actions: vec![
                    "Consider reinvestment".to_string(),
                    "Review yield performance".to_string(),
                ],
                delivered: false,
            };

            // Add to history but don't use normal delivery (this is immediate)
            self.alert_history.push_back(alert.clone());
            
            println!("💰 DIVIDEND ALERT: {}", alert.title);
            println!("   💸 {}", alert.message);
            println!();
        }

        Ok(())
    }

    /// Get watchlist summary
    pub fn get_watchlist_summary(&self) -> WatchlistSummary {
        let total_companies = self.watchlist.len();
        let active_companies = self.watchlist.values().filter(|e| e.is_active).count();
        let total_alerts_24h = self.get_recent_alerts().len();
        
        let last_scan_minutes = (Utc::now() - self.last_scan).num_minutes();

        WatchlistSummary {
            total_companies,
            active_companies,
            total_alerts_24h,
            last_scan_minutes,
            monitoring_active: true,
        }
    }
}

/// Watchlist summary information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistSummary {
    pub total_companies: usize,
    pub active_companies: usize,
    pub total_alerts_24h: usize,
    pub last_scan_minutes: i64,
    pub monitoring_active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_watchlist_creation() {
        let data_feed = DataFeedManager::new();
        let mut alerter = AIAlerter::new(data_feed);

        alerter.add_to_watchlist(
            "FDF-SEA".to_string(),
            "Seafood Paradise".to_string(),
            1000.0
        ).unwrap();

        assert_eq!(alerter.watchlist.len(), 1);
        assert!(alerter.watchlist.contains_key("FDF-SEA"));
    }

    #[tokio::test]
    async fn test_price_alerts() {
        let data_feed = DataFeedManager::new();
        let mut alerter = AIAlerter::new(data_feed);

        alerter.add_to_watchlist(
            "FDF-TEST".to_string(),
            "Test Company".to_string(),
            500.0
        ).unwrap();

        let price_alerts = vec![
            PriceAlert {
                alert_type: PriceAlertType::PriceAbove(3.0),
                threshold: 3.0,
                enabled: true,
                last_triggered: None,
            }
        ];

        alerter.set_price_alerts("FDF-TEST", price_alerts).unwrap();

        let entry = alerter.watchlist.get("FDF-TEST").unwrap();
        assert_eq!(entry.price_alerts.len(), 1);
    }
}
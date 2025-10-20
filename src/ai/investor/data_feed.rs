//! 📊 Data Feed - Real-time Metrics Integration
//! 
//! Connects to Business Brain, Growth Engine, and other data sources
//! to provide real-time company metrics for investment analysis

use super::opportunity::CompanyMetrics;
use crate::api::go_backend::GoBackendClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::{Duration, Instant};

/// 📡 Real-time metrics from various data sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeMetrics {
    pub symbol: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    // Business Brain metrics
    pub monthly_revenue: f64,
    pub monthly_profit: f64,
    pub customer_count: u32,
    pub order_frequency: f64,
    
    // Growth Engine metrics
    pub campaign_roi: f64,
    pub ad_spend: f64,
    pub conversion_rate: f64,
    pub cac: f64, // Customer Acquisition Cost
    
    // Social metrics
    pub social_mentions: u32,
    pub sentiment_score: f64, // -1.0 to 1.0
    pub viral_coefficient: f64,
    pub engagement_rate: f64,
    
    // Financial health
    pub cash_flow: f64,
    pub burn_rate: f64,
    pub runway_months: f64,
}

/// 🔄 Data feed manager with caching and auto-refresh
pub struct DataFeedManager {
    /// Cached metrics
    metrics_cache: HashMap<String, (RealTimeMetrics, Instant)>,
    /// Cache TTL
    cache_ttl: Duration,
    /// Backend client
    backend_client: Option<GoBackendClient>,
    /// Last update time
    last_full_update: Instant,
}

impl DataFeedManager {
    /// Create new data feed manager
    pub fn new() -> Self {
        Self {
            metrics_cache: HashMap::new(),
            cache_ttl: Duration::from_secs(300), // 5 minutes
            backend_client: None,
            last_full_update: Instant::now(),
        }
    }

    /// Initialize with backend client
    pub fn with_backend(mut self, client: GoBackendClient) -> Self {
        self.backend_client = Some(client);
        self
    }

    /// Fetch fresh metrics from Business Brain
    pub async fn fetch_metrics_from_brain(&mut self) -> Result<Vec<CompanyMetrics>> {
        tracing::info!("🧠 Fetching metrics from Business Brain...");

        // In real implementation, this would call Business Brain API
        let real_time_data = self.fetch_real_time_data().await?;
        
        let mut company_metrics = Vec::new();
        
        for data in real_time_data {
            let metrics = self.convert_to_company_metrics(data).await?;
            company_metrics.push(metrics);
        }

        tracing::info!("✅ Fetched {} company metrics", company_metrics.len());
        Ok(company_metrics)
    }

    /// Fetch real-time data from all sources
    async fn fetch_real_time_data(&mut self) -> Result<Vec<RealTimeMetrics>> {
        let mut all_metrics = Vec::new();

        // Fetch from Business Brain API
        if let Some(brain_metrics) = self.fetch_from_business_brain().await? {
            all_metrics.extend(brain_metrics);
        }

        // Fetch from Growth Engine API
        if let Some(growth_metrics) = self.fetch_from_growth_engine().await? {
            // Merge with business brain data
            self.merge_growth_data(&mut all_metrics, growth_metrics);
        }

        // Fetch from Social Analytics API
        if let Some(social_metrics) = self.fetch_from_social_analytics().await? {
            // Merge with existing data
            self.merge_social_data(&mut all_metrics, social_metrics);
        }

        // Update cache
        let now = Instant::now();
        for metric in &all_metrics {
            self.metrics_cache.insert(metric.symbol.clone(), (metric.clone(), now));
        }

        Ok(all_metrics)
    }

    /// Fetch from Business Brain API
    async fn fetch_from_business_brain(&self) -> Result<Option<Vec<RealTimeMetrics>>> {
        // Mock implementation - in real app, call actual Business Brain API
        tracing::info!("📊 Calling Business Brain API...");
        
        // Simulate API call
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let mock_data = vec![
            RealTimeMetrics {
                symbol: "FDF-SEA".to_string(),
                timestamp: chrono::Utc::now(),
                monthly_revenue: 185_000.0,
                monthly_profit: 78_000.0,
                customer_count: 2_450,
                order_frequency: 2.3,
                campaign_roi: 0.0, // Will be filled by Growth Engine
                ad_spend: 0.0,
                conversion_rate: 0.0,
                cac: 0.0,
                social_mentions: 0,
                sentiment_score: 0.0,
                viral_coefficient: 0.0,
                engagement_rate: 0.0,
                cash_flow: 45_000.0,
                burn_rate: 12_000.0,
                runway_months: 18.5,
            },
            RealTimeMetrics {
                symbol: "FDF-TRK".to_string(),
                timestamp: chrono::Utc::now(),
                monthly_revenue: 152_000.0,
                monthly_profit: 53_200.0,
                customer_count: 1_890,
                order_frequency: 2.8,
                campaign_roi: 0.0,
                ad_spend: 0.0,
                conversion_rate: 0.0,
                cac: 0.0,
                social_mentions: 0,
                sentiment_score: 0.0,
                viral_coefficient: 0.0,
                engagement_rate: 0.0,
                cash_flow: 38_000.0,
                burn_rate: 15_000.0,
                runway_months: 14.2,
            },
        ];

        Ok(Some(mock_data))
    }

    /// Fetch from Growth Engine API
    async fn fetch_from_growth_engine(&self) -> Result<Option<HashMap<String, GrowthMetrics>>> {
        tracing::info!("🚀 Calling Growth Engine API...");
        
        // Simulate API call
        tokio::time::sleep(Duration::from_millis(150)).await;
        
        let mut growth_data = HashMap::new();
        
        growth_data.insert("FDF-SEA".to_string(), GrowthMetrics {
            campaign_roi: 2.1,
            ad_spend: 8_500.0,
            conversion_rate: 0.034,
            cac: 28.50,
            campaigns_active: 3,
            last_campaign_performance: 1.85,
        });
        
        growth_data.insert("FDF-TRK".to_string(), GrowthMetrics {
            campaign_roi: 1.8,
            ad_spend: 12_000.0,
            conversion_rate: 0.028,
            cac: 35.20,
            campaigns_active: 2,
            last_campaign_performance: 1.95,
        });

        Ok(Some(growth_data))
    }

    /// Fetch from Social Analytics API
    async fn fetch_from_social_analytics(&self) -> Result<Option<HashMap<String, SocialMetrics>>> {
        tracing::info!("📱 Calling Social Analytics API...");
        
        // Simulate API call
        tokio::time::sleep(Duration::from_millis(120)).await;
        
        let mut social_data = HashMap::new();
        
        social_data.insert("FDF-SEA".to_string(), SocialMetrics {
            mentions: 1_250,
            sentiment_score: 0.72,
            viral_coefficient: 1.15,
            engagement_rate: 0.048,
            trending_score: 0.85,
            influencer_mentions: 23,
        });
        
        social_data.insert("FDF-TRK".to_string(), SocialMetrics {
            mentions: 890,
            sentiment_score: 0.68,
            viral_coefficient: 1.22,
            engagement_rate: 0.052,
            trending_score: 0.78,
            influencer_mentions: 15,
        });

        Ok(Some(social_data))
    }

    /// Merge growth engine data
    fn merge_growth_data(&self, metrics: &mut Vec<RealTimeMetrics>, growth_data: HashMap<String, GrowthMetrics>) {
        for metric in metrics.iter_mut() {
            if let Some(growth) = growth_data.get(&metric.symbol) {
                metric.campaign_roi = growth.campaign_roi;
                metric.ad_spend = growth.ad_spend;
                metric.conversion_rate = growth.conversion_rate;
                metric.cac = growth.cac;
            }
        }
    }

    /// Merge social analytics data
    fn merge_social_data(&self, metrics: &mut Vec<RealTimeMetrics>, social_data: HashMap<String, SocialMetrics>) {
        for metric in metrics.iter_mut() {
            if let Some(social) = social_data.get(&metric.symbol) {
                metric.social_mentions = social.mentions;
                metric.sentiment_score = social.sentiment_score;
                metric.viral_coefficient = social.viral_coefficient;
                metric.engagement_rate = social.engagement_rate;
            }
        }
    }

    /// Convert real-time metrics to CompanyMetrics
    async fn convert_to_company_metrics(&self, data: RealTimeMetrics) -> Result<CompanyMetrics> {
        // Calculate growth rates (compare with previous period)
        let (sales_growth, orders_growth) = self.calculate_growth_rates(&data).await;
        
        // Calculate retention from customer metrics
        let retention = self.calculate_retention(&data);
        
        // Calculate margin
        let margin = if data.monthly_revenue > 0.0 {
            data.monthly_profit / data.monthly_revenue
        } else {
            0.0
        };
        
        // Calculate risk score
        let risk = self.calculate_risk_score(&data);
        
        // Convert social metrics
        let social_momentum = (data.sentiment_score + 1.0) / 2.0 * data.engagement_rate * 10.0;
        
        // Get current price (mock for now)
        let price = self.get_current_price(&data.symbol);

        Ok(CompanyMetrics::new(data.symbol.clone(), self.get_company_name(&data.symbol), price)
            .with_growth(1.0 + sales_growth, 1.0 + orders_growth)
            .with_financials(margin, data.campaign_roi)
            .with_operations(retention, risk)
            .with_social(social_momentum.min(1.0)))
    }

    /// Calculate growth rates compared to previous period
    async fn calculate_growth_rates(&self, data: &RealTimeMetrics) -> (f64, f64) {
        // In real implementation, compare with historical data
        // For now, derive from current metrics
        let base_growth = data.viral_coefficient - 1.0;
        let sales_growth = base_growth * 0.8;
        let orders_growth = base_growth * 1.2;
        
        (sales_growth.max(-0.5).min(1.0), orders_growth.max(-0.5).min(1.0))
    }

    /// Calculate customer retention rate
    fn calculate_retention(&self, data: &RealTimeMetrics) -> f64 {
        // Use order frequency and engagement as proxy for retention
        let base_retention = 0.6; // Base retention rate
        let frequency_bonus = (data.order_frequency - 1.0) * 0.1;
        let engagement_bonus = data.engagement_rate * 2.0;
        
        (base_retention + frequency_bonus + engagement_bonus).min(0.95).max(0.2)
    }

    /// Calculate risk score based on financial health
    fn calculate_risk_score(&self, data: &RealTimeMetrics) -> f64 {
        let mut risk: f64 = 0.5; // Base medium risk
        
        // Runway risk
        if data.runway_months < 6.0 {
            risk += 0.3;
        } else if data.runway_months < 12.0 {
            risk += 0.1;
        }
        
        // Cash flow risk
        if data.cash_flow < 0.0 {
            risk += 0.2;
        }
        
        // Burn rate risk
        if data.burn_rate > data.monthly_revenue * 0.5 {
            risk += 0.15;
        }
        
        risk.min(0.9).max(0.1)
    }

    /// Get current token price (mock)
    fn get_current_price(&self, symbol: &str) -> f64 {
        match symbol {
            "FDF-SEA" => 2.45, // Slightly up from demo
            "FDF-TRK" => 1.15,
            "FDF-PIZ" => 1.68,
            "FDF-BAR" => 3.25,
            "FDF-VIP" => 0.82,
            _ => 1.00,
        }
    }

    /// Get company display name
    fn get_company_name(&self, symbol: &str) -> String {
        match symbol {
            "FDF-SEA" => "Seafood Paradise".to_string(),
            "FDF-TRK" => "Street Fusion Truck".to_string(),
            "FDF-PIZ" => "Artisan Pizza Corner".to_string(),
            "FDF-BAR" => "Mixology Lounge".to_string(),
            "FDF-VIP" => "Elite Members Club".to_string(),
            _ => symbol.to_string(),
        }
    }

    /// Get cached metrics or fetch fresh if expired
    pub async fn get_metrics(&mut self, symbol: &str) -> Result<Option<RealTimeMetrics>> {
        let now = Instant::now();
        
        // Check cache first
        if let Some((metrics, cached_at)) = self.metrics_cache.get(symbol) {
            if now.duration_since(*cached_at) < self.cache_ttl {
                return Ok(Some(metrics.clone()));
            }
        }
        
        // Fetch fresh data
        let fresh_data = self.fetch_real_time_data().await?;
        
        // Find the requested symbol
        for data in fresh_data {
            if data.symbol == symbol {
                return Ok(Some(data));
            }
        }
        
        Ok(None)
    }

    /// Check if any significant changes occurred (for alerts)
    pub async fn detect_significant_changes(&mut self) -> Result<Vec<MetricAlert>> {
        let mut alerts = Vec::new();
        let current_data = self.fetch_real_time_data().await?;
        
        for current in current_data {
            if let Some((previous, _)) = self.metrics_cache.get(&current.symbol) {
                alerts.extend(self.compare_metrics(previous, &current));
            }
        }
        
        Ok(alerts)
    }

    /// Compare metrics and generate alerts
    fn compare_metrics(&self, previous: &RealTimeMetrics, current: &RealTimeMetrics) -> Vec<MetricAlert> {
        let mut alerts = Vec::new();
        
        // Revenue change
        let revenue_change = (current.monthly_revenue - previous.monthly_revenue) / previous.monthly_revenue;
        if revenue_change.abs() > 0.15 { // 15% change
            alerts.push(MetricAlert {
                symbol: current.symbol.clone(),
                alert_type: AlertType::RevenueChange,
                message: format!(
                    "{} revenue {}: {:+.1}% (${:.0} -> ${:.0})",
                    current.symbol,
                    if revenue_change > 0.0 { "surge" } else { "drop" },
                    revenue_change * 100.0,
                    previous.monthly_revenue,
                    current.monthly_revenue
                ),
                severity: if revenue_change.abs() > 0.25 { AlertSeverity::High } else { AlertSeverity::Medium },
                timestamp: current.timestamp,
            });
        }
        
        // Social sentiment change
        let sentiment_change = current.sentiment_score - previous.sentiment_score;
        if sentiment_change.abs() > 0.2 { // 20% sentiment change
            alerts.push(MetricAlert {
                symbol: current.symbol.clone(),
                alert_type: AlertType::SentimentChange,
                message: format!(
                    "{} sentiment {}: {:.2} -> {:.2}",
                    current.symbol,
                    if sentiment_change > 0.0 { "improved" } else { "declined" },
                    previous.sentiment_score,
                    current.sentiment_score
                ),
                severity: AlertSeverity::Low,
                timestamp: current.timestamp,
            });
        }
        
        // Runway warning
        if current.runway_months < 6.0 && previous.runway_months >= 6.0 {
            alerts.push(MetricAlert {
                symbol: current.symbol.clone(),
                alert_type: AlertType::RunwayWarning,
                message: format!(
                    "{} runway critical: {:.1} months remaining",
                    current.symbol,
                    current.runway_months
                ),
                severity: AlertSeverity::Critical,
                timestamp: current.timestamp,
            });
        }
        
        alerts
    }

    /// Force refresh all metrics
    pub async fn force_refresh(&mut self) -> Result<()> {
        tracing::info!("🔄 Force refreshing all metrics...");
        self.fetch_real_time_data().await?;
        self.last_full_update = Instant::now();
        Ok(())
    }
}

impl Default for DataFeedManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 🚀 Growth Engine metrics
#[derive(Debug, Clone)]
struct GrowthMetrics {
    pub campaign_roi: f64,
    pub ad_spend: f64,
    pub conversion_rate: f64,
    pub cac: f64,
    pub campaigns_active: u32,
    pub last_campaign_performance: f64,
}

/// 📱 Social Analytics metrics  
#[derive(Debug, Clone)]
struct SocialMetrics {
    pub mentions: u32,
    pub sentiment_score: f64,
    pub viral_coefficient: f64,
    pub engagement_rate: f64,
    pub trending_score: f64,
    pub influencer_mentions: u32,
}

/// 🚨 Metric change alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAlert {
    pub symbol: String,
    pub alert_type: AlertType,
    pub message: String,
    pub severity: AlertSeverity,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Alert types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    RevenueChange,
    ProfitChange,
    SentimentChange,
    RunwayWarning,
    CampaignPerformance,
    SocialSpike,
    RiskIncrease,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_data_feed_manager() {
        let mut manager = DataFeedManager::new();
        let metrics = manager.fetch_metrics_from_brain().await.unwrap();
        
        assert!(!metrics.is_empty());
        assert!(metrics[0].sales_growth_30d >= 1.0); // Should be growth multiplier
    }

    #[tokio::test]
    async fn test_metric_alerts() {
        let mut manager = DataFeedManager::new();
        
        // Fetch initial data to populate cache
        let _ = manager.fetch_real_time_data().await.unwrap();
        
        // Fetch again to trigger comparison
        let alerts = manager.detect_significant_changes().await.unwrap();
        
        // First run might not have alerts since cache was empty
        // In real usage, this would detect changes over time
        println!("Generated {} alerts", alerts.len());
    }
}